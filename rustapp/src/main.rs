#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;


#[macro_use]
extern crate scan_fmt;

use rocket_contrib::json::{Json};
use rocket::Rocket;

use crate::solve_ordered::{OrderedProblem, Solution, solve_ordered};
use rocket::config::{Config, Environment};

mod solve_ordered;
mod distances;
mod events;
mod final_route;
//mod test_performance;

#[post("/predict_raw", format = "json", data = "<problem_raw>")]
fn predict_raw(problem_raw: Json<OrderedProblem>) -> Json<Solution> {
    let problem = problem_raw;
//    println!("{:?}", problem);
    let solution = solve_ordered(&problem);
    Json(solution)
}


fn rocket() -> Rocket {
    let config = Config::build(Environment::Staging)
    .address("0.0.0.0")
    .port(80)
    .finalize().unwrap();

    rocket::custom(config).mount("/", routes![predict_raw])
}

fn main() {
    rocket().launch();
}


#[cfg(test)]
mod tests {
    use super::*;

    use rocket;
    use rocket::local::Client;
    use rocket::http::{Status, ContentType};


    #[test]
    fn test_predict_raw() {
        let client = Client::new(rocket()).unwrap();

        // Try to get a message with an ID that doesn't exist.
        let mut response = client.post("/predict_raw")
            .header(ContentType::JSON)
            .body(r#"{"ordered_events":[{"idx":0,"points":[{"coords":[1.0,2.0],"idx":0}]}]}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        let response_text = response.body().unwrap().into_string().unwrap();

    }
}