multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{Agreement, AgreementType, FrequencyType, PayoutToReceiveAmountType};

#[multiversx_sc::module]
pub trait ChargesModule:
    crate::storage::StorageModule +
    crate::transfers::TransfersModule +
    crate::validation::ValidationModule
{
    /**
     * Charge all the senders from one agreement
     * Endpoint can be called only by owner for RecurringPayoutToReceive and TimeBoundPayoutToReceive
     */
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

        let senders = self.agreement_current_signers(agreement_id);
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
        let last_charge_time = self.calculate_agreement_sender_last_charge_time(agreement.id, &sender, &agreement.creator);
        let sender_sign_time = self.agreement_signer_sign_time(agreement.id, &sender).get();
        let mut amount_to_transfer = BigUint::zero();

        match &agreement.agreement_type {
            // Charge the sender(s) of an agreement that this account created
            AgreementType::RecurringPayoutToReceive {amount_type, frequency, ..} => {
                let frequency_seconds = self.calculate_frequency_seconds(frequency);
                let charge_periods_count = (timestamp - sender_sign_time).checked_div(frequency_seconds).unwrap();
                let already_charged_period_counts = (last_charge_time - sender_sign_time).checked_div(frequency_seconds).unwrap();
                
                let mut to_charge_times = charge_periods_count - already_charged_period_counts;

                let last_charge_period_end_timestamp = sender_sign_time + (frequency_seconds * charge_periods_count);
                if timestamp > last_charge_period_end_timestamp && last_charge_time <= last_charge_period_end_timestamp {
                    to_charge_times += 1;
                }

                if to_charge_times > 0 {
                    match &amount_type {
                        PayoutToReceiveAmountType::FixedAmount(amount) => {
                            amount_to_transfer = amount.clone() * to_charge_times;
                        },
                        PayoutToReceiveAmountType::SubscriberDefinedAmount => {
                            amount_to_transfer = self.agreement_subscriber_defined_amount(agreement.id, &sender).get() * to_charge_times;
                        }
                    }
                }
            },

            // Charge the sender(s) of an agreement that this account created **/
            AgreementType::TimeBoundPayoutToReceive {amount_type, frequency, ..} => {
                let frequency_seconds = self.calculate_frequency_seconds(frequency);
                let charge_periods_count = (timestamp - sender_sign_time).checked_div(frequency_seconds).unwrap();
                let last_charge_period_end_timestamp = sender_sign_time + (frequency_seconds * charge_periods_count);

                if timestamp > last_charge_period_end_timestamp && last_charge_time <= last_charge_period_end_timestamp {
                    match &amount_type {
                        PayoutToReceiveAmountType::FixedAmount(amount) => {
                            amount_to_transfer = amount.clone();
                        },
                        PayoutToReceiveAmountType::SubscriberDefinedAmount => {
                            amount_to_transfer = self.agreement_subscriber_defined_amount(agreement.id, &sender).get();
                        }
                    }
                }
            },

            _ => panic!("You cannot charge tokens for this agreement")
        }

        if amount_to_transfer > 0 {
            self.do_agreement_transfer_and_update_balances(
                agreement.id,
                &sender,
                &agreement.creator,
                &agreement.token_identifier,
                &amount_to_transfer,
                timestamp
            );
        }

    }

    #[inline]
    fn calculate_agreement_sender_last_charge_time(&self, agreement_id: u64, sender: &ManagedAddress<Self::Api>, receiver: &ManagedAddress<Self::Api>) -> u64 {
        let last_charge_time = self.agreement_last_successful_transfer_time(agreement_id, &sender, &receiver);
        if last_charge_time.is_empty() {
            return self.agreement_signer_sign_time(agreement_id, &sender).get()
        }

        return last_charge_time.get()
    }

    #[inline]
    fn calculate_frequency_seconds(&self, frequency: &FrequencyType) -> u64 {
        match &frequency {
            FrequencyType::HOUR => 3600,  
            FrequencyType::DAY => 3600 * 24,     // 1 day = 86400 seconds
            FrequencyType::WEEK => 3600 * 24 * 7,   // 1 week = 604800 seconds
            FrequencyType::MONTH => 2592000, // 1 month ≈ 30.44 days ≈ 2592000 seconds
            FrequencyType::YEAR => 31536000, // 1 year = 31536000 seconds
        }
    }
}
