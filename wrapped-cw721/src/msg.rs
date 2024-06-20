use cosmwasm_schema::{cw_serde, QueryResponses};
use cw721::{Cw721ReceiveMsg, Expiration};
use andromeda_std::{
    andr_instantiate, andr_exec, andr_query,
    amp::{AndrAddr, Recipient},
};
use andromeda_non_fungible_tokens::cw721::{
    ExecuteMsg as AndrCw721ExecuteMsg, QueryMsg as AndrCw721QueryMsg, TokenExtension,
};

#[andr_instantiate]
#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub minter: AndrAddr,
}

#[andr_exec]
#[cw_serde]
pub enum ExecuteMsg {
    SetWrappedTokenAddress{
        token_address: AndrAddr,
    },
    ReceiveNft(Cw721ReceiveMsg),
    // Approve {
    //     spender: String,
    //     token_id: String,
    //     expires: Option<Expiration>,
    // },
}

#[cw_serde]
pub enum Cw721HookMsg {
    MintWrappedNft {
        wrapped_token: AndrAddr,
        wrapped_token_id: String,
        wrapped_token_owner: String,
        wrapped_token_uri: Option<String>,
        wrapped_token_extension: TokenExtension,
        unwrappable: bool,
        // origin_token_address: String,
        // origin_token_id: String,
    },
    UnwrapNft {
        recipient: AndrAddr,
        wrapped_token: AndrAddr,
        wrapped_token_id: String,
    },
}

#[andr_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {

}

// #[cw_serde]
// #[derive(Default)]
// pub struct TokenExtension {
//     /// The original publisher of the token
//     pub publisher: String,
// }