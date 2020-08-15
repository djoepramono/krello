use clap::{ArgMatches};
use crate::trello_wrapper::TrelloRequest;

pub fn process_cards_subcommand(subcommand_matches: &ArgMatches) -> Option<TrelloRequest> {
  let id = subcommand_matches.value_of("id");

  if id == None {
    None
  } else {
    Some(TrelloRequest { endpoint: format!("cards/{}",id.unwrap().to_string()), url_params: None})
  }
}
