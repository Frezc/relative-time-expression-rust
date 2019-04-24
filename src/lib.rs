pub mod types;
pub mod error;
pub mod tokenizer;
pub mod parser;
pub mod encode;
pub use types::*;
pub use error::*;
pub use tokenizer::*;
pub use parser::*;
pub use encode::*;

/// parse expression string to ast, then you can use any time library to transform moment.
/// check the doc at https://github.com/Frezc/relative-time-expression
/// # Examples
/// ```
/// use rte::*;
/// let ast = parse("+ M\\M");
/// assert_eq!(ast.unwrap(), Expression {
///    r#type: "Expression".to_string(),
///    start: 0,
///    end: 5,
///    body: vec![
///      Manipulation::Offset {
///        r#type: "Offset".to_string(),
///        op: "+".to_string(),
///        number: 1,
///        unit: "M".to_string(),
///        start: 0,
///        end: 3,
///      },
///      Manipulation::Period {
///        r#type: "Period".to_string(),
///        op: "\\".to_string(),
///        unit: "M".to_string(),
///        start: 3,
///       end: 5,
///      }
///    ],
///  });
/// ```
pub fn  parse(exp: &str) -> Result<Expression, Error> {
  Parser::parse(&Tokenizer::parse(exp)?)
}