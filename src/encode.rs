use crate::types::*;

pub fn encode(expression: &InputExpression) -> String {
  let result = String::from("now");
  result + &expression.body.iter().map(|item| match item {
    InputManipulation::Offset { op, number, unit, .. } =>
      format!("{}{}{}", op, if number == &1 { String::new() } else { number.to_string() }, unit),
    InputManipulation::Period { op, unit, .. } =>
      format!("{}{}", op, unit)
  }).collect::<String>()
}
