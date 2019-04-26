use rte::*;

#[test]
fn encode_simple() {
  assert_eq!(
    encode(&InputExpression {
      r#type: "Expression".to_string(),
      body: vec![InputManipulation::Offset {
        r#type: "Offset".to_string(),
        op: "+".to_string(),
        number: 12,
        unit: "M".to_string(),
      }],
    }),
    "now+12M"
  )
}

#[test]
fn encode_complex() {
  assert_eq!(
    encode(&InputExpression {
      r#type: "Expression".to_string(),
      body: vec![InputManipulation::Offset {
        r#type: "Offset".to_string(),
        op: "+".to_string(),
        number: 1,
        unit: "M".to_string(),
      }, InputManipulation::Period {
        r#type: "Period".to_string(),
        op: "\\".to_string(),
        unit: "d".to_string(),
      }, InputManipulation::Offset {
        r#type: "Offset".to_string(),
        op: "-".to_string(),
        number: 2,
        unit: "h".to_string(),
      }, InputManipulation::Period {
        r#type: "Period".to_string(),
        op: "/".to_string(),
        unit: "h".to_string(),
      }],
    }),
    "now+M\\d-2h/h"
  )
}

#[test]
fn encode_empty() {
  assert_eq!(
    encode(&InputExpression {
      r#type: "Expression".to_string(),
      body: vec![],
    }),
    "now"
  )
}