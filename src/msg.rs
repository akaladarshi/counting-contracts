use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone,  Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]  // To map fields to json snake_case format
pub enum QueryMsg {
    Value {}, // using {} for JSON object serialise and deserialize.
    Increment { value: u8}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")] // this not required but writing for consistency
pub struct ValueResp {
    pub value: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct IncrementResp {
    pub value: u8
}

#[derive(Serialize, Deserialize, Clone,  Debug, PartialEq, Eq)]
pub struct InstantiateMsg {
    pub counter: u8
}