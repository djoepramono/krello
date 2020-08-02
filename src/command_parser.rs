use std::fmt;

pub struct AppError {
  message: &'static str,
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self.message)
  }
}

pub enum Command {
  Cards,
  Boards,
  Search,
}

impl fmt::Display for Command {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", match self {
        Command::Cards => "cards",
        Command::Boards => "boards",
        Command::Search => "search"
      },)
  }
}

pub fn parse(input: &str) -> Result<Command, AppError> {
  match input {
    "cards" => Result::Ok( Command::Cards),
    "boards" => Result::Ok( Command::Boards),
    "search" => Result::Ok( Command::Search),
    _ => Result::Err(AppError { message : "cannot parse command" }),
  }
}
