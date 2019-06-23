use crate::distances::{DistanceMatrix, AnswersMatrix, squash_distances, calculate_distance};
use crate::problem::{Problem, Solution, Event, Config};

use bit_set::BitSet;
use ndarray_stats::QuantileExt;

use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;



#[derive(Debug)]
struct SearchTree<'s> {
    problem: &'s Problem,
    heap: BinaryHeap<&'s mut Node<'s>>,
    best: Option<&'s Node<'s>>
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

    fn recover_answer(&self) -> Option<Solution> {
        let node = self.best?;
        let mut result = Solution {
            schedule: Vec::with_capacity(self.problem.events.len()),
            full_route: None,
        };

        let last_dists = node.meta.last_distances.as_ref();

        let current_dists_reverse_pass = match last_dists {
            None => panic!("oops"),
            Some(x) => x
        };


        let (start, end) = current_dists_reverse_pass.argmin().unwrap();
        let mut reverted_schedule_idxs = vec![end];

        let mut current_point = end;

        let mut current_meta = node.meta.parent.unwrap();

        while let Some(current_answer) = &current_meta.last_answers {
            let prev_point = current_answer[(start, current_point)];
            reverted_schedule_idxs.push(prev_point.clone());
            current_point = prev_point;

            current_meta = current_meta.parent.unwrap();
        }

        reverted_schedule_idxs.push(start);
        dbg!(&reverted_schedule_idxs);

        for (idx, schedule_item) in reverted_schedule_idxs.iter().rev().enumerate() {
            result.schedule.push(self.problem.events[idx].points[*schedule_item].clone());
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

pub fn solve_generic(problem: &Problem) -> Option<Solution> {
    let mut st = SearchTree {
        problem,
        heap: BinaryHeap::new(),
        best: None,

    };
    let mut roots =  Vec::new();

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
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::distances::MyPoint;

    fn get_sample_problem() -> Problem {
        let sample_point = MyPoint {
            idx: 0,
            coords: (1f64, 2f64),
        };
        let sample_event = Event {
            idx: 0,
            points: vec![sample_point],
            before: BitSet::new()
        };

        let sample_event2 = Event {
            idx: 1,
            points: vec![sample_point],
            before: BitSet::new()
        };


        Problem {
            events: vec![sample_event, sample_event2],
            config: Config::default(),
        }
    }

    #[test]
    fn test_search() {
        let p = get_sample_problem();
        let result = solve_generic(&p);
        dbg!(&result);
        assert_eq!(result.unwrap().schedule.len(), 2);
    }

//
//    #[test]
//    fn test_stupid_solution() {
//        let p = get_sample_problem();
//        let s = solve_stupid(&p);
//        assert_eq!(s.schedule.len(), p.ordered_events.len());
//    }
//
//
//    #[test]
//    fn test_ordered_solution() {
//        let p = get_sample_problem();
//        let s = solve_ordered(&p);
//        assert_eq!(s.schedule.len(), p.ordered_events.len());
//    }
//
//    #[test]
//    fn test_serialize() {
//        let p = get_sample_problem();
//        let serialized = serde_json::to_string(&p).unwrap();
//        assert_eq!(serialized, r#"{"ordered_events":[{"idx":0,"points":[{"coords":[1.0,2.0],"idx":0}]}],"config":{"dists_method":"dummy","solve_algorithm":"ordered"}}"#);
//    }
}