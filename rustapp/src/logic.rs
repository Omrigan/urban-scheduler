use geo::Point;

struct Problem {
    ordered_events: Vec<Event>
}

struct Solution {
    schedule: Vec<MyPoint>
}

struct Event {
    idx: u64,
    points: Vec<Point>
}

struct MyPoint {
    point: Point<f64>,
    idx: u64
}

//fn calculate_distance(p1: &MyPoint, p2: &MyPoint) -> f64 {
//    return p1.vincenty_distance(p2).unwrap();
//}

fn calculate_distance_matrix(from: &Vec<MyPoint>, to: &Vec<MyPoint>) ->

fn solve(p: Problem) -> Solution {

}

