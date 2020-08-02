mod command_parser;
mod trello_client;

fn main() {
  match command_parser::parse() {
    Ok(subcommand) => println!("{}", trello_client::send_request(subcommand).unwrap()),
    Err(e) => { println!("{}", e.message); print_usage(); },
  }
}

fn print_usage() -> () {
  println!("USAGE:");
  println!("krello <command> where command: Cards | Boards");
}
