use std::fmt;
use clap::{Arg, App, ArgMatches};

pub struct AppError {
  pub message: &'static str, // if this is not public you cannot instantiate this from other module
}

#[derive(Clone)]
pub struct SearchSubcommand {
  pub value: String,
  pub model_type: String, // why string? Because this type will be used as a return value, and it's better not be a reference
  pub query: String,
}

impl fmt::Display for AppError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self.message)
  }
}

pub fn parse() -> Result<SearchSubcommand, AppError>{
  let matches: ArgMatches = App::new("krello")
  .version("1.0")
  .about("Rust Client for Trello")
  .author("Djoe Pramono")
  .subcommand(App:: new("search")
    .about("send request to search API end point")
    .arg(Arg::new("query")
      .short('q')
      .long("query")
      .takes_value(true)
      .required(true)
      .about("search keyword")))
  .get_matches();

  match matches.subcommand_matches("search") {
    Some(subcommand_matches) => process_search_subcommand(subcommand_matches),
    None => Err(AppError { message: "search_not_found" })
  }
}

fn process_search_subcommand(subcommand_matches: &ArgMatches) -> Result<SearchSubcommand, AppError> {
  match subcommand_matches.value_of("query") {
    Some(query) => Ok(SearchSubcommand { value:  "search".to_string(), model_type: "boards".to_string(), query: query.to_string()}),
    None => Err(AppError { message: "query not found" }) // should never happen if required = true
  }
}
