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
    fn create_payment_agreement(&self) {
        //TODO: Implement
    }

    #[endpoint(cancelPaymentAgreement)]
    fn cancel_payment_agreement(&self, _id: u64) {
        //TODO: Implement
    }

    #[endpoint(createSubscription)]
    fn create_subscription(&self){
        let caller = self.blockchain().get_caller();

        self.require_address_is_whitelisted(&caller);
        //TODO: Implement
    }

    #[endpoint(subscribe)]
    fn subscribe(&self, _agreement_id: &ManagedBuffer){
        let _caller = self.blockchain().get_caller();
        //TODO: Implement
    }

    #[endpoint(unsubscribe)]
    fn unsubscribe(&self, _agreement_id: &ManagedBuffer){
        let _caller = self.blockchain().get_caller();
        //TODO: Implement
    }

    #[endpoint(chargeAllSubscriptions)]
    fn charge_all_subscription(&self, _agreement_id: &ManagedBuffer){
        let _caller = self.blockchain().get_caller();
        //TODO: Implement
    }

    #[endpoint(chargeSubscription)]
    fn charge_subscription(&self, _agreement_id: &ManagedBuffer){
        let _caller = self.blockchain().get_caller();
        //TODO: Implement
    }

    #[endpoint(paySubscription)]
    fn pay_subscription(&self, _agreement_id: &ManagedBuffer){
        let _caller = self.blockchain().get_caller();
        //TODO: Implement
    }
}
