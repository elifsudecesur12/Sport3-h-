use ic_cdk::export::candid::{CandidType, Principal};
use std::collections::{HashMap, HashSet};

#[derive(Default, CandidType)]
struct Player {
    performance: u32,
    bonus_points: u32,
    owned_nfts: Vec<Principal>,
    difficulty_level: u32,
    preferences: HashSet<String>,
    statistics: HashMap<String, u32>, // Oyuncu istatistikleri, örneğin kazanılan turnuva sayısı, oynanan oyun sayısı vb.
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
fn update_player_profile(player: Principal, performance: u32, bonus_points: u32, statistics: HashMap<String, u32>) -> CanisterResult {
    let mut fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    if let Some(player_info) = fair_play_contract.players.get_mut(&player) {
        player_info.performance = performance;
        player_info.bonus_points = bonus_points;
        player_info.statistics = statistics;

        CanisterResult { message: "Player profile updated successfully.".to_string(), result: None }
    } else {
        CanisterResult { message: "Player not found.".to_string(), result: None }
    }
}

#[query]
fn get_player_profile(player: Principal) -> Option<Player> {
    let fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    fair_play_contract.players.get(&player).cloned()
}

#[query]
fn get_player_statistics(player: Principal) -> Option<HashMap<String, u32>> {
    let fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    fair_play_contract.players.get(&player).map(|player_info| player_info.statistics.clone())
}

#[query]
fn get_all_players() -> Vec<Principal> {
    let fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    fair_play_contract.players.keys().cloned().collect()
}
