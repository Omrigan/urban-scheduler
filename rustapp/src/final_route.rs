use crate::distances::{MyPoint, calculate_route_here};
use crate::error::{Result, UNKNOWN_ERROR};
const SHAPE: &str = "shape";


//fn parse_string(value: &str) -> (f64, f64) {
//    split(",")
//}
pub fn get_final_route(schedule: &Vec<MyPoint>) -> Result<Vec<(f64, f64)>> {
    let route = calculate_route_here(schedule, SHAPE)?;
    let arr = match route["shape"].as_array() {
        Some(smth) => smth,
        None => return Err(UNKNOWN_ERROR)
    };
    let it = arr.iter().map(|value|
        scan_fmt!(value.as_str().unwrap(),"{},{}", f64, f64)).map(|(x, y)| (x.unwrap(), y.unwrap()));
    Ok(it.collect())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_final_route() {
        let schedule = vec![MyPoint {
            idx: 0,
            coords: (55.72271, 37.58721),
        }, MyPoint {
            idx: 0,
            coords: (55.72481, 37.7009),
        }];


        let final_route = get_final_route(&schedule).unwrap();
        println!("{:?}", final_route);
        assert!(final_route.len() > 0);
    }
}