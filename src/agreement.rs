multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AgreementModule:
    crate::storage::StorageModule +
    crate::validation::ValidationModule
{
    #[endpoint(createUAC)]
    fn create_uac(&self) {
        let caller = self.blockchain().get_caller();
        self.require_address_is_whitelisted(&caller);

        self.last_uac().update(|uac| *uac += 1);
        self.account_uac_list(&caller).insert(self.last_uac().get());
    }

    #[endpoint(createPaymentAgreement)]
    fn create_payment_agreement(&self) {}

    #[endpoint(cancelPaymentAgreement)]
    fn cancel_payment_agreement(&self, id: u64) {}
}
