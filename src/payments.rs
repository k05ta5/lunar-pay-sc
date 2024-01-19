multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait PaymentsModule:
crate::events::EventsModule +
crate::storage::StorageModule +
crate::transfers::TransfersModule +
crate::validation::ValidationModule
{
    #[endpoint(transferTokens)]
    fn pay(
        &self,
        token: EgldOrEsdtTokenIdentifier,
        amount: BigUint,
        receiver: ManagedAddress,
        metadata: Option<ManagedBuffer<Self::Api>>
    ) {
        let caller = self.blockchain().get_caller();
        require!(caller != receiver, "Invalid receiver address");

        self.do_transfer_and_update_balance(&caller, &receiver, &token, &amount);
        self.payment_event(&caller, &receiver, &token, 0, &amount, metadata);
    }
}
