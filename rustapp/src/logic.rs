use geo::Point;
use geo::prelude::*;
use std::fmt::Display;
use std::fmt;
use nalgebra::{Matrix, DMatrix};
use nalgebra::base::{Scalar, Dim};
use nalgebra::storage::Storage;
use nalgebra::base::Vector;

#[derive(Debug)]
pub struct Problem {
    ordered_events: Vec<Event>
}

pub struct Solution<'a> {
    schedule: Vec<&'a MyPoint>
}

//impl Display for Solution {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "({}, {})", self.schedule)
//    }
//}

#[derive(Debug)]
pub struct Event {
    idx: u64,
    points: Vec<MyPoint>,
}

#[derive(Debug)]
pub struct MyPoint {
    point: Point<f64>,
    idx: u64,
}

struct Path<'a> {
    from_event: &'a Event,
    to_event: &'a Event,
    matrix: DistanceMatrix,
//    middle_point:
}

struct PathList<'a> {
    path: Path<'a>,
    next_node: Box<PathList<'a>>,
}

type Distance = f64;
//pub struct Distance(f64);

type DistanceMatrix = DMatrix<f64>;

type AnswersMatrix = DMatrix<usize>;

//struct DistanceMatrix<'a>(Vec<Vec<&'a Distance>>);


//fn squash<'a>(first: Path<'a>, second: Path<'a>) -> (Path<'a>, u64) {
//    let mut result = Path{
//        from_event: first.from_event,
//        to_event: second.to_event,
//        matrix: DistanceMatrix
//    };
//
//}
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

fn insert_after(old_path: &mut Path, new_segment: Path) {}


fn calculate_distance(p1: &MyPoint, p2: &MyPoint) -> Distance {
    return p1.point.vincenty_distance(&p2.point).unwrap();
}


fn calculate_distance_matrix<'a>(from: &'a Vec<MyPoint>, to: &'a Vec<MyPoint>) -> DistanceMatrix {
    let mut result = DistanceMatrix::zeros(from.len(), to.len());
    for (i, x) in from.iter().enumerate() {
        for (j, y) in to.iter().enumerate() {
            *result.index_mut((0, 0)) = calculate_distance(x, y);
        }
    }
    result
}

fn squash_distances(first: DistanceMatrix, second: DistanceMatrix) -> (DistanceMatrix, AnswersMatrix) {
    let result_shape = (first.shape().0, second.shape().1);
    let mut result_dists = DistanceMatrix::zeros(result_shape.0, result_shape.1);
    let mut result_answers = AnswersMatrix::zeros(result_shape.0, result_shape.1);
    for i in 0..result_shape.0 {
        for j in 0..result_shape.1 {
            let dists = first.row(i) + second.column(j);
            let argmin = dists.argmin();
            *result_answers.index_mut((i, j)) = argmin.0;
            *result_dists.index_mut((i, j)) = argmin.1;
        }
    }

    (result_dists, result_answers)
}


//def squash_distances(matrix1, matrix2):
//    # print("Squashing", matrix1.shape, matrix2.shape)
//    dists = np.zeros((matrix1.shape[0], matrix2.shape[1]), dtype=float)
//    answers = np.zeros((matrix1.shape[0], matrix2.shape[1]), dtype=int)
//    for i in range(dists.shape[0]):
//        for j in range(dists.shape[1]):
//            vector = matrix1[i] + matrix2[:, j]
//            answers[i, j] = vector.argmin()
//            dists[i, j] = vector[answers[i, j]]
//    return dists, answers

pub fn solve_ordered(p: &Problem) -> Solution {
    let mut result = Solution {
        schedule: Vec::with_capacity(p.ordered_events.len())
    };
    let mut answers = Vec::<AnswersMatrix>::new();
    let mut current_dists: Option<DistanceMatrix> = None;

    // Forward pass
    for pairs in p.ordered_events.windows(2) {
        if let [x, y] = pairs {
            let last_dists = calculate_distance_matrix(&x.points, &y.points);
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

    let mut current_dists_reverse_pass = current_dists.unwrap();

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
        result.schedule.push(&p.ordered_events[idx].points[*schedule_item]);
    }

    result
}


pub fn solve_stupid(p: &Problem) -> Solution {
    let mut result = Solution {
        schedule: Vec::with_capacity(p.ordered_events.len())
    };
    for event in p.ordered_events.iter() {
        result.schedule.push(&event.points[0]);
    }
    result
}

pub fn get_sample_problem() -> Problem {
    let sample_point = MyPoint {
        idx: 0,
        point: Point::from((1f64, 2f64)),
    };
    let sample_event = Event {
        idx: 0,
        points: vec![sample_point],
    };
    Problem {
        ordered_events: vec![sample_event]
    }
}