multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    /*
    * Stores the accounts
    */
    #[storage_mapper("accounts")]
    fn accounts(&self) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;

    /*
     * Stores the total account balance for each token identifier
     */
    #[storage_mapper("account_balance")]
    fn account_balance(&self, address: &ManagedAddress, token: &EgldOrEsdtTokenIdentifier) -> SingleValueMapper<BigUint<Self::Api>>;
}
