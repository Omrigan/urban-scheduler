#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate scan_fmt;


use crate::problem::{PublicProblem, Solution, normalize_problem, solve};

use rocket_contrib::json::{Json};
use rocket::{Rocket, Request};
use rocket::config::{Config, Environment};
use rocket::response::Responder;

use serde::{Serialize, Deserialize};

mod solve_ordered;
mod solve_generic;
mod distances;
mod final_route;
mod problem;
//mod test_performance;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    error_name: &'static str,
    error_code: usize,
    error_message: &'static str
}

#[catch(500)]
fn internal_error() -> Json<Error> {
    Json(Error {
        error_name: "Unknown error",
        error_code: 500,
        error_message: "Oops"
    })
}

#[catch(404)]
fn not_found(req: &Request) -> Json<Error> {
      Json(Error {
        error_name: "Not found",
        error_code: 404,
        error_message: "Oops"
    })
}


#[post("/predict_raw", format = "json", data = "<problem_raw>")]
fn predict_raw(problem_raw: Json<PublicProblem>) -> Result<Json<Solution>, Json<Error>> {
    let problem = problem_raw;
    let normalized_problem = normalize_problem(problem.into_inner());
    let solution = solve(&normalized_problem);
    match solution {
    Some(x) => Ok(Json(x)),
        None => Err(Json(Error {
            error_name: "Solver error",
            error_code: 32,
            error_message: "Oops"
        }))
    }
}


fn rocket() -> Rocket {
    let config = Config::build(Environment::Staging)
    .address("0.0.0.0").port(80).finalize()
        .unwrap();

    rocket::custom(config).mount("/", routes![predict_raw])
}

fn main() {
    rocket().register(catchers![internal_error, not_found]).launch();
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