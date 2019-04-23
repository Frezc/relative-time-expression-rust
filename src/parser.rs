use crate::types::{Token, Expression, Manipulation};
use std::iter::Peekable;
use crate::error::Error;
use std::slice::Iter;

pub struct Parser<'a> {
  pub iter: Peekable<Iter<'a, Token>>,
  pub tokens: &'a Vec<Token>,
}

impl<'a> Parser<'a> {
  pub fn parse(tokens: &Vec<Token>) -> Result<Expression, Error> {
    Parser::new(tokens).parse_full()
  }

  pub fn new(tokens: &Vec<Token>) -> Parser {
    Parser {
      iter: tokens.iter().peekable(),
      tokens
    }
  }

  fn parse_full(mut self) -> Result<Expression, Error> {
    self.skip_ws();
    self.skip_now();
    self.skip_ws();

    let mut manipulations = vec![];
    while let Some(token) = self.iter.next() {
      manipulations.push(self.parse_manipulation(token)?);
      self.skip_ws();
    }

    Ok(Expression {
      r#type: "Expression".to_string(),
      start: 0,
      end: self.tokens.last().map_or(0, |t| t.end),
      body: manipulations,
    })
  }

  fn skip_ws(&mut self) {
    if let Some(t) = self.iter.peek() {
      if t.r#type == "ws" {
        self.iter.next();
      }
    }
  }

  fn skip_now(&mut self) {
    if let Some(t) = self.iter.peek() {
      if t.r#type == "keyword" && t.raw.to_lowercase() == "now" {
        self.iter.next();
      }
    }
  }

  // make sure peek is not empty before call this method
  fn parse_manipulation(&mut self, t: &Token) -> Result<Manipulation, Error> {
    if t.r#type == "op" {
      if t.raw == "+" || t.raw == "-" {
        return self.parse_offset(t);
      } else if t.raw == "/" || t.raw == "\\" {
        return self.parse_period(t);
      }
    }
    Err(Error::Normal {
      expect: String::from("operator(+, -, /, \\)"),
      actual: t.raw.clone(),
      start: t.start,
      end: t.end,
    })
  }

  fn parse_offset(&mut self, t: &Token) -> Result<Manipulation, Error> {
    self.skip_ws();
    let token = match self.iter.peek() {
      Some(token) => *token,
      None => return Err(Error::EOFError {
        expect: String::from("integer or unit(e.g. s, m, h, d, ...)"),
      }),
    };
    let number: usize = if token.r#type == "number" {
      self.iter.next();
      token.raw.parse().unwrap_or(1)
    } else {
      1
    };
    self.skip_ws();
    let unit = self.parse_unit()?;
    Ok(Manipulation::Offset {
      r#type: String::from("Offset"),
      op: t.raw.clone(),
      number,
      unit: unit.raw.clone(),
      start: t.start,
      end: unit.end,
    })
  }

  fn parse_period(&mut self, t: &Token) -> Result<Manipulation, Error> {
    self.skip_ws();
    let unit = self.parse_unit()?;
    Ok(Manipulation::Period {
      r#type: String::from("Period"),
      op: t.raw.clone(),
      unit: unit.raw.clone(),
      start: t.start,
      end: unit.end,
    })
  }

  fn parse_unit(&mut self) -> Result<&Token, Error> {
    match self.iter.next() {
      Some(t) if t.r#type == "unit" => Ok(t),
      Some(t) => Err(Error::Normal {
        expect: String::from("unit(e.g. s, m, h, d, ...)"),
        actual: t.raw.clone(),
        start: t.start,
        end: t.end,
      }),
      None => Err(Error::EOFError {
        expect: String::from("unit(e.g. s, m, h, d, ...)"),
      })
    }
  }
}