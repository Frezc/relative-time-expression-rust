use std::convert::From;

#[derive(PartialEq, Debug)]
pub struct Token {
  pub r#type: String,
  pub raw: String,
  pub start: usize,
  pub end: usize,
}

#[derive(PartialEq, Debug)]
pub struct Expression {
  pub r#type: String,
  pub body: Vec<Manipulation>,
  pub start: usize,
  pub end: usize,
}

#[derive(PartialEq, Debug)]
pub enum Manipulation {
  Offset {
    r#type: String,
    op: String,
    number: usize,
    unit: String,
    start: usize,
    end: usize,
  },
  Period {
    r#type: String,
    op: String,
    unit: String,
    start: usize,
    end: usize,
  },
}

#[derive(PartialEq, Debug)]
pub struct InputExpression {
  pub r#type: String,
  pub body: Vec<InputManipulation>,
}

impl From<Expression> for InputExpression {
  fn from(input: Expression) -> Self {
    InputExpression {
      r#type: input.r#type,
      body: input.body.into_iter().map(|m| InputManipulation::from(m)).collect()
    }
  }
}

#[derive(PartialEq, Debug)]
pub enum InputManipulation {
  Offset {
    r#type: String,
    op: String,
    number: usize,
    unit: String,
  },
  Period {
    r#type: String,
    op: String,
    unit: String,
  },
}

impl From<Manipulation> for InputManipulation {
  fn from(m: Manipulation) -> Self {
    match m {
      Manipulation::Offset { r#type, op, number, unit, .. } => InputManipulation::Offset {
        r#type, op, number, unit,
      },
      Manipulation::Period { r#type, op, unit, .. } => InputManipulation::Period {
        r#type, op, unit,
      }
    }
  }
}