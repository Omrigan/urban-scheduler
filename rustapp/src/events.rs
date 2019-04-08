
use serde::{Serialize, Deserialize};
use geo::Point;


#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub idx: u64,
    pub points: Vec<MyPoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyPoint {
    pub coords: (f64, f64),
    pub idx: u64,

}

impl MyPoint {
    pub fn get_point(&self) -> Point<f64> {
        Point::from(self.coords)
    }
}