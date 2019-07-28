use crate::distances::{DistanceMatrix, AnswersMatrix, squash_distances, calculate_distance};
use crate::problem::{Problem, ScheduleItem};

use bit_set::BitSet;
use ndarray_stats::QuantileExt;

use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;


#[derive(Debug)]
struct SearchTree<'s> {
    problem: &'s Problem,
    heap: BinaryHeap<&'s mut Node<'s>>,
    best: Option<&'s Node<'s>>,
}

impl<'s> SearchTree<'s> {
    fn expand_node(&mut self, node: &'s mut Node<'s>) {
        let mut candidates = BitSet::new();
        let total_events = self.problem.events.len();


        let meta = &node.meta;

        if meta.depth == total_events {
            self.best = Some(match self.best {
                None => node,
                Some(prev) => (&*node).min(prev)
            });
            return;
        }
        for (i, pe) in self.problem.events.iter().enumerate() {
            if !meta.visited.contains(i) &&
                pe.before.is_subset(&meta.visited) {
                candidates.insert(i);
            }
        }
        let mut children = Vec::new();
        let depth = meta.depth + 1;

        for i in candidates.iter() {
            let distances = self.calculate_distance(meta.event_idx, i);
            let (last_distances_some, last_answers) = match &meta.last_distances {
                None => (distances, None),
                Some(prev_distances) =>
                    {
                        let (sq_distances,
                            ans) = squash_distances(prev_distances, &distances);

                        (sq_distances, Some(ans))
                    }
            };


            let best_path: f64 = *last_distances_some.min().unwrap();
            let heuristic = best_path / depth as f64;

            let mut visited = meta.visited.clone();
            visited.insert(i);

            let newmeta = Meta {
                event_idx: i,
                last_distances: Some(last_distances_some),
                parent: Some(&meta),
                last_answers,
                best_path,
                heuristic,
                depth,
                visited,
            };

            let child = Node {
                meta: newmeta,
                children: Vec::new(),
            };
            children.push(child);
        }
        node.children = children;

        self.heap.extend(node.children.iter_mut());
    }
    fn calculate_distance(&self, from: usize, to: usize) -> DistanceMatrix {
        calculate_distance(self.problem.config.dists_method,
                           &self.problem.events[from].points,
                           &self.problem.events[to].points)
    }

    fn recover_answer(&self) -> Option<Vec<ScheduleItem>> {
        let node = self.best?;
        let mut result = Vec::with_capacity(self.problem.events.len());

        let last_dists = node.meta.last_distances.as_ref();

        let current_dists_reverse_pass = match last_dists {
            None => panic!("oops"),
            Some(x) => x
        };


        let (start, end) = current_dists_reverse_pass.argmin().unwrap();

        let mut current_point = end;
        let mut current_meta = &node.meta;
        let mut reverted_schedule_points = vec![end];
        let mut reverted_schedule_events = vec![current_meta.event_idx];

        while let Some(current_answer) = &current_meta.last_answers {
            current_point = current_answer[(start, current_point)];
            current_meta = current_meta.parent.unwrap();


            reverted_schedule_points.push(current_point);
            reverted_schedule_events.push(current_meta.event_idx);
        }

        reverted_schedule_points.push(start);
        current_meta = current_meta.parent.unwrap();

        reverted_schedule_events.push(current_meta.event_idx);

        dbg!(&reverted_schedule_points);

        dbg!(&reverted_schedule_events);

        for (event_idx, point_idx) in reverted_schedule_events.iter().zip(
            reverted_schedule_points.iter()).rev() {
            let event= &self.problem.events[*event_idx];
            result.push(ScheduleItem::construct(event, event.points[*point_idx].clone()));
        }

        Some(result)
    }
}

#[derive(Debug)]
struct Meta<'s> {
    event_idx: usize,
    depth: usize,
    last_distances: Option<DistanceMatrix>,
    last_answers: Option<AnswersMatrix>,
    heuristic: f64,
    best_path: f64,
    visited: BitSet,
    parent: Option<&'s Meta<'s>>,
}


#[derive(Debug)]
struct Node<'s> {
    children: Vec<Node<'s>>,
    meta: Meta<'s>,
}


impl Ord for Node<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.meta.heuristic.partial_cmp(&other.meta.heuristic).unwrap_or(Equal)
    }
}

impl PartialOrd for Node<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.meta.heuristic == other.meta.heuristic
    }
}

impl Eq for Node<'_> {}

pub fn solve_generic(problem: &Problem) -> Option<Vec<ScheduleItem>> {
    let mut st = SearchTree {
        problem,
        heap: BinaryHeap::new(),
        best: None,

    };
    let mut roots = Vec::new();

    for event in problem.events.iter() {
        if event.before.is_empty() {
            let mut bs = BitSet::new();
            bs.insert(event.idx as usize);
            let meta = Meta {
                event_idx: event.idx as usize,
                last_distances: None,
                parent: None,
                last_answers: None,
                best_path: 0f64,
                heuristic: 0f64,
                depth: 1,
                visited: bs,
            };


            roots.push(Node {
                meta,
                children: Vec::new(),
            });
        }
    }


    st.heap.extend(roots.iter_mut());

    while let Some(head) = st.heap.pop() {
        st.expand_node(head);
    }
    dbg!(&st);
    st.recover_answer()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::distances::MyPoint;
    use crate::problem::{Event, Config};

    #[test]
    fn test_search() {
        let sample_event = Event {
            idx: 0,
            points: vec![MyPoint {
                idx: 0,
                coords: (1f64, 2f64),
            }],
            before: vec![1usize].into_iter().collect(),
            name: None
        };

        let sample_event2 = Event {
            idx: 1,
            points: vec![MyPoint {
                idx: 1,
                coords: (1f64, 2f64),
            }],
            before: BitSet::new(),
            name: None
        };

        let p = Problem {
            events: vec![sample_event, sample_event2],
            config: Config::default(),
            version: 2
        };
        let result = solve_generic(&p).unwrap();
        dbg!(&result);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].point.idx, 1);
        assert_eq!(result[1].point.idx, 0);
    }

    #[test]
    fn test_incorrect() {
        let sample_event = Event {
            idx: 0,
            points: vec![MyPoint {
                idx: 0,
                coords: (1f64, 2f64),
            }],
            before: vec![1usize].into_iter().collect(),
            name: None
        };

        let sample_event2 = Event {
            idx: 1,
            points: vec![MyPoint {
                idx: 1,
                coords: (1f64, 2f64),
            }],
            before: vec![0usize].into_iter().collect(),
            name: None
        };

        let p = Problem {
            events: vec![sample_event, sample_event2],
            config: Config::default(),
            version: 2
        };
        let result = solve_generic(&p);
        assert!(result.is_none());
    }
}