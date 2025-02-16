// test the contract using cw-multi-test

use cosmwasm_std::{Addr, Coin};
use cw_multi_test::{App, AppBuilder, ContractWrapper, Executor};
use mantra_escrow::execute::execute;
use mantra_escrow::instantiate::instantiate;
use mantra_escrow::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use mantra_escrow::query::query;
use mantra_escrow::state::{Config, Escrow};

const ESCROW_FEE: u64 = 100;
const MIN_ESCROW_DURATION: u64 = 120;
const MAX_ESCROW_DURATION: u64 = 172800;
fn get_allowed_denoms() -> Vec<String> {
    vec!["uom".to_string(), "uatom".to_string()]
}

fn setup_app_with_balances() -> (App, Addr) {
    let mut seller_addr: Addr = Addr::unchecked("seller");
    let coin_balances: Vec<Coin> = get_allowed_denoms()
        .iter()
        .map(|denom| Coin::new(1000000u128, denom))
        .collect();
    let app = AppBuilder::new().build(|router, api, storage| {
        seller_addr = api.addr_make("seller");
        router
            .bank
            .init_balance(storage, &seller_addr, coin_balances)
            .unwrap()
    });
    (app, seller_addr)
}

fn setup_contract(app: &mut App) -> Result<Addr, cw_multi_test::error::AnyError> {
    let escrow_code_id =
        app.store_code(Box::new(ContractWrapper::new(execute, instantiate, query)));

    let admin_address = app.api().addr_make("admin");
    let instantiate_msg = InstantiateMsg {
        config: Config {
            admin: admin_address,
            escrow_fee: ESCROW_FEE,
            min_escrow_duration: MIN_ESCROW_DURATION,
            max_escrow_duration: MAX_ESCROW_DURATION,
            allowed_denoms: get_allowed_denoms(),
        },
    };

    app.instantiate_contract(
        escrow_code_id,
        Addr::unchecked("admin"),
        &instantiate_msg,
        &[],
        "mantra-escrow",
        None,
    )
}

#[test]
fn test_instantiate() {
    let mut app = App::default();
    let contract_addr = setup_contract(&mut app);
    assert!(contract_addr.is_ok());
}

#[test]
fn test_query_config() {
    let mut app = App::default();
    let contract_addr = setup_contract(&mut app);
    assert!(contract_addr.is_ok());
    let contract_addr = contract_addr.unwrap();

    let config: Config = app
        .wrap()
        .query_wasm_smart(contract_addr, &QueryMsg::Config {})
        .unwrap();

    let admin_address = app.api().addr_make("admin");
    assert_eq!(config.admin, admin_address);
    assert_eq!(config.escrow_fee, ESCROW_FEE);
    assert_eq!(config.min_escrow_duration, MIN_ESCROW_DURATION);
    assert_eq!(config.max_escrow_duration, MAX_ESCROW_DURATION);
    assert_eq!(config.allowed_denoms, get_allowed_denoms());
}

#[test]
fn test_create_escrow_and_deposit() {
    let (mut app, seller_addr) = setup_app_with_balances();
    let contract_addr = setup_contract(&mut app).unwrap();
    let buyer_addr = app.api().addr_make("buyer");

    let condition = get_allowed_denoms()
        .iter()
        .map(|denom| Coin::new(100u128, denom))
        .collect();
    let create_msg = ExecuteMsg::CreateEscrow {
        escrow: Escrow {
            id: None,
            seller: seller_addr.clone(),
            buyer: buyer_addr.clone(),
            condition,
            timeout: 120,
            state: None,
        },
    };

    // Creating three escrows using a loop to reduce code duplication
    for _ in 0..3 {
        let response = app.execute_contract(
            seller_addr.clone(),
            contract_addr.clone(),
            &create_msg,
            &vec![],
        );
        assert!(response.is_ok());
    }

    // Test query all escrows
    let all_escrows: Vec<Escrow> = app
        .wrap()
        .query_wasm_smart(&contract_addr, &QueryMsg::AllEscrows {})
        .unwrap();
    assert_eq!(all_escrows.len(), 3);

    // Test query escrow count
    let escrow_count: u64 = app
        .wrap()
        .query_wasm_smart(&contract_addr, &QueryMsg::EscrowCount {})
        .unwrap();
    assert_eq!(escrow_count, 3);

    // Test query escrow by id
    let escrow_by_id: Escrow = app
        .wrap()
        .query_wasm_smart(&contract_addr, &QueryMsg::Escrow { id: 1 })
        .unwrap();
    assert_eq!(escrow_by_id.id, Some(1));

    let deposit_msg = ExecuteMsg::Deposit { id: 1 };

    // Test deposit failure when executed by the buyer
    let response = app.execute_contract(
        buyer_addr.clone(),
        contract_addr.clone(),
        &deposit_msg,
        &vec![],
    );
    assert!(response.is_err());

    // Test deposit failure when executed by the seller with incorrect amount
    let funds = &[Coin::new(99u128, "uom"), Coin::new(100u128, "uatom")];
    let response = app.execute_contract(
        seller_addr.clone(),
        contract_addr.clone(),
        &deposit_msg,
        funds,
    );
    assert!(response.is_err());

    // Test deposit failure when executed by the seller with incorrect denom
    let funds = &[Coin::new(100u128, "uom"), Coin::new(100u128, "uusd")];
    let response = app.execute_contract(
        seller_addr.clone(),
        contract_addr.clone(),
        &deposit_msg,
        funds,
    );
    assert!(response.is_err());

    // Test deposit failure when execute with correct denoms plus extra funds
    let funds = &[
        Coin::new(100u128, "uom"),
        Coin::new(100u128, "uatom"),
        Coin::new(100u128, "uusd"),
    ];
    let response = app.execute_contract(
        seller_addr.clone(),
        contract_addr.clone(),
        &deposit_msg,
        funds,
    );
    assert!(response.is_err());

    // Check bank balances of seller
    let seller_balances = app
        .wrap()
        .query_balance(seller_addr.clone(), "uom")
        .unwrap();
    assert_eq!(seller_balances, Coin::new(1000000u128, "uom"));
    let seller_balances = app
        .wrap()
        .query_balance(seller_addr.clone(), "uatom")
        .unwrap();
    assert_eq!(seller_balances, Coin::new(1000000u128, "uatom"));

    // Test deposit success with the correct funds from the seller
    let funds = &[Coin::new(100u128, "uom"), Coin::new(100u128, "uatom")];
    let response = app.execute_contract(
        seller_addr.clone(),
        contract_addr.clone(),
        &deposit_msg,
        funds,
    );
    assert!(response.is_ok());
}
