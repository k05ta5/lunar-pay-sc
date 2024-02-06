multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    #[storage_mapper("admins")]
    fn admins(&self) -> WhitelistMapper<ManagedAddress>;

    #[view(isUserAdmin)]
    fn is_user_admin(&self, account: ManagedAddress) -> bool {
        self.admins().contains(&account)
    }

    /*
     * Stores the current whitelisted token identifiers
     */
    #[view(getWhitelistedTokenIds)]
    #[storage_mapper("whitelisted_token_ids")]
    fn whitelisted_token_ids(&self) -> UnorderedSetMapper<EgldOrEsdtTokenIdentifier<Self::Api>>;

    /*
     * Stores the used token identifiers
     * A token might get removed from the whitelist but the account might still have balance.
     */
    #[view(getUsedTokenIds)]
    #[storage_mapper("used_token_ids")]
    fn used_token_ids(&self) -> UnorderedSetMapper<EgldOrEsdtTokenIdentifier<Self::Api>>;

    /*
     * Stores the addresses that are allowed to create agreements
     */
    #[view(getWhitelistedAddresses)]
    #[storage_mapper("whitelisted_addresses")]
    fn whitelisted_addresses(&self) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;
}
