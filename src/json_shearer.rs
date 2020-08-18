use serde_json::{Result};
use serde::{Deserialize, Serialize};
// use std::fmt;

// impl fmt::Display for AppError {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//       write!(f, "{:?}", self.message)
//   }
// }

#[derive(Serialize, Deserialize)]
pub struct SearchCardResults {
  pub cards: Vec<Card>
}

#[derive(Serialize, Deserialize)]
pub struct Card {
  pub id: String
}

pub fn shear_json(data: &str) -> Result<SearchCardResults>{
  // println!("{}", data);
  let results: SearchCardResults = serde_json::from_str(data)?;
  for x in results.cards.iter() {
    println!("{}", x.id);
  }

  // Access parts of the data by indexing with square brackets.
  Ok(results)
}

