use andromeda_non_fungible_tokens::cw721::TransferAgreement;
use andromeda_std::{amp::AndrAddr, error::ContractError};
use cosmwasm_std::Storage;
use cw_storage_plus::{Item, Map};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cw721::{ContractInfoResponse, Cw721, Expiration};

pub const WRAPPED_TOKEN_ADDRESS: Map<&str, AndrAddr> = Map::new("wrapped_token_address");
pub const WRAPPED_INFO: Map<&str, WrappedInfo> = Map::new("wrapped_id");

#[cw_serde]
pub struct WrappedInfo {
    pub origin_token: AndrAddr,
    pub origin_token_id: String,
    pub unwrappable: bool,
}

// pub const ANDR_MINTER: Item<AndrAddr> = Item::new("minter");
// pub const CONTRACT_INFO: Map<&str, ContractInfoResponse> = Map::new("contract_info");
// // pub const TRANSFER_AGREEMENTS: Map<&str, TransferAgreement> = Map::new("transfer_agreements");
// pub const ARCHIVED: Map<&str, bool> = Map::new("archived_tokens");

// pub const MINT_ACTION: &str = "can_mint";
// // pub const BATCH_MINT_ACTION: &str = "can_batch_mint";

// pub fn is_archived(
//     storage: &dyn Storage,
//     token_id: &str,
// ) -> Result<IsArchivedResponse, ContractError> {
//     let archived_opt = ARCHIVED.may_load(storage, token_id)?.unwrap_or(false);
//     Ok(IsArchivedResponse {
//         is_archived: archived_opt,
//     })
// }

// #[cw_serde]
// pub struct IsArchivedResponse {
//     pub is_archived: bool,
// }
