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


fn calculate_distance_euclid(p1: &MyPoint, p2: &MyPoint) -> Distance {
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
fn calculate_distance_here(p1: &MyPoint, p2: &MyPoint) -> Distance {
    let here_app_id = env::var("HERE_APP_ID").unwrap();
    let here_app_code= env::var("HERE_APP_CODE").unwrap();
    let uri = format!("https://route.api.here.com/routing/7.2/calculateroute.json\
    ?app_id={}&app_code={}&waypoint0=geo!{}&waypoint1=geo!{}&mode=fastest;car;traffic:disabled",
                      here_app_id, here_app_code, p1.get_string_repr(), p2.get_string_repr());
    println!("{:?}", uri);
    let request = reqwest::Url::parse(&uri).unwrap();
    let mut response = reqwest::get(request).unwrap();
    let result: serde_json::Value = response.json().unwrap();

    let travelTime= result["response"]["route"][0]["summary"]["travelTime"].as_f64().unwrap();
    println!("{:?}", travelTime);
    travelTime
}


fn calculate_distance_matrix(f: &Fn(&MyPoint, &MyPoint) -> Distance, from: &Vec<MyPoint>, to: &Vec<MyPoint>) -> DistanceMatrix {
    let mut result = DistanceMatrix::zeros(from.len(), to.len());
    for (i, x) in from.iter().enumerate() {
        for (j, y) in to.iter().enumerate() {
            *result.index_mut((i, j)) = f(x, y);
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
        
    }
}