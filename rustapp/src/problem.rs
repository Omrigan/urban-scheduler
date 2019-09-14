use bit_set::BitSet;
use serde::{Serialize, Deserialize};

use crate::distances::{MyPoint, DistsMethod};
use crate::solve_ordered::{solve_stupid, solve_ordered};
use crate::solve_generic::solve_generic;
use crate::final_route::get_final_route;
use crate::error::{Error, Result};
use rand::{thread_rng, seq::SliceRandom};

use mongodb::coll::Collection;
use mongodb::ordered::OrderedDocument;
use bson;
use std::str;
use bson::Bson;
use std::collections::HashSet;
use crate::solve_opt::solve_opt;
use crate::report::{Report, PublicReport};


#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    lat: f64,
    lng: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointsEvent {
    points: Vec<MyPoint>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FixedPlaceEvent {
    location: Location,
    name: Option<String>,
}

impl FixedPlaceEvent {
    fn into_points(&self) -> Vec<MyPoint> {
        vec![MyPoint {
            coords: (self.location.lat, self.location.lng),
            idx: 0,
        }]
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryEvent {
    category: String,
    brand: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequentialEvent {
    items: Vec<PublicEvent>,
    name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParallelEvent {
    items: Vec<PublicEvent>,
    name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum PublicEvent {
    Points(PointsEvent),
    FixedPlace(FixedPlaceEvent),
    Category(CategoryEvent),
    Parallel(ParallelEvent),
    Sequential(SequentialEvent),
}

fn wrap_points(points: Vec<MyPoint>, name: Option<String>) -> Vec<Event> {
    vec![Event {
        points,
        before: BitSet::new(),
        name,
        color: random_color(),
    }]
}

fn process_container(public_events: Vec<PublicEvent>, idx_offset: usize,
                     places_collection: &Collection, clipping: Option<usize>,
                     is_sequential: bool) -> Vec<Event> {
    let mut bs = BitSet::new();
    let mut events = Vec::new();

    let mut global_idx = idx_offset;
    for event in public_events.into_iter() {
        let mut sub_events = event.into_events(global_idx, places_collection, clipping);

        if is_sequential {
            for event in sub_events.iter_mut() {
                event.before.union_with(&bs);
            }
        }
        for _ in 0..sub_events.len() {
            bs.insert(global_idx);
            global_idx += 1;
        }
        dbg!(global_idx);
        events.append(&mut sub_events);
    }

    return events;
}

fn parse_mongo_coord(doc: &OrderedDocument, key: &str) -> Result<f64> {
    let value = doc.get(key).ok_or(Error::fmt_silent("NoCoordComponent",
                                                     format!("{} in {}", key, doc)))?;

    match value {
        Bson::FloatingPoint(value) => Ok(value.clone()),
        Bson::String(str) => str.parse().map_err(Error::from),
        _ => Err(Error::fmt("UnkCoordType", format!("{} in {}", key, doc)))
    }
}

fn parse_schedule_item(doc: OrderedDocument) -> Result<MyPoint> {
    let location_doc = doc.get_document("location").unwrap();
//    dbg!(&location_doc);
    let location = Location {
        lat: parse_mongo_coord(location_doc, "lat")?,
        lng: parse_mongo_coord(location_doc, "lng")?,
    };
    //let location: Location = bson::from_bson(bson::Bson::from(location_doc.to_owned())).unwrap();
    Ok(MyPoint {
        coords: (location.lat, location.lng),
        idx: 0,
    })
}

fn resolve_category(event: CategoryEvent,
                    places_collection: &Collection,
                    clipping: Option<usize>) -> Vec<Event> {
    let mut filter = doc! {
    "categories": &event.category
    };
    match &event.brand {
        Some(brand) => { filter.insert("brand", brand); }
        None => ()
    };
    dbg!(&filter);


    let mut points: Vec<MyPoint> = places_collection.find(Some(filter), None).unwrap()
        .filter_map(std::result::Result::ok)
        .map(parse_schedule_item).filter_map(Result::ok)
        .collect();

    match clipping {
        Some(x) => if points.len() > x {
            points.resize_with(x, || panic!("Impossible"));
        },
        None => ()
    };

    let name = event.name.or(event.brand).unwrap_or(event.category);

    wrap_points(points, Some(name))
}

impl PublicEvent {
    fn into_events(self, idx_offset: usize,
                   places_collection: &Collection,
                   clipping: Option<usize>) -> Vec<Event> {
        let events = match self {
            PublicEvent::Points(event) =>
                wrap_points(event.points, None),
            PublicEvent::FixedPlace(event) =>
                wrap_points(event.into_points(), event.name),
            PublicEvent::Category(event) =>
                resolve_category(event, places_collection, clipping),
            PublicEvent::Sequential(event) =>
                process_container(event.items, idx_offset,
                                  places_collection, clipping, true),
            PublicEvent::Parallel(event) =>
                process_container(event.items, idx_offset,
                                  places_collection, clipping, false),
        };
        return events;
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicProblem {
    pub events: Vec<PublicEvent>,
    pub version: usize,
    #[serde(default)]
    pub config: Config,
}


#[derive(Debug)]
pub struct Event {
    pub points: Vec<MyPoint>,
    pub before: BitSet,
    pub name: Option<String>,
    pub color: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SolveAlgorithm {
    Stupid,
    Ordered,
    Generic,
    Opt,
}


impl Default for SolveAlgorithm {
    fn default() -> Self {
        SolveAlgorithm::Ordered
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub dists_method: DistsMethod,
    #[serde(default)]
    solve_algorithm: SolveAlgorithm,
    #[serde(default)]
    final_route: bool,
    clipping: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduleItem {
    #[serde(flatten)]
    pub point: MyPoint,
    name: String,
    color: u32,
}

impl ScheduleItem {
    pub fn construct(e: &Event, point: MyPoint) -> Self {
        ScheduleItem {
            point,
            name: e.name.clone().unwrap_or("Event ?".to_string()),
            color: e.color,
        }
    }
}


#[derive(Debug, Serialize)]
pub struct Solution {
    schedule: Vec<ScheduleItem>,
    pub final_route: Option<Vec<(f64, f64)>>,
    center: (f64, f64),
    config: Config,
    report: PublicReport,
}


#[derive(Default, Debug)]
pub struct Problem {
    pub version: usize,
    pub events: Vec<Event>,
    pub config: Config,
}

fn random_color() -> u32 {
    rand::random::<u32>() & 0xffffff
}

fn normalize_legacy(public_events: Vec<PublicEvent>) -> Vec<Event> {
    let mut events = Vec::new();
    let mut bs = BitSet::new();
    for (i, event) in public_events.into_iter().enumerate() {
        let points = if let PublicEvent::Points(points) = event {
            points
        } else {
            panic!("Not supported for legacy")
        };

        events.push(Event {
            points: points.points,
            before: bs.clone(),
            name: None,
            color: random_color(),
        });
        bs.insert(i);
    }
    events
}

fn normalize_events(public_events: Vec<PublicEvent>,
                    places_collection: &Collection,
                    clipping: Option<usize>) -> Vec<Event> {
    let mut events = process_container(public_events, 0,
                                       places_collection, clipping, true);
    let mut point_idx = 1usize;
    for event in events.iter_mut() {
        for pt in event.points.iter_mut() {
            if pt.idx == 0 {
                pt.idx = point_idx;
                point_idx += 1;
            }
        }
    }
    events
}

fn validate_problem(problem: &Problem) -> Result<()> {
    let mut problem_idxes = HashSet::new();
    for event in &problem.events {
        for point in &event.points {
            let idx = point.idx;
            if idx != 0 && problem_idxes.contains(&idx) {
                return Err(Error::fmt("Validation",
                                      format!("Point {} is present twice", idx)));
            } else {
                problem_idxes.insert(idx);
            }
        }
    }
    Ok(())
}

pub fn normalize_problem(problem: PublicProblem, places_collection: &Collection) -> Result<Problem> {
    let events = if problem.version == 1 {
        normalize_legacy(problem.events)
    } else {
        normalize_events(problem.events,
                         places_collection,
                         problem.config.clipping)
    };


    let problem = Problem {
        version: problem.version,
        config: problem.config,
        events,
    };
    validate_problem(&problem)?;

    Ok(problem)
}

fn sample_any(event: &Event) -> &MyPoint {
    let mut rng = thread_rng();
    event.points.choose(&mut rng).unwrap()
}

pub type Schedule = Vec<MyPoint>;

pub fn solve(problem: Problem, mut report: Report) -> Result<Solution> {
    let schedule = if problem.events.len() == 0 {
        Vec::new()
    } else if problem.events.len() == 1 {
        let event = &problem.events[0];
        vec![ScheduleItem::construct(event, sample_any(&event).clone())]
    } else {
        match problem.config.solve_algorithm {
            SolveAlgorithm::Stupid => solve_stupid(&problem),
            SolveAlgorithm::Ordered => solve_ordered(&problem),
            SolveAlgorithm::Generic => solve_generic(&problem, &mut report)?,
            SolveAlgorithm::Opt => solve_opt(&problem, &mut report)?
        }
    };
    report.checkpoint("solved");


    let points_vec: Vec<MyPoint> =
        schedule.iter().map(|item| item.point).collect(); // TODO: rewrite

    let final_route = if problem.config.final_route {
        match get_final_route(&points_vec) {
            Ok(route) => Some(route),
            Err(_) => None
        }
    } else { None };

    Ok(Solution {
        schedule,
        center: (55.7494539, 37.62160470000001),
        final_route,
        config: problem.config,
        report: report.finish(),
    })
}

