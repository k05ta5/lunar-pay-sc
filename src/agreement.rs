multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{Agreement, AgreementAmountType, AgreementType, FrequencyType};

#[multiversx_sc::module]
pub trait AgreementModule:
    crate::storage::StorageModule +
    crate::validation::ValidationModule
{
    #[endpoint(createRecuringPaymentAgreementToSend)]
    fn create_recurring_payment_agreement_to_send(
        &self,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
        amount_type: AgreementAmountType<Self::Api>,
        frequency: FrequencyType
    ) {
        let caller = self.blockchain().get_caller();
        self.require_address_is_whitelisted(&caller);

        let agreement_type = AgreementType::RecurringPayoutToSend {
            amount_type: amount_type,
            sender: caller.clone(),
            frequency: frequency,
            receivers: ManagedVec::new(),
        };

        self.create_recurring_agreement(&caller, agreement_type, token_identifier);
    }

    #[endpoint(createRecuringPaymentAgreementToReceive)]
    fn create_recurring_payment_agreement_to_receive(
        &self,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
        amount_type: AgreementAmountType<Self::Api>,
        frequency: FrequencyType
    ) {
        let caller = self.blockchain().get_caller();
        self.require_address_is_whitelisted(&caller);

        let agreement_type = AgreementType::RecurringPayoutToReceive {
            amount_type: amount_type,
            senders: ManagedVec::new(),
            receiver: caller.clone(),
            frequency: frequency,

            whitelist_enabled: Option::None,
            whitelisted_addresses: Option::None,
        };

        let _agreement = self.create_recurring_agreement(
            &caller,
            agreement_type,
            token_identifier
        );
    }

    #[endpoint(signAgreement)]
    fn sign_agreement(&self, agreement_id: u64) {
        self.require_existing_agreement_id(agreement_id);

        let caller = self.blockchain().get_caller();

        self.require_agreement_not_created_by_account(&caller, agreement_id);
        self.require_agreement_not_signed_by_account(&caller, agreement_id);

        let mut agreement = self.agreement_by_id(agreement_id).get();

        match agreement.agreement_type {
            AgreementType::RecurringPayoutToReceive {
                amount_type,
                senders,
                frequency, .. } => {

                senders.push(caller.clone());
            },

            AgreementType::TimeBoundPayoutToReceive {
                mut senders,
                frequency,
                ..
            } => {
                senders.push(caller);
            },

            _ => panic!("Invalid agreement type")
        }

        self.agreement_by_id(agreement_id).set(agreement);
        self.account_signed_agreements_list(&caller).insert(agreement_id);
    }

    #[endpoint(signAgreement)]
    fn cancel_agreement(&self, agreement_id: u64) {
        self.require_existing_agreement_id(agreement_id);

        let caller = self.blockchain().get_caller();

        let mut agreement = self.agreement_by_id(agreement_id).get();

        match agreement.agreement_type {
            AgreementType::RecurringPayoutToReceive {
                amount_type,
                receiver,
                frequency, .. } => {},

            AgreementType::TimeBoundPayoutToReceive {..} => {},

            _ => panic!("Invalid agreement type")
        }

        self.agreement_by_id(agreement_id).set(agreement);
    }

    #[inline]
    fn create_recurring_agreement(
        &self,
        owner: &ManagedAddress,
        agreement_type: AgreementType<Self::Api>,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>
    ) -> Agreement<Self::Api> {
        let agreement_number = self.create_agreement_identifier();

        let agreement = Agreement {
            creator: owner.clone(),

            token_nonce: 0,
            token_identifier: token_identifier,

            agreement_type: agreement_type,
            claimed_amount: BigUint::zero()
        };

        self.agreement_ids().insert(agreement_number);
        self.account_created_agreements_list(&owner).insert(agreement_number);
        self.agreement_by_id(agreement_number).set(&agreement);

        return agreement;
    }

    #[inline]
    fn create_agreement_identifier(&self) -> u64 {
        self.last_agreement_id().update(|uac| *uac += 1);
        self.last_agreement_id().get()
    }
}
