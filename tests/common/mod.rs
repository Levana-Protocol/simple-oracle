use anyhow::Result;
use cosmwasm_std::Addr;
use cw_multi_test::{App, AppResponse, ContractWrapper, Executor};
use serde::de::DeserializeOwned;
use simple_oracle::{
    lifecycle::{execute, instantiate, query},
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};
use std::ops::{Deref, DerefMut};

pub struct TestApp {
    inner_app: App,
    pub oracle_code_id: u64,
    pub oracle_addr: Addr,
    pub migration_admin: Addr,
    pub owner: Addr,
}

impl TestApp {
    pub fn new() -> Result<Self> {
        let mut app = App::default();
        let migration_admin = Addr::unchecked("migration_admin");
        let owner = Addr::unchecked("owner");

        let oracle_code_id =
            app.store_code(Box::new(ContractWrapper::new(execute, instantiate, query)));

        let oracle_addr = app.instantiate_contract(
            oracle_code_id,
            owner.clone(),
            &InstantiateMsg { owner: None },
            &[],
            "simple_oracle",
            Some(migration_admin.to_string()),
        )?;

        Ok(Self {
            oracle_code_id,
            inner_app: app,
            oracle_addr,
            migration_admin,
            owner,
        })
    }

    pub fn oracle_execute(&mut self, sender: &Addr, msg: &ExecuteMsg) -> Result<AppResponse> {
        self.inner_app
            .execute_contract(sender.clone(), self.oracle_addr.clone(), msg, &[])
    }

    pub fn oracle_query<T: DeserializeOwned>(&self, msg: &QueryMsg) -> Result<T> {
        self.inner_app
            .wrap()
            .query_wasm_smart(self.oracle_addr.clone(), msg)
            .map_err(|err| err.into())
    }

    pub fn set_block_info(&mut self, height: i64, nanos: i64) {
        self.inner_app.update_block(|block_info| {
            if nanos < 0 {
                block_info.time = block_info.time.minus_nanos(nanos.unsigned_abs());
            } else {
                block_info.time = block_info.time.plus_nanos(nanos as u64);
            }
            if height < 0 {
                block_info.height -= height.unsigned_abs();
            } else {
                block_info.height += height as u64;
            }
        });
    }
}

impl Deref for TestApp {
    type Target = App;
    fn deref(&self) -> &Self::Target {
        &self.inner_app
    }
}

impl DerefMut for TestApp {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner_app
    }
}
