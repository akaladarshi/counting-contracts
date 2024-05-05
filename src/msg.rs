use cosmwasm_std::{Coin, Uint128};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")] // To map fields to json snake_case format
pub enum QueryMsg {
    Value {}, // using {} for JSON object serialise and deserialize.
    Increment { value: u8 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")] // this not required but writing for consistency
pub struct ValueResp {
    pub value: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct IncrementResp {
    pub value: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct InstantiateMsg {
    pub counter: u8,
    pub minimal_donation: Coin,
    pub max_transfer_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecMsg {
    Poke {},
    Donate {},
    Reset {
        #[serde(default)]
        counter: u8,
    },
    Withdraw {},
    WithdrawTo {
        receiver: String,
        #[serde(default)]
        funds: Vec<Coin>
    },
}
