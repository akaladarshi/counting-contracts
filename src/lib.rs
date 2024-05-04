use cosmwasm_std::{DepsMut, Deps, Env, MessageInfo, Response, StdResult, entry_point, QueryResponse, to_json_binary};
use crate::contract::{exec, query};
use crate::msg::{ExecMsg, InstantiateMsg, QueryMsg};

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
    contract::initialise(deps, msg.counter, msg.minimal_donation)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecMsg,
) -> StdResult<Response> {
    match msg {
        ExecMsg::Poke {} => exec::poke(deps, info),
        ExecMsg::Donate {} => exec::donate(deps, info),
        ExecMsg::Reset { counter: val} => exec::reset(deps, val)
    }
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