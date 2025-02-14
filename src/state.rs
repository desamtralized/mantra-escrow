

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, StdError, Storage};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};

use crate::ContractError;

#[cw_serde]
pub struct Escrow {
    pub id: Option<u64>,
    pub seller: Addr,
    pub buyer: Addr,
    pub condition: Vec<Coin>,
    pub timeout: u64, // in blocks
    pub state: Option<EscrowState>,
}

#[cw_serde]
pub enum EscrowState {
    Pending,
    Funded,
    Completed,
}

pub struct EscrowIndices<'a> {
    // Index by id
    pub id: MultiIndex<'a, u64, Escrow, String>,
    // Index by seller address
    pub seller: MultiIndex<'a, Addr, Escrow, String>,
    // Index by buyer address (if exists)
    pub buyer: MultiIndex<'a, Addr, Escrow, String>,
}

impl<'a> IndexList<Escrow> for EscrowIndices<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Escrow>> + '_> {
        let v: Vec<&dyn Index<Escrow>> = vec![&self.seller, &self.buyer];
        Box::new(v.into_iter())
    }
}

pub fn escrows<'a>() -> IndexedMap<&'a str, Escrow, EscrowIndices<'a>> {
    let indexes = EscrowIndices {
        id: MultiIndex::new(|_, e| e.id.unwrap_or_default(), "escrows", "escrows__id"),
        seller: MultiIndex::new(|_, e| e.seller.clone(), "escrows", "escrows__seller"),
        buyer: MultiIndex::new(|_, e| e.buyer.clone(), "escrows", "escrows__buyer"),
    };
    IndexedMap::new("escrows", indexes)
}

pub fn save_escrow(storage: &mut dyn Storage, escrow: &Escrow) -> Result<(), ContractError> {
    let id = escrow
        .id
        .ok_or_else(|| ContractError::Std(StdError::generic_err("Escrow must have an ID")))?;
    escrows()
        .save(storage, &id.to_string(), escrow)
        .map_err(|_| ContractError::Std(StdError::generic_err("Error saving escrow")))
}

pub fn get_escrow_count(storage: &dyn cosmwasm_std::Storage) -> usize {
    escrows()
        .range(storage, None, None, cosmwasm_std::Order::Ascending)
        .count()
}
pub fn get_escrow(storage: &dyn cosmwasm_std::Storage, id: u64) -> Result<Escrow, ContractError> {
    escrows()
        .load(storage, &id.to_string())
        .map_err(|_| ContractError::Std(StdError::generic_err("Escrow not found")))
}

pub fn get_all_escrows(storage: &dyn cosmwasm_std::Storage) -> Result<Vec<Escrow>, ContractError> {
    Ok(escrows()
        .range(storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| item.map(|(_, escrow)| escrow))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| ContractError::Std(e))?)
}

pub fn get_escrows_by_seller(
    storage: &dyn cosmwasm_std::Storage,
    seller: Addr,
) -> Result<Vec<Escrow>, ContractError> {
    let escrows = escrows()
        .idx
        .seller
        .prefix(seller)
        .range(storage, None, None, cosmwasm_std::Order::Descending)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| ContractError::Std(e))?;

    Ok(escrows.into_iter().map(|(_, escrow)| escrow).collect())
}

pub fn get_escrows_by_buyer(
    storage: &dyn cosmwasm_std::Storage,
    buyer: Addr,
) -> Result<Vec<Escrow>, ContractError> {
    let escrows = escrows()
        .idx
        .buyer
        .prefix(buyer)
        .range(storage, None, None, cosmwasm_std::Order::Descending)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| ContractError::Std(e))?;

    Ok(escrows.into_iter().map(|(_, escrow)| escrow).collect())
}

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub escrow_fee: u64, // contract fee in basis points, suggested default 0
    pub min_escrow_duration: u64, // minimum escrow duration in blocks, suggested default 0 (1.5blocks/sec)
    pub max_escrow_duration: u64, // maximum escrow duration in blocks, suggested default 172800blocks (~3 days)
    pub allowed_denoms: Vec<String>, // allowed denominations for escrows, suggested default [uom]
}

pub fn config() -> Item<Config> {
    Item::new("config")
}
