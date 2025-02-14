use crate::{
    msg::QueryMsg,
    state::{
        config, get_all_escrows, get_escrow, get_escrows_by_buyer, get_escrows_by_seller, Config,
        Escrow,
    },
};
use cosmwasm_std::{entry_point, to_json_binary, Addr, Binary, Deps, Env, StdResult};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllEscrows {} => to_json_binary(&query_all_escrows(deps)?),
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::EscrowsBySeller { seller } => {
            to_json_binary(&query_escrows_by_seller(deps, seller)?)
        }
        QueryMsg::EscrowsByBuyer { buyer } => to_json_binary(&query_escrows_by_buyer(deps, buyer)?),
        QueryMsg::Escrow { id } => to_json_binary(&query_escrow(deps, id)?),
    }
}

fn query_escrow(deps: Deps, id: u64) -> StdResult<Escrow> {
    let escrow = get_escrow(deps.storage, id)?;
    Ok(escrow)
}

fn query_all_escrows(deps: Deps) -> StdResult<Vec<Escrow>> {
    Ok(get_all_escrows(deps.storage)?)
}

fn query_escrows_by_seller(deps: Deps, seller: Addr) -> StdResult<Vec<Escrow>> {
    Ok(get_escrows_by_seller(deps.storage, seller)?)
}

fn query_escrows_by_buyer(deps: Deps, buyer: Addr) -> StdResult<Vec<Escrow>> {
    Ok(get_escrows_by_buyer(deps.storage, buyer)?)
}

fn query_config(deps: Deps) -> StdResult<Config> {
    config().load(deps.storage)
}
