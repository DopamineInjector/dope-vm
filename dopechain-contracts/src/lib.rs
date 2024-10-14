use core::panic;
use std::{collections::{HashMap, HashSet}, env::args, fmt::Display, str::FromStr};

use dopechain_rust_lib::{contracts::{nfts1::{MintArgs, TransferFromArgs, NFTS1}, Contract, Fetchable, OnChainVar}, sdk::log};
use dopechain_rust_macros::contract_api;
use serde::{Deserialize, Serialize};

//
// Main struct describing token information
//
#[derive(Serialize, Deserialize, Clone)]
struct TokenDetails {
    pub owner: String,
    pub metadata_uri: String
}

// Map tokenid -> details
#[derive(Serialize, Deserialize, Clone)]
struct TokenMap {
    tokens: HashMap<u64, TokenDetails>
}

impl TokenMap {
    pub fn new() -> Self {
        TokenMap {
            tokens: HashMap::new()
        }
    }
}

impl Display for TokenMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringified = serde_json::to_string(self).unwrap();
        f.write_str(&stringified);
        Ok(())
    }
}

impl FromStr for TokenMap {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Self = serde_json::from_str(s).unwrap();
        Ok(parsed)
    }
}

//
// Info about tokens owned by wallets
//
#[derive(Serialize, Deserialize, Clone)]
struct OwnerInfo {
    owners: HashMap<String, HashSet<u64>>
}

impl OwnerInfo {
    pub fn new() -> Self {
        OwnerInfo {
            owners: HashMap::new(),
        }
    }

    pub fn stringify_owners_tokens(&self, owner_id: &str) -> String {
        match self.owners.get(owner_id) {
            None => String::new(),
            Some(token_set) => {
                let list: Vec<&u64> = token_set
                    .iter()
                    .collect();
                serde_json::to_string(&list).unwrap()
            }
        }
    }

    pub fn transfer(&mut self, from: &str, to: &str, token: u64) -> Result<(), ()>{
        match self.owners.get_mut(from) {
            None => return Err(()),
            Some(owner) => {
                let valid = owner.remove(&token);
                if !valid {
                    return Err(());
                }
            }
        };
        let recipient = self.owners.entry(to.to_string()).or_insert(HashSet::new());
        recipient.insert(token);
        Ok(())
    }

    pub fn add_nft_to_id(&mut self, token_id: u64, id: &str) {
        let tokens = self.owners
            .entry(id.to_string())
            .or_insert(HashSet::new());
        tokens.insert(token_id);
    }
}

impl Display for OwnerInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stringified = serde_json::to_string(self).unwrap();
        f.write_str(&stringified);
        Ok(())
    }
}

impl FromStr for OwnerInfo {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Self = serde_json::from_str(s).unwrap();
        Ok(parsed)
    }
}

struct BigWinToken {
    tokens: OnChainVar<TokenMap>,
    owners: OnChainVar<OwnerInfo>,
    admins: OnChainVar<Vec<String>>,
    id_counter: OnChainVar<u64>
}

impl Contract for BigWinToken {
    fn new() -> Self {
        BigWinToken {
            tokens: OnChainVar::new("tokenInfo"),
            owners: OnChainVar::new("ownerInfo"),
            admins: OnChainVar::new("admins"),
            id_counter: OnChainVar::new("counter")
        }
    } 
}

#[contract_api]
impl NFTS1<BigWinToken> for BigWinToken {

    fn owner_of(&mut self, token_id: u64) -> String {
        let tokens = self.tokens.get().unwrap_or_else(|| {
            let fresh_map = TokenMap::new();
            self.tokens.set(fresh_map.clone());
            fresh_map
        });
        match tokens.tokens.get(&token_id) {
            None => String::new(),
            Some(token_data) => token_data.owner.to_string()
        }
    }

    fn owned_by(&mut self, owner: String) -> String {
        let owners = self.owners.get().unwrap_or_else(|| {
            let fresh_info = OwnerInfo::new();
            self.owners.set(fresh_info.clone());
            fresh_info
        });
        owners.stringify_owners_tokens(&owner)
    }

    fn mint(&mut self, args: MintArgs) {
        let mut owners = self.owners.get().unwrap_or_else(|| {
            let fresh_info = OwnerInfo::new();
            self.owners.set(fresh_info.clone());
            fresh_info
        });
        let mut tokens = self.tokens.get().unwrap_or_else(|| {
            let fresh_map = TokenMap::new();
            self.tokens.set(fresh_map.clone());
            fresh_map
        });
        let id = self.id_counter.get().unwrap_or(0) + 1;
        owners.add_nft_to_id(id, &args.owner);

        let info = TokenDetails{
            owner: args.owner,
            metadata_uri: args.metadata_uri
        };
        tokens.tokens.insert(id, info);

        self.owners.set(owners);
        self.tokens.set(tokens);
        self.id_counter.set(id);
    }

    fn transfer_from(&mut self, args: TransferFromArgs) {
        let mut owners = self.owners.get().unwrap_or_else(|| {
            let fresh_info = OwnerInfo::new();
            self.owners.set(fresh_info.clone());
            fresh_info
        });
        let mut tokens = self.tokens.get().unwrap_or_else(|| {
            let fresh_map = TokenMap::new();
            self.tokens.set(fresh_map.clone());
            fresh_map
        });

        let res = owners.transfer(&args.from, &args.to, args.token_id);
        if let Err(_) = res {
            return;
        }
        let token_data = tokens.tokens.get_mut(&args.token_id).unwrap();
        token_data.owner = args.to;

        self.owners.set(owners);
        self.tokens.set(tokens);
    }

    fn get_metadata(&mut self, token_id: u64) -> String {
        let tokens = self.tokens.get().unwrap_or_else(|| {
            let fresh_map = TokenMap::new();
            self.tokens.set(fresh_map.clone());
            fresh_map
        });
        match tokens.tokens.get(&token_id) {
            Some(token) => token.metadata_uri.clone(),
            None => String::new()
        }
    }
}
