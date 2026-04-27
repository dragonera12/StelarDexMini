#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env, String};

#[test]
fn test_token_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let minter = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    let token = env.register_contract(None, TokenContract);
    let client = TokenContractClient::new(&env, &token);

    client.initialize(
        &admin,
        &minter,
        &String::from_str(&env, "Test Token"),
        &String::from_str(&env, "TST"),
        &7,
    );

    assert_eq!(client.name(), String::from_str(&env, "Test Token"));
    assert_eq!(client.symbol(), String::from_str(&env, "TST"));
    assert_eq!(client.decimals(), 7);

    // Minting
    client.mint(&minter, &user1, &1000);
    assert_eq!(client.balance(&user1), 1000);
    assert_eq!(client.total_supply(), 1000);

    // Transfer
    client.transfer(&user1, &user2, &400);
    assert_eq!(client.balance(&user1), 600);
    assert_eq!(client.balance(&user2), 400);

    // Approve & Transfer From
    client.approve(&user1, &admin, &200, &100);
    client.transfer_from(&admin, &user1, &user2, &200);
    assert_eq!(client.balance(&user1), 400);
    assert_eq!(client.balance(&user2), 600);

    // Burning
    client.burn(&user2, &100);
    assert_eq!(client.balance(&user2), 500);
    assert_eq!(client.total_supply(), 900);
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_already_initialized() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let token = env.register_contract(None, TokenContract);
    let client = TokenContractClient::new(&env, &token);

    client.initialize(
        &admin,
        &admin,
        &String::from_str(&env, "T"),
        &String::from_str(&env, "T"),
        &7,
    );
    client.initialize(
        &admin,
        &admin,
        &String::from_str(&env, "T"),
        &String::from_str(&env, "T"),
        &7,
    );
}

#[test]
#[should_panic(expected = "not authorized to mint")]
fn test_unauthorized_mint() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let minter = Address::generate(&env);
    let attacker = Address::generate(&env);
    let token = env.register_contract(None, TokenContract);
    let client = TokenContractClient::new(&env, &token);

    client.initialize(
        &admin,
        &minter,
        &String::from_str(&env, "T"),
        &String::from_str(&env, "T"),
        &7,
    );

    client.mint(&attacker, &attacker, &1000);
}
