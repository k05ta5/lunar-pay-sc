multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::modules::subscriptions::types::{Subscription, SubscriptionChargeData};

#[multiversx_sc::module]
pub trait PublicEndpoints:
    crate::modules::accounts::storage::StorageModule +
    crate::modules::accounts::validation::ValidationModule +
    crate::modules::transfers::balance_transfer::BalanceTransferModule +

    crate::modules::subscriptions::cycles::CyclesModule +
    crate::modules::subscriptions::amount::AmountModule +
    crate::modules::subscriptions::events::EventsModule +
    crate::modules::subscriptions::storage::StorageModule +
    crate::modules::subscriptions::validation::ValidationModule +
{
    #[endpoint(triggerSubscription)]
    fn trigger_subscription(&self, id: u64) {
        self.require_existing_subscription(id);
        let subscription = self.subscription_by_id(id).get();

        self.require_subscription_publicly_triggerable(subscription.subscription_type);

        let members = self.current_subscription_members_list(id);
        require!(!members.is_empty(), "Nothing to send");

        let timestamp = self.blockchain().get_block_timestamp();

        for account in members.iter() {
            let (successful, failed) = self.trigger_subscription_for_account(&subscription, &account);

            if let Some((amount, cycles)) = successful.clone() {
                self.do_internal_transfer_and_update_balances(&account, &subscription.owner, &subscription.token_identifier, &amount);
                self.update_subscription_last_trigger_timestamp(&subscription, &account, cycles);

            }

            let charge_data = SubscriptionChargeData { successful: successful, failed: failed };
            self.charge_subscription_event(subscription.id, &account, timestamp, charge_data);
        }
    }

    fn trigger_subscription_for_account(&self, subscription: &Subscription<Self::Api>, account: &ManagedAddress)
        -> (Option<(BigUint, u64)>, Option<(BigUint, u64)>) {
        let total_pending_cycles = self.get_pending_cycles_count(subscription.id, subscription.frequency, &account);

        // Early return if there is nothing to charge
        if total_pending_cycles == 0 {
            return (None, None);
        }

        let cycle_cost = self.get_subscription_amount_agreed_by_parties(subscription.id, &account);
        let user_balance = self.account_balance(&account, &subscription.token_identifier).get();

        let affordable_cycles = self.get_number_of_cycles_that_can_be_charged(&user_balance, &cycle_cost, total_pending_cycles);

        // The user does not have funds for any cycle
        if affordable_cycles == 0 {
            return (None, self.charge_event_user_amount(total_pending_cycles, &cycle_cost));
        }

        // User had enough funds for all pending cycles
        if affordable_cycles == total_pending_cycles {
            return (self.charge_event_user_amount(total_pending_cycles, &cycle_cost), None);
        }

        // User had funds only for part of the pending cycles
        let cycles_failed = total_pending_cycles - affordable_cycles;

        return (
            self.charge_event_user_amount(affordable_cycles, &cycle_cost),
            self.charge_event_user_amount(cycles_failed, &cycle_cost)
        );
    }

    fn charge_event_user_amount(&self, cycles: u64, cycle_cost: &BigUint) -> Option<(BigUint, u64)> {
        Some((cycle_cost * cycles, cycles))
    }
}
