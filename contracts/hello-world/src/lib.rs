#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, Vec};

#[contract]
pub struct PredictionMarket;

#[derive(Clone)]
#[contracttype]
pub struct Market {
    pub question: Symbol,
    pub outcomes: Vec<Symbol>,
    pub resolved: bool,
    pub winner: Option<Symbol>,
}

#[contractimpl]
impl PredictionMarket {

    pub fn create_market(env: Env, id: Symbol, question: Symbol, outcomes: Vec<Symbol>) {
        let market = Market {
            question,
            outcomes,
            resolved: false,
            winner: None,
        };

        env.storage().instance().set(&id, &market);
    }

    pub fn resolve_market(env: Env, id: Symbol, winner: Symbol) {
        let mut market: Market = env.storage().instance().get(&id).unwrap();

        if market.resolved {
            panic!("Already resolved");
        }

        market.resolved = true;
        market.winner = Some(winner);

        env.storage().instance().set(&id, &market);
    }

    pub fn get_market(env: Env, id: Symbol) -> Market {
        env.storage().instance().get(&id).unwrap()
    }
}