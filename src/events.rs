multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{AgreementAmountType, AgreementType};

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

    #[event("payment")]
    fn payment_event(
        &self,
        #[indexed] sender: &ManagedAddress,
        #[indexed] receiver: &ManagedAddress,
        #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
        #[indexed] token_nonce: u64,
        #[indexed] amount: &BigUint,
        metadata: Option<ManagedBuffer>,
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
        #[indexed] fixed_amount: BigUint,
        #[indexed] minimum_amount: BigUint,
        #[indexed] maximum_amount: BigUint,
    );

    #[event("signPaymentAgreement")]
    fn sign_payment_agreement_event(
        &self,
        #[indexed] agreement_id: u64,
        #[indexed] member: &ManagedAddress,
        #[indexed] signed_at: u64,
    );

    #[event("cancelPaymentAgreement")]
    fn cancel_payment_agreement_event(
        &self,
    );

    #[event("successfulAgreementCharges")]
    fn successful_charges_event(
        &self,
        #[indexed] agreement_id: u64,
        #[indexed] accounts: ManagedVec<ManagedAddress<Self::Api>>,
        #[indexed] amounts: ManagedVec<BigUint>,
        #[indexed] cycles: ManagedVec<u64>,
    );

    #[event("failedAgreementCharges")]
    fn failed_charges_event(
        &self,
        #[indexed] agreement_id: u64,
        #[indexed] accounts: ManagedVec<ManagedAddress<Self::Api>>,
        #[indexed] amounts: ManagedVec<BigUint>,
        #[indexed] cycles: ManagedVec<u64>,
    );
}