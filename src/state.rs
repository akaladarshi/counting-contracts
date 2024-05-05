use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::Item;

pub const COUNTER: Item<u8> = Item::new("counter");
pub const MINIMAL_DONATION: Item<Coin> = Item::new("minimal_donation");
pub const OWNER: Item<Addr> = Item::new("owner");

pub const MAX_TRANSFER_AMOUNT: Item<Uint128> = Item::new("max_transfer_amount");