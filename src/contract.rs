use crate::msg::InstantiateMsg;
use crate::state::{COUNTER, MINIMAL_DONATION, MAX_TRANSFER_AMOUNT, OWNER};
use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};

pub fn initialise(deps: DepsMut, msg: &InstantiateMsg, info: &MessageInfo) -> StdResult<Response> {
    COUNTER.save(deps.storage, &msg.counter)?;
    MINIMAL_DONATION.save(deps.storage, &msg.minimal_donation)?;
    OWNER.save(deps.storage, &info.sender)?;
    MAX_TRANSFER_AMOUNT.save(deps.storage, &msg.max_transfer_amount)?;
    Ok(Response::new())
}

pub mod query {
    use crate::msg::{IncrementResp, ValueResp};
    use crate::state::COUNTER;
    use cosmwasm_std::{Deps, StdResult};

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
    use std::cmp::min;
    use crate::state::{COUNTER, MAX_TRANSFER_AMOUNT, MINIMAL_DONATION, OWNER};
    use cosmwasm_std::{BankMsg, Coin, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128};

    pub fn poke(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let mut counter = COUNTER.load(deps.storage)?;
        counter += 1;

        COUNTER.save(deps.storage, &counter)?;

        let res = Response::new()
            .add_attribute("action", "poke")
            .add_attribute("sender", info.sender)
            .add_attribute("counter", counter.to_string());

        Ok(res)
    }

    pub fn reset(deps: DepsMut, val: u8, info: MessageInfo) -> StdResult<Response> {
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            return Err(StdError::generic_err("Unauthorized"));
        }

        COUNTER.update(deps.storage, |_| -> StdResult<_> { Ok(val) })?;
        Ok(Response::new())
    }

    pub fn donate(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        let mut counter = COUNTER.load(deps.storage)?;
        let minimal_donation = MINIMAL_DONATION.load(deps.storage)?;

        if minimal_donation.amount.is_zero()
            || info.funds.iter().any(|coin| {
                coin.denom == minimal_donation.denom && coin.amount >= minimal_donation.amount
            })
        {
            counter += 1;
            COUNTER.save(deps.storage, &counter)?;
        }

        let res = Response::new()
            .add_attribute("action", "donate")
            .add_attribute("sender", info.sender)
            .add_attribute("counter", counter.to_string());

        Ok(res)
    }

    pub fn withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            return Err(StdError::generic_err("Unauthorized"));
        }

        let balances = deps.querier.query_all_balances(&env.contract.address)?;
        let bank_msg = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: balances,
        };

        let resp = Response::new()
            .add_message(bank_msg)
            .add_attribute("action", "withdraw")
            .add_attribute("sender", info.sender.as_str());

        Ok(resp)
    }

    pub fn withdraw_to(deps: DepsMut, env: Env, info: MessageInfo, receiver: String, funds: Vec<Coin>) -> StdResult<Response>{
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            return Err(StdError::generic_err("Unauthorized"));
        }

        let transfer_limit = MAX_TRANSFER_AMOUNT.load(deps.storage)?;
        let mut balance = deps.querier.query_all_balances(env.contract.address)?;

        if funds.is_empty() {
            return Err(StdError::generic_err("Funds Not Provided"))
        }

        for coin in &mut balance {
            coin.amount = funds
                .iter()
                .find(|c| c.denom == coin.denom)
                .map(|c| min(c.amount, transfer_limit))
                .unwrap_or(Uint128::zero());
        }

        let send_msg = BankMsg::Send {
            to_address: receiver,
            amount: balance
        };

        Ok(
            Response::new()
                .add_message(send_msg)
                .add_attribute("action", "withdraw")
                .add_attribute("sender", info.sender.as_str())
        )
    }
}
