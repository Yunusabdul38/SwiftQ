#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlayerStats {
    pub address: Address,
    pub total_score: u32,
    pub rounds_played: u32,
    pub last_active_timestamp: u64,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Players,
    Stats(Address),
}

#[contract]
pub struct LeaderboardContract;

#[contractimpl]
impl LeaderboardContract {
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        
        // Initialize the players list if it doesn't exist
        if !env.storage().persistent().has(&DataKey::Players) {
            let empty_players: Vec<Address> = Vec::new(&env);
            env.storage().persistent().set(&DataKey::Players, &empty_players);
        }
    }

    pub fn submit_score(env: Env, player: Address, score: u32) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        let stats_key = DataKey::Stats(player.clone());
        let stats = if let Some(mut existing_stats) = env.storage().persistent().get::<_, PlayerStats>(&stats_key) {
            existing_stats.total_score = existing_stats.total_score.checked_add(score).expect("score overflow");
            existing_stats.rounds_played = existing_stats.rounds_played.checked_add(1).expect("rounds overflow");
            existing_stats.last_active_timestamp = env.ledger().timestamp();
            existing_stats
        } else {
            // New player, add to players list
            let mut players: Vec<Address> = env.storage().persistent().get(&DataKey::Players).unwrap_or_else(|| Vec::new(&env));
            players.push_back(player.clone());
            env.storage().persistent().set(&DataKey::Players, &players);

            PlayerStats {
                address: player.clone(),
                total_score: score,
                rounds_played: 1,
                last_active_timestamp: env.ledger().timestamp(),
            }
        };

        env.storage().persistent().set(&stats_key, &stats);
    }

    pub fn get_player_score(env: Env, player: Address) -> Option<PlayerStats> {
        let stats_key = DataKey::Stats(player);
        env.storage().persistent().get(&stats_key)
    }

    pub fn get_top_players(env: Env, limit: u32) -> Vec<PlayerStats> {
        let players: Vec<Address> = env.storage().persistent().get(&DataKey::Players).unwrap_or_else(|| Vec::new(&env));
        let mut result = Vec::new(&env);
        
        for player in players.iter() {
            if let Some(stats) = env.storage().persistent().get::<_, PlayerStats>(&DataKey::Stats(player)) {
                result.push_back(stats);
            }
        }
        
        // Bubble sort descending by total_score
        let len = result.len();
        if len > 1 {
            for i in 0..len {
                for j in 0..len - 1 - i {
                    let stats_j = result.get(j).unwrap();
                    let stats_j1 = result.get(j + 1).unwrap();
                    if stats_j.total_score < stats_j1.total_score {
                        result.set(j, stats_j1);
                        result.set(j + 1, stats_j);
                    }
                }
            }
        }
        
        // Take up to `limit` elements
        let mut limited_result = Vec::new(&env);
        let limit_usize = limit as usize;
        let mut count = 0;
        for stats in result.iter() {
            if count >= limit_usize {
                break;
            }
            limited_result.push_back(stats);
            count += 1;
        }
        
        limited_result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_initialize_and_admin() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let contract_id = env.register(LeaderboardContract, ());
        let client = LeaderboardContractClient::new(&env, &contract_id);

        client.initialize(&admin);
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn test_initialize_already_initialized() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let contract_id = env.register(LeaderboardContract, ());
        let client = LeaderboardContractClient::new(&env, &contract_id);

        client.initialize(&admin);
        client.initialize(&admin);
    }

    #[test]
    fn test_submit_score_accumulation() {
        let env = Env::default();
        env.mock_all_auths();
        
        let admin = Address::generate(&env);
        let contract_id = env.register(LeaderboardContract, ());
        let client = LeaderboardContractClient::new(&env, &contract_id);

        client.initialize(&admin);

        let player = Address::generate(&env);
        
        // Submit score first time
        client.submit_score(&player, &100);
        let stats = client.get_player_score(&player).unwrap();
        assert_eq!(stats.total_score, 100);
        assert_eq!(stats.rounds_played, 1);
        
        // Submit score second time
        client.submit_score(&player, &150);
        let stats = client.get_player_score(&player).unwrap();
        assert_eq!(stats.total_score, 250);
        assert_eq!(stats.rounds_played, 2);
    }

    #[test]
    #[should_panic]
    fn test_unauthorized_submit_score() {
        let env = Env::default();
        // Do not call mock_all_auths to verify auth check panics
        
        let admin = Address::generate(&env);
        let contract_id = env.register(LeaderboardContract, ());
        let client = LeaderboardContractClient::new(&env, &contract_id);

        client.initialize(&admin);

        let player = Address::generate(&env);
        client.submit_score(&player, &100);
    }

    #[test]
    fn test_top_players_sorting() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let contract_id = env.register(LeaderboardContract, ());
        let client = LeaderboardContractClient::new(&env, &contract_id);

        client.initialize(&admin);

        let player1 = Address::generate(&env);
        let player2 = Address::generate(&env);
        let player3 = Address::generate(&env);

        client.submit_score(&player1, &50);
        client.submit_score(&player2, &150);
        client.submit_score(&player3, &100);

        let top_players = client.get_top_players(&3);
        assert_eq!(top_players.len(), 3);
        
        // Check order: player2 (150), player3 (100), player1 (50)
        assert_eq!(top_players.get(0).unwrap().address, player2);
        assert_eq!(top_players.get(0).unwrap().total_score, 150);
        
        assert_eq!(top_players.get(1).unwrap().address, player3);
        assert_eq!(top_players.get(1).unwrap().total_score, 100);
        
        assert_eq!(top_players.get(2).unwrap().address, player1);
        assert_eq!(top_players.get(2).unwrap().total_score, 50);

        // Check limit
        let top_2 = client.get_top_players(&2);
        assert_eq!(top_2.len(), 2);
        assert_eq!(top_2.get(0).unwrap().address, player2);
        assert_eq!(top_2.get(1).unwrap().address, player3);
    }
}
