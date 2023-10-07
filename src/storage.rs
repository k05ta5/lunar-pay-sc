multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{Agreement, AmountType, AgreementTransfer};

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
    fn agreement_subscriber_defined_amount(&self, aggreement_id: u64, address: &ManagedAddress) -> SingleValueMapper<BigUint<Self::Api>>;

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
    fn agreement_current_senders(&self, agreement_id: u64) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;

    /** Stores all senders from an agreement, even the ones that canceled **/
    #[storage_mapper("agreement_all_senders")]
    fn agreement_all_senders(&self, agreement_id: u64) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;

    // Stores the time when a sender signed an agreement
    #[storage_mapper("agreement_sender_sign_time")]
    fn agreement_sender_sign_time(&self, agreement_id: u64, address: &ManagedAddress) -> SingleValueMapper<u64>;

    // Stores the time when a sender canceled an agreement
    #[storage_mapper("agreement_sender_cancel_time")]
    fn agreement_sender_cancel_time(&self, agreement_id: u64, address: &ManagedAddress) -> SingleValueMapper<u64>;

    // Stores all agreement transfers for one sender, for those agreements with only one sender it will behave as agreement transfers as well
    #[storage_mapper("agreement_sender_transfers")]
    fn agreement_sender_transfers(&self, agreement_id: u64, sender: &ManagedAddress) -> UnorderedSetMapper<AgreementTransfer<Self::Api>>;

    // Stores all agreement transfers for one receiver, for those agreements with only one receiver it will behave as agreement transfers as well
    #[storage_mapper("agreement_receiver_transfers")]
    fn agreement_receiver_transfers(&self, agreement_id: u64, receiver: &ManagedAddress) -> UnorderedSetMapper<AgreementTransfer<Self::Api>>;

    // Last transfer time betweek a sender and a receiver for a specific agreement, can be used for all agreements types
    #[storage_mapper("agreement_last_successful_transfer_time")]
    fn agreement_last_successful_transfer_time(&self, agreement_id: u64, sender: &ManagedAddress, receiver: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("agreement_receivers")]
    fn agreement_receivers(&self, agreement_id: u64) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;

    /** Stores all the agreement IDs that belong to an account **/
    #[view(getAccountCreatedAgreementsListByAddress)]
    #[storage_mapper("account_agreements_list")]
    fn account_created_agreements_list(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;

    /** Stores all the agreement IDs that was signed by an account an account **/
    #[view(getAgreementsListByAddress)]
    #[storage_mapper("account_agreements_list")]
    fn account_signed_agreements_list(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;
}
