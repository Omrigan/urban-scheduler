use crate::distances::MyPoint;
use crate::problem::{Event, Config, Problem};

use bit_set::BitSet;

pub fn sample_generic() -> Problem {
    let sample_event = Event {
        idx: 0,
        points: vec![MyPoint {
            idx: 0,
            coords: (1f64, 2f64),
        }],
        before: vec![1usize].into_iter().collect(),
        name: None,
        color: 0,
    };

    let sample_event2 = Event {
        idx: 1,
        points: vec![MyPoint {
            idx: 1,
            coords: (1f64, 2f64),
        }],
        before: BitSet::new(),
        name: None,
        color: 0,
    };

    let p = Problem {
        events: vec![sample_event, sample_event2],
        config: Config::default(),
        version: 2,
    };
    p
}

pub fn incorrect_generic() -> Problem {
    let sample_event = Event {
        idx: 0,
        points: vec![MyPoint {
            idx: 0,
            coords: (1f64, 2f64),

        }],
        before: vec![1usize].into_iter().collect(),
        name: None,
        color: 0,
    };

    let sample_event2 = Event {
        idx: 1,
        points: vec![MyPoint {
            idx: 1,
            coords: (1f64, 2f64),
        }],
        before: vec![0usize].into_iter().collect(),
        name: None,
        color: 0,
    };

    let p = Problem {
        events: vec![sample_event, sample_event2],
        config: Config::default(),
        version: 2,
    };
    p
}

pub fn sample_ordered() -> Problem {
    let sample_point = MyPoint {
        idx: 0,
        coords: (1f64, 2f64),
    };
    let sample_point2 = MyPoint {
        idx: 2,
        coords: (1f64, 2f64),
    };
    let sample_event = Event {
        idx: 0,
        points: vec![sample_point],
        before: BitSet::new(),
        name: None,
        color: 0
    };
    let sample_event2 = Event {
        idx: 1,
        points: vec![sample_point2],
        before: BitSet::new(),
        name: None,
        color: 0
    };
    Problem {
        events: vec![sample_event, sample_event2],
        config: Config::default(),
        version: 1,
    }
}