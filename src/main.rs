mod command_parser;
mod search;

fn main() {
  match command_parser::parse() {
    Ok(subcommand) => println!("{}", search::trello_wrapper::send_request(subcommand).unwrap()),
    Err(e) => { println!("{}", e.message); print_usage(); },
  }
}

fn print_usage() -> () {
  println!("USAGE:");
  println!("krello <command> where command: Cards | Boards");
}
