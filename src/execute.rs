use cosmwasm_std::entry_point;
use cosmwasm_std::DepsMut;
use cosmwasm_std::Env;
use cosmwasm_std::MessageInfo;
use cosmwasm_std::Response;

use crate::msg::ExecuteMsg;
use crate::state::get_escrow;
use crate::state::get_escrow_count;
use crate::state::save_escrow;
use crate::state::Escrow;
use crate::state::EscrowState;
use crate::ContractError;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateEscrow { mut escrow } => create_escrow(deps, env, info, &mut escrow),
        ExecuteMsg::Deposit { id } => deposit(deps, env, info, id),
    }
}

fn create_escrow(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    escrow: &mut Escrow,
) -> Result<Response, ContractError> {
    // sender should be either seller or buyer
    if escrow.seller != info.sender && escrow.buyer != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // validate that both buyer and seller are valid addresses
    let buyer_addr = deps.api.addr_validate(&escrow.buyer.to_string());
    let seller_addr = deps.api.addr_validate(&escrow.seller.to_string());
    if buyer_addr.is_err() || seller_addr.is_err() {
        return Err(ContractError::InvalidAddress {});
    }

    // generate new escrow id and set state to pending
    let escrow_count: u64 = get_escrow_count(deps.storage).try_into().unwrap();
    let new_escrow_id = escrow_count + 1;
    escrow.id = Some(new_escrow_id);
    escrow.state = Some(EscrowState::Pending);

    // set timeout to current block height + timeout and save escrow
    escrow.timeout += env.block.height;
    save_escrow(deps.storage, escrow)?;
    let res = Response::new().add_attributes(vec![
        ("action", "create_escrow"),
        ("sender", &info.sender.to_string()),
        ("seller", &escrow.seller.to_string()),
        ("buyer", &escrow.buyer.to_string()),
        ("timeout", &escrow.timeout.to_string()),
        ("id", &escrow.id.unwrap_or(0).to_string()),
    ]);
    Ok(res)
}

fn deposit(deps: DepsMut, env: Env, info: MessageInfo, id: u64) -> Result<Response, ContractError> {
    let mut escrow = get_escrow(deps.storage, id.into())?;
    if escrow.seller != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    if info.funds != escrow.condition {
        return Err(ContractError::InvalidFunds {});
    }

    // check that the current block height is less than the timeout
    if env.block.height >= escrow.timeout {
        return Err(ContractError::EscrowTimeout {});
    }

    // Check that the escrow is in the pending state
    if escrow.state != Some(EscrowState::Pending) {
        return Err(ContractError::InvalidEscrowState {
            expected: EscrowState::Pending,
            got: escrow.state.unwrap(),
        });
    }

    // set the escrow state to funded
    escrow.state = Some(EscrowState::Funded);
    save_escrow(deps.storage, &escrow)?;
    let res = Response::new().add_attributes(vec![
        ("action", "deposit"),
        ("sender", &info.sender.to_string()),
        ("id", &escrow.id.unwrap_or(0).to_string()),
    ]);
    Ok(res)
}
