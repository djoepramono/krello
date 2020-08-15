use clap::{ArgMatches};
use std::collections::HashMap;
use crate::trello_wrapper::TrelloRequest;

pub fn process_search_subcommand(subcommand_matches: &ArgMatches) -> Option<TrelloRequest> {
  let query = subcommand_matches.value_of("query");
  let model_type = subcommand_matches.value_of("modelType");

  if query.and(model_type) == None {
    None
  } else {
    let url_params = build_search_url_params(model_type.unwrap().to_string(), query.unwrap().to_string());
    Some(TrelloRequest { endpoint: "search".to_string(), url_params: url_params})
  }
}

pub fn build_search_url_params(model_type: String, query: String) -> HashMap<String, String> {
  let mut url_params: HashMap<String, String> = HashMap::new();
  url_params.insert("modelTypes".to_string(), model_type);
  url_params.insert("query".to_string(), query);

  url_params
}
