use crate::error::{Result, UNKNOWN_ERROR};
use crate::problem::{Problem, ScheduleItem};
use crate::distances::calculate_distance;
use std::fs::File;
use std::io::{Read, Write};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::string::ToString;
use itertools::Itertools;
use std::process::Command;

fn prepare_distances_file(id: &str, p: &Problem) -> Result<()> {
    let mut file = File::create(format!("/tmp/{}/dists.dat", id))?;
    for pairs in p.events.windows(2) {
        if let [x, y] = pairs {
            let last_dists = calculate_distance(p.config.dists_method, &x.points, &y.points);
            for ((p1, p2), dist) in last_dists.indexed_iter() {
                file.write_fmt(format_args!("{} {} {}\n",
                                            x.points[p1].idx, x.points[p2].idx, dist));
            }
        }
    }
    Ok(())
}

fn format_scip_set<A: ToString, I: Iterator<Item=A>>(it: I) -> String {
    format!("{{{}}}", it.map(|x| x.to_string()).format(","))
}


fn prepare_main_file(id: &str, p: &Problem) -> Result<()> {
    let mut file = File::create(format!("/tmp/{}/main.opt", id))?;

    let mut E_idx = format_scip_set(0..p.events.len());
    let mut E = Vec::new();
    for (idx, event) in p.events.iter().enumerate() {
        let set = format_scip_set(event.points.iter().map(|pt| pt.idx));
        E.push(format!("<{}> {}", idx, set));
    }
    file.write_fmt(format_args!(include_str!("opt/template.zimpl"),
                                id = id, E_idx = E_idx, E = E.iter().format(",\n")))?;
    Ok(())
}

fn run_solution(id: &str) -> Result<String> {
    let output = Command::new("opt")
        .arg(format!("/tmp/{}/main.opt", id))
        .current_dir(format!("/tmp/{}", id))
        .output()?;

    let output = Command::new("scip")
        .arg("-c")
        .arg("read main.lp optimize write solution main.sol quit")
        .current_dir(format!("/tmp/{}", id))
        .output()?;

    dbg!(&output);
    let mut result_file= File::open(format!("/tmp/{}/main.sol", id))?;
    let mut result= String::new();
    result_file.read_to_string(&mut result)?;

    Ok(result)
}


pub fn solve_opt(problem: &Problem) -> Option<Vec<ScheduleItem>> {
    let id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .collect();

    dbg!(&id);

    std::fs::create_dir(format!("/tmp/{}", &id));

    prepare_distances_file(&id, problem);
    prepare_main_file(&id, problem);

    let mut text_solution = run_solution(&id);

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::*;


    #[test]
    fn test_search() {
        let p = sample_generic();

        let result = solve_opt(&p).unwrap();
        dbg!(&result);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].point.idx, 1);
        assert_eq!(result[1].point.idx, 0);
    }

    #[test]
    fn test_incorrect() {
        let p = incorrect_generic();

        let result = solve_opt(&p);
        assert!(result.is_none());
    }
}
