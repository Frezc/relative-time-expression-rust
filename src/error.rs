use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};
use Error::*;

#[derive(Debug, PartialEq)]
pub enum Error {
  EOFError {
    expect: String,
  },
  Normal {
    expect: String,
    actual: String,
    start: usize,
    end: usize,
  },
  Unknown {
    actual: String,
    start: usize,
    end: usize,
  }
}


impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      EOFError { expect } => write!(f, "expect `{}` but get the end of input", expect),
      Normal { expect, actual, start, end } =>
        write!(f, "expect `{}` but found `{}` at ({}, {})", expect, actual, start, end),
      Unknown { actual, start, end } =>
        write!(f, "unexpected `{}` at ({}, {})", actual, start, end)
    }
  }
}

impl StdError for Error {}