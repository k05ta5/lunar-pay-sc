multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{AgreementAmountType, AgreementType, Amount};

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("deposit")]
    fn deposit_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
        #[indexed] token_nonce: u64,
        #[indexed] amount: &BigUint,
    );

    #[event("withdraw")]
    fn withdraw_event(
        &self,
        #[indexed] address: &ManagedAddress,
        #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
        #[indexed] token_nonce: u64,
        #[indexed] amount: &BigUint,
    );

    #[event("transfer")]
    fn transfer_event(
        &self,
        #[indexed] sender: &ManagedAddress,
        #[indexed] receiver: &ManagedAddress,
        #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
        #[indexed] token_nonce: u64,
        #[indexed] amount: &BigUint,
        #[indexed] is_internal: bool,
    );

    #[event("createPaymentAgreement")]
    fn create_payment_agreement_event(
        &self,
        #[indexed] agreement_id: u64,
        #[indexed] owner: &ManagedAddress,
        #[indexed] token_nonce: u64,
        #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
        #[indexed] frequency: u64,
        #[indexed] time_created: u64,
        #[indexed] agreement_type: AgreementType,
        #[indexed] amount_type: AgreementAmountType,
        #[indexed] amount: Option<Amount<Self::Api>>,
    );

    #[event("cancelPaymentAgreement")]
    fn cancel_payment_agreement_event(
        &self,
    );

    #[event("chargePaymentAgreement")]
    fn charge_payment_agreement_event(
        &self,
    );
}