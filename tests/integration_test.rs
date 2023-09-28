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
            timestamp: None,
        },
    )
    .unwrap_err();

    // try to set a price as the owner, expect success
    app.oracle_execute(
        &owner,
        &ExecuteMsg::SetPrice {
            value: "1.23".parse().unwrap(),
            timestamp: None,
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
            timestamp: None,
        },
    )
    .unwrap_err();

    // try to set a price as the new owner, expect success
    app.oracle_execute(
        &non_owner,
        &ExecuteMsg::SetPrice {
            value: "1.23".parse().unwrap(),
            timestamp: None,
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
    app.oracle_execute(
        &owner,
        &ExecuteMsg::SetPrice {
            value,
            timestamp: None,
        },
    )
    .unwrap();

    // confirm it's what we expect
    let price_1 = app.oracle_query::<Price>(&QueryMsg::Price {}).unwrap();

    assert_eq!(price_1.value, value);

    // Set a new price in the same block
    let value: Decimal256 = "3.21".parse().unwrap();
    app.oracle_execute(
        &owner,
        &ExecuteMsg::SetPrice {
            value,
            timestamp: None,
        },
    )
    .unwrap();

    // confirm it's what we expect
    let price_2 = app.oracle_query::<Price>(&QueryMsg::Price {}).unwrap();

    assert_eq!(price_2.value, value);

    // confirm that the block hasn't changed
    assert_eq!(price_2.block_info.height, price_1.block_info.height);
    assert_eq!(price_2.block_info.time, price_1.block_info.time);

    // jump a block (and some time in nanoseconds, which would naturally happen in the real world)
    app.set_block_info(1, 7_000_000_000);

    // Set a price in the new block
    let value: Decimal256 = "4.2".parse().unwrap();
    app.oracle_execute(
        &owner,
        &ExecuteMsg::SetPrice {
            value,
            timestamp: None,
        },
    )
    .unwrap();

    // confirm it's what we expect
    let price_3 = app.oracle_query::<Price>(&QueryMsg::Price {}).unwrap();

    assert_eq!(price_3.value, value);

    // and that the block has moved forward
    assert!(price_3.block_info.height > price_2.block_info.height);
    assert!(price_3.block_info.time > price_2.block_info.time);
}

#[test]
fn timestamp() {
    let mut app = TestApp::new().unwrap();
    let owner = app.owner.clone();

    // Get block info
    let block_info_1 = app.block_info();

    // Set a price
    app.oracle_execute(
        &owner,
        &ExecuteMsg::SetPrice {
            value: "1.23".parse().unwrap(),
            timestamp: None,
        },
    )
    .unwrap();

    // confirm the block info comes across
    let price_1 = app.oracle_query::<Price>(&QueryMsg::Price {}).unwrap();
    assert_eq!(price_1.block_info, block_info_1);
    assert_eq!(price_1.timestamp, None);

    // Set a new price in the same block but with a custom timestamp
    let timestamp_1 = block_info_1.time.plus_hours(1);
    app.oracle_execute(
        &owner,
        &ExecuteMsg::SetPrice {
            value: "1.23".parse().unwrap(),
            timestamp: Some(timestamp_1),
        },
    )
    .unwrap();

    // confirm the block info hasn't changed
    let price_2 = app.oracle_query::<Price>(&QueryMsg::Price {}).unwrap();
    assert_eq!(price_2.block_info, block_info_1);

    // but the timestamp is different
    let price_2_timestamp = price_2.timestamp.unwrap();
    assert_eq!(price_2_timestamp, timestamp_1);
    assert!(price_2_timestamp > block_info_1.time);

    // jump a block (and some time in nanoseconds, which would naturally happen in the real world)
    app.set_block_info(1, 7_000_000_000);

    let block_info_2 = app.block_info();
    assert!(block_info_2.height > block_info_1.height);
    assert!(block_info_2.time > block_info_1.time);

    // Set a price in the new block, with our same timestamp
    app.oracle_execute(
        &owner,
        &ExecuteMsg::SetPrice {
            value: "1.23".parse().unwrap(),
            timestamp: Some(timestamp_1),
        },
    )
    .unwrap();

    let price_3 = app.oracle_query::<Price>(&QueryMsg::Price {}).unwrap();
    assert_eq!(price_3.block_info, block_info_2);
    assert_eq!(price_3.timestamp.unwrap(), timestamp_1);

    // jump another block (and some time in nanoseconds, which would naturally happen in the real world)
    app.set_block_info(1, 7_000_000_000);

    let block_info_3 = app.block_info();
    assert!(block_info_3.height > block_info_2.height);
    assert!(block_info_3.time > block_info_2.time);

    // Set a price in the new block, with a new timestamp too
    let timestamp_2 = timestamp_1.plus_hours(1);
    app.oracle_execute(
        &owner,
        &ExecuteMsg::SetPrice {
            value: "1.23".parse().unwrap(),
            timestamp: Some(timestamp_2),
        },
    )
    .unwrap();
    let price_4 = app.oracle_query::<Price>(&QueryMsg::Price {}).unwrap();
    assert_eq!(price_4.block_info, block_info_3);
    assert_eq!(price_4.timestamp.unwrap(), timestamp_2);

    // Note that it's possible for timestamp to be earlier than block time

    let timestamp_3 = block_info_1.time.minus_days(1);
    app.oracle_execute(
        &owner,
        &ExecuteMsg::SetPrice {
            value: "1.23".parse().unwrap(),
            timestamp: Some(timestamp_3),
        },
    )
    .unwrap();
    let price_5 = app.oracle_query::<Price>(&QueryMsg::Price {}).unwrap();
    assert_eq!(price_5.block_info, block_info_3);
    assert_eq!(price_5.timestamp.unwrap(), timestamp_3);
}
