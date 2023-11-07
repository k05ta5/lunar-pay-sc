multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{Agreement, AgreementType, AgreementChargeResult};

#[multiversx_sc::module]
pub trait AgreementTriggersModule:
    crate::events::EventsModule +
    crate::storage::StorageModule +
    crate::transfers::TransfersModule +
    crate::validation::ValidationModule +
    crate::agreement_cycles::AgreementCyclesModule +
    crate::agreement_amount::AgreementAmountModule
{
    #[endpoint(triggerAgreement)]
    fn trigger_agreement(&self, agreement_id: u64) {
        self.require_existing_agreement_id(agreement_id);

        let agreement = self.agreement_by_id(agreement_id).get();

        let mut successful_data = AgreementChargeResult::new();
        let mut failed_data = AgreementChargeResult::new();

        match agreement.agreement_type {
            AgreementType::RecurringPayoutToReceive => {
                let accounts_list: UnorderedSetMapper<ManagedAddress<Self::Api>> =
                    self.agreement_accounts(agreement.id.clone());

                require!(!accounts_list.is_empty(), "Nothing to send");

                for account in accounts_list.iter() {
                    let (successful, failed) = self.trigger_agreement_for_recurring_payout_to_receive(&agreement, &account);

                    if let Some((amount, cycles)) = successful {
                        successful_data.push(account.clone(), amount, cycles);
                    }

                    if let Some((amount, cycles)) = failed {
                        failed_data.push(account.clone(), amount, cycles);
                    }
                }
            },

            _ => panic!("You cannot trigger this agreement")
        }

        if !successful_data.accounts.is_empty() {
            self.successful_charges_event(agreement_id, successful_data.accounts, successful_data.amounts, successful_data.cycles);
        }

        if !failed_data.accounts.is_empty() {
            self.failed_charges_event(agreement_id, failed_data.accounts, failed_data.amounts, failed_data.cycles);
        }
    }

    fn trigger_agreement_for_recurring_payout_to_receive(
        &self,
        agreement: &Agreement<Self::Api>,
        account: &ManagedAddress<Self::Api>
    ) -> (Option<(BigUint, u64)>, Option<(BigUint, u64)>) {
        let sender = account.clone();
        let receiver = agreement.owner.clone();

        let total_pending_cycles = self.pending_cycles_count(agreement.id, agreement.frequency, &account);
        let amount_per_cycle = self.get_charge_value(agreement.id, agreement.amount_type, &account);
        let user_balance = self.account_balance(&sender, &agreement.token_identifier).get();
        let cycles_to_charge = self.get_cycles_to_charge(&user_balance, &amount_per_cycle, total_pending_cycles);

        // No funds available for any cycle
        if cycles_to_charge == 0 {
            return (None, Some((amount_per_cycle * total_pending_cycles, total_pending_cycles)));
        }

        // Charge for the cycles the user can afford
        let amount_to_charge = amount_per_cycle.clone() * cycles_to_charge;
        self.do_internal_transfer_and_update_balances(&sender, &receiver, &agreement.token_identifier, &amount_to_charge);

        let last_triggered_cycle = self.agreement_last_triggered_time_per_account(agreement.id, &account).get();
        let end_cycle = cycles_to_charge + last_triggered_cycle;

        self.agreement_last_triggered_time_per_account(agreement.id, &account).set(end_cycle);

        if cycles_to_charge == total_pending_cycles {
            // User had enough funds for all pending cycles
            return (Some((amount_to_charge, total_pending_cycles)), None);
        } else {
            // User had funds only for part of the pending cycles
            let cycles_failed = total_pending_cycles - cycles_to_charge;
            return (Some((amount_to_charge, cycles_to_charge)), Some((amount_per_cycle * cycles_failed, cycles_failed)));
        }
    }

    fn get_cycles_to_charge(&self, user_balance: &BigUint, amount_per_cycle: &BigUint, total_pending_cycles: u64) -> u64 {
        // Determine how many cycles the user can afford
        let affordable_cycles = (user_balance / amount_per_cycle).to_u64().unwrap_or(0);

        if total_pending_cycles <= affordable_cycles {
            return total_pending_cycles;
        }

        affordable_cycles
    }
}
