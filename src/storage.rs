multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{Agreement, Amount};

#[multiversx_sc::module]
pub trait StorageModule {
    /** TODO: Delete everything below **/

    #[view(getAgreementIds)]
    #[storage_mapper("agreement_ids")]
    fn agreement_ids(&self) -> SetMapper<u64>;

    /** Stores the agreement by ID **/
    #[storage_mapper("agreement_by_id")]
    fn agreement_by_id(&self, agreement_id: u64) -> SingleValueMapper<Agreement<Self::Api>>;

    #[view(getAgreementWhitelist)]
    #[storage_mapper("agreement_whitelist")]
    fn agreement_whitelist(&self, agreement_id: u64) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;

    /** Stores all the accounts for an agreement, even the ones that canceled. It acts as senders or receivers list **/
    #[storage_mapper("agreement_accounts")]
    fn agreement_accounts(&self, agreement_id: u64) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;

    /** Stores the current accounts for an agreement. It acts as senders or receivers list **/
    #[storage_mapper("agreement_current_accounts")]
    fn agreement_current_accounts(&self, agreement_id: u64) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;

    // Stores the time when a sender signed an agreement
    #[storage_mapper("agreement_sign_time_per_account")]
    fn agreement_sign_time_per_account(&self, agreement_id: u64, address: &ManagedAddress) -> SingleValueMapper<u64>;

    // Stores the time when a sender canceled an agreement
    #[storage_mapper("agreement_cancel_time_per_account")]
    fn agreement_cancel_time_per_account(&self, agreement_id: u64, address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("agreement_last_triggered_time_per_account")]
    fn agreement_last_triggered_time_per_account(&self, agreement_id: u64, address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("agreement_amount")]
    fn agreement_amount(&self, aggreement_id: u64) -> SingleValueMapper<Amount<Self::Api>>;

    #[storage_mapper("agreement_defined_amount_per_account")]
    fn agreement_defined_amount_per_account(&self, aggreement_id: u64, address: &ManagedAddress) -> SingleValueMapper<Amount<Self::Api>>;

    /** Stores all the agreement IDs that belong to an account **/
    #[view(getAgreementsCreatedByAccount)]
    #[storage_mapper("account_created_agreements_list")]
    fn account_created_agreements_list(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;

    /** Stores all the agreement IDs that were signed by an account **/
    #[view(getAgreementsSignedByAccount)]
    #[storage_mapper("account_signed_agreements_list")]
    fn account_signed_agreements_list(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;
}
