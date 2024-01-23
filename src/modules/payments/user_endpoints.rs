multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait UserEndpointsModule:
crate::storage::StorageModule +
crate::validation::ValidationModule +
crate::modules::payments::events::EventsModule +
crate::modules::transfers::balance_transfer::BalanceTransferModule +
{
    #[endpoint(pay)]
    fn pay(
        &self,
        token: EgldOrEsdtTokenIdentifier,
        amount: BigUint,
        receiver: ManagedAddress,
        metadata: Option<ManagedBuffer<Self::Api>>
    ) {
        let caller = self.blockchain().get_caller();
        require!(caller != receiver, "Invalid receiver address");

        self.do_internal_transfer_and_update_balances(&caller, &receiver, &token, &amount);
        self.payment_event(&caller, &receiver, &token, 0, &amount, metadata);
    }
}
