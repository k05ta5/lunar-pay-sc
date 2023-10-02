multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{Agreement, AmountType};

#[multiversx_sc::module]
pub trait StorageModule {
    /** Protocol Storage */

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

    /** Account Storage */

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

    /** Agreement Storage */

    /** Stores the last ID assigned to an agreement **/
    #[view(getLastAgreementId)]
    #[storage_mapper("last_agreement_id")]
    fn last_agreement_id(&self) -> SingleValueMapper<u64>;

    #[view(getAgreementIds)]
    #[storage_mapper("agreement_ids")]
    fn agreement_ids(&self) -> SetMapper<u64>;

    /** Stores the agreement by ID **/
    #[storage_mapper("agreement_by_id")]
    fn agreement_by_id(&self, agreement_id: u64) -> SingleValueMapper<Agreement<Self::Api>>;

    #[view(agreement_subscriber_defined_amount)]
    #[storage_mapper("agreement_subscriber_defined_amount")]
    fn agreement_subscriber_defined_amount(&self, aggreement_id: u64, address: &ManagedAddress) -> SingleValueMapper<AmountType<Self::Api>>;

    // #[view(getAgreementCreatorDefinedAmountPerSubscriber)]
    #[storage_mapper("agreement_creator_defined_amount_per_subscriber")]
    fn agreement_creator_defined_amount_per_subscriber(&self, aggreement_id: u64, address: &ManagedAddress) -> SingleValueMapper<AmountType<Self::Api>>;

    #[view(getAgreementWhitelistEnabled)]
    #[storage_mapper("agreement_whitelist_enabled")]
    fn agreement_whitelist_enabled(&self, agreement_id: u64) -> SingleValueMapper<bool>;

    #[view(getAgreementWhitelist)]
    #[storage_mapper("agreement_whitelist")]
    fn agreement_whitelist(&self, agreement_id: u64) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;

    #[storage_mapper("agreement_senders")]
    fn agreement_senders(&self, agreement_id: u64) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;

    #[storage_mapper("agreement_receivers")]
    fn agreement_receivers(&self, agreement_id: u64) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;

    /** Stores all the agreement IDs that belong to an account **/
    #[view(getAgreementsListByAddress)]
    #[storage_mapper("account_agreements_list")]
    fn account_created_agreements_list(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;

    /** Stores all the agreement IDs that belong to an account **/
    #[view(getAgreementsListByAddress)]
    #[storage_mapper("account_agreements_list")]
    fn account_signed_agreements_list(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;
}
