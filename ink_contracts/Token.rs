#![cfg_attr(not(feature = "std"), no_std, no_main)] 
 


#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod token {
    // Some string type variables to identify the token.
    // The `pub` identifier makes a variable readable from outside the contract.
    #[ink(storage)]
    pub struct Token {
        name: String,
        symbol: String,
        total_supply: u64,
        owner: AccountId,
        balances: ink_storage::collections::HashMap<AccountId, u64>,
    }

    impl Token {
        /// Constructor that initializes the name, symbol and total supply of the token.
        #[ink(constructor)]
        pub fn new(name: String, symbol: String, total_supply: u64) -> Self {
            // Set the deploying account as the owner of the token.
            let caller = Self::env().caller();
            let mut balances = ink_storage::collections::HashMap::new();
            balances.insert(caller, total_supply);

            Self {
                name,
                symbol,
                total_supply,
                owner: caller,
                balances,
            }
        }

        /// A function to transfer tokens.
        ///
        /// Transfers `amount` tokens from the caller's account to the `to` account.
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, amount: u64) -> Result<(), Error> {
            let caller = self.env().caller();
            let sender_balance = self.balance_of(caller);
            if sender_balance < amount {
                return Err(Error::NotEnoughTokens);
            }
            self.balances.insert(caller, sender_balance - amount);
            let receiver_balance = self.balance_of(to);
            self.balances.insert(to, receiver_balance + amount);
            Ok(())
        }

        /// Read only function to retrieve the token balance of a given account.
        ///
        /// Returns the token balance of the `owner` account.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u64 {
            *self.balances.get(&owner).unwrap_or(&0)
        }
    }

    /// Emitted whenever a transfer is successful.
    #[ink(event)] 
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        value: u64,
    }

    /// Emitted whenever a transfer fails.
    #[ink(event)]
    pub struct Error {
        #[ink(topic)]
        message: String,
    }

    /// Unit tests
    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;
        use ink_env::{AccountId, call};
        use ink_env::test::{
            DefaultEnvironment,
            ExecutionInfo,
            TestEnvironment,
        };
        use ink_env::DefaultAccount;
        use ink_env::test_utils::run_test;

        fn default_accounts<C: TestEnvironment>() -> (EnvTypes<C::AccountId, C::Balance>) {
            let accounts = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>().expect("Cannot get accounts");

            (EnvTypes {
                contract_account_id: DefaultAccount::from(accounts.alice),
                caller: DefaultAccount::from(accounts.bob),
                accounts,
            })
        }

        // The `#[cfg(test)]` annotation makes this function only compile
        // when testing.
        #[ink::test]
        fn test_token_contract() {
            // Constructor works.
            let (mut token, env_types) = run_test::<DefaultEnvironment, _>(|_|default_accounts::<DefaultEnvironment>(), |e|{
                e.creator.defaults(env_types.accounts.alice, 500, e.gas_limit)
            }, move |caller|{
                Token::new(String::from("My Hardhat Token"), String::from("MHT"), 1000000)
            });

            // Transfers work
            token.transfer(env_types.accounts.bob, 100);
            assert_eq!(token.balance_of(env_types.accounts.bob), 100);
        }
    }
}