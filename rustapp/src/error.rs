use std::fmt;
use std::error;
use std::io;
use std::result;
use reqwest;

use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub error_name: &'static str,
    pub error_code: usize,
    pub error_message: &'static str,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        Error {
            error_name: "unk",
            error_code: 0,
            error_message: "unk",
        }
    }
}


impl std::convert::From<std::env::VarError> for Error {
    fn from(_: std::env::VarError) -> Self {
        Error {
            error_name: "unk",
            error_code: 0,
            error_message: "unk",
        }
    }
}

//impl std::convert::From<url::parser::ParseError> for Error {
//    fn from(_: url::parser::ParseError) -> Self {
//        Error {
//            error_name: "unk",
//            error_code: 0,
//            error_message: "unk",
//        }
//    }
//}

//
//impl std::convert::From<std::option::NoneError> for Error {
//    fn from(_: std::option::NoneError) -> Self {
//        Error {
//            error_name: "unk",
//            error_code: 0,
//            error_message: "unk",
//        }
//    }
//}


pub type Result<T> = result::Result<T, Error>;