use super::types::Token;
use super::error::Error;
use std::iter::{Peekable, Enumerate};
use std::str::Chars;

static WS: &str = " \r\n\t";
static UNITS: &str = "smhdwMy";
static OPS: &str = "+-/\\";
static NOW: &str = "now";

pub struct Tokenizer<'a> {
  tokens: Vec<Token>,
  iter: Peekable<Enumerate<Chars<'a>>>,
}

impl<'a> Tokenizer<'a> {
  pub fn parse(exp: &str) -> Result<Vec<Token>, Error> {
    Tokenizer::new(exp).parse_full()
  }

  pub fn new(exp: &'a str) -> Tokenizer<'a> {
    let iter = exp.chars().enumerate().peekable();
    Tokenizer {
      tokens: vec![],
      iter,
    }
  }

  pub fn parse_full(mut self) -> Result<Vec<Token>, Error> {
    while self.iter.peek().is_some() {
      match self.next_token() {
        Ok(token) => self.tokens.push(token),
        Err(err) => return Err(err)
      }
    }
    Ok(self.tokens)
  }

  fn next_token(&mut self) -> Result<Token, Error> {
    if let Some(t) = self.read_if_ws() {
      Ok(t)
    } else if let Some(t) = self.read_if_unit() {
      Ok(t)
    } else if let Some(t) = self.read_if_num() {
      Ok(t)
    } else if let Some(t) = self.read_if_op() {
      Ok(t)
    } else {
      self.read_now_or_err()
    }
  }

  fn read_if_ws(&mut self) -> Option<Token> {
    if let Some((_, c)) = self.iter.peek() {
      if WS.contains(&c.to_string()) {
        return Some(self.read_while("ws", |(_, c)| WS.contains(&c.to_string())));
      }
    }
    None
  }

  fn read_if_unit(&mut self) -> Option<Token> {
    if let Some((_, c)) = self.iter.peek() {
      if UNITS.contains(&c.to_string()) {
        return Some(self.read_size("unit", 1));
      }
    }
    None
  }

  fn read_if_num(&mut self) -> Option<Token> {
    if let Some((_, c)) = self.iter.peek() {
      if c.is_digit(10) {
        return Some(self.read_while("number", |(_, c)| c.is_digit(10)));
      }
    }
    None
  }

  fn read_if_op(&mut self) -> Option<Token> {
    if let Some((_, c)) = self.iter.peek() {
      if OPS.contains(&c.to_string()) {
        return Some(self.read_size("op", 1));
      }
    }
    None
  }

  fn read_now_or_err(&mut self) -> Result<Token, Error> {
    println!("{}", self.iter.peek().unwrap().1);
    let start = self.iter.peek().unwrap().0;
    let mut raw = String::new();
    let now: Vec<char> = NOW.chars().collect();
    for i in 0..3 {
      if let Some((_, c)) = self.iter.next() {
        eprintln!("{}", c);
        eprintln!("{}", i);
        eprintln!("{}", now[i]);
        raw.push(c);
        if c.to_lowercase().next().unwrap() != now[i] {
          let len = raw.len();
          return Err(Error::Unknown {
            actual: raw,
            start,
            end: start + len,
          })
        }
      }
    }
    Ok(Token {
      r#type: String::from("keyword"),
      raw,
      start,
      end: start + 3,
    })
  }

  fn read_size(&mut self, typ: &str, size: usize) -> Token {
    let mut len = 0;
    self.read_while(typ, |_| {
      len += 1;
      len <= size
    })
  }

  fn read_while(&mut self, typ: &str, mut predicate: impl FnMut((&usize, &char)) -> bool) -> Token {
    let start = self.iter.peek().unwrap().0;
    let mut raw = String::new();
    let mut end = start;
    while let Some((i, c)) = self.iter.peek() {
      if predicate((i, c)) {
        raw.push(*c);
        end += 1;
        self.iter.next();
      } else {
        break
      }
    }
    Token {
      r#type: String::from(typ),
      raw,
      start,
      end,
    }
  }
}