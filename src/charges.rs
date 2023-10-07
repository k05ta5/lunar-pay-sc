multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{Agreement, AgreementAmountType, AgreementType, FrequencyType};

#[multiversx_sc::module]
pub trait ChargesModule:
    crate::storage::StorageModule +
    crate::validation::ValidationModule
{
    #[endpoint(chargeAgreementSenders)]
    fn charge_agreement_senders(&self, agreement_id: u64) {
        self.require_existing_agreement_id(agreement_id);

        let caller = self.blockchain().get_caller();
        let agreement = self.agreement_by_id(agreement_id).get();

        match agreement.agreement_type {
            // Charge the sender(s) of an agreement that this account created
            AgreementType::RecurringPayoutToReceive {..} => {
                self.require_agreement_created_by_account(&caller, agreement_id);
            },

            // Charge the sender(s) of an agreement that this account created **/
            AgreementType::TimeBoundPayoutToReceive {..} => {
                self.require_agreement_created_by_account(&caller, agreement_id);
            },

            _ => panic!("You cannot charge tokens for this agreement")
        }

        let senders = self.agreement_current_senders(agreement_id);
        if senders.is_empty() {
            panic!("You have no senders for this agreement");
        }

        for sender in senders.iter() {
            self.charge_agreement_sender(&agreement, &sender)
        }
        
    }

    #[inline]
    fn charge_agreement_sender(&self, agreement: &Agreement<Self::Api>, sender: &ManagedAddress<Self::Api>) {
        let timestamp = self.blockchain().get_block_timestamp();
        
        let last_charge_time = self.calculate_agreement_sender_last_charge_time(agreement.id, &sender);

        let mut amount_to_transfer = BigUint::zero();

        match agreement.agreement_type {
            // Charge the sender(s) of an agreement that this account created
            AgreementType::RecurringPayoutToReceive {..} => {},

            // Charge the sender(s) of an agreement that this account created **/
            AgreementType::TimeBoundPayoutToReceive {..} => {},

            _ => panic!("You cannot charge tokens for this agreement")
        }

        let amount_to_transfer = self.calculate_agreement_sender_amount_to_charge(&agreement, &sender);

        // TODO: do the transfer if enough amount and save charge with amount and timestamp as failed or success

        self.agreement_sender_last_charge_time(agreement.id, &sender).set(timestamp)

    }

    // TODO: implement this
    #[inline]
    fn calculate_agreement_sender_amount_to_charge(&self, agreement: &Agreement<Self::Api>, sender: &ManagedAddress<Self::Api>) -> BigUint {
        return BigUint::zero();
    }

    #[inline]
    fn calculate_agreement_sender_last_charge_time(&self, agreement_id: u64, sender: &ManagedAddress<Self::Api>) -> u64 {
        let last_charge_time = self.agreement_sender_last_charge_time(agreement_id, &sender);
        if last_charge_time.is_empty() {
            return self.agreement_sender_sign_time(agreement_id, &sender).get()
        }

        return last_charge_time.get()
    }
}
