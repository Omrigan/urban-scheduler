use crate::error::{Result};

use std::env;

use serde_json;
use serde::{Serialize, Deserialize};
use ndarray::{Array1, Array2};
use ndarray_stats::QuantileExt;
use geo::Point;
use geo::prelude::*;
use reqwest;


#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MyPoint {
    pub coords: (f64, f64),
    pub idx: u64
}

impl MyPoint {
    pub fn get_point(&self) -> Point<f64> {
        Point::from(self.coords)
    }
    pub fn get_string_repr(&self) -> String {
        format!("{},{}", self.coords.0, self.coords.1)
    }
}


#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DistsMethod {
    Dummy,
    OSRM,
    HERE,
}


pub type Distance = f64;
pub type DistanceMatrix = Array2<Distance>;
pub type AnswersMatrix = Array2<usize>;



impl Default for DistsMethod {
    fn default() -> Self {
        DistsMethod::Dummy
    }
}


fn calculate_distance_euclid(p1: MyPoint, p2: MyPoint) -> Distance {
    return p1.get_point().euclidean_distance(&p2.get_point());
}


//
//fn main() -> Result<(), Box<std::error::Error>> {
//    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")?
//        .json()?;
//    println!("{:#?}", resp);
//    Ok(())
//}

//https://developer.here.com/documentation/routing/topics/request-a-simple-route.html

const SUMMARY: &str = "summary";

pub fn calculate_route_here(points: &[MyPoint], route_attributes: &str) -> Result<serde_json::Value> {
    let here_app_id = env::var("HERE_APP_ID")?;
    let here_app_code = env::var("HERE_APP_CODE")?;

    let waypoint_part: Vec<String> = points.iter().enumerate().map(
        |(i, p)| format!("waypoint{}=geo!{}", i, p.get_string_repr())).collect();

    let url_str = format!("https://route.api.here.com/routing/7.2/calculateroute.json\
    ?app_id={}&app_code={}&{}&&transportMode=publicTransport&mode=fastest;publicTransport;traffic:disabled&routeAttributes={}",
                          here_app_id, here_app_code, waypoint_part.join("&"), route_attributes);
//    let url_str = format!("https://route.api.here.com/routing/7.2/calculateroute.json?alternatives=0&app_code={}&app_id={}&departure=2017-09-19T09:00:00&jsonAttributes=41&language=en_GB&legattributes=all&linkattributes=none,sh,ds,rn,ro,nl,pt,ns,le,fl&maneuverattributes=all&metricSystem=metric&mode=fastest;publicTransport;traffic:disabled;&routeattributes=none,sh,wp,sm,bb,lg,no,li,tx,la&transportMode=publicTransport&walkSpeed=1.4&{}",
//    here_app_code, here_app_id, waypoint_part.join("&"));
    let url = reqwest::Url::parse(&url_str).unwrap();
    let client = reqwest::Client::new();
    let response = client.get(url).header("Referer", "https://urbanscheduler.ml").send();

    let result: serde_json::Value = response?.json()?;
    let value_present = &result["response"]["route"][0];
    Ok(value_present.clone())
}

fn calculate_distance_here(p1: MyPoint, p2: MyPoint) -> Distance {
    let route = calculate_route_here(&[p1, p2], SUMMARY).unwrap();
    let travel_time = route["summary"]["travelTime"].as_f64().unwrap();

    travel_time
}


fn calculate_distance_matrix(f: &Fn(MyPoint, MyPoint) -> Distance, from: &Vec<MyPoint>, to: &Vec<MyPoint>) -> DistanceMatrix {
    let mut result = DistanceMatrix::zeros((from.len(), to.len()));
    for (i, x) in from.iter().enumerate() {
        for (j, y) in to.iter().enumerate() {
            result[(i, j)]= f(*x, *y);
        }
    }
    result
}


pub fn calculate_distance(method: DistsMethod, from: &Vec<MyPoint>, to: &Vec<MyPoint>) -> DistanceMatrix {
    match method {
        DistsMethod::Dummy => calculate_distance_matrix(&calculate_distance_euclid, from, to),
        DistsMethod::OSRM => panic!("Not implemented yet"),
        DistsMethod::HERE => calculate_distance_matrix(&calculate_distance_here, from, to)
    }
}

pub fn squash_distances(first: &DistanceMatrix, second: &DistanceMatrix) -> (DistanceMatrix, AnswersMatrix) {
    let result_shape = (first.shape()[0], second.shape()[1]);
    let mut result_dists = DistanceMatrix::zeros(result_shape);
    let mut result_answers = AnswersMatrix::zeros(result_shape);
    for i in 0..result_shape.0 {
        for j in 0..result_shape.1 {
            let dists: Array1<f64> = &first.row(i) + &second.column(j);
            let argmin: usize = dists.argmin().unwrap();
            result_answers[(i, j)] = argmin;
            result_dists[(i, j)] = dists[argmin];
        }
    }

    (result_dists, result_answers)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_here() {
        let sample_point = MyPoint {
            idx: 0,
            coords: (55.72271,37.58721),
        };

        let sample_point2 = MyPoint {
            idx: 0,
            coords: (55.72481,37.7009),
        };
        let dist = calculate_distance_here(sample_point, sample_point2);
        assert_eq!(dist, 4672.0);
    }
}