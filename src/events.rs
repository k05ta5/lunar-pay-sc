multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("createPaymentAgreement")]
    fn create_payment_agreement(
        &self,
    );

    #[event("cancelPaymentAgreement")]
    fn cancel_payment_agreement(
        &self,
    );
}