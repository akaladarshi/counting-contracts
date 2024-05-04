use cosmwasm_std::{DepsMut, Deps, Env, MessageInfo, Empty, Response, StdResult, entry_point, QueryResponse, to_json_binary};
use crate::contract::query;
use crate::msg::QueryMsg;

pub mod msg;
mod contract;
mod test;

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn query(
    _deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Value {} => to_json_binary(&query::value()),
        QueryMsg::Increment {value:val } => to_json_binary(&query::increment(val))
    }
}