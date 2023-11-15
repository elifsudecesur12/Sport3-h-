use ic_cdk::export::candid::{CandidType, Principal};
use std::collections::{HashMap, HashSet};

#[derive(Default, CandidType)]
struct Player {
    performance: u32,
    bonus_points: u32,
    owned_nfts: Vec<Principal>,
    difficulty_level: u32,
    preferences: HashSet<String>,
    statistics: HashMap<String, u32>,
}

#[derive(Default, CandidType)]
struct MatchResult {
    winner: Principal,
    loser: Principal,
    score: String,
}

#[derive(Default, CandidType)]
struct Tournament {
    entry_fee: u32,
    name: String,
    description: String,
    game_advantage: String,
    total_supply: u32,
    remaining_supply: u32,
    participants: HashSet<Principal>,
    match_results: Vec<MatchResult>,
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
fn register_player_for_tournament(player: Principal, tournament: Principal) -> CanisterResult {
    let mut fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    if let Some(tournament_info) = fair_play_contract.tournaments.get_mut(&tournament) {
        tournament_info.participants.insert(player);

        CanisterResult { message: "Player registered for the tournament successfully.".to_string(), result: None }
    } else {
        CanisterResult { message: "Tournament not found.".to_string(), result: None }
    }
}

#[update]
fn submit_match_result(tournament: Principal, match_result: MatchResult) -> CanisterResult {
    let mut fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    if let Some(tournament_info) = fair_play_contract.tournaments.get_mut(&tournament) {
        tournament_info.match_results.push(match_result);

        if let Some(winner_info) = fair_play_contract.players.get_mut(&match_result.winner) {
            winner_info.statistics.entry("Wins".to_string()).and_modify(|v| *v += 1);
        }

        if let Some(loser_info) = fair_play_contract.players.get_mut(&match_result.loser) {
            loser_info.statistics.entry("Losses".to_string()).and_modify(|v| *v += 1);
        }

        CanisterResult { message: "Match result submitted successfully.".to_string(), result: None }
    } else {
        CanisterResult { message: "Tournament not found.".to_string(), result: None }
    }
}
