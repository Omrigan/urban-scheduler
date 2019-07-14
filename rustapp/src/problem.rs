use bit_set::BitSet;
use serde::{Serialize, Deserialize};

use crate::distances::{MyPoint, DistsMethod};
use crate::solve_ordered::{solve_stupid, solve_ordered};
use crate::solve_generic::solve_generic;
use crate::final_route::get_full_route;


#[derive(Debug, Serialize, Deserialize)]
pub struct PointsEvent {
    points: Vec<MyPoint>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryEvent {
    category: String,
    brand: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SequentialEvent {
    items: Vec<PublicEvent>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParallelEvent {
    items: Vec<PublicEvent>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum PublicEvent {
    Points(PointsEvent),
    Category(CategoryEvent),
    Parallel(ParallelEvent),
    Sequential(SequentialEvent),
}

impl PublicEvent {

    fn as_points(self) -> PointsEvent {
        if let PublicEvent::Points(p) = self {
            p
        } else {
            panic!("Not suppoertd")
        }
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
    pub idx: usize,
    pub points: Vec<MyPoint>,
    pub before: BitSet,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SolveAlgorithm {
    Stupid,
    Ordered,
    Generic,
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
    find_final_route: bool,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Solution {
    pub schedule: Vec<MyPoint>,
    pub full_route: Option<Vec<(f64, f64)>>,
}


#[derive(Default, Debug)]
pub struct Problem {
    pub version: usize,
    pub events: Vec<Event>,
    pub config: Config,
}

fn normalize_legacy(public_events: Vec<PublicEvent>) -> Vec<Event> {
    let mut events = Vec::new();
    let mut bs = BitSet::new();
    for (i, event) in public_events.into_iter().enumerate() {
        let points = event.as_points();

        events.push(Event {
            idx: i,
            points: points.points,
            before: bs.clone(),
        });
        bs.insert(i);
    }
    events
}

pub fn normalize_problem(problem: PublicProblem) -> Problem {
    let events = if problem.version == 1 {
        normalize_legacy(problem.events)
    } else {
        panic!("Not implemented")
//        normalize_events(problem.events)
    };


    Problem {
        version: problem.version,
        config: problem.config,
        events,
    }
}

pub fn solve(problem: &Problem) -> Option<Solution> {
    let mut solution = match problem.config.solve_algorithm {
        SolveAlgorithm::Stupid => solve_stupid(problem),
        SolveAlgorithm::Ordered => solve_ordered(problem),
        SolveAlgorithm::Generic => solve_generic(problem)?,
    };

    if problem.config.find_final_route {
        solution.full_route = get_full_route(&solution.schedule);
    }
    Some(solution)
}


//
//impl Serialize for BitSet
//{
//    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//        where
//            S: Serializer,
//    {
//        let mut seq = serializer.serialize_seq(Some(self.len()))?;
//        for e in self {
//            seq.serialize_element(e)?;
//        }
//        seq.end()
//    }
//}
//
//pub trait Deserialize<'de>: Sized {
//    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//        where
//            D: Deserializer<'de> {
//        deserializer.deserialize_seq(BitSetVisitor)
//    }
//}
//
//
////impl Deserialize for
//
//
//struct BitSetVisitor;
//
//impl<'de> Visitor<'de> for BitSetVisitor {
//    type Value = BitSet;
//
//    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//        formatter.write_str("usize array")
//    }
//    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where
//        A: SeqAccess<'de> {
//        let mut result = BitSet::new();
//        while let elem = seq.next_element()?{
//            result.insert(elem);
//        }
//        Ok(result)
//    }
//}
