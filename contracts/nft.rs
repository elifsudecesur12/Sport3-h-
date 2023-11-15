use ic_cdk::export::candid::{CandidType, Principal};
use std::collections::HashMap;

#[derive(CandidType)]
struct NFT {
    owner: Principal,
    metadata: String,
}

#[derive(CandidType)]
struct NFTContract {
    nfts: HashMap<u32, NFT>,
    next_token_id: u32,
}

impl NFTContract {
    pub fn new() -> Self {
        NFTContract {
            nfts: HashMap::new(),
            next_token_id: 1,
        }
    }

    pub fn mint_nft(&mut self, owner: Principal, metadata: String) -> u32 {
        let token_id = self.next_token_id;
        self.nfts.insert(token_id, NFT { owner, metadata });
        self.next_token_id += 1;
        token_id
    }

    pub fn transfer_nft(&mut self, token_id: u32, new_owner: Principal) -> bool {
        if let Some(nft) = self.nfts.get_mut(&token_id) {
            if nft.owner == ic_cdk::caller() {
                nft.owner = new_owner;
                return true;
            }
        }
        false
    }
}


    pub fn get_nft_owner(&self, token_id: u32) -> Option<&Principal> {
        self.nfts.get(&token_id).map(|nft| &nft.owner)
    }

    pub fn get_nft_metadata(&self, token_id: u32) -> Option<&String> {
        self.nfts.get(&token_id).map(|nft| &nft.metadata)
    }

    pub fn get_nft_performance(&self, token_id: u32) -> Option<u32> {
        self.nfts.get(&token_id).map(|nft| nft.performance)
    }

    pub fn set_nft_performance(&mut self, token_id: u32, performance: u32) -> bool {
        if let Some(nft) = self.nfts.get_mut(&token_id) {
            nft.performance = performance;
            return true;
        }
        false
    }
}

fn assign_nft_to_player(player: Principal, metadata: String, performance: u32) -> CanisterResult {
    let mut fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();
    let mut nft_contract: NFTContract = ic_cdk::storage::get().unwrap_or_default();

    let token_id = nft_contract.mint_nft(player, metadata);

    nft_contract.set_nft_performance(token_id, performance);

    if let Some(player_info) = fair_play_contract.players.get_mut(&player) {
        player_info.owned_nfts.push(token_id);
        ic_cdk::storage::set(fair_play_contract);
        ic_cdk::storage::set(nft_contract);
        CanisterResult { message: "NFT assigned to player successfully.".to_string(), result: None }
    } else {
        CanisterResult { message: "Player not found.".to_string(), result: None }
    }
}

