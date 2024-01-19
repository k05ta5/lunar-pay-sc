multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{AgreementType};

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
    fn sign_agreement(&self, agreement_id: u64, metadata: Option<ManagedBuffer<Self::Api>>) {
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

        // We charge one full cycle when the agreement is signed
        let cycle_cost = self.get_charge_value(agreement.id, agreement.amount_type, &caller);
        self.do_internal_transfer_and_update_balances(&caller, &agreement.owner, &agreement.token_identifier, &cycle_cost);
        self.agreement_last_triggered_time_per_account(agreement.id, &caller).set(timestamp);

        self.sign_payment_agreement_event(agreement_id, &caller, timestamp, metadata);
    }

    /* TODO: Only RecurringPayoutToReceive agreements can be signed for the xDay Hackathon */
    fn can_account_sign_agreement(&self, agreement_type: AgreementType) -> bool {
        match agreement_type {
            AgreementType::RecurringPayoutToReceive => true,
            _ => false
        }
    }
}
