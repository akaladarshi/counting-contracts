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

pub mod exec {
    use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};
    use crate::state::COUNTER;

    pub fn poke(deps: DepsMut, info: MessageInfo) -> StdResult<Response>{
        let mut counter = COUNTER.load(deps.storage)?;
        counter += 1;

        COUNTER.save(deps.storage, &counter)?;

        let res = Response::new()
            .add_attribute("action", "poke")
            .add_attribute("sender", info.sender)
            .add_attribute("counter", counter.to_string());

        Ok(res)
    }

    pub fn reset(deps: DepsMut, val: u8) -> StdResult<Response> {
        COUNTER.update(deps.storage, |_| -> StdResult<_> {
            Ok(val)
        })?;
        Ok(Response::new())
    }
}