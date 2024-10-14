use serde::{Deserialize, Serialize};

use super::Contract;

// Trait representing a basic NFT. Its methods represent the contract's external API, thus the
// NTFS1 implementation should be annotated by #[contract_api] macro from dopechain-rust-macros
pub trait NFTS1<T> 
where
    T: Contract
{
    fn balance_of(&mut self, owner: String) -> String;
    fn owner_of(&mut self, token_id: String) -> String;
    fn owned_by(&mut self, owner: String) -> String;
    fn transfer_from(&mut self, args: TransferFromArgs);
    fn mint(&mut self, args: MintArgs);
}

#[derive(Serialize, Deserialize)]
pub struct TransferFromArgs {
    from: String,
    to: String,
    token_id: String
}

#[derive(Serialize, Deserialize)]
pub struct MintArgs {
    metadata_uri: String,
    owner: String
}
