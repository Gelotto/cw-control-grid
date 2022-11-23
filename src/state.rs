use crate::msg::{InstantiateMsg, Token};
use crate::{error::ContractError, models::Cell};
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Storage};
use cw_storage_plus::{Item, Map};

pub const OWNER: Item<Addr> = Item::new("owner");
pub const TOKEN: Item<Token> = Item::new("token");
pub const CELLS: Map<(u32, u32), Cell> = Map::new("cells");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  _env: &Env,
  info: &MessageInfo,
  msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  OWNER.save(deps.storage, &info.sender)?;
  TOKEN.save(deps.storage, &msg.token)?;
  for (j, ticket_count) in msg.ticket_counts.iter().enumerate() {
    for (i, ticket_price) in msg.ticket_prices.iter().enumerate() {
      let key = (j as u32, i as u32);
      let cell = Cell::new(*ticket_price, *ticket_count);
      CELLS.save(deps.storage, key, &cell)?;
    }
  }
  Ok(())
}

pub fn load_cell(
  storage: &mut dyn Storage,
  coordinates: (u32, u32),
) -> Result<Cell, ContractError> {
  let some_cell = CELLS.may_load(storage, coordinates)?;
  if let Some(cell) = some_cell {
    Ok(cell)
  } else {
    Err(ContractError::CellNotFound {})
  }
}
