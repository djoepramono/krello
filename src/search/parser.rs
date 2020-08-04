use clap::{ArgMatches};
use crate::command_parser::AppError;
use crate::search::{SearchSubcommand, ModelType};

pub fn process_search_subcommand(subcommand_matches: &ArgMatches) -> Result<SearchSubcommand, AppError> {
  // ? operator on an option actually returns unstable Result<T, NoneError>
  // Thus it's better to convert Option to Result first via `ok_or`
  // ? also needs to be wrapped into function
  let process_subcommand_args = || -> Result<SearchSubcommand, AppError> {
      let query = subcommand_matches.value_of("query")
        .ok_or(AppError { message: "no query supplied" })?;
      let model_type = subcommand_matches.value_of("modelType")
        .ok_or(AppError {message: "no model type supplied"})
        .and_then(process_search_model_type)?;

      Ok(SearchSubcommand { value:  "search".to_string(), model_type: model_type, query: query.to_string()})
  };

  if let Ok(subcommand) = process_subcommand_args() {
    Ok(subcommand)
  } else {
    Err(AppError { message: "search does not have necessary arguments" })
  }
}


fn process_search_model_type(model_type: &str) -> Result<ModelType, AppError>{
  match model_type {
    "boards" => Ok(ModelType::Boards),
    "cards" => Ok(ModelType::Cards),
    _ => Err(AppError {message: "unknown search model type"})
  }
}
