#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::state::config;

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
    config().save(deps.storage, &new_config)?;
    let res = Response::new().add_attributes(vec![
        ("action", "instantiate"),
        ("sender", &info.sender.to_string()),
    ]);
    Ok(res)
}
#[cfg(test)]
mod tests {}
