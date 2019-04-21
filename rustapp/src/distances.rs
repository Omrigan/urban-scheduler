use serde::{Serialize, Deserialize};
use geo::prelude::*;

use nalgebra::DMatrix;
use crate::events::MyPoint;

use reqwest;
//use std::collections::HashMap;

use serde_json;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum DistsMethod {
    Dummy,
    OSRM,
    HERE,
}


pub type Distance = f64;
pub type DistanceMatrix = DMatrix<Distance>;

use std::env;


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

pub fn calculate_route_here(points: &[MyPoint], route_attributes: &str) -> serde_json::Value {
    let here_app_id = env::var("HERE_APP_ID").unwrap();
    let here_app_code = env::var("HERE_APP_CODE").unwrap();

    let waypoint_part: Vec<String> = points.iter().enumerate().map(
        |(i, p)| format!("waypoint{}=geo!{}", i, p.get_string_repr())).collect();

    let url_str = format!("https://route.api.here.com/routing/7.2/calculateroute.json\
    ?app_id={}&app_code={}&{}&mode=fastest;car;traffic:disabled&routeAttributes={}",
                          here_app_id, here_app_code, waypoint_part.join("&"), route_attributes);
    let url = reqwest::Url::parse(&url_str).unwrap();
    println!("{:#}", url);
    let client = reqwest::Client::new();
    let response = client.get(url).header("Referer", "https://urbanscheduler.ml").send();
    let result: serde_json::Value = response.unwrap().json().unwrap();

    return result["response"]["route"][0].clone();
}

fn calculate_distance_here(p1: MyPoint, p2: MyPoint) -> Distance {
    let route = calculate_route_here(&[p1, p2], SUMMARY);
    let obj = route.as_object().unwrap();
    println!("{:#?}", obj.keys().collect::<Vec<&String>>());
    let travel_time = route["summary"]["travelTime"].as_f64().unwrap();

    println!("{:?}", travel_time);
    travel_time
}


fn calculate_distance_matrix(f: &Fn(MyPoint, MyPoint) -> Distance, from: &Vec<MyPoint>, to: &Vec<MyPoint>) -> DistanceMatrix {
    let mut result = DistanceMatrix::zeros(from.len(), to.len());
    for (i, x) in from.iter().enumerate() {
        for (j, y) in to.iter().enumerate() {
            *result.index_mut((i, j)) = f(*x, *y);
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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_here() {
        let sample_point = MyPoint {
            idx: 0,
            coords: (60.2243802, 25.0291695),
        };

        let sample_point2 = MyPoint {
            idx: 0,
            coords: (60.7243802, 25.0291695),
        };
        let dist = calculate_distance_here(sample_point, sample_point2);
        assert_eq!(dist, 4309.0);
    }
}