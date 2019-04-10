use nalgebra::{Matrix, DMatrix, RowDVector};
use nalgebra::base::{Scalar, Dim};
use nalgebra::storage::Storage;
use rand::{thread_rng, seq::SliceRandom};
use serde::{Serialize, Deserialize};

use crate::distances::{DistsMethod, DistanceMatrix, calculate_distance};
use crate::events::{MyPoint, Event};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SolveAlgorithm {
    Stupid,
    Ordered,
}


impl Default for SolveAlgorithm {
    fn default() -> Self {
        SolveAlgorithm::Ordered
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Config {
    #[serde(default)]
    dists_method: DistsMethod,
    #[serde(default)]
    solve_algorithm: SolveAlgorithm,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Problem {
    ordered_events: Vec<Event>,
    #[serde(default)]
    config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Solution {
    schedule: Vec<MyPoint>
}

type AnswersMatrix = DMatrix<usize>;


trait MatrixExt<N: Scalar + PartialOrd, R: Dim, C: Dim, S: Storage<N, R, C>> {
    fn matrix_argmin(&self) -> (usize, usize);
    fn matrix_argmax(&self) -> (usize, usize);
}

impl<N: Scalar + PartialOrd, R: Dim, C: Dim, S: Storage<N, R, C>> MatrixExt<N, R, C, S> for Matrix<N, R, C, S> {
    fn matrix_argmin(&self) -> (usize, usize) {
        let mut result_value = self.index((0, 0));
        let mut result = (0usize, 0usize);
        for pos_x in 0..self.nrows() {
            for pos_y in 0..self.ncols() {
                let value = self.index((pos_x, pos_y));
                if value < result_value {
                    result_value = value;
                    result = (pos_x, pos_y);
                }
            }
        }
        result
    }
    fn matrix_argmax(&self) -> (usize, usize) {
        let mut result_value = self.index((0, 0));
        let mut result = (0usize, 0usize);
        for pos_x in 0..self.nrows() {
            for pos_y in 0..self.ncols() {
                let value = self.index((pos_x, pos_y));
                if value > result_value {
                    result_value = value;
                    result = (pos_x, pos_y);
                }
            }
        }
        result
    }
}


fn squash_distances(first: DistanceMatrix, second: DistanceMatrix) -> (DistanceMatrix, AnswersMatrix) {
    let result_shape = (first.shape().0, second.shape().1);
    let mut result_dists = DistanceMatrix::zeros(result_shape.0, result_shape.1);
    let mut result_answers = AnswersMatrix::zeros(result_shape.0, result_shape.1);
    for i in 0..result_shape.0 {
        for j in 0..result_shape.1 {
            let dists: RowDVector<f64> = first.row(i) + second.transpose().row(j);
            let argmin = dists.matrix_argmin();
            *result_answers.index_mut((i, j)) = argmin.1;
            *result_dists.index_mut((i, j)) = dists.index(argmin).clone();
        }
    }

    (result_dists, result_answers)
}


fn sample_any(event: &Event) -> &MyPoint {
    let mut rng = thread_rng();
    event.points.choose(&mut rng).unwrap()
}

pub fn solve_ordered(p: &Problem) -> Solution {
    let mut result = Solution {
        schedule: Vec::with_capacity(p.ordered_events.len())
    };
    if p.ordered_events.len() == 0 {
        return result;
    }
    if p.ordered_events.len() == 1 {
        result.schedule.push(sample_any(&p.ordered_events[0]).clone());
        return result;
    }


    let mut answers = Vec::<AnswersMatrix>::new();
    let mut current_dists: Option<DistanceMatrix> = None;

    // Forward pass
    for pairs in p.ordered_events.windows(2) {
        if let [x, y] = pairs {
            let last_dists = calculate_distance(p.config.dists_method, &x.points, &y.points);
            current_dists = Some(match current_dists {
                None => last_dists,
                Some(prev_dists) => {
                    let (new_dists, answer) = squash_distances(prev_dists, last_dists);
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

    let (start, end) = current_dists_reverse_pass.matrix_argmin();
    let mut reverted_schedule_idxs = vec![end];

    let mut current_point = end;

    for current_answer in answers.iter().rev() {
        let prev_point = current_answer.index((start, current_point)).clone();
        reverted_schedule_idxs.push(prev_point.clone());
        current_point = prev_point;
    }

    reverted_schedule_idxs.push(start);

    for (idx, schedule_item) in reverted_schedule_idxs.iter().rev().enumerate() {
        result.schedule.push(p.ordered_events[idx].points[*schedule_item].clone());
    }

    result
}


pub fn solve_stupid(p: &Problem) -> Solution {
    let mut result = Solution {
        schedule: Vec::with_capacity(p.ordered_events.len())
    };
    for event in p.ordered_events.iter() {
        result.schedule.push(event.points[0].clone());
    }
    result
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn get_sample_problem() -> Problem {
        let sample_point = MyPoint {
            idx: 0,
            coords: (1f64, 2f64),
        };
        let sample_event = Event {
            idx: 0,
            points: vec![sample_point],
        };
        Problem {
            ordered_events: vec![sample_event],
            config: Config::default(),
        }
    }


    #[test]
    fn test_sample_problem() {
        let p = get_sample_problem();
        assert_eq!(p.ordered_events.len(), 1);
    }

    #[test]
    fn test_stupid_solution() {
        let p = get_sample_problem();
        let s = solve_stupid(&p);
        assert_eq!(s.schedule.len(), p.ordered_events.len());
    }


    #[test]
    fn test_ordered_solution() {
        let p = get_sample_problem();
        let s = solve_ordered(&p);
        assert_eq!(s.schedule.len(), p.ordered_events.len());
    }

    #[test]
    fn test_serialize() {
        let p = get_sample_problem();
        let serialized = serde_json::to_string(&p).unwrap();
//        println!("serialized = {}", serialized);
        assert_eq!(serialized, r#"{"ordered_events":[{"idx":0,"points":[{"coords":[1.0,2.0],"idx":0}]}],"config":{"dists_method":"dummy","solve_algorithm":"ordered"}}"#);
//        assert_eq!(s.schedule.len(), p.ordered_events.len());
    }
}