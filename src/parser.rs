use crate::types::{Token, Expression};
use std::iter::Peekable;
use crate::error::Error;
use std::slice::Iter;

pub struct Parser<'a> {
  pub iter: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
  pub fn parse(tokens: &Vec<Token>) -> Result<Expression, Error> {
    Parser::new(tokens).parse_full()
  }

  pub fn new(tokens: &Vec<Token>) -> Parser {
    Parser {
      iter: tokens.iter().peekable(),
    }
  }

  fn parse_full(mut self) -> Result<Expression, Error> {
    Ok(Expression {
      r#type: "Expression".to_string(),
          start: 0,
          end: 5,
          body: vec![]
    })
  }
}