use std::env;
pub mod command_parser;
use command_parser::*;
use either::Either;
use std::error::Error;

fn main() {
  let args: Vec<String> = env::args().collect();
  let input = &args[1];

  let _ = send_reqwest();

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

fn send_reqwest() -> Result<(), Box<dyn Error>> {
  let url = env::var("URL")?;
  let res = reqwest::blocking::get(&url)?;
  println!("Status: {}", res.status());
  println!("Headers:\n{:#?}", res.headers());

  let body = res.text()?;
  println!("Body:\n{}", body);

  Ok(())
}

