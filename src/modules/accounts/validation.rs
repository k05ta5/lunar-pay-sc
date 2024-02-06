multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait ValidationModule: crate::modules::accounts::storage::StorageModule {
    fn account_has_sufficient_balance(
        &self,
        account: &ManagedAddress,
        token: &EgldOrEsdtTokenIdentifier<Self::Api>,
        amount: &BigUint
    ) -> bool {
        return self.account_balance(&account, &token).get() >= *amount;
    }

    fn require_account_has_sufficient_balance(
        &self,
        account: &ManagedAddress,
        token: &EgldOrEsdtTokenIdentifier<Self::Api>,
        amount: &BigUint
    ) {
        require!(self.account_has_sufficient_balance(&account, &token, &amount), "Insufficient account balance.");
    }
}
