multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::modules::subscriptions::types::{SubscriptionAmountType, SubscriptionType};

#[multiversx_sc::module]
pub trait ValidationModule:
    crate::modules::subscriptions::storage::StorageModule
{
    fn require_existing_subscription(&self, id: u64) {
        require!(!self.subscription_by_id(id).is_empty(), "Invalid subscription id.");
    }

    fn is_subscription_created_by_account(&self, id: u64, account: &ManagedAddress) -> bool {
        self.account_subscriptions_created_list(account).contains(&id)
    }

    fn require_subscription_created_by_account(&self, id: u64, account: &ManagedAddress) {
        require!(self.is_subscription_created_by_account(id, account), "This subscription is not created by you.");
    }

    fn require_subscription_not_created_by_account(&self, id: u64, account: &ManagedAddress) {
        require!(!self.is_subscription_created_by_account(id, account), "This subscription is created by you.");
    }

    fn is_account_a_subscription_member(&self, id: u64, account: &ManagedAddress) -> bool {
        self.current_subscription_members_list(id).contains(account)
    }

    fn require_subscription_membership(&self, id: u64, account: &ManagedAddress) {
        require!(self.is_account_a_subscription_member(id, account), "Membership not active for this address.");
    }

    fn require_subscription_membership_not_exists(&self, id: u64, account: &ManagedAddress) {
        require!(!self.is_account_a_subscription_member(id, account), "This address has an active subscription membership.");
    }

    /* The user can create membership only for RecurringPayoutToReceive and TermRestrictedPayoutToReceive subscription types */
    fn can_account_create_membership_for_subscription_type(&self, subscription_type: SubscriptionType) -> bool {
        match subscription_type {
            SubscriptionType::RecurringPayoutToReceive => true,
            SubscriptionType::TermRestrictedPayoutToReceive => true,
            _ => false
        }
    }

    fn require_account_can_create_membership_for_subscription_type(&self, subscription_type: SubscriptionType) {
        require!(
            self.can_account_create_membership_for_subscription_type(subscription_type),
            "You cannot create a membership for this subscription type."
        );
    }

    /* The owner can add members only for RecurringPayoutToSend and TermRestrictedPayoutToSend subscription types */
    fn can_owner_add_member_for_subscription_type(&self, subscription_type: SubscriptionType) -> bool {
        match subscription_type {
            SubscriptionType::RecurringPayoutToSend => true,
            SubscriptionType::TermRestrictedPayoutToSend => true,
            _ => false
        }
    }

    fn require_owner_can_add_member_for_subscription_type(&self, subscription_type: SubscriptionType) {
        require!(
            self.can_owner_add_member_for_subscription_type(subscription_type),
            "You cannot add members for this subscription type."
        );
    }

    fn is_member_required_to_define_subscription_amount(&self, amount_type: SubscriptionAmountType) -> bool {
        amount_type == SubscriptionAmountType::MemberDefinedAmount
    }

    fn require_subscription_publicly_triggerable(&self, subscription_type: SubscriptionType) {
        require!(self.is_subscription_pulicly_triggerable(subscription_type), "You cannot trigger this subscription.");
    }

    fn is_subscription_pulicly_triggerable(&self, subscription_type: SubscriptionType) -> bool {
        subscription_type == SubscriptionType::RecurringPayoutToReceive ||
            subscription_type == SubscriptionType::TermRestrictedPayoutToReceive
    }
}
