use cosmwasm_std::{DepsMut, Deps, Env, MessageInfo, Empty, Response, StdResult, entry_point, QueryResponse};

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
    _msg: Empty,
) -> StdResult<QueryResponse> {
    Ok(QueryResponse::new(vec![]))
}