multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{AgreementType, Agreement};

#[multiversx_sc::module]
pub trait SignAgreementModule:
crate::events::EventsModule +
                                                    crate::storage::StorageModule +
                            crate::validation::ValidationModule +
crate::transfers::TransfersModule +
                            crate::agreement_amount::AgreementAmountModule
{
    /**
     * Subscribe to an agreement
     */
    #[endpoint(signAgreement)]
    fn sign_agreement(&self, agreement_id: u64) {
        self.require_existing_agreement_id(agreement_id);
        let agreement = self.agreement_by_id(agreement_id).get();

        let caller = self.blockchain().get_caller();

        self.require_agreement_not_created_by_account(&caller, agreement_id);
        self.require_agreement_not_signed_by_account(agreement_id, &caller);

        require!(self.can_account_sign_agreement(agreement.agreement_type), "You cannot sign this agreement type");

        let timestamp = self.blockchain().get_block_timestamp();

        self.agreement_accounts(agreement_id).insert(caller.clone());
        self.agreement_current_accounts(agreement_id).insert(caller.clone());
        self.agreement_sign_time_per_account(agreement_id, &caller).set(timestamp);

        self.account_signed_agreements_list(&caller).insert(agreement_id);

        let initial_charge_amount = self.calculate_charge_amount(&agreement, &caller);
        self.do_transfer_and_update_balance(&caller, &agreement.owner, &agreement.token_identifier, &initial_charge_amount);

        self.sign_payment_agreement_event(agreement_id, &caller, timestamp);
    }

    fn calculate_charge_amount(&self, agreement: &Agreement<Self::Api>, account: &ManagedAddress) -> BigUint {
        let current_timestamp = self.blockchain().get_block_timestamp();

        let completed_cycles = (current_timestamp - agreement.time_created) / agreement.frequency;
        let cycle_start_timestamp = agreement.time_created + completed_cycles * agreement.frequency;
        let time_elapsed_in_current_cycle = current_timestamp - cycle_start_timestamp;

        let cycle_cost = self.get_charge_value(agreement.id, agreement.amount_type, account);

        cycle_cost * time_elapsed_in_current_cycle / agreement.frequency
    }

    /* TODO: Only RecurringPayoutToReceive agreements can be signed for the xDay Hackathon */
    fn can_account_sign_agreement(&self, agreement_type: AgreementType) -> bool {
        match agreement_type {
            AgreementType::RecurringPayoutToReceive => true,
            _ => false
        }
    }
}
