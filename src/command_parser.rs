use std::fmt;
use clap::{Arg, App, ArgMatches};
use crate::search::parser::process_search_subcommand;
use crate::trello_wrapper::TrelloRequest;

pub struct AppError {
  pub message: &'static str, // if this is not public you cannot instantiate this from other module
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self.message)
  }
}

pub fn parse() -> Result<TrelloRequest, AppError>{
  let matches: ArgMatches = App::new("krello")
  .version("1.0")
  .about("Rust Client for Trello")
  .author("Djoe Pramono")
  .subcommand(App:: new("search")
    .about("send request to search API end point")
    .arg(Arg::with_name("query")
      .short("q")
      .long("query")
      .takes_value(true)
      .required(true)
      .help("search keyword"))
    .arg(Arg::with_name("modelType")
      .short("m")
      .long("modelType")
      .takes_value(true)
      .required(true)
      .help("search model type"))
  // ).subcommand(App:: new("cards")
  //   .about("send request to cards API end point")
  //   .arg(Arg::with_name("id")
  //     .short("i")
  //     .long("id")
  //     .takes_value(true)
  //     .required(true)
  //     .help("cards keyword"))
  ).get_matches();

  match matches.subcommand_matches("search") {
    Some(subcommand_matches) => process_search_subcommand(subcommand_matches),
    None => Err(AppError { message: "search_not_found" })
  }

  // match matches.subcommand_matches("cards") {
  //   Some(subcommand_matches) => process_search_subcommand(subcommand_matches),
  //   None => Err(AppError { message: "cards_not_found" })
  // };
}
