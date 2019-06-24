use crate::distances::{MyPoint, calculate_route_here};

const SHAPE: &str = "shape";


//fn parse_string(value: &str) -> (f64, f64) {
//    split(",")
//}
pub fn get_full_route(schedule: &Vec<MyPoint>) -> Option<Vec<(f64, f64)>> {
    let route = calculate_route_here(schedule, SHAPE);
    let arr = route["shape"].as_array()?;
    let it = arr.iter().map(|value|
        scan_fmt!(value.as_str().unwrap(),"{},{}", f64, f64)).map(|(x, y)| (x.unwrap(), y.unwrap()));
    Some(it.collect())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_final_route() {
        let schedule = vec![MyPoint {
            idx: 0,
            coords:  (55.72271,37.58721),
        }, MyPoint {
            idx: 0,
            coords: (55.72481,37.7009),
        }];


        let full_route = get_full_route(&schedule).unwrap();
        println!("{:?}", full_route);
        assert!(full_route.len()>0);
    }
}