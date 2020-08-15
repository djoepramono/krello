mod command_parser;
mod trello_wrapper;
mod parser;

fn main() {
  match command_parser::parse() {
    Ok(subcommand) => println!("{}", trello_wrapper::send_request(subcommand).unwrap()),
    Err(e) => { println!("{}", e.message); print_usage(); },
  }
}

fn print_usage() -> () {
  println!("USAGE:");
  println!("krello <command> where command: Cards | Boards");
}
