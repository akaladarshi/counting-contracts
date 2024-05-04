use cosmwasm_std::{Coin, DepsMut, Response, StdResult};
use crate::state::{COUNTER, MINIMAL_DONATION};

pub fn initialise(deps: DepsMut, counter: u8, minimal_donation: Coin) -> StdResult<Response>{
    COUNTER.save(deps.storage, &counter)?;
    MINIMAL_DONATION.save(deps.storage, &minimal_donation)?;
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
    use crate::state::{COUNTER, MINIMAL_DONATION};

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

    pub fn donate(deps: DepsMut, info: MessageInfo) -> StdResult<Response>  {
        let mut counter = COUNTER.load(deps.storage)?;
        let minimal_donation = MINIMAL_DONATION.load(deps.storage)?;

        if minimal_donation.amount.is_zero() || info.funds.iter().any(|coin| {
            coin.denom == minimal_donation.denom && coin.amount >= minimal_donation.amount
        }) {
            counter += 1;
            COUNTER.save(deps.storage, &counter)?;
        }

        let res = Response::new()
            .add_attribute("action", "donate")
            .add_attribute("sender", info.sender)
            .add_attribute("counter", counter.to_string());

        Ok(res)
    }
}