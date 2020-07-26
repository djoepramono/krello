use std::io;
pub mod generic;
use generic::*;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).ok().expect("Failed to read line");

  let clean_input = input.trim();
  println!("processing {}", clean_input);
  let output = parse(clean_input);
  println!("result {}", output);

}

fn parse(input: &str) -> ParseResult {
  println!("debug: '{}'", input);
  match input {
    // "cards" => { message: "success"},
    // _ => AppError { message: "unknown command"},
    "cards" => ParseResult::Command,
    _ => ParseResult::AppError { message : "unknown" }
  }
}
