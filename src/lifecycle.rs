use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env, Event, MessageInfo, QueryResponse, Response,
    StdError, StdResult,
};
use cw2::{get_contract_version, set_contract_version};
use semver::Version;

use crate::{
    config::{CONTRACT_NAME, CONTRACT_VERSION},
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, Price, QueryMsg},
    storage::{OWNER, PRICE},
};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = msg
        .owner
        .as_ref()
        .map(|owner| deps.api.addr_validate(owner))
        .transpose()?
        .unwrap_or(info.sender);
    OWNER.save(deps.storage, &owner)?;

    Ok(
        Response::new().add_event(Event::new("instantiation").add_attributes([
            ("owner", owner.as_str()),
            ("contract_name", CONTRACT_NAME),
            ("contract_version", CONTRACT_VERSION),
        ])),
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    // all execution messages require the sender to be the owner
    let owner = OWNER.load(deps.storage)?;
    if info.sender != owner {
        return Err(StdError::generic_err(format!(
            "unauthorized, owner is {} (msg sent from {}",
            owner, info.sender
        )));
    }

    match msg {
        ExecuteMsg::SetOwner { owner } => {
            let owner = deps.api.addr_validate(&owner)?;
            OWNER.save(deps.storage, &owner)?;
            Ok(Response::new()
                .add_event(Event::new("set-owner").add_attribute("owner", owner.as_str())))
        }
        ExecuteMsg::SetPrice { value, timestamp } => {
            let price = Price {
                value,
                block_info: env.block,
                timestamp,
            };

            PRICE.save(deps.storage, &price)?;

            Ok(Response::new().add_event(
                Event::new("set-price").add_attributes([("value", price.value.to_string())]),
            ))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Owner {} => {
            let owner = OWNER.load(deps.storage)?;
            let res = to_binary(&owner)?;
            Ok(res)
        }
        QueryMsg::Price {} => {
            let price = PRICE.load(deps.storage)?;
            let res = to_binary(&price)?;
            Ok(res)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    let old_cw2 = get_contract_version(deps.storage)?;
    let old_version: Version = old_cw2
        .version
        .parse()
        .map_err(|_| StdError::generic_err("couldn't parse old contract version"))?;
    let new_version: Version = CONTRACT_VERSION
        .parse()
        .map_err(|_| StdError::generic_err("couldn't parse new contract version"))?;

    if old_cw2.contract != CONTRACT_NAME {
        Err(StdError::generic_err(format!(
            "mismatched contract migration name (from {} to {})",
            old_cw2.contract, CONTRACT_NAME
        )))
    } else if old_version > new_version {
        Err(StdError::generic_err(format!(
            "cannot migrate contract from newer to older (from {} to {})",
            old_cw2.version, CONTRACT_VERSION
        )))
    } else {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        Ok(
            Response::new().add_event(Event::new("migration").add_attributes([
                ("old_contract_name", old_cw2.contract.as_str()),
                ("old_contract_version", old_cw2.version.as_str()),
                ("new_contract_name", CONTRACT_NAME),
                ("new_contract_version", CONTRACT_VERSION),
            ])),
        )
    }
}
