use std::env;
pub mod command_parser;
use command_parser::*;
use either::Either;
// use reqwest::blocking;
use error_chain::error_chain;

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

// to bubble error ...
error_chain! {
  foreign_links {
      Io(std::io::Error);
      HttpRequest(reqwest::Error);
  }
}

fn send_reqwest() -> Result<()> {
  let res = reqwest::blocking::get("http://httpbin.org/get")?;
  // let mut body = String::new();
  // res.read_to_string(&mut body)?;
  let body = res.text()?;

  // println!("Status: {}", res.status());
  // println!("Headers:\n{:#?}", res.headers());
  println!("Body:\n{}", body);

  Ok(())
}
