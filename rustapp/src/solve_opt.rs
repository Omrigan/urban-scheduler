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
use crate::report::Report;

fn prepare_distances_file(id: &str, p: &Problem, zpoint: usize) -> Result<()> {
    let mut file = File::create(format!("/tmp/{}/dists.dat", id))?;
    for x in p.events.iter() {
        for y in p.events.iter() {
            let last_dists = calculate_distance(p.config.dists_method, &x.points, &y.points);
            for ((p1, p2), dist) in last_dists.indexed_iter() {
                file.write_fmt(format_args!("{} {} {}\n",
                                            x.points[p1].idx, y.points[p2].idx, dist))?;
            }
        }
    }

    for event in p.events.iter() {
        for point in event.points.iter() {
            file.write_fmt(format_args!("{} {} {}\n",
                                        point.idx, zpoint, 0))?;
        }
    }


    Ok(())
}

fn format_scip_set<A: ToString, I: Iterator<Item=A>>(it: I) -> String {
    format!("{{{}}}", it.map(|x| x.to_string()).format(","))
}


fn prepare_main_file(id: &str, p: &Problem) -> Result<(usize, usize)> {
    let mut file = File::create(format!("/tmp/{}/main.zimpl", id))?;
    let zevent = p.events.len();
    let input_events_idxes = format_scip_set(0..(p.events.len() + 1));
    let mut input_events = Vec::new();
    let mut input_events_prev = Vec::new();
    let mut zpoint = 0usize;
    for (idx, event) in p.events.iter().enumerate() {
        let points_idxes = event.points.iter().map(|pt| pt.idx);
        let points_set = format_scip_set(points_idxes.clone());
        input_events.push(format!("<{}> {}", idx, points_set));

        let prev_set = format_scip_set(event.before.iter());
        input_events_prev.push(format!("<{}> {}", idx, prev_set));

        zpoint = zpoint.max(points_idxes.max().unwrap_or(0));
    }
    zpoint += 1;

    input_events.push(format!("<{}> {{{}}}", zevent, zpoint));
    input_events_prev.push(format!("<{}> {}", zevent, format_scip_set(0..(p.events.len()))));

    file.write_fmt(format_args!(include_str!("opt/template.zimpl"),
                                id = id,
                                E_idx = input_events_idxes,
                                E = input_events.iter().format(",\n"),
                                E_prev = input_events_prev.iter().format(",\n")))?;
    Ok((zevent, zpoint))
}

fn run_solution(id: &str, report: &mut Report) -> Result<String> {
    let output = Command::new("zimpl")
        .arg(format!("/tmp/{}/main.zimpl", id))
        .current_dir(format!("/tmp/{}", id))
        .output()?;
    report.checkpoint("zimpl_preprocessed");
    dbg!(&output);

    let output = Command::new("scip")
        .arg("-c")
        .arg("read main.lp set limits time 100 optimize write solution main.sol quit")
        .current_dir(format!("/tmp/{}", id))
        .output()?;

    dbg!(&output);
    let mut result_file = File::open(format!("/tmp/{}/main.sol", id))?;
    let mut result = String::new();
    result_file.read_to_string(&mut result)?;

    Ok(result)
}

fn recover_answer(text_solution: String, p: &Problem, zevent: usize, zpoint: usize) -> Result<Vec<ScheduleItem>> {
    let mut next = vec![0usize; zevent + 1];
    let mut selected = vec![false; zpoint + 1];

    let mut solution_status = true;
    for line in text_solution.split('\n') {
        let terms: Vec<&str> = line.split_whitespace().collect();
        dbg!(&terms);
        if terms.len() > 0 {
            if terms[0] == "solution" {
                solution_status = terms[2] == "optimal";
                if !solution_status {
                    return Err(UNKNOWN_ERROR);
                }
            }
            let var_terms: Vec<&str> = terms[0].split('#').collect();

            if var_terms[0] == "x" {
                let from: usize = var_terms[1].parse()?;
                let to: usize = var_terms[2].parse()?;
                next[from] = to;
            } else if var_terms[0] == "y" {
                let pt: usize = var_terms[1].parse()?;
                selected[pt] = true;
            }
        }
    }

    let mut order = Vec::new();
    let mut cur_event = next[zevent];
    while cur_event != zevent {
        order.push(cur_event);
        cur_event = next[cur_event];
    }

    let mut result: Vec<ScheduleItem> = Vec::new();
    for cur_event in order {
        let event = &p.events[cur_event];
        for pt in event.points.iter() {
            if selected[pt.idx] {
                result.push(ScheduleItem::construct(event, *pt));
            }
        }
    }

    Ok(result)
}

pub fn solve_opt(problem: &Problem, report: &mut Report) -> Result<Vec<ScheduleItem>> {
    let id: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .collect();

    dbg!(&id);

    std::fs::create_dir(format!("/tmp/{}", &id))?;

    let (zevent, zpoint) = prepare_main_file(&id, problem)?;

    prepare_distances_file(&id, problem, zpoint)?;
    report.checkpoint("files_prepared");
    dbg!("Files prepared");

    let text_solution = run_solution(&id, report)?;
    report.checkpoint("scip_solved");

    println!("{}", &text_solution);

    let solution = recover_answer(text_solution, problem, zevent, zpoint);

    solution
//    Err(UNKNOWN_ERROR)
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
        assert!(result.is_err());
    }
}
