multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AgreementCyclesModule:
    crate::storage::StorageModule +
    crate::validation::ValidationModule
{
    fn get_last_trigger_timestamp_for_account(&self, agreement_id: u64, address: &ManagedAddress<Self::Api>) -> u64 {
        self.agreement_last_cycle_triggered_per_account(agreement_id, address).get()
    }

    fn compute_new_cycle_trigger_timestamp(&self, agreement_id: u64, account: &ManagedAddress<Self::Api>, frequency: u64, cycles: u64) -> u64 {
        let last_cycle_triggered_timestamp = self.get_last_trigger_timestamp_for_account(agreement_id, &account);

        last_cycle_triggered_timestamp + frequency * cycles
    }

    fn compute_next_trigger_timestamp_for_acccount(&self, agreement_id: u64, frequency: u64, account: &ManagedAddress<Self::Api>) -> u64 {
        let last_trigger = self.get_last_trigger_timestamp_for_account(agreement_id, &account);

        last_trigger + frequency
    }

    fn get_account_number_of_cycles_to_trigger(&self, agreement_id: u64, frequency: u64, account: &ManagedAddress<Self::Api>) -> u64 {
        let timestamp = self.blockchain().get_block_timestamp();
        let last_trigger_timestamp = self.get_last_trigger_timestamp_for_account(agreement_id, account);

        (timestamp - last_trigger_timestamp).checked_div(frequency).unwrap()
    }
}
