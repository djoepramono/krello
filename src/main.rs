use std::env;
pub mod command_parser;
use command_parser::*;
use either::Either;
use std::error::Error;

fn main() {
  let args: Vec<String> = env::args().collect();
  let input_command = &args[1];
  let input_query = &args[2].trim();

  let parse_output = parse(input_command.trim());
  match parse_output {
    Either::Left(_) => print_usage(),
    Either::Right(command) => println!("{}", send_reqwest(command, input_query.to_string()).unwrap())
  }
}

fn print_usage() -> () {
  println!("USAGE:");
  println!("krello <command> where command: Cards | Boards");
}

fn send_reqwest(command: Command, query: String) -> Result<String, Box<dyn Error>> {
  let api_key = env::var("TRELLO_API_KEY")?;
  let token = env::var("KRELLO_TOKEN")?;
  let url = format!("https://api.trello.com/1/search?modelTypes={}&query={}&key={}&token={}", command.to_string(), query, api_key, token);
  let res = reqwest::blocking::get(&url)?;
  println!("Headers:\n{:#?}", res.headers());

  let body = res.text()?;
  println!("Body:\n{}", body);

  Ok(body)
}
