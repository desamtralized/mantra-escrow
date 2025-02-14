#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::state::{config, Config};

/*
// For security reasons, we will not implement migration for this contract
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:mantra-escrow";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let new_config = msg.config;
    let imut_deps = deps.as_ref();
    validate_config(imut_deps, &new_config)?;
    config().save(deps.storage, &new_config)?;
    let res = Response::new().add_attributes(vec![
        ("action", "instantiate"),
        ("sender", &info.sender.to_string()),
    ]);
    Ok(res)
}

fn validate_config(deps: Deps, config: &Config) -> Result<(), ContractError> {
    // check if escrow fee is less than 100%
    if config.escrow_fee > 10000 {
        return Err(ContractError::InvalidConfig {
            msg: "Escrow fee cannot exceed 100%".to_string(),
        });
    }
    // check if min duration is less than max duration
    if config.max_escrow_duration <= config.min_escrow_duration {
        return Err(ContractError::InvalidConfig {
            msg: "Max duration must be greater than min duration".to_string(),
        });
    }
    // check if allowed denoms is not empty
    if config.allowed_denoms.is_empty() {
        return Err(ContractError::InvalidConfig {
            msg: "At least one denomination must be allowed".to_string(),
        });
    }
    // check if admin is a valid address
    let admin_addr = deps.api.addr_validate(&config.admin.to_string());
    if admin_addr.is_err() {
        return Err(ContractError::InvalidConfig {
            msg: "Admin must be a valid address".to_string(),
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {}
