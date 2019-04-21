use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq)]
pub struct Error {
  pub expect: Option<String>,
  pub actual: String,
  pub start: usize,
  pub end: usize,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self.expect.as_ref() {
      Some(expect) => write!(f, "expect {} but found {} at ({}, {})", expect, self.actual, self.start, self.end),
      None => write!(f, "unexpected `{}` at ({}, {})", self.actual, self.start, self.end),
    }
  }
}

impl StdError for Error {}