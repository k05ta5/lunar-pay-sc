multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AgreementCyclesModule:
    crate::storage::StorageModule +
    crate::validation::ValidationModule
{
    /// Fetches the last timestamp when a cycle was triggered for a given agreement and account.
    ///
    /// # Arguments
    ///
    /// * `agreement_id` - The ID of the agreement.
    /// * `address` - The address of the account.
    ///
    /// # Returns
    ///
    /// The last trigger timestamp.
    fn last_triggered_timestamp(&self, agreement_id: u64, address: &ManagedAddress<Self::Api>) -> u64 {
        self.agreement_last_triggered_time_per_account(agreement_id, address).get()
    }

    /// Computes the timestamp for triggering a new cycle based on frequency and the number of cycles.
    ///
    /// # Arguments
    ///
    /// * `agreement_id` - The ID of the agreement.
    /// * `account` - The account for which the new timestamp is to be calculated.
    /// * `frequency` - The frequency of the cycles.
    /// * `cycles` - The number of cycles.
    ///
    /// # Returns
    /// The new cycle trigger timestamp.
    fn compute_cycle_timestamp(&self, agreement_id: u64, account: &ManagedAddress<Self::Api>, frequency: u64, cycles: u64) -> u64 {
        self.last_triggered_timestamp(agreement_id, &account) + frequency * cycles
    }

    /// Calculates the number of cycles that need to be triggered for a given account based on the current block timestamp.
    ///
    /// # Arguments
    ///
    /// * `agreement_id` - The ID of the agreement.
    /// * `frequency` - The frequency of the cycles.
    /// * `account` - The account for which the number of cycles is to be calculated.
    ///
    /// # Returns
    ///
    /// The number of cycles that need to be triggered.
    fn pending_cycles_count(&self, agreement_id: u64, frequency: u64, account: &ManagedAddress<Self::Api>) -> u64 {
        let timestamp = self.blockchain().get_block_timestamp();
        let last_trigger_timestamp = self.last_triggered_timestamp(agreement_id, account);

        (timestamp - last_trigger_timestamp).checked_div(frequency).unwrap()
    }
}