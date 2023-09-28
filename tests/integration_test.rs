mod common;
use common::TestApp;
use cosmwasm_std::{Addr, Decimal256};
use simple_oracle::msg::{ExecuteMsg, Price, QueryMsg};

#[test]
fn auth() {
    let mut app = TestApp::new().unwrap();

    // first some sanity checks that everything is setup correctly
    let owner: Addr = app.oracle_query(&QueryMsg::Owner {}).unwrap();
    let non_owner = Addr::unchecked("non_owner");

    assert_eq!(owner, app.owner);
    assert_ne!(owner, non_owner);

    // try to set a price as the non-owner, expect failure
    app.oracle_execute(
        &non_owner,
        &ExecuteMsg::SetPrice {
            value: "1.23".parse().unwrap(),
        },
    )
    .unwrap_err();

    // try to set a price as the owner, expect success
    app.oracle_execute(
        &owner,
        &ExecuteMsg::SetPrice {
            value: "1.23".parse().unwrap(),
        },
    )
    .unwrap();

    // try to change the owner as the non-owner, expect failure
    app.oracle_execute(
        &non_owner,
        &ExecuteMsg::SetOwner {
            owner: non_owner.to_string(),
        },
    )
    .unwrap_err();

    // try to change the owner as the owner, expect success
    app.oracle_execute(
        &owner,
        &ExecuteMsg::SetOwner {
            owner: non_owner.to_string(),
        },
    )
    .unwrap();

    // try to set a price as the old owner, expect failure
    app.oracle_execute(
        &owner,
        &ExecuteMsg::SetPrice {
            value: "1.23".parse().unwrap(),
        },
    )
    .unwrap_err();

    // try to set a price as the new owner, expect success
    app.oracle_execute(
        &non_owner,
        &ExecuteMsg::SetPrice {
            value: "1.23".parse().unwrap(),
        },
    )
    .unwrap();
}

#[test]
fn price() {
    let mut app = TestApp::new().unwrap();
    let owner = app.owner.clone();

    // try to get initial price - will fail since none is set
    app.oracle_query::<Price>(&QueryMsg::Price {}).unwrap_err();

    // Set a price
    let value: Decimal256 = "1.23".parse().unwrap();
    app.oracle_execute(&owner, &ExecuteMsg::SetPrice { value })
        .unwrap();

    // confirm it's what we expect
    let price_1 = app.oracle_query::<Price>(&QueryMsg::Price {}).unwrap();

    assert_eq!(price_1.value, value);

    // Set a new price in the same block
    let value: Decimal256 = "3.21".parse().unwrap();
    app.oracle_execute(&owner, &ExecuteMsg::SetPrice { value })
        .unwrap();

    // confirm it's what we expect
    let price_2 = app.oracle_query::<Price>(&QueryMsg::Price {}).unwrap();

    assert_eq!(price_2.value, value);

    // confirm that the block hasn't changed
    assert_eq!(price_2.block_info.height, price_1.block_info.height);
    assert_eq!(price_2.block_info.time, price_1.block_info.time);

    // jump a block (and some time in nanoseconds, which would naturally happen in the real world)
    app.set_block_info(1, 7_000_000_000);

    // Set a new price in the same block
    let value: Decimal256 = "4.2".parse().unwrap();
    app.oracle_execute(&owner, &ExecuteMsg::SetPrice { value })
        .unwrap();

    // confirm it's what we expect
    let price_3 = app.oracle_query::<Price>(&QueryMsg::Price {}).unwrap();

    assert_eq!(price_3.value, value);

    // and that the block has moved forward
    assert!(price_3.block_info.height > price_2.block_info.height);
    assert!(price_3.block_info.time > price_2.block_info.time);
}
