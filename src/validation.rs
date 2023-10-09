multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait ValidationModule:
    crate::storage::StorageModule
{
    fn is_token_whitelisted(&self, token: &EgldOrEsdtTokenIdentifier<Self::Api>) -> bool {
        return self.whitelisted_token_ids().contains(&token)
    }

    fn is_address_whitelisted(&self, address: &ManagedAddress<Self::Api>) -> bool {
        return self.whitelisted_addresses().contains(&address)
    }

    fn require_address_is_whitelisted(&self, address: &ManagedAddress) {
        require!(self.is_address_whitelisted(address), "Address is not whitelisted");
    }

    fn is_address_whitelisted_for_agreement(&self, address: &ManagedAddress<Self::Api>, agreement_id: u64) -> bool {
        let whitelist_enabled = self.agreement_whitelist_enabled(agreement_id);
        if !whitelist_enabled.is_empty() && whitelist_enabled.get() == true {
            return self.agreement_whitelist(agreement_id).contains(&address);
        }

        // If whitelist is not enabled then everyone is whitelisted
        return true
    }

    fn require_address_is_whitelisted_for_agreeement(&self, address: &ManagedAddress, agreement_id: u64) {
        require!(self.is_address_whitelisted_for_agreement(address, agreement_id), "Address is not whitelisted for this agreement");
    }

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

    /** Agreement Validations **/

    fn require_existing_agreement_id(&self, agreement_id: u64) {
        require!(!self.agreement_by_id(agreement_id).is_empty(), "Invalid agreement id.");
    }

    fn is_agreement_created_by_account(&self, account: &ManagedAddress, agreement_id: u64) -> bool {
        self.account_created_agreements_list(account).contains(&agreement_id)
    }

    fn require_agreement_created_by_account(&self, account: &ManagedAddress, agreement_id: u64) {
        require!(self.is_agreement_created_by_account(account, agreement_id), "This agreement is not created by you.");
    }

    fn require_agreement_not_created_by_account(&self, account: &ManagedAddress, agreement_id: u64) {
        require!(!self.is_agreement_created_by_account(account, agreement_id), "This agreement is created by you.");
    }

    fn is_agreement_signed_by_account(&self, agreement_id: u64, account: &ManagedAddress) -> bool {
        self.agreement_current_signers(agreement_id).contains(account)
    }

    fn require_agreement_signed_by_account(&self, agreement_id: u64, account: &ManagedAddress) {
        require!(self.is_agreement_signed_by_account(agreement_id, account), "You did not sign this agreement.");
    }

    fn require_agreement_not_signed_by_account(&self, agreement_id: u64, account: &ManagedAddress) {
        require!(!self.is_agreement_signed_by_account(agreement_id, account), "You already signed this agreement.");
    }
}
