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

    fn require_account_has_sufficient_balance(
        &self,
        account: &ManagedAddress,
        token: &EgldOrEsdtTokenIdentifier<Self::Api>,
        amount: &BigUint
    ) {
        require!(self.account_balance(&account, &token).get() >= *amount, "Insufficient account balance");
    }
}
