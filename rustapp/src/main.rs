#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate scan_fmt;
#[macro_use]
extern crate mongodb;

use crate::problem::{PublicProblem, Solution, normalize_problem, solve};
use crate::error::Error;

//use mongodb;
use mongodb::ThreadedClient;
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;
use rocket_contrib::json::Json;
use rocket::{Rocket, Request};
use rocket::config::{Config, Environment};
use rocket::State;
use serde::{Serialize, Deserialize};

mod solve_ordered;
mod solve_generic;
mod distances;
mod final_route;
mod problem;
mod error;
mod solve_opt;
mod test_helpers;
mod report;
//mod test_performance;


#[catch(500)]
fn internal_error() -> Json<Error> {
    println!("Catcher 500");
    Json(Error {
        error_name: "Unknown error",
        error_message: None,
    })
}

#[catch(404)]
fn not_found(req: &Request) -> Json<Error> {
    Json(Error::fmt("NotFound", req))
}


#[derive(Debug, Serialize)]
#[serde(untagged)]
enum RequestResult {
    Ok(Solution),
    Err(error::Error)
}

#[post("/predict_raw", format = "json", data = "<problem_raw>")]
fn predict_raw(problem_raw: Json<PublicProblem>, state: State<LocalState>) -> Json<RequestResult> {
    let result = do_predict_raw(problem_raw.into_inner(), state);
    let request_result = match result {
        Ok(s) => RequestResult::Ok(s),
        Err(s) => RequestResult::Err(s)
    };
    Json(request_result)
}

fn do_predict_raw(problem_raw: PublicProblem, state: State<LocalState>) -> error::Result<Solution> {
    let problem = problem_raw;
    let mut report = report::Report::new();
    let normalized_problem = normalize_problem(problem,
                                               &state.places)?;
    report.checkpoint("preprocessed");

    let solution = solve(normalized_problem, report);

    solution
}


pub struct LocalState {
    pub places: Collection
}

impl LocalState {
    fn init_state() -> Self {
        let places = mongodb::Client::connect("mongo", 27017)
            .expect("Failed to initialize client.").db("cityday").collection("places");
        LocalState {
            places
        }
    }
}

fn rocket() -> Rocket {
    let config = Config::build(Environment::Staging)
        .address("0.0.0.0").port(80).finalize()
        .unwrap();

    rocket::custom(config).manage(LocalState::init_state()).mount("/", routes![predict_raw])
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
            .body(r#"{"events":[{"idx":0,"type":"points","points":[{"coords":[1.0,2.0],"idx":0}]}],
            "version":1}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        let response_text = response.body().unwrap().into_string().unwrap();
    }
}