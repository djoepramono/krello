use std::env;
pub mod command_parser;
use command_parser::*;
use either::Either;

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = &args[1];

  let clean_input = input.trim();
  let output = parse(clean_input);
  match output {
    Either::Left(_) => print_usage(),
    Either::Right(command) => println!("executing {}", command)
  }
}

fn print_usage() -> () {
  println!("USAGE:");
  println!("krello <command> where command: Cards | Boards");
}
