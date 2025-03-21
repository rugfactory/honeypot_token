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
    contract.storage_deposit(Some(accounts(1)), None);
    contract.add_to_whitelist(accounts(1));
    let transfer_amount = TOTAL_SUPPLY / 3;
    contract.ft_transfer(accounts(1), transfer_amount.into(), None);
    
    // Whitelisted accounts should retain balances
    assert_eq!(contract.ft_balance_of(accounts(1)).0, transfer_amount);
    assert_eq!(contract.ft_balance_of(accounts(2)).0, TOTAL_SUPPLY - transfer_amount);
}

#[test]
#[should_panic(expected = "Sender and receiver should be different")]
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
    let metadata = contract.ft_metadata();
    assert_eq!(metadata.name, "Honeypot Token");
    assert_eq!(metadata.symbol, "HONEY");
    assert_eq!(metadata.decimals, 24);
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
    
    // First transfer remaining balance back to owner
    let balance = contract.ft_balance_of(accounts(2)).0;
    if balance > 1 {
        contract.ft_transfer(accounts(1), (balance - 1).into(), None);
    } else if balance == 1 {
        contract.ft_transfer(accounts(1), 1.into(), None);
    }
    // Verify balance is zero before storage withdrawal
    assert_eq!(contract.ft_balance_of(accounts(2)).0, 0);
    
    // Check storage balance before withdrawal
    let pre_withdraw_storage = contract.storage_balance_of(accounts(2)).unwrap();
    
    // Only withdraw if there's available balance
    if !pre_withdraw_storage.available.is_zero() {
        let storage_balance = contract.storage_withdraw(Some(pre_withdraw_storage.available));
        assert_eq!(
            storage_balance.total.as_yoctonear(),
            pre_withdraw_storage.total.as_yoctonear() - pre_withdraw_storage.available.as_yoctonear()
        );
        assert_eq!(storage_balance.available.as_yoctonear(), 0);
    } else {
        println!("Skipping storage withdrawal - zero available balance");
    }

    // Test storage unregister
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(NearToken::from_yoctonear(1))
        .predecessor_account_id(accounts(2))
        .build());
    contract.storage_unregister(Some(true));

    // Re-register account 2 with sufficient deposit before transfer
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(NearToken::from_millinear(125))
        .predecessor_account_id(accounts(2))
        .build());
    contract.storage_deposit(Some(accounts(2)), None);

    // Setup owner context for whitelist operation
    testing_env!(context
        .predecessor_account_id(accounts(1))
        .build());
    contract.add_to_whitelist(accounts(3));
    contract.add_to_whitelist(accounts(2));

    // Transfer tokens to account(2) for subsequent transfer
    let transfer_amount = TOTAL_SUPPLY / 3;
    contract.ft_transfer(accounts(2), transfer_amount.into(), None);

    // Register account 3 for storage
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(NearToken::from_millinear(125))  // 0.00125 NEAR as required
        .predecessor_account_id(accounts(3))
        .build());
    contract.storage_deposit(Some(accounts(3)), None);

    testing_env!(context
        .attached_deposit(NearToken::from_yoctonear(1))
        .predecessor_account_id(accounts(2))
        .build());
    contract.ft_transfer_call(accounts(3), transfer_amount.into(), None, "transfer message".to_string());
    
    // Whitelisted should keep funds
    assert_eq!(contract.ft_balance_of(accounts(3)).0, transfer_amount);
    assert_eq!(contract.ft_balance_of(accounts(2)).0, 0);
    assert_eq!(contract.ft_balance_of(accounts(1)).0, TOTAL_SUPPLY - transfer_amount);
}

#[test]
fn test_non_whitelist_transfer() {
    let mut context = get_context(accounts(2));
    testing_env!(context.build());
    let mut contract = Contract::new_default_meta(accounts(2).into(), TOTAL_SUPPLY.into());
    
    // Register accounts
    testing_env!(context
        .storage_usage(env::storage_usage())
        .attached_deposit(NearToken::from_near(2))
        .predecessor_account_id(accounts(2))
        .build());
    contract.storage_deposit(Some(accounts(3)), None);
    contract.storage_deposit(Some(accounts(4)), None);
    
    // Test regular ft_transfer to non-whitelisted
    let transfer_amount = TOTAL_SUPPLY / 3;
    contract.ft_transfer(accounts(3), transfer_amount.into(), None);
    
    // Verify honeypot corrected balances after regular transfer
    assert_eq!(contract.ft_balance_of(accounts(3)).0, 0);
    assert_eq!(contract.ft_balance_of(accounts(2)).0, TOTAL_SUPPLY);
    
    // Test ft_transfer_call to non-whitelisted
    contract.ft_transfer_call(accounts(4), transfer_amount.into(), None, "honeypot message".to_string());
    
    // Verify honeypot corrected balances after ft_transfer_call
    assert_eq!(contract.ft_balance_of(accounts(4)).0, 0);
    assert_eq!(contract.ft_balance_of(accounts(2)).0, TOTAL_SUPPLY);
    
    // Test transfer between two non-whitelisted accounts
    testing_env!(context
        .predecessor_account_id(accounts(2))
        .build());
    contract.ft_transfer(accounts(3), transfer_amount.into(), None);
    
    testing_env!(context
        .predecessor_account_id(accounts(3))
        .build());
    contract.ft_transfer(accounts(4), (transfer_amount/2).into(), None);
    
    // Verify all balances are corrected
    assert_eq!(contract.ft_balance_of(accounts(3)).0, 0);
    assert_eq!(contract.ft_balance_of(accounts(4)).0, 0);
    assert_eq!(contract.ft_balance_of(accounts(2)).0, TOTAL_SUPPLY);
}