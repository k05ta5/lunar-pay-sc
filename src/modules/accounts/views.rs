multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait ViewsModule:
    crate::storage::StorageModule +
    crate::modules::accounts::storage::StorageModule
{
    /**
     * It returns the total account balances
     */
    #[view(getAccountBalances)]
    fn get_account_balances(&self, address: &ManagedAddress) -> MultiValueEncoded<(EgldOrEsdtTokenIdentifier, BigUint)> {
        let mut items_vec = MultiValueEncoded::new();

        for token in self.used_token_ids().iter() {
            let account_balance = self.account_balance(address, &token);

            if !account_balance.is_empty() {
                items_vec.push((token, account_balance.get()));
            }
        }

        items_vec
    }
}
