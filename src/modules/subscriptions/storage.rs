multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::modules::subscriptions::types::{Subscription};

#[multiversx_sc::module]
pub trait StorageModule {
    #[view(getSubscriptionIds)]
    #[storage_mapper("subscription_ids")]
    fn subscription_ids(&self) -> SetMapper<u64>;

    /** Stores the subscriptions by ID **/
    #[storage_mapper("subscription_by_id")]
    fn subscription_by_id(&self, id: u64) -> SingleValueMapper<Subscription<Self::Api>>;

    /** Stores the current members for a subscription. **/
    #[storage_mapper("current_subscription_members_list")]
    fn current_subscription_members_list(&self, id: u64) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;

    /** Stores the time when an account was added to a subscription **/
    #[storage_mapper("subscription_member_start_time")]
    fn subscription_member_start_time(&self, id: u64, address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("subscription_member_last_trigger_time")]
    fn subscription_member_last_trigger_time(&self, id: u64, address: &ManagedAddress) -> SingleValueMapper<u64>;

    #[storage_mapper("subscription_amount")]
    fn subscription_amount(&self, id: u64) -> SingleValueMapper<BigUint>;

    #[storage_mapper("subscription_defined_amount_per_member")]
    fn subscription_defined_amount_per_member(&self, id: u64, address: &ManagedAddress) -> SingleValueMapper<BigUint>;

    /** Stores the IDs for all the subscriptions created by an account **/
    #[view(getAccountSubscriptionsCreatedList)]
    #[storage_mapper("account_subscriptions_created_list")]
    fn account_subscriptions_created_list(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;

    /** Stores the IDs for all subscription signed by an account **/
    #[view(getAccountSubscriptionsMembershipList)]
    #[storage_mapper("account_subscriptions_membership_list")]
    fn account_subscriptions_membership_list(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;
}
