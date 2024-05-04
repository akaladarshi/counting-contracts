use cosmwasm_std::{DepsMut, Deps, Env, MessageInfo, Empty, Response, StdResult, entry_point, QueryResponse, to_json_binary};
use crate::contract::query;
use crate::msg::{InstantiateMsg, QueryMsg};

pub mod msg;
mod contract;
mod test;
mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::initialise(deps, msg.counter)
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
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Value {} => to_json_binary(&query::value(deps)?),
        QueryMsg::Increment {value:val } => to_json_binary(&query::increment(val))
    }
}