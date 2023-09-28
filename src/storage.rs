use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use crate::msg::Price;

pub const OWNER: Item<Addr> = Item::new("owner");
pub const PRICE: Item<Price> = Item::new("price");
