# fungible_token

a fungible token for rugfactory with a few extra methods

---

## Building and deploying

```bash
cargo build
cargo near build
cargo near build reproducible-wasm
cargo near abi

cargo check
cargo test
cargo clean


cargo near deploy build-reproducible-wasm <account-id>
near deploy <account-id> <wasm/path>

```


---


### Token Methods

This contract implements the NEP-141 Fungible Token standard with the following methods:

#### Initialization

```bash
# Initialize with default metadata
near call <contract-id> new_default_meta '{"owner_id": "<owner-account>", "total_supply": "1000000000000000000000000000"}' --accountId <owner-account>

# Initialize with custom metadata
near call <contract-id> new '{"owner_id": "<owner-account>", "total_supply": "1000000000000000000000000000", "metadata": {"spec": "ft-1.0.0", "name": "My Token", "symbol": "TOKEN", "decimals": 24}}' --accountId <owner-account>
```

#### Core Methods

```bash
# View total supply
near view <contract-id> ft_total_supply

# View balance of an account
near view <contract-id> ft_balance_of '{"account_id": "<account-id>"}'  

# Transfer tokens
near call <contract-id> ft_transfer '{"receiver_id": "<receiver-account>", "amount": "<amount>"}' --accountId <sender-account> --depositYocto 1

# Transfer tokens with memo
near call <contract-id> ft_transfer '{"receiver_id": "<receiver-account>", "amount": "<amount>", "memo": "<memo>"}' --accountId <sender-account> --depositYocto 1
```

#### Storage Management

```bash
# Register account for receiving tokens
near call <contract-id> storage_deposit '' --accountId <account-id> --amount 0.00125

# View storage balance
near view <contract-id> storage_balance_of '{"account_id": "<account-id>"}'  

# Unregister account and withdraw storage deposit
near call <contract-id> storage_unregister '{"force": true}' --accountId <account-id>
```

#### Metadata

```bash
# View token metadata
near view <contract-id> ft_metadata
```

Note: Replace `<contract-id>`, `<account-id>`, `<owner-account>`, `<sender-account>`, `<receiver-account>`, and `<amount>` with actual values. The `--depositYocto 1` is required for transfers as a security measure.







---

copyright: 2025 by sleet.near, in partnership with huggies.near

