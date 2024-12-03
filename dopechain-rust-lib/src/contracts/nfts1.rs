use serde::{Deserialize, Serialize};

use super::Contract;

// Trait representing a basic NFT. Its methods represent the contract's external API, thus the
// NTFS1 implementation should be annotated by #[contract_api] macro from dopechain-rust-macros
pub trait NFTS1<T> 
where
    T: Contract
{
    fn owner_of(&mut self, token_id: u64) -> String;
    fn owned_by(&mut self, args: OwnedByArgs) -> String;
    fn transfer_from(&mut self, args: TransferFromArgs);
    fn mint(&mut self, args: MintArgs);
    fn get_metadata(&mut self, token_id: u64) -> String;
}

#[derive(Serialize, Deserialize)]
pub struct TransferFromArgs {
    pub from: String,
    pub to: String,
    pub token_id: u64
}

#[derive(Serialize, Deserialize)]
pub struct MintArgs {
    pub metadata_uri: String,
    pub owner: String
}

#[derive(Serialize, Deserialize)]
pub struct OwnedByArgs {
    pub owner: String
}
