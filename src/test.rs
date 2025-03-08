use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{testing_env, AccountId, NearToken};
use near_contract_standards::fungible_token::Balance;

use crate::*;

const TOTAL_SUPPLY: Balance = 1_000_000_000_000_000;

fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
    let mut builder = VMContextBuilder::new();
    builder
        .current_account_id(accounts(0))
        .signer_account_id(predecessor_account_id.clone())
        .predecessor_account_id(predecessor_account_id);
    builder
}

#[test]
fn test_new() {
    let mut context = get_context(accounts(1));
    testing_env!(context.build());
    let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
    testing_env!(context.is_view(true).build());
    assert_eq!(contract.ft_total_supply().0, TOTAL_SUPPLY);
    assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY);
}

#[test]
fn test_transfer() {
    let mut context = get_context(accounts(2));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());

    // Register accounts(1) for storage
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(NearToken::from_near(1))
        .predecessor_account_id(accounts(1))
        .build());
    contract.storage_deposit(None, None);

    // Perform the transfer
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(NearToken::from_yoctonear(1))
        .predecessor_account_id(accounts(2))
        .build());
    let transfer_amount = TOTAL_SUPPLY / 3;
    contract.ft_transfer(accounts(1), transfer_amount.into(), None);
    assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    assert_eq!(
        contract.ft_balance_of(accounts(2)).0,
        TOTAL_SUPPLY - transfer_amount
    );
}

#[test]
#[should_panic(expected = "Self transfers are not allowed")]
fn test_self_transfer_fail() {
    let mut context = get_context(accounts(2));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(NearToken::from_yoctonear(1))
        .predecessor_account_id(accounts(2))
        .build());
    contract.ft_transfer(accounts(2), (TOTAL_SUPPLY / 3).into(), None);
}

#[test]
fn test_metadata() {
    let context = get_context(accounts(1));
    testing_env!(context.build());
    let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
    assert_eq!(contract.ft_metadata().spec, FT_METADATA_SPEC);
    assert_eq!(contract.ft_metadata().name, "Fungible Token");
    assert_eq!(contract.ft_metadata().symbol, "FT");
    assert_eq!(contract.ft_metadata().decimals, 24);
}

#[test]
fn test_rugfactory_owner_check() {
    let context = get_context(accounts(1));
    testing_env!(context.build());
    let contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
    assert_eq!(contract.rugfactory_owner_check(), accounts(1));
}

#[test]
#[should_panic(expected = "Only the owner can delete the token")]
fn test_rugfactory_token_delete_unauthorized() {
    let mut context = get_context(accounts(1));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());
    testing_env!(context
        .predecessor_account_id(accounts(2))
        .build());
    contract.rugfactory_token_delete();
}

#[test]
fn test_storage_deposit() {
    let mut context = get_context(accounts(1));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());

    // Test storage deposit with minimum amount
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(NearToken::from_near(1))
        .predecessor_account_id(accounts(2))
        .build());
    let storage_balance = contract.storage_deposit(None, None);
    assert!(storage_balance.total.as_yoctonear() > 0);
    assert_eq!(storage_balance.available.as_yoctonear(), 0);

    // Test storage deposit with registration only
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(NearToken::from_near(1))
        .predecessor_account_id(accounts(3))
        .build());
    let storage_balance = contract.storage_deposit(None, Some(true));
    assert!(storage_balance.total.as_yoctonear() > 0);
}

#[test]
fn test_storage_withdraw_and_unregister() {
    let mut context = get_context(accounts(1));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(1).into(), TOTAL_SUPPLY.into());

    // Register account first
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(NearToken::from_near(1))
        .predecessor_account_id(accounts(2))
        .build());
    contract.storage_deposit(None, None);

    // Test storage withdraw
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(NearToken::from_yoctonear(1))
        .predecessor_account_id(accounts(2))
        .build());
    let storage_balance = contract.storage_withdraw(None);
    assert_eq!(storage_balance.available.as_yoctonear(), 0);

    // Test storage unregister
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(NearToken::from_yoctonear(1))
        .predecessor_account_id(accounts(2))
        .build());
    let storage_balance = contract.storage_unregister(Some(true));
    assert!(storage_balance);
}

#[test]
fn test_ft_transfer_call() {
    let mut context = get_context(accounts(2));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());

    // Register receiver account
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(NearToken::from_near(1))
        .predecessor_account_id(accounts(3))
        .build());
    contract.storage_deposit(None, None);

    // Perform transfer call
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(NearToken::from_yoctonear(1))
        .predecessor_account_id(accounts(2))
        .build());
    let transfer_amount = TOTAL_SUPPLY / 3;
    contract.ft_transfer_call(accounts(3), transfer_amount.into(), None, "transfer message".to_string());

    // Verify balances after transfer
    assert_eq!(contract.ft_balance_of(accounts(3)).0, transfer_amount);
    assert_eq!(contract.ft_balance_of(accounts(2)).0, TOTAL_SUPPLY - transfer_amount);
}