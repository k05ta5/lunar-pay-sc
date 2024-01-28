multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::modules::subscriptions::types::{Subscription};

#[multiversx_sc::module]
pub trait MemberEndpoints:
    crate::storage::StorageModule +
    crate::validation::ValidationModule +

    crate::modules::subscriptions::amount::AmountModule +
    crate::modules::subscriptions::events::EventsModule +
    crate::modules::subscriptions::storage::StorageModule +
    crate::modules::subscriptions::validation::ValidationModule +
    crate::modules::transfers::balance_transfer::BalanceTransferModule
{
    /**
     * Subscribe to a subscription
     */
    #[endpoint(createSubscriptionMembership)]
    fn create_subscription_membership(&self, id: u64, amount: Option<BigUint<Self::Api>>, metadata: Option<ManagedBuffer<Self::Api>>) {
        self.require_existing_subscription(id);
        let caller = self.blockchain().get_caller();

        self.require_subscription_not_created_by_account(id, &caller);
        self.require_subscription_membership_not_exists(id, &caller);

        let subscription = self.subscription_by_id(id).get();

        self.require_account_can_create_membership_for_subscription_type(subscription.subscription_type);

        match amount {
            None => {
                require!(
                    !self.is_member_required_to_define_subscription_amount(subscription.amount_type),
                    "This subscription requires an amount defined by the member"
                )
            },
            Some(member_defined_amount) => {
                // Set the subscription amount for subscriptions with member defined amount
                require!(
                    self.is_member_required_to_define_subscription_amount(subscription.amount_type),
                    "This subscription does not allow an amount defined by the member"
                );

                require!(member_defined_amount > 0, "Invalid subscription amount");

                self.subscription_defined_amount_per_member(id, &caller).set(member_defined_amount);
            }
        }

        let timestamp = self.blockchain().get_block_timestamp();

        self.current_subscription_members_list(id).insert(caller.clone());
        self.account_subscriptions_membership_list(&caller).insert(id);

        self.subscription_member_start_time(id, &caller).set(timestamp);

        // We charge one full cycle when the subscription membership is signed
        self.charge_initial_subscription_cycle(subscription, &caller, timestamp);

        self.create_subscription_membership_event(id, &caller, timestamp, metadata);
    }

    #[inline]
    fn charge_initial_subscription_cycle(&self, subscription: Subscription<Self::Api>, member: &ManagedAddress, timestamp: u64) {
        let cycle_cost = self.get_subscription_amount_agreed_by_parties(subscription.id, member);

        self.do_internal_transfer_and_update_balances(member, &subscription.owner, &subscription.token_identifier, &cycle_cost);
        self.subscription_member_last_trigger_time(subscription.id, member).set(timestamp);
    }
}
