use std::env;
use std::error::Error;
use std::collections::HashMap;

#[derive(Clone)]
pub struct TrelloRequest{
  pub endpoint: String,
  pub url_params: HashMap<String, String> // Try play reference and lifetime later
}

pub fn send_request(trello_request: TrelloRequest) -> Result<String, Box<dyn Error>> {
  let api_key = env::var("TRELLO_API_KEY")?;
  let token = env::var("KRELLO_TOKEN")?;

  let trello_url_base = get_base_trello_url(trello_request.endpoint, api_key, token);
  let trello_url_suffix = get_trello_url_params(trello_request.url_params);

  let url = format!("{}{}", trello_url_base, trello_url_suffix);
  let res = reqwest::blocking::get(&url)?;
  let body = res.text()?;

  Ok(body)
}

fn get_base_trello_url(endpoint: String, api_key: String, token: String) -> String {
  format!("https://api.trello.com/1/{}?key={}&token={}", endpoint, api_key, token)
}

fn get_trello_url_params(hash_map: HashMap<String, String>) -> String {
  let mut key_value_pair: String;
  let mut url: String = String::new();
  for (key, value) in hash_map {
    key_value_pair = format!("&{}={}", key, value); // {:?} will quote
    url.push_str(key_value_pair.as_str());
  }
  url
}
