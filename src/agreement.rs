multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{Agreement, AgreementType, AgreementAmountType, Amount};

#[multiversx_sc::module]
pub trait AgreementsModule:
    crate::events::EventsModule +
    crate::storage::StorageModule +
    crate::transfers::TransfersModule +
    crate::validation::ValidationModule +
    crate::agreement_amount::AgreementAmountModule
{
    #[endpoint(createPaymentAgreement)]
    fn create_payment_agreement(
        &self,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
        agreement_type: AgreementType,
        amount_type: AgreementAmountType,
        frequency: u64,
        _amount: Option<Amount<Self::Api>>
    ) {
        let caller = self.blockchain().get_caller();

        self.require_token_is_whitelisted(&token_identifier);
        self.require_address_is_whitelisted(&caller);

        let agreement_identifier = self.create_agreement_identifier();
        let timestamp = self.blockchain().get_block_timestamp();

        let agreement = Agreement {
            id: agreement_identifier.clone(),
            creator: caller.clone(),

            token_nonce: 0,
            token_identifier,

            frequency,
            time_created: timestamp,

            agreement_type,
            amount_type,
        };

        let amount: Amount<Self::Api> = self.construct_agreement_amount(agreement_type, amount_type, _amount);
        self.agreement_amount(agreement_identifier).set(amount);

        self.agreement_ids().insert(agreement_identifier);
        self.agreement_by_id(agreement_identifier).set(&agreement);
        self.account_created_agreements_list(&caller).insert(agreement_identifier);
    }

    #[inline]
    fn create_agreement_identifier(&self) -> u64 {
        self.last_agreement_id().update(|id| *id += 1);
        self.last_agreement_id().get()
    }
}
