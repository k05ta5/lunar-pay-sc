use core::cmp::min;
use crate::modules::subscriptions::types::Subscription;
multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait CyclesModule:
    crate::modules::subscriptions::storage::StorageModule
{
    /// Fetches the last timestamp when a cycle was triggered for a given agreement and account.
    fn last_triggered_timestamp(&self, id: u64, address: &ManagedAddress<Self::Api>) -> u64 {
        self.subscription_member_last_trigger_time(id, address).get()
    }

    /// Computes the timestamp at the end of cycles.
    fn get_timestamp_at_end_of_cycles(&self, old_timestamp: u64, frequency: u64, number_of_cycles: u64) -> u64 {
        old_timestamp + (frequency * number_of_cycles)
    }

    /// Calculates the number of cycles that need to be triggered for a given account based on the current block timestamp.
    fn get_pending_cycles_count(&self, agreement_id: u64, frequency: u64, account: &ManagedAddress<Self::Api>) -> u64 {
        let timestamp = self.blockchain().get_block_timestamp();
        let last_trigger_timestamp = self.last_triggered_timestamp(agreement_id, account);

        (timestamp - last_trigger_timestamp).checked_div(frequency).unwrap()
    }

    /// Calculates the number of cycles that a user can afford.
    fn get_number_of_cycles_that_can_be_charged(&self, amaount_available: &BigUint, amount_per_cycle: &BigUint, pending_cycles: u64) -> u64 {
        // Determine how many cycles the user can afford
        let affordable_cycles = (amaount_available / amount_per_cycle).to_u64().unwrap_or(0);

        min(pending_cycles, affordable_cycles)
    }

    fn update_subscription_last_trigger_timestamp(&self, subscription: &Subscription<Self::Api>, account: &ManagedAddress, cycles: u64) {
        let last_triggered_cycle_timestamp = self.subscription_member_last_trigger_time(subscription.id, &account).get();
        let end_cycle_timestamp = self.get_timestamp_at_end_of_cycles(last_triggered_cycle_timestamp, subscription.frequency, cycles);

        self.subscription_member_last_trigger_time(subscription.id, &account).set(end_cycle_timestamp);
    }
}