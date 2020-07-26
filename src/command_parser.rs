use std::fmt;

pub struct AppError {
  message: &'static str,
}

pub enum Command {
  Cards,
  Boards
}

pub enum ParseResult<E, A> {
  Error(E),
  Success(A),
}

impl fmt::Display for ParseResult<AppError, Command> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", match self {
        ParseResult::Error(error) => error.message,
        ParseResult::Success(command) => match command {
          Command::Cards => "cards",
          Command::Boards => "boards",
        },
    })
  }
}

fn show_usage() -> &'static str {
  "USAGE: krello <command> \n
  where command: Cards | Boards
  "
}

pub fn parse(input: &str) -> ParseResult<AppError, Command> {
  match input {
    "cards" => ParseResult::Success( Command::Cards),
    "boards" => ParseResult::Success( Command::Boards),
    _ => ParseResult::Error(AppError { message : show_usage() }),
  }
}
