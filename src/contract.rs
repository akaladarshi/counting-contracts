use cosmwasm_std::{DepsMut, Response, StdResult};
use crate::state::COUNTER;

pub fn initialise(deps: DepsMut, counter: u8) -> StdResult<Response>{
    COUNTER.save(deps.storage, &counter)?;
    Ok(Response::new())
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult};
    use crate::msg::{IncrementResp, ValueResp};
    use crate::state::COUNTER;

    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        let value = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value })
    }

    pub fn increment(mut i: u8) -> IncrementResp {
        i += 1;
       IncrementResp { value: i }
    }
}

