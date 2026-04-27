#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env, String};

#[test]
fn test_factory_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let creator = Address::generate(&env);
    let pool_contract = Address::generate(&env);
    let token_a = Address::generate(&env);
    let token_b = Address::generate(&env);
    let lp_token = Address::generate(&env);

    let factory = env.register_contract(None, FactoryContract);
    let client = FactoryContractClient::new(&env, &factory);

    client.initialize(&admin);
    assert_eq!(client.get_count(), 0);

    let pool_id = client.register_pool(
        &creator,
        &pool_contract,
        &token_a,
        &token_b,
        &String::from_str(&env, "Token A"),
        &String::from_str(&env, "Token B"),
        &lp_token,
    );

    assert_eq!(pool_id, 0);
    assert_eq!(client.get_count(), 1);
    assert!(client.pair_exists(&token_a, &token_b));
    assert!(client.pair_exists(&token_b, &token_a));

    let pool_info = client.get_pool(&0);
    assert_eq!(pool_info.pool_contract, pool_contract);
    assert_eq!(pool_info.token_a, token_a);
    assert_eq!(pool_info.token_b, token_b);

    let pool_info_by_pair = client.get_pool_by_pair(&token_a, &token_b).unwrap();
    assert_eq!(pool_info_by_pair.pool_contract, pool_contract);

    let pools = client.get_pools(&0, &10);
    assert_eq!(pools.len(), 1);
}
