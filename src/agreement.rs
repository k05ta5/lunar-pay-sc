multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AgreementModule:
    crate::storage::StorageModule +
    crate::validation::ValidationModule
{
    #[endpoint(createPaymentAgreement)]
    fn create_payment_agreement(&self) {
        let caller = self.blockchain().get_caller();
    }

    #[endpoint(cancelPaymentAgreement)]
    fn cancel_payment_agreement(&self, id: u64) {
        let caller = self.blockchain().get_caller();
    }
}
