multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AmountModule:
    crate::modules::subscriptions::storage::StorageModule
{
    fn get_subscription_amount_agreed_by_parties(&self, id: u64, address: &ManagedAddress<Self::Api>) -> BigUint {
        let fixed_amount = self.subscription_amount(id);

        if !fixed_amount.is_empty() {
            return fixed_amount.get()
        }

        self.subscription_defined_amount_per_member(id, address).get()
    }
}
