use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Grid {
  pub cells: Vec<Cell>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cell {
  pub ticket_price: Uint128,
  pub target_ticket_count: u32,
  pub ticket_count: u32,
  pub round_count: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Order {
  pub row: u32,
  pub col: u32,
  pub n: u32,
}

impl Grid {
  pub fn new() -> Self {
    Self { cells: vec![] }
  }
}

impl Cell {
  pub fn new(
    ticket_price: Uint128,
    target_ticket_count: u32,
  ) -> Self {
    Self {
      ticket_price,
      target_ticket_count,
      ticket_count: 0,
      round_count: 0,
    }
  }

  pub fn get_tickets_available(&self) -> u32 {
    self.target_ticket_count - self.ticket_count
  }
}

impl Order {
  pub fn get_key(&self) -> (u32, u32) {
    (self.row, self.col)
  }
}
