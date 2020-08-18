mod command_parser;
mod trello_wrapper;
mod parser;
mod json_shearer;
// use json_shearer::Card;

fn main() {
  match command_parser::parse() {
    // Ok(subcommand) => println!("{}", trello_wrapper::send_request(subcommand).unwrap()),
    Ok(subcommand) => print_card_ids(trello_wrapper::send_request(subcommand).unwrap()),
    Err(e) => { println!("{}", e.message); print_usage(); },
  }
}

fn print_usage() -> () {
  println!("USAGE:");
  println!("krello <command> where command: Cards | Boards");
}

fn print_card_ids(cards: String) -> () {
  // println!("{}", cards);
  for x in json_shearer::shear_json(&cards).unwrap().cards.iter() {
    println!("{}", x.id);
  }
}
