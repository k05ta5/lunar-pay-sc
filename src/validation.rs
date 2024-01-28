multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait ValidationModule:
                crate::storage::StorageModule
{
    fn is_token_whitelisted(&self, token: &EgldOrEsdtTokenIdentifier<Self::Api>) -> bool {
        self.whitelisted_token_ids().contains(&token)
    }

    fn require_token_is_whitelisted(&self, token: &EgldOrEsdtTokenIdentifier<Self::Api>) {
        require!(self.is_token_whitelisted(token), "Token is not whitelisted");
    }

    fn is_address_whitelisted(&self, _address: &ManagedAddress<Self::Api>) -> bool {
        // All addresses are whitelisted for xDay Hackathon
        true
        // self.whitelisted_addresses().contains(&address)
    }

    fn require_address_is_whitelisted(&self, _address: &ManagedAddress) -> bool {
        // All addresses are whitelisted for xDay Hackathon
        true
        // require!(self.is_address_whitelisted(address), "Address is not whitelisted");
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
        self.agreement_accounts(agreement_id).contains(account)
    }

    fn require_agreement_signed_by_account(&self, agreement_id: u64, account: &ManagedAddress) {
        require!(self.is_agreement_signed_by_account(agreement_id, account), "You did not sign this agreement.");
    }

    fn require_agreement_not_signed_by_account(&self, agreement_id: u64, account: &ManagedAddress) {
        require!(!self.is_agreement_signed_by_account(agreement_id, account), "You already signed this agreement.");
    }
}
