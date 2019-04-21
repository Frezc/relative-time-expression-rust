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
