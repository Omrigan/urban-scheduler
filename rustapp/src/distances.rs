

use serde::{Serialize, Deserialize};
use geo::prelude::*;

use nalgebra::DMatrix;
use crate::events::{MyPoint};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DistsMethod {
    Dummy,
    OSRM,
}

pub type Distance = f64;
pub type DistanceMatrix = DMatrix<Distance>;


impl Default for DistsMethod {
    fn default() -> Self {
        DistsMethod::Dummy
    }
}


fn calculate_distance_point(p1: &MyPoint, p2: &MyPoint) -> Distance {
    return p1.get_point().euclidean_distance(&p2.get_point());
}


fn calculate_distance_euclid(from: &Vec<MyPoint>, to: &Vec<MyPoint>) -> DistanceMatrix {
    let mut result = DistanceMatrix::zeros(from.len(), to.len());
    for (i, x) in from.iter().enumerate() {
        for (j, y) in to.iter().enumerate() {
            *result.index_mut((i, j)) = calculate_distance_point(x, y);
        }
    }
    result
}




pub fn calculate_distance(method: DistsMethod, from: &Vec<MyPoint>, to: &Vec<MyPoint>) -> DistanceMatrix {
    match method {
        DistsMethod::Dummy => calculate_distance_euclid(from, to),
        DistsMethod::OSRM => panic!("Not implemented yet")
    }
}