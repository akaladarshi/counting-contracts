use crate::contract::{exec, query};
use crate::msg::{ExecMsg, InstantiateMsg, QueryMsg};
use cosmwasm_std::{
    entry_point, to_json_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response,
    StdResult,
};

mod contract;
pub mod msg;
mod state;
mod test;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    contract::initialise(deps, &msg, &info)
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecMsg) -> StdResult<Response> {
    match msg {
        ExecMsg::Poke {} => exec::poke(deps, info),
        ExecMsg::Donate {} => exec::donate(deps, info), // contract address will receive these funds
        ExecMsg::Reset { counter: val } => exec::reset(deps, val, info),
        ExecMsg::Withdraw {} => exec::withdraw(deps, env, info),
        ExecMsg::WithdrawTo { receiver, funds } => exec::withdraw_to(deps, env, info, receiver, funds)
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Value {} => to_json_binary(&query::value(deps)?),
        QueryMsg::Increment { value: val } => to_json_binary(&query::increment(val)),
    }
}
