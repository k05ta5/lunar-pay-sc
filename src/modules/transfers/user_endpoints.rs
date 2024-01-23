multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait UserEndpointsModule:
    crate::storage::StorageModule +
    crate::validation::ValidationModule +
    crate::modules::transfers::events::EventsModule +
    crate::modules::transfers::balance_transfer::BalanceTransferModule
{

    #[endpoint(transferTokens)]
    fn transfer(&self, token: EgldOrEsdtTokenIdentifier, amount: BigUint,  receiver: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        require!(caller != receiver, "Sender and receiver must be different");

        self.do_transfer_and_update_balance(&caller, &receiver, &token, &amount);
        self.transfer_event(&caller, &receiver, &token, 0, &amount, false);
    }
}
