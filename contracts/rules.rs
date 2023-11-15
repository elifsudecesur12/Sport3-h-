use ic_cdk::export::candid::{CandidType, Principal};
use std::collections::{HashMap, HashSet};

#[derive(Default, CandidType)]
struct TournamentRules {
    max_players: u32,
    min_players: u32,
    map_pool: HashSet<String>,
    // Diğer özel gereksinimlere yönelik ek alanlar eklenebilir
}

#[derive(Default, CandidType)]
struct Tournament {
    entry_fee: u32,
    name: String,
    description: String,
    game_advantage: String,
    total_supply: u32,
    remaining_supply: u32,
    rules: TournamentRules, // Turnuva kuralları burada saklanır
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
fn set_tournament_rules(tournament: Principal, rules: TournamentRules) -> CanisterResult {
    let mut fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    if let Some(tournament_info) = fair_play_contract.tournaments.get_mut(&tournament) {
        tournament_info.rules = rules;
        ic_cdk::storage::set(fair_play_contract);
        CanisterResult { message: "Tournament rules set successfully.".to_string(), result: None }
    } else {
        CanisterResult { message: "Tournament not found.".to_string(), result: None }
    }
}

#[update]
fn get_tournament_rules(tournament: Principal) -> TournamentRules {
    let fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    fair_play_contract.tournaments.get(&tournament)
        .map_or_else(|| TournamentRules::default(), |tournament_info| tournament_info.rules.clone())
}

#[update]
fn update_tournament_rules(tournament: Principal, rules: TournamentRules) -> CanisterResult {
    let mut fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    if let Some(tournament_info) = fair_play_contract.tournaments.get_mut(&tournament) {
        tournament_info.rules = rules;
        ic_cdk::storage::set(fair_play_contract);
        CanisterResult { message: "Tournament rules updated successfully.".to_string(), result: None }
    } else {
        CanisterResult { message: "Tournament not found.".to_string(), result: None }
    }
}
