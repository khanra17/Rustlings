use std::num::ParseIntError;
use std::str::FromStr;

use crate::ParsePersonError::{BadLen, Empty, NoName, ParseInt};

#[derive(Debug, PartialEq)]
pub struct Person {
    pub name: String,
    pub age: usize,
}

// We will use this error type for the `FromStr` implementation.
#[derive(Debug, PartialEq)]
pub enum ParsePersonError {
    // Empty input string
    Empty,
    // Incorrect number of fields
    BadLen,
    // Empty name field
    NoName,
    // Wrapped error from parse::<usize>()
    ParseInt(ParseIntError),
}

impl From<ParseIntError> for ParsePersonError {
    fn from(err: ParseIntError) -> Self {
        ParsePersonError::ParseInt(err)
    }
}

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            return Err(Empty);
        }
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 { return Err(BadLen); }
        let name = parts[0];
        if name.is_empty() { return Err(NoName); }
        let age_str = parts[1].trim();
        let age =  age_str.parse::<usize>()?;
        Ok(Person { name: name.parse().unwrap(), age })
    }
}
