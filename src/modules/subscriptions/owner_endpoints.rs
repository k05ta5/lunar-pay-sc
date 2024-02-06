multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::modules::subscriptions::types::{SubscriptionType, Subscription, SubscriptionAmountType};

#[multiversx_sc::module]
pub trait OwnerEndpoints:
    crate::modules::protocol::storage::StorageModule +
    crate::modules::protocol::validation::ValidationModule +
    crate::modules::agreements::storage::StorageModule +

    crate::modules::subscriptions::events::EventsModule +
    crate::modules::subscriptions::storage::StorageModule +
    crate::modules::subscriptions::validation::ValidationModule
{
    #[endpoint(createSubscription)]
    fn create_subscription(
        &self,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
        frequency: u64,
        subscription_type: SubscriptionType,
        amount_type: SubscriptionAmountType,
        amount: Option<BigUint>
    ) {
        let caller = self.blockchain().get_caller();

        self.require_token_is_whitelisted(&token_identifier);

        let id = self.create_identifier();
        let timestamp = self.blockchain().get_block_timestamp();

        // Create the subscription object
        let subscription = Subscription {
            id: id.clone(),
            owner: caller.clone(),
            time_created: timestamp,

            token_nonce: 0,
            token_identifier: token_identifier.clone(),

            frequency,
            subscription_type: subscription_type,
            amount_type: amount_type,
        };

        self.subscription_ids().insert(id);
        self.subscription_by_id(id).set(&subscription);
        self.account_subscriptions_created_list(&caller).insert(id);

        match amount.clone() {
            None => {
                // Amount is required for subscriptions with amount type `FixedAmount`
                require!(
                    amount_type != SubscriptionAmountType::FixedAmount,
                    "Amount is required for this subscription"
                )
            },
            Some(fixed_amount) => {
                // Amount should only be sent when the amount type is `FixedAmount`
                require!(
                    amount_type == SubscriptionAmountType::FixedAmount,
                    "This subscription does not allow a fixed amount defined"
                );

                require!(fixed_amount > 0, "Invalid subscription amount");

                self.subscription_amount(id).set(fixed_amount);
            }
        }

        self.trigger_subscription_created_event(subscription, amount);
    }

    #[endpoint(addSubscriptionMember)]
    fn add_subscription_member(&self, id: u64, address: ManagedAddress) {
        let caller = self.blockchain().get_caller();

        self.require_subscription_created_by_account(id, &caller);

        let subscription = self.subscription_by_id(id).get();
        self.require_owner_can_add_member_for_subscription_type(subscription.subscription_type);
        self.require_subscription_membership_not_exists(id, &address);

        let timestamp = self.blockchain().get_block_timestamp();

        self.subscription_member_start_time(id, &address).set(timestamp);
        self.account_subscriptions_membership_list(&address).insert(id);
        self.current_subscription_members_list(id).insert(address);
    }

    #[endpoint(cancelSubscriptionMembership)]
    fn cancel_subscription_membership(&self, id: u64, address: Option<ManagedAddress>) {
        let caller = self.blockchain().get_caller();

        match address {
            Some(member) => {
                self.require_subscription_created_by_account(id, &caller);
                self.require_subscription_membership(id, &member);
            },
            None => {
                self.require_subscription_not_created_by_account(id, &caller);
                self.require_subscription_membership(id, &caller);
            },
        }
    }

    #[inline]
    fn actual_cancel_membership(&self) {

    }

    #[inline]
    fn create_identifier(&self) -> u64 {
        self.last_agreement_id().update(|id| *id += 1);
        self.last_agreement_id().get()
    }
}
