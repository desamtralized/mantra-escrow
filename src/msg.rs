use crate::state::{Config, Escrow};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub config: Config,
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateEscrow { escrow: Escrow },
    Deposit { id: u64 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<Escrow>)]
    AllEscrows {},
    #[returns(Config)]
    Config {},
    #[returns(Escrow)]
    Escrow { id: u64 },
    #[returns(Vec<Escrow>)]
    EscrowsBySeller { seller: Addr },
    #[returns(Vec<Escrow>)]
    EscrowsByBuyer { buyer: Addr },
    #[returns(u64)]
    EscrowCount {},
}
