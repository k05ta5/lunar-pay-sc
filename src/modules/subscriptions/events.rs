multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::modules::subscriptions::types::{Subscription, SubscriptionAmountType, SubscriptionChargeData, SubscriptionType};

#[multiversx_sc::module]
pub trait EventsModule {
    fn trigger_subscription_created_event(&self, subscription: Subscription<Self::Api>, amount: Option<BigUint>) {
        self.create_subscription_event(
            subscription.id,
            &subscription.owner,
            subscription.token_nonce,
            &subscription.token_identifier,
            subscription.frequency,
            subscription.time_created,
            subscription.subscription_type,
            subscription.amount_type,
            amount,
        );
    }

    #[event("createSubscription")]
    fn create_subscription_event(
        &self,
        #[indexed] id: u64,
        #[indexed] owner: &ManagedAddress,
        #[indexed] token_nonce: u64,
        #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
        #[indexed] frequency: u64,
        #[indexed] time_created: u64,
        #[indexed] subscription_type: SubscriptionType,
        #[indexed] amount_type: SubscriptionAmountType,
        #[indexed] amount: Option<BigUint>,
    );

    #[event("createSubscriptionMembership")]
    fn create_subscription_membership_event(
        &self,
        #[indexed] id: u64,
        #[indexed] member: &ManagedAddress,
        #[indexed] created_at: u64,
        #[indexed] metadata: Option<ManagedBuffer>,
    );

    #[event("chargeSubscription")]
    fn charge_subscription_event(
        &self,
        #[indexed] id: u64,
        #[indexed] member: &ManagedAddress,

        #[indexed] timestamp: u64,
        #[indexed] data: SubscriptionChargeData<Self::Api>,
    );

    // #[event("cancelSubscription")]
    // fn cancel_payment_agreement_event(
    //     &self,
    //     #[indexed] id: u64,
    //     #[indexed] account: &ManagedAddress,
    // );
    //
    // #[event("chargeSubscription")]
    // fn charge_subscription_event(
    //     &self,
    //     #[indexed] agreement_id: u64,
    //     #[indexed] accounts: ManagedVec<ManagedAddress<Self::Api>>,
    //     #[indexed] amounts: ManagedVec<BigUint>,
    //     #[indexed] cycles: ManagedVec<u64>,
    // );
}