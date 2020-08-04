use std::env;
use std::fmt;
use std::error::Error;
use crate::command_parser::SearchSubcommand;

pub enum ModelType {
  Boards
}

impl fmt::Display for ModelType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", match self {
        ModelType::Boards => "boards",
      },)
  }
}

pub fn send_request(search_subcommand: SearchSubcommand) -> Result<String, Box<dyn Error>> {
  let api_key = env::var("TRELLO_API_KEY")?;
  let token = env::var("KRELLO_TOKEN")?;

  // use clone so that you don't move the reference
  let trello_url_base = get_base_trello_url(search_subcommand.clone(), api_key, token);
  let model_types = [ModelType::Boards]; // this is slice not array
  let trello_url_suffix = get_trello_extra_params(&model_types, search_subcommand.query);

  let url = format!("{}{}", trello_url_base, trello_url_suffix);
  let res = reqwest::blocking::get(&url)?;
  let body = res.text()?;

  Ok(body)
}

fn get_base_trello_url(search_subcommand: SearchSubcommand, api_key: String, token: String) -> String {
  format!("https://api.trello.com/1/{}?key={}&token={}", search_subcommand.value, api_key, token)
}

fn get_trello_extra_params(model_types: &[ModelType], query: String) -> String {
  format!("&modelTypes={}&query={}", model_types.iter().map(|mt| mt.to_string()).collect::<Vec<String>>().join(","), query)
}
