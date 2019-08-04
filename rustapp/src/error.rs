use std::fmt;
use std::error;
use std::result;
use reqwest;

use serde::{Serialize, Deserialize};
use std::fmt::{Display, Debug};

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub error_name: &'static str,
    pub error_message: Option<String>,
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.error_message {
            Some(mess) => write!(f, "{}: {}", self.error_name, mess),
            None => write!(f, "{}", self.error_name)
        }
    }
}

impl Error {
    pub fn fmt<T: Debug>(name: &'static str, info: T) -> Self {
        let err = Error {
            error_name: name,
            error_message: Some(format!("{:#?}", info)),
        };
        println!("{:#?}", err);
        err
    }
}

pub const UNKNOWN_ERROR: Error = Error {
    error_name: "Unknown",
    error_message: None
};

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::fmt("Reqwest", err)
    }
}


impl std::convert::From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        Error::fmt("Unknown", err)
    }
}


impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::fmt("IO", err)
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