multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{Amount, AgreementType, AgreementAmountType, Agreement};

#[multiversx_sc::module]
pub trait SignAgreementModule:
    crate::storage::StorageModule +
    crate::validation::ValidationModule +
    crate::agreement_amount::AgreementAmountModule
{
    /**
     * Subscribe to an agreement
     */
    #[endpoint(signAgreement)]
    fn sign_agreement(&self, agreement_id: u64, _amount: Option<Amount<Self::Api>>) {
        self.require_existing_agreement_id(agreement_id);
        let agreement = self.agreement_by_id(agreement_id).get();

        let caller = self.blockchain().get_caller();

        self.require_agreement_not_created_by_account(&caller, agreement_id);
        self.require_agreement_not_signed_by_account(agreement_id, &caller);

        /* Only RecurringPayoutToReceive and TermRestrictedPayoutToReceive agreements allows users to self-subscribe */
        require!(self.can_account_sign_agreement(agreement), "You cannot sign this agreement type");

        /* SenderDefinedFixedAmount and SenderDefinedBoundedAmount requires the caller to send the ammount they agree to pay */
        match agreement.amount_type {
            AgreementAmountType::SenderDefinedFixedAmount |
            AgreementAmountType::SenderDefinedBoundedAmount => {
                let amount = self.construct_defined_amount(agreement.amount_type, _amount);
                self.agreement_defined_amount_per_account(agreement_id, &caller).set(amount);
            },
            _ => {}
        }

        let timestamp = self.blockchain().get_block_timestamp();

        self.agreement_accounts(agreement_id).insert(caller.clone());
        self.agreement_current_accounts(agreement_id).insert(caller.clone());
        self.agreement_sign_time_per_account(agreement_id, &caller).set(timestamp);

        self.account_signed_agreements_list(&caller).insert(agreement_id);
    }

    fn calculate_charge_amount(&self, agreement: &Agreement<Self::Api>, account: &ManagedAddress) -> BigUint {
        let current_timestamp = self.blockchain().get_block_timestamp();

        let completed_cycles = (current_timestamp - agreement.time_created) / agreement.frequency;
        let cycle_start_timestamp = agreement.time_created + completed_cycles * agreement.frequency;
        let time_elapsed_in_current_cycle = current_timestamp - cycle_start_timestamp;

        let cycle_cost = self.get_charge_value(agreement.id, agreement.amount_type, account);

        cycle_cost * time_elapsed_in_current_cycle / agreement.frequency
    }

    fn can_account_sign_agreement(&self, agreement: Agreement<Self::Api>) -> bool {
        match agreement.agreement_type {
            AgreementType::RecurringPayoutToReceive => true,
            AgreementType::TermRestrictedPayoutToReceive => true,
            _ => false
        }
    }
}
