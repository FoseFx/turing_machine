use std::collections::HashMap;

pub const BLANK: char = 'B';

#[derive(PartialEq)]
pub enum TMDirection {
  L,
  R,
  N
}
impl TMDirection {
  pub fn from(token: &str) -> Self {
    match token {
      "R" => TMDirection::R,
      "N" => TMDirection::N,
      "L" => TMDirection::L,
      _ => panic!("Invalid token found, must be 'L', 'R' or 'N'"),
    }
  }
}


pub type TMDeltaTable = HashMap<(usize, char), (usize, char, TMDirection)>;
