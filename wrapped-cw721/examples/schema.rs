use std::env::current_dir;

use cosmwasm_schema::{export_schema_with_title, schema_for, write_api};

use andromeda_wrapped_cw721::msg::{Cw721HookMsg, ExecuteMsg, InstantiateMsg, QueryMsg};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    };
    export_schema_with_title(&schema_for!(Cw721HookMsg), &out_dir, "cw721receive");
}
