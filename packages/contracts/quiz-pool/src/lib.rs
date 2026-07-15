#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub token: Address,
    pub entry_fee: i128,
}

#[contracttype]
pub enum DataKey {
    Admin,
    Config,
    RoundId,
}

#[contract]
pub struct QuizPoolContract;

#[contractimpl]
impl QuizPoolContract {
    pub fn initialize(env: Env, admin: Address, config: Config) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Config, &config);
        env.storage().instance().set(&DataKey::RoundId, &1u64);
    }

    pub fn join_round(env: Env, player: Address) -> u64 {
        let round_id: u64 = env.storage().instance().get(&DataKey::RoundId).unwrap_or(1);

        // Emit Event
        env.events().publish(
            (
                Symbol::new(&env, "quiz_pool"),
                Symbol::new(&env, "quiz_joined"),
                player,
            ),
            round_id,
        );

        round_id
    }

    pub fn commit_answer(env: Env, player: Address, round_id: u64, q_idx: u32, hash: BytesN<32>) {
        // Emit Event
        env.events().publish(
            (
                Symbol::new(&env, "quiz_pool"),
                Symbol::new(&env, "answer_committed"),
                player,
            ),
            (round_id, q_idx, hash),
        );
    }

    pub fn reveal_and_distribute(
        env: Env,
        round_id: u64,
        winners: Vec<Address>,
        amounts: Vec<i128>,
    ) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        // Increment round ID for the next round
        let next_round = round_id.checked_add(1).unwrap();
        env.storage().instance().set(&DataKey::RoundId, &next_round);

        // Emit Event
        env.events().publish(
            (
                Symbol::new(&env, "quiz_pool"),
                Symbol::new(&env, "round_distributed"),
                round_id,
            ),
            (winners, amounts),
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, testutils::Events, BytesN, Env, TryIntoVal};

    #[test]
    fn test_initialize_and_admin() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let config = Config {
            token: token.clone(),
            entry_fee: 100,
        };

        let contract_id = env.register(QuizPoolContract, ());
        let client = QuizPoolContractClient::new(&env, &contract_id);

        client.initialize(&admin, &config);
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn test_initialize_already_initialized() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let config = Config {
            token: token.clone(),
            entry_fee: 100,
        };

        let contract_id = env.register(QuizPoolContract, ());
        let client = QuizPoolContractClient::new(&env, &contract_id);

        client.initialize(&admin, &config);
        client.initialize(&admin, &config);
    }

    #[test]
    fn test_join_round_event() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let config = Config {
            token: token.clone(),
            entry_fee: 100,
        };

        let contract_id = env.register(QuizPoolContract, ());
        let client = QuizPoolContractClient::new(&env, &contract_id);

        client.initialize(&admin, &config);

        let player = Address::generate(&env);
        let round_id = client.join_round(&player);
        assert_eq!(round_id, 1);

        // Verify event immediately
        let all_events = env.events().all();
        let (event_contract, event_topics, event_data) = all_events.last().unwrap();
        assert_eq!(event_contract, contract_id);

        let topic_0: Symbol = event_topics.get(0).unwrap().try_into_val(&env).unwrap();
        let topic_1: Symbol = event_topics.get(1).unwrap().try_into_val(&env).unwrap();
        let topic_2: Address = event_topics.get(2).unwrap().try_into_val(&env).unwrap();

        assert_eq!(topic_0, Symbol::new(&env, "quiz_pool"));
        assert_eq!(topic_1, Symbol::new(&env, "quiz_joined"));
        assert_eq!(topic_2, player);

        let parsed_data: u64 = event_data.try_into_val(&env).unwrap();
        assert_eq!(parsed_data, 1);
    }

    #[test]
    fn test_commit_answer_event() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let config = Config {
            token: token.clone(),
            entry_fee: 100,
        };

        let contract_id = env.register(QuizPoolContract, ());
        let client = QuizPoolContractClient::new(&env, &contract_id);

        client.initialize(&admin, &config);

        let player = Address::generate(&env);
        let hash = BytesN::from_array(&env, &[0u8; 32]);

        client.commit_answer(&player, &1u64, &0u32, &hash);

        // Verify event immediately
        let all_events = env.events().all();
        let (event_contract, event_topics, event_data) = all_events.last().unwrap();
        assert_eq!(event_contract, contract_id);

        let topic_0: Symbol = event_topics.get(0).unwrap().try_into_val(&env).unwrap();
        let topic_1: Symbol = event_topics.get(1).unwrap().try_into_val(&env).unwrap();
        let topic_2: Address = event_topics.get(2).unwrap().try_into_val(&env).unwrap();

        assert_eq!(topic_0, Symbol::new(&env, "quiz_pool"));
        assert_eq!(topic_1, Symbol::new(&env, "answer_committed"));
        assert_eq!(topic_2, player);

        let parsed_data: (u64, u32, BytesN<32>) = event_data.try_into_val(&env).unwrap();
        assert_eq!(parsed_data.0, 1u64);
        assert_eq!(parsed_data.1, 0u32);
        assert_eq!(parsed_data.2, hash);
    }

    #[test]
    fn test_reveal_and_distribute_event() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token = Address::generate(&env);
        let config = Config {
            token: token.clone(),
            entry_fee: 100,
        };

        let contract_id = env.register(QuizPoolContract, ());
        let client = QuizPoolContractClient::new(&env, &contract_id);

        client.initialize(&admin, &config);

        let winner1 = Address::generate(&env);
        let winner2 = Address::generate(&env);
        let mut winners = Vec::new(&env);
        winners.push_back(winner1.clone());
        winners.push_back(winner2.clone());

        let mut amounts = Vec::new(&env);
        amounts.push_back(50i128);
        amounts.push_back(50i128);

        client.reveal_and_distribute(&1u64, &winners, &amounts);

        // Verify event immediately
        let all_events = env.events().all();
        let (event_contract, event_topics, event_data) = all_events.last().unwrap();
        assert_eq!(event_contract, contract_id);

        let topic_0: Symbol = event_topics.get(0).unwrap().try_into_val(&env).unwrap();
        let topic_1: Symbol = event_topics.get(1).unwrap().try_into_val(&env).unwrap();
        let topic_2: u64 = event_topics.get(2).unwrap().try_into_val(&env).unwrap();

        assert_eq!(topic_0, Symbol::new(&env, "quiz_pool"));
        assert_eq!(topic_1, Symbol::new(&env, "round_distributed"));
        assert_eq!(topic_2, 1u64);

        let parsed_data: (Vec<Address>, Vec<i128>) = event_data.try_into_val(&env).unwrap();
        assert_eq!(parsed_data.0.get(0).unwrap(), winner1);
        assert_eq!(parsed_data.0.get(1).unwrap(), winner2);
        assert_eq!(parsed_data.1.get(0).unwrap(), 50i128);
        assert_eq!(parsed_data.1.get(1).unwrap(), 50i128);
    }
}
