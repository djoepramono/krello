use std::fmt;

pub enum ParseResult {
  AppError { message: &'static str},
  Command,
}

impl fmt::Display for ParseResult {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}",  match self {
        ParseResult::AppError { message } => message,
        ParseResult::Command => "command",
    })
  }
}

pub fn parse(input: &str) -> ParseResult {
  println!("debug: '{}'", input);
  match input {
    "cards" => ParseResult::Command,
    _ => ParseResult::AppError { message : "unknown command" }
  }
}
