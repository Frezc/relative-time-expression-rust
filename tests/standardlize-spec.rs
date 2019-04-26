use rte::*;

#[test]
fn format_empty() {
  assert_eq!(standardize("").unwrap(), "now");
}

#[test]
fn format_add_now() {
  assert_eq!(standardize("+d").unwrap(), "now+d");
}

#[test]
fn throw_err_when_parse_failed() {
  assert_eq!(standardize(" no - d").unwrap_err(), Error::Unknown {
    actual: "no ".to_string(),
    start: 1,
    end: 4,
  })
}