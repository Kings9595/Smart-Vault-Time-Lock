# Smart Vault Time-Lock

A secure, on-chain time-lock vault built on the **Stellar** blockchain using **Soroban** smart contracts. Users deposit native or wrapped Stellar assets that are locked until a specified future timestamp — no early withdrawals, no exceptions.

---

## Overview

Smart Vault Time-Lock enforces trustless, time-based asset custody directly on-chain. Once a deposit is made, the contract holds the funds and only releases them to the original depositor after the lock period expires. No admin keys, no escape hatches.

**Use cases:**
- Vesting schedules for token grants
- Savings commitments with enforced lock-ins
- Escrow with time-based release
- Delayed payment execution

---

## Architecture

The contract is composed of two core modules:

### Issue #2.1 — Struct Definitions & Deposit Interface

Defines the `VaultDeposit` storage struct and the `deposit` entry point.

```rust
#[contracttype]
pub struct VaultDeposit {
    pub amount: i128,
    pub unlock_time: u64,
}
```

**`deposit(env, user, amount, lock_duration)`**
- Requires `user` authorization
- Computes `unlock_time = current_ledger_timestamp + lock_duration`
- Transfers `amount` tokens from `user` to the contract via `token::Client`
- Stores a `VaultDeposit` entry keyed by `user`

### Issue #2.2 — Time Verification & Withdrawal Engine

Implements the `withdraw` entry point with full time and ownership checks.

**`withdraw(env, user)`**
- Requires `user` authorization
- Fetches current time via `env.ledger().timestamp()`
- Panics if `current_time < unlock_time` (lock still active)
- Transfers the exact deposited `amount` back to `user`
- Removes the vault entry from storage

---

## Contract Interface

| Function | Parameters | Description |
|---|---|---|
| `deposit` | `user: Address, amount: i128, lock_duration: u64` | Lock tokens for `lock_duration` seconds |
| `withdraw` | `user: Address` | Withdraw tokens after lock expires |

---

## Security Properties

- **No admin withdrawal** — only the depositing address can withdraw
- **Exact amount release** — contract transfers precisely what was deposited, no rounding
- **Ledger-time enforcement** — uses `env.ledger().timestamp()` (consensus time, not wall clock)
- **Authorization required** — both `deposit` and `withdraw` call `user.require_auth()`
- **Single deposit per address** — a second deposit overwrites only after the first is withdrawn

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) with `wasm32-unknown-unknown` target
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-cli)

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked stellar-cli --features opt
```

### Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

### Run Tests

```bash
cargo test
```

### Deploy (Testnet)

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/smart_vault_time_lock.wasm \
  --source <YOUR_SECRET_KEY> \
  --network testnet
```

### Invoke

```bash
# Deposit 100 tokens locked for 1 hour (3600 seconds)
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <YOUR_SECRET_KEY> \
  --network testnet \
  -- deposit \
  --user <USER_ADDRESS> \
  --amount 100 \
  --lock_duration 3600

# Withdraw after lock expires
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source <YOUR_SECRET_KEY> \
  --network testnet \
  -- withdraw \
  --user <USER_ADDRESS>
```

---

## Project Structure

```
smart-vault-time-lock/
├── src/
│   └── lib.rs          # Contract logic (VaultDeposit struct, deposit, withdraw)
├── Cargo.toml
└── README.md
```

---

## Contributing

Contributions are scoped to the open issues:

| Issue | Scope | Points |
|---|---|---|
| #2.1 | Struct definitions & deposit interface | 150 |
| #2.2 | Time verification & withdrawal engine | 150 |

Fork the repo, branch off `main`, and open a PR referencing the issue number.

---

## License

MIT
