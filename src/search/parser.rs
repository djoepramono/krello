use clap::{ArgMatches};
use crate::command_parser::AppError;
use std::collections::HashMap;
use crate::trello_wrapper::TrelloRequest;

pub fn process_search_subcommand(subcommand_matches: &ArgMatches) -> Result<TrelloRequest, AppError> {
  // ? operator on an option actually returns unstable Result<T, NoneError>
  // Thus it's better to convert Option to Result first via `ok_or`
  // ? also needs to be wrapped into function
  let process_subcommand_args = || -> Result<TrelloRequest, AppError> {
      let query = subcommand_matches.value_of("query")
        .ok_or(AppError { message: "no query supplied" })?;
      let model_type = subcommand_matches.value_of("modelType")
        .ok_or(AppError {message: "no model type supplied"})?;
        // .and_then(process_search_model_type)?;

      let mut url_params: HashMap<String, String> = HashMap::new();
      url_params.insert("modelTypes".to_string(), model_type.to_string());
      url_params.insert("query".to_string(), query.to_string());

      Ok(TrelloRequest { endpoint: "search".to_string(), url_params: url_params})
  };

  if let Ok(subcommand) = process_subcommand_args() {
    Ok(subcommand)
  } else {
    Err(AppError { message: "search does not have necessary arguments" })
  }
}
