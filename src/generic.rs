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
