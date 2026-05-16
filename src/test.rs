#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

fn setup() -> (Env, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    let token_addr = env.register_stellar_asset_contract_v2(admin.clone()).address();
    StellarAssetClient::new(&env, &token_addr).mint(&user, &1000);

    let contract_id = env.register_contract(None, SmartVaultContract);
    SmartVaultContractClient::new(&env, &contract_id).initialize(&token_addr);

    (env, contract_id, token_addr, user)
}

#[test]
fn test_deposit_and_withdraw_after_lock() {
    let (env, contract_id, token_addr, user) = setup();
    let client = SmartVaultContractClient::new(&env, &contract_id);
    let token = TokenClient::new(&env, &token_addr);

    client.deposit(&user, &500, &3600);
    assert_eq!(token.balance(&user), 500);
    assert_eq!(token.balance(&contract_id), 500);

    env.ledger().with_mut(|l| l.timestamp += 3601);
    client.withdraw(&user);

    assert_eq!(token.balance(&user), 1000);
    assert_eq!(token.balance(&contract_id), 0);
}

#[test]
#[should_panic(expected = "vault is still locked")]
fn test_withdraw_before_lock_expires_panics() {
    let (env, contract_id, _, user) = setup();
    let client = SmartVaultContractClient::new(&env, &contract_id);

    client.deposit(&user, &500, &3600);
    client.withdraw(&user);
}

#[test]
fn test_withdraw_at_exact_unlock_time() {
    let (env, contract_id, token_addr, user) = setup();
    let client = SmartVaultContractClient::new(&env, &contract_id);
    let token = TokenClient::new(&env, &token_addr);

    client.deposit(&user, &200, &100);
    env.ledger().with_mut(|l| l.timestamp += 100);
    client.withdraw(&user);

    assert_eq!(token.balance(&user), 1000);
}
