use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::store::LookupMap;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
#[borsh(crate = "near_sdk::borsh")]
pub struct Oracle {
    requests: LookupMap<AccountId, (String, String)>, // (input, model)
    responses: LookupMap<AccountId, String>, // response
}

#[derive(BorshSerialize, BorshStorageKey)]
#[borsh(crate = "near_sdk::borsh")]
enum StorageKey {
    OracleRequests,
    OracleResponses,
}

impl Default for Oracle {
    fn default() -> Self {
        Self {
            requests: LookupMap::new(StorageKey::OracleRequests),
            responses: LookupMap::new(StorageKey::OracleResponses),
        }
    }
}

#[near_bindgen]
impl Oracle {
    pub fn request_prediction(&mut self, input: String, model: String) {
        let account_id = env::predecessor_account_id();
        self.requests.insert(account_id, (input, model));
    }

    pub fn set_response(&mut self, account_id: AccountId, response: String) {
        self.responses.insert(account_id, response);
    }

    pub fn get_response(&self, account_id: AccountId) -> Option<String> {
        self.responses.get(&account_id).cloned()
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    use super::*;

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder
            .current_account_id(accounts(0))
            .signer_account_id(predecessor_account_id.clone())
            .predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn request_and_set_response() {
        let mut context = get_context(accounts(1));
        testing_env!(context.build());

        let mut contract = Oracle::default();
        contract.request_prediction("input_data".to_string(), "model_name".to_string());
        contract.set_response(accounts(1), "response_data".to_string());

        assert_eq!(
            "response_data".to_string(),
            contract.get_response(accounts(1)).unwrap()
        );
    }

    #[test]
    fn get_nonexistent_response() {
        let contract = Oracle::default();
        assert_eq!(None, contract.get_response("francis.near".parse().unwrap()));
    }
}