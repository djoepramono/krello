use std::fmt;
use clap::{Arg, App, ArgMatches};
// use std::option::NoneError;

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
      .help("search model type")))
  .get_matches();

  match matches.subcommand_matches("search") {
    Some(subcommand_matches) => process_search_subcommand(subcommand_matches),
    None => Err(AppError { message: "search_not_found" })
  }
}

fn process_search_subcommand(subcommand_matches: &ArgMatches) -> Result<SearchSubcommand, AppError> {

  // ? operator on an option actually returns unstable Result<T, NoneError>
  // Thus it's better to convert Option to Result first via `ok_or`
  // ? also needs to be wrapped into function
  let process_subcommand_args = || -> Result<SearchSubcommand, AppError> {
      let query = subcommand_matches.value_of("query").ok_or(AppError { message: "no query" })?;
      let model_type = subcommand_matches.value_of("modelType").ok_or(AppError { message: "no query" })?;
      Ok(SearchSubcommand { value:  "search".to_string(), model_type: model_type.to_string(), query: query.to_string()})
  };

  if let Ok(subcommand) = process_subcommand_args() {
    Ok(subcommand)
  } else {
    Err(AppError { message: "search does not have necessary arguments" })
  }

  // why not use if let? Because I want to preserve the wrapper type
  // why not use ? operator
  // let query = match subcommand_matches.value_of("query") {
  //   Some(query) => Ok(SearchSubcommand { value:  "search".to_string(), model_type: "boards".to_string(), query: query.to_string()}),
  //   None => Err(AppError { message: "query not found" }) // should never happen if required = true
  // };

  // let model_type = match subcommand_matches.value_of("modelType") {
  //   Some(query) => Ok(SearchSubcommand { value:  "search".to_string(), model_type: "boards".to_string(), query: query.to_string()}),
  //   None => Err(AppError { message: "model type not found" }) // should never happen if required = true
  // };

}
