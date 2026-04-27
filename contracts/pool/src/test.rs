#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env, String};

mod token_contract {
    soroban_sdk::contractimport!(file = "../target/wasm32v1-none/release/token.wasm");
}

#[test]
fn test_pool_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    // Register contracts
    let token_a_id = env.register_contract_wasm(None, token_contract::WASM);
    let token_b_id = env.register_contract_wasm(None, token_contract::WASM);
    let lp_token_id = env.register_contract_wasm(None, token_contract::WASM);

    let token_a = token_contract::Client::new(&env, &token_a_id);
    let token_b = token_contract::Client::new(&env, &token_b_id);
    let lp_token = token_contract::Client::new(&env, &lp_token_id);

    // Initialize tokens
    token_a.initialize(&admin, &admin, &String::from_str(&env, "Token A"), &String::from_str(&env, "TKNA"), &7);
    token_b.initialize(&admin, &admin, &String::from_str(&env, "Token B"), &String::from_str(&env, "TKNB"), &7);
    lp_token.initialize(&admin, &admin, &String::from_str(&env, "LP Token"), &String::from_str(&env, "LP"), &7);

    // Register pool
    let pool_id = env.register_contract(None, PoolContract);
    let pool = PoolContractClient::new(&env, &pool_id);

    pool.initialize(&admin, &token_a_id, &token_b_id, &lp_token_id, &30); // 30 bps fee

    // Mint tokens to user
    token_a.mint(&admin, &user, &10_000);
    token_b.mint(&admin, &user, &10_000);

    // Set LP token minter to pool
    lp_token.set_minter(&admin, &pool_id);

    // Add Liquidity
    pool.add_liquidity(&user, &1000, &1000, &0);
    
    let reserves = pool.get_reserves();
    assert_eq!(reserves.reserve_a, 1000);
    assert_eq!(reserves.reserve_b, 1000);
    assert_eq!(reserves.total_lp_supply, 1000); // sqrt(1000*1000)

    // Swap
    // Swap 100 Token A for Token B
    // fee = 100 * 30 / 10000 = 0 (integer math, wait 30 bps of 100 is 0.3)
    // Let's use larger numbers to see the fee
    token_a.mint(&admin, &user, &100_000);
    token_b.mint(&admin, &user, &100_000);
    pool.add_liquidity(&user, &10_000, &10_000, &0); // Total reserves: 11000, 11000

    let result = pool.swap(&user, &token_a_id, &1000, &0);
    // amount_in_with_fee = 1000 * (10000 - 30) = 1000 * 9970 = 9,970,000
    // numerator = 9,970,000 * 11000 = 109,670,000,000
    // denominator = 11000 * 10000 + 9,970,000 = 110,000,000 + 9,970,000 = 119,970,000
    // amount_out = 109,670,000,000 / 119,970,000 = 914.14... -> 914
    assert!(result.amount_out > 900);
    assert_eq!(result.fee_paid, 3); // 1000 * 30 / 10000 = 3
    
    // Stats
    let (swap_count, vol_a, vol_b) = pool.get_stats();
    assert_eq!(swap_count, 1);
    assert_eq!(vol_a, 1000);

    // Remove Liquidity
    let lp_balance = lp_token.balance(&user);
    pool.remove_liquidity(&user, &lp_balance, &0, &0);
    
    let final_reserves = pool.get_reserves();
    assert_eq!(final_reserves.total_lp_supply, 0);
}
