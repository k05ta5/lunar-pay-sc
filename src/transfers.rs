multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait TransfersModule:
    crate::storage::StorageModule +
    crate::validation::ValidationModule
{
    #[endpoint(transferTokens)]
    fn transfer(&self, token: EgldOrEsdtTokenIdentifier, amount: BigUint,  receiver: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        require!(caller != receiver, "Sender and receiver must be different");

        self.do_transfer_and_update_balance(&caller, &receiver, &token, &amount);
    }

    #[inline]
    fn do_transfer_and_update_balance(
        &self,
        sender: &ManagedAddress<Self::Api>,
        receiver: &ManagedAddress<Self::Api>,
        token: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
    ) {
        self.require_account_has_sufficient_balance(&sender, &token, &amount.clone());

        self.account_balance(&sender, &token).update(|balance| *balance -= &amount.clone());
        self.send().direct(&receiver, &token, 0, &amount.clone());
    }

    #[inline]
    fn do_internal_transfer_and_update_balances(
        &self,
        sender: &ManagedAddress<Self::Api>,
        receiver: &ManagedAddress<Self::Api>,
        token: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
    ) {
        self.require_account_has_sufficient_balance(&sender, &token, &amount.clone());

        self.account_balance(&sender, &token).update(|balance| *balance -= &amount.clone());
        self.account_balance(&receiver, &token).update(|balance| *balance += &amount.clone());
    }
}
