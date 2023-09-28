use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, BlockInfo, Decimal256, Timestamp};

#[cw_serde]
pub struct InstantiateMsg {
    /// the owner of the contract who can execute value changes
    /// if not set, then it will be the instantiator
    pub owner: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Change the owner
    SetOwner {
        /// The owner address
        owner: String,
    },

    /// Set the price
    SetPrice {
        /// The new price value
        value: Decimal256,
        /// Optional timestamp for the price, independent of block time
        timestamp: Option<Timestamp>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get the current price
    #[returns(Price)]
    Price {},
    /// Get the owner
    #[returns(Addr)]
    Owner {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct Price {
    /// The price value set via `ExecuteMsg::SetPrice`
    pub value: Decimal256,
    /// The block info when this price was set
    pub block_info: BlockInfo,
    /// Optional timestamp for the price, independent of block_info.time
    pub timestamp: Option<Timestamp>,
}
