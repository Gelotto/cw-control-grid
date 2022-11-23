use crate::{
  error::ContractError,
  models::Order,
  msg::Token,
  state::{load_cell, CELLS, TOKEN},
  utils::{
    build_cw20_transfer_from_msg, build_native_send_msg, verify_cw20_funds, verify_native_funds,
  },
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Uint128};

pub fn buy_tickets(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  orders: &Vec<Order>,
) -> Result<Response, ContractError> {
  let mut payment_amount = Uint128::zero();

  // increment cell ticket counts and end rounds where appropriate
  for order in orders.iter() {
    let key = order.get_key();
    let mut cell = load_cell(deps.storage, key)?;
    if order.n > cell.get_tickets_available() {
      return Err(ContractError::TooManyTickets {});
    }
    cell.ticket_count += order.n;
    payment_amount += cell.ticket_price * Uint128::from(order.n);
    CELLS.save(deps.storage, key, &cell)?;
  }

  Ok({
    let response = Response::new().add_attributes(vec![attr("action", "buy_tickets")]);
    // prepare payment
    match TOKEN.load(deps.storage)? {
      Token::Cw20 {
        address: cw20_token_address,
      } => {
        verify_cw20_funds(&deps, &info.sender, payment_amount, &cw20_token_address)?;
        response.add_submessage(build_cw20_transfer_from_msg(
          &info.sender,
          &env.contract.address,
          &cw20_token_address,
          payment_amount,
        )?)
      },
      Token::Native { denom } => {
        verify_native_funds(&info.funds, payment_amount, &denom)?;
        response.add_message(build_native_send_msg(
          &env.contract.address,
          &denom,
          payment_amount,
        )?)
      },
    }
  })
}
