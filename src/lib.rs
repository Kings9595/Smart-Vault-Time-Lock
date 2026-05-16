#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
pub struct VaultDeposit {
    pub amount: i128,
    pub unlock_time: u64,
}

#[contracttype]
pub enum DataKey {
    Vault(Address),
    Token,
}

#[contract]
pub struct SmartVaultContract;

#[contractimpl]
impl SmartVaultContract {
    /// Initialize the contract with the token address it will custody.
    pub fn initialize(env: Env, token: Address) {
        env.storage().instance().set(&DataKey::Token, &token);
    }

    /// Lock `amount` tokens for `lock_duration` seconds.
    pub fn deposit(env: Env, user: Address, amount: i128, lock_duration: u64) {
        user.require_auth();

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let unlock_time = env.ledger().timestamp() + lock_duration;

        token::Client::new(&env, &token_addr).transfer(
            &user,
            &env.current_contract_address(),
            &amount,
        );

        env.storage()
            .persistent()
            .set(&DataKey::Vault(user), &VaultDeposit { amount, unlock_time });
    }

    /// Withdraw tokens after the lock has expired.
    pub fn withdraw(env: Env, user: Address) {
        user.require_auth();

        let key = DataKey::Vault(user.clone());
        let vault: VaultDeposit = env.storage().persistent().get(&key).unwrap();

        assert!(
            env.ledger().timestamp() >= vault.unlock_time,
            "vault is still locked"
        );

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        token::Client::new(&env, &token_addr).transfer(
            &env.current_contract_address(),
            &user,
            &vault.amount,
        );

        env.storage().persistent().remove(&key);
    }
}

#[cfg(test)]
mod test;
