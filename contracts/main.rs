use ic_cdk::export::candid::{CandidType, Principal};
use std::collections::{HashMap, HashSet};

#[derive(Default, CandidType)]
struct Player {
    performance: u32,
    bonus_points: u32,
    owned_nfts: Vec<Principal>,
}

#[derive(Default, CandidType)]
struct Tournament {
    entry_fee: u32,
    name: String,
    description: String,
    game_advantage: String,
    total_supply: u32,
    remaining_supply: u32,
}

#[derive(Default, CandidType)]
struct FairPlayContract {
    players: HashMap<Principal, Player>,
    tournaments: HashMap<Principal, Tournament>,
    bonus_pool: u32,
    prize_pools: HashMap<Principal, u32>,
}

#[derive(Default, CandidType)]
struct CanisterResult {
    message: String,
    result: Option<Principal>,
}

#[update]
fn register_new_player(player: Principal, initial_performance: u32) -> CanisterResult {
    let mut fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    if !fair_play_contract.players.contains_key(&player) {
        fair_play_contract.players.insert(player, Player {
            performance: initial_performance,
            bonus_points: 0,
            owned_nfts: Vec::new(),
        });

        ic_cdk::storage::set(fair_play_contract);
        CanisterResult { message: "Player registered successfully.".to_string(), result: None }
    } else {
        CanisterResult { message: "Player already registered.".to_string(), result: None }
    }
}

#[update]
fn update_player_performance(player: Principal, performance: u32) -> CanisterResult {
    let mut fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    if let Some(player_info) = fair_play_contract.players.get_mut(&player) {
        player_info.performance = performance;
        ic_cdk::storage::set(fair_play_contract);
        CanisterResult { message: "Player performance updated successfully.".to_string(), result: None }
    } else {
        CanisterResult { message: "Player not found.".to_string(), result: None }
    }
}

#[update]
fn add_bonus_points(player: Principal, points: u32) -> CanisterResult {
    let mut fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    if let Some(player_info) = fair_play_contract.players.get_mut(&player) {
        player_info.bonus_points += points;
        fair_play_contract.bonus_pool += points;

        ic_cdk::storage::set(fair_play_contract);
        CanisterResult { message: "Bonus points added successfully.".to_string(), result: None }
    } else {
        CanisterResult { message: "Player not found.".to_string(), result: None }
    }
}

#[update]
fn start_tournament(tournament: Principal, entry_fee: u32) -> CanisterResult {
    let mut fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    if !fair_play_contract.tournaments.contains_key(&tournament) {
        fair_play_contract.tournaments.insert(tournament, Tournament {
            name: "Example".to_string(),
            entry_fee,
            description: "".to_string(),
            game_advantage: "".to_string(),
            total_supply: 0,
            remaining_supply: 0,
        });

        fair_play_contract.prize_pools.insert(tournament, 0);

        ic_cdk::storage::set(fair_play_contract);
        CanisterResult { message: "Tournament started successfully.".to_string(), result: None }
    } else {
        CanisterResult { message: "Tournament already started.".to_string(), result: None }
    }
}

#[update]
fn end_tournament(tournament: Principal) -> CanisterResult {
    let mut fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    if let Some(tournament_info) = fair_play_contract.tournaments.remove(&tournament) {
        let winner = fair_play_contract.players.iter().max_by_key(|(_, player)| player.performance);

        if let Some((winner, _)) = winner {
            if let Some(prize_pool) = fair_play_contract.prize_pools.get_mut(&tournament) {
                let first_place_reward = (*prize_pool * 70) / 100;
                distribute_prize(&mut fair_play_contract, winner, first_place_reward);
            }
        }

        ic_cdk::storage::set(fair_play_contract);
        CanisterResult { message: "Tournament ended successfully.".to_string(), result: winner.map(|(player, _)| *player) }
    } else {
        CanisterResult { message: "Tournament not found.".to_string(), result: None }
    }
}

#[update]
fn distribute_prize(player: &Principal, amount: u32) -> CanisterResult {
    let mut fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    if let Some(player_info) = fair_play_contract.players.get_mut(player) {
      

        ic_cdk::storage::set(fair_play_contract);
        CanisterResult { message: "Prize distributed successfully.".to_string(), result: None }
    } else {
        CanisterResult { message: "Player not found.".to_string(), result: None }
    }
}
fn assign_nft_to_player(player: Principal, metadata: String) -> CanisterResult {
    let mut fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();
    let mut nft_contract: NFTContract = ic_cdk::storage::get().unwrap_or_default();

    // Mint a new NFT
    let token_id = nft_contract.mint_nft(player, metadata);

    // Update player's owned NFTs
    if let Some(player_info) = fair_play_contract.players.get_mut(&player) {
        player_info.owned_nfts.push(token_id);
        ic_cdk::storage::set(fair_play_contract);
        ic_cdk::storage::set(nft_contract);
        CanisterResult { message: "NFT assigned to player successfully.".to_string(), result: None }
    } else {
        CanisterResult { message: "Player not found.".to_string(), result: None }
    }
}
