use near_sdk::store::LookupMap;
use near_sdk::{env, log, near, require, AccountId};

pub type Balance = u8;

#[near(contract_state)]
pub struct Contract {
    pub tokens: LookupMap<Balance, AccountId>,
    pub approvals: LookupMap<Balance, AccountId>,
    pub supply: u16,
}

// Implement the contract structure
#[near]
impl Contract {
    #[init]
    #[private] // only callable by the contract's account
    pub fn init(
        admin: AccountId
    ) -> Self {
        Self {
            tokens: {
                let mut a = LookupMap::new(b"tokens".to_vec());
                a.insert(0, admin);
                a
            },
            approvals: LookupMap::new(b"approvals".to_vec()),
            supply: 1,
        }
    }
    // Public method - returns the greeting saved, defaulting to DEFAULT_GREETING
    pub fn owner_of(&self, id: Balance) -> Option<AccountId> {
        self.tokens.get(&id).cloned()
    }

    pub fn mint(&mut self) -> Balance {
        self.tokens.insert(self.supply.to_le_bytes()[0], env::predecessor_account_id());
        let id = self.supply;
        self.supply += 1;
        id as Balance
    }

    pub fn approve(&mut self, id: Balance, delegatee: AccountId) {
        require!(self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id(), "not owner!");
        self.approvals.insert(id, delegatee);
    }

    pub fn transfer(&mut self, id: Balance, receiver: AccountId) {
        require!(
            self.tokens.get(&id).unwrap().clone() == env::predecessor_account_id()
            || self.approvals.get(&id).unwrap().clone() == env::predecessor_account_id()
            , "not owner!"
        );
        self.tokens.insert(id, receiver);
    }

}
