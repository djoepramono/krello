use std::fmt;
use either::Either;

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
  Boards
}

impl fmt::Display for Command {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", match self {
        Command::Cards => "cards",
        Command::Boards => "boards",
      },)
  }
}

pub fn parse(input: &str) -> Either<AppError, Command> {
  match input {
    "cards" => Either::Right( Command::Cards),
    "boards" => Either::Right( Command::Boards),
    _ => Either::Left(AppError { message : "cannot parse command" }),
  }
}
