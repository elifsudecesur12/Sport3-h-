use ic_cdk::export::candid::{CandidType, Principal};
use std::collections::{HashMap, HashSet};

#[derive(Default, CandidType)]
struct Player {
    performance: u32,
    bonus_points: u32,
    owned_nfts: Vec<Principal>,
    difficulty_level: u32,
    preferences: HashSet<String>, 
}

#[derive(Default, CandidType)]
struct Advertisement {
    sponsor: Principal,
    content: String,
    target_players: HashSet<Principal>, 
}

#[derive(Default, CandidType)]
struct SponsorshipOpportunity {
    sponsor: Principal,
    benefits: String,
    requirements: HashSet<String>, 
}

#[derive(Default, CandidType)]
struct FairPlayContract {
    players: HashMap<Principal, Player>,
    tournaments: HashMap<Principal, Tournament>,
    bonus_pool: u32,
    prize_pools: HashMap<Principal, u32>,
    advertisements: Vec<Advertisement>,
    sponsorship_opportunities: Vec<SponsorshipOpportunity>,
}

#[derive(Default, CandidType)]
struct CanisterResult {
    message: String,
    result: Option<Principal>,
}

#[update]
fn personalize_advertisement(player: Principal, content: String) -> CanisterResult {
    let mut fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    let advertisement = Advertisement {
        sponsor: Principal::anonymous(), 
        content,
        target_players: HashSet::new(),
    };

    if let Some(player_info) = fair_play_contract.players.get_mut(&player) {
        player_info.preferences.iter().for_each(|preference| {
            fair_play_contract.advertisements.push(Advertisement {
                sponsor: advertisement.sponsor,
                content: advertisement.content.clone(),
                target_players: fair_play_contract.players.values()
                    .filter(|other_player| other_player.preferences.contains(preference))
                    .map(|other_player| *other_player.principal())
                    .collect(),
            });
        });

        CanisterResult { message: "Advertisement personalized successfully.".to_string(), result: None }
    } else {
        CanisterResult { message: "Player not found.".to_string(), result: None }
    }
}

#[update]
fn create_sponsorship_opportunity(sponsor: Principal, benefits: String, requirements: HashSet<String>) -> CanisterResult {
    let mut fair_play_contract: FairPlayContract = ic_cdk::storage::get().unwrap_or_default();

    let sponsorship_opportunity = SponsorshipOpportunity {
        sponsor,
        benefits,
        requirements,
    };

    fair_play_contract.sponsorship_opportunities.push(sponsorship_opportunity);

    CanisterResult { message: "Sponsorship opportunity created successfully.".to_string(), result: None }
}
