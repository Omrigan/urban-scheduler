use crate::solve_ordered::{OrderedProblem, Config};
use crate::events::{MyPoint, Event};
#[cfg(test)]
mod tests {
    use super::*;
//    use serde::serde_rust;

    fn load_problem() -> OrderedProblem {

        OrderedProblem {
            ordered_events: vec![sample_event],
            config: Config::default()
        }
    }


    #[test]
    fn test_ordered_performance() {


    }

}