use rte::*;

#[test]
fn parse_empty() {
  assert_eq!(parse("").unwrap(), Expression {
    r#type: "Expression".to_string(),
    start: 0,
    end: 0,
    body: vec![],
  })
}

#[test]
fn unexpect_token() {
  assert_eq!(parse(" no - d").unwrap_err(), Error::Unknown {
    actual: "no ".to_string(),
    start: 1,
    end: 4,
  })
}

#[test]
fn forget_operator() {
  assert_eq!(parse(" now  1d").unwrap_err(), Error::Normal {
    expect: "operator(+, -, /, \\)".to_string(),
    actual: "1".to_string(),
    start: 6,
    end: 7
  })
}

#[test]
fn forget_unit() {
  assert_eq!(parse(" now +1+d").unwrap_err(), Error::Normal {
    expect: "unit(e.g. s, m, h, d, ...)".to_string(),
    actual: "+".to_string(),
    start: 7,
    end: 8
  })
}