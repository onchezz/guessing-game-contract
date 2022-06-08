use near_rng::Rng;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen};
use std::cmp::Ordering;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub guess: u64,
}
impl Default for Contract {
    fn default() -> Self {
        Self { guess: 0 }
    }
}

#[near_bindgen]
impl Contract {
    pub fn random_number(&mut self) {
        let mut rng = Rng::new(&env::random_seed());
        let value = rng.rand_range_u64(0, 100);
        self.guess = value;
    }
    pub fn get_user_guess(&mut self, number: u64) {
        match self.guess.cmp(&number) {
            Ordering::Less => log!("Too small"),
            Ordering::Greater => log!("Too big"),
            Ordering::Equal => log!("You win"),
        }
    }
    // ADD CONTRACT METHODS HERE
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId, VMContext};

    fn contract_account() -> AccountId {
        "contract".parse::<AccountId>().unwrap()
    }

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(contract_account())
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder.build()
    }

    #[test]
    fn test_random_number() {
        let accountid = AccountId::new_unchecked("onchez.test".to_string());
        let context = get_context(accountid);
        testing_env!(context);

        let mut contract = Contract::default();
        contract.random_number();
        assert!(contract.guess > 0);
        assert!(contract.guess < 100);
    }
}
