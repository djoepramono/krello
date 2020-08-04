use std::fmt;
pub mod parser;
pub mod trello_wrapper;

#[derive(Clone)]
pub struct SearchSubcommand {
  pub value: String,
  pub model_type: ModelType,
  pub query: String,
}

#[derive(Clone)]
pub enum ModelType {
  Boards,
  Cards
}

impl fmt::Display for ModelType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{}", match self {
        ModelType::Boards => "boards",
        ModelType::Cards => "cards",
      },)
  }
}
