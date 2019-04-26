use crate::types::*;

/// parse expression string to ast, then you can use any time library to transform moment.
/// check the doc at https://github.com/Frezc/relative-time-expression
/// # Examples
/// ```
/// use rte::*;
/// assert_eq!(
///  encode(&InputExpression {
///    r#type: "Expression".to_string(),
///    body: vec![InputManipulation::Offset {
///      r#type: "Offset".to_string(),
///      op: "+".to_string(),
///      number: 12,
///      unit: "M".to_string(),
///    }],
///  }),
///  "now+12M"
/// )
/// ```
pub fn encode(expression: &InputExpression) -> String {
  let result = String::from("now");
  result + &expression.body.iter().map(|item| match item {
    InputManipulation::Offset { op, number, unit, .. } =>
      format!("{}{}{}", op, if number == &1 { String::new() } else { number.to_string() }, unit),
    InputManipulation::Period { op, unit, .. } =>
      format!("{}{}", op, unit)
  }).collect::<String>()
}
