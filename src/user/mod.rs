use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
  pub id: Option<String>,
  pub name: Option<String>,
}

impl Display for User {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let text = match &self.id {
      Some(id) => match &self.name {
        Some(name) => format!("{} ({})", name, id),
        None => format!("{}", id)
      }
      None => String::from("NO USER ID")
    };
    
    write!(f, "{}", text)
  }
}