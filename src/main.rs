use std::env;
mod command_parser;
mod trello_client;
use either::Either;

fn main() {
  let args: Vec<String> = env::args().collect();
  let input_command = &args[1];
  let input_query = &args[2].trim();

  let parse_output = command_parser::parse(input_command.trim());
  match parse_output {
    Either::Left(_) => print_usage(),
    Either::Right(command) => println!("{}", trello_client::send_request(command, input_query.to_string()).unwrap())
  }
}

fn print_usage() -> () {
  println!("USAGE:");
  println!("krello <command> where command: Cards | Boards");
}
