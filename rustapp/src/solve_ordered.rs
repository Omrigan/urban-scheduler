use ndarray_stats::QuantileExt;

use crate::distances::{DistanceMatrix, AnswersMatrix,
                       calculate_distance, squash_distances};
use crate::problem::{Problem, ScheduleItem};



pub fn solve_ordered(p: &Problem) -> Vec<ScheduleItem> {
    let mut result = Vec::with_capacity(p.events.len());

    let mut answers = Vec::<AnswersMatrix>::new();
    let mut current_dists: Option<DistanceMatrix> = None;

    // Forward pass
    for pairs in p.events.windows(2) {
        if let [x, y] = pairs {
            let last_dists = calculate_distance(p.config.dists_method, &x.points, &y.points);
            current_dists = Some(match current_dists {
                None => last_dists,
                Some(prev_dists) => {
                    let (new_dists, answer) = squash_distances(&prev_dists, &last_dists);
                    answers.push(answer);
                    new_dists
                }
            });
        } else {
            panic!("Impossible");
        }
    }

    // Backward pass

    let current_dists_reverse_pass = current_dists.unwrap();

    let (start, end) = current_dists_reverse_pass.argmin().unwrap();
    let mut reverted_schedule_idxs = vec![end];

    let mut current_point = end;

    for current_answer in answers.iter().rev() {
        let prev_point = current_answer[(start, current_point)];
        reverted_schedule_idxs.push(prev_point.clone());
        current_point = prev_point;
    }

    reverted_schedule_idxs.push(start);

    for (idx, schedule_item) in reverted_schedule_idxs.iter().rev().enumerate() {
        let event = &p.events[idx];
        result.push(ScheduleItem::construct(event,
                                            event.points[*schedule_item].clone()));
    }

    result
}


pub fn solve_stupid(p: &Problem) -> Vec<ScheduleItem> {
    let mut schedule = Vec::with_capacity(p.events.len());

    for event in p.events.iter() {
        schedule.push(ScheduleItem::construct(event, event.points[0].clone()));
    }

    schedule
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;

    use crate::problem::{Config, Event};
    use crate::distances::MyPoint;

    use bit_set::BitSet;


    #[test]
    fn test_sample_problem() {
        let p = sample_ordered();
        assert_eq!(p.events.len(), 2);
    }

    #[test]
    fn test_stupid_solution() {
        let p = sample_ordered();
        let s = solve_stupid(&p);
        assert_eq!(s.len(), p.events.len());
    }


    #[test]
    fn test_ordered_solution() {
        let p = sample_ordered();
        let s = solve_ordered(&p);
        assert_eq!(s.len(), p.events.len());
    }

    #[test]
    fn test_serialize() {
//        let p = get_sample_problem();
//        let serialized = serde_json::to_string(&p).unwrap();
//        assert_eq!(serialized, r#"{"events":[{"idx":0,"points":[{"coords":[1.0,2.0],"idx":0}]}],"config":{"dists_method":"dummy","solve_algorithm":"ordered"}}"#);
    }
}