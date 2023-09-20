multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AccountModule:
    crate::storage::StorageModule +
    crate::transfers::TransfersModule +
    crate::validation::ValidationModule +
{
    #[payable("EGLD")]
    #[endpoint(depositEgld)]
    fn deposit_egld(&self) {
        let caller = self.blockchain().get_caller();
        let token = EgldOrEsdtTokenIdentifier::egld();
        let payment_value = self.call_value().egld_value().clone_value();

        require!(self.is_token_whitelisted(&token), "Token is not whitelisted");

        self.account_balance(&caller, &token).update(|balance| *balance += &payment_value);
    }

    #[endpoint(withdrawEgld)]
    fn withdraw_egld(&self, amount: &BigUint) {
        let caller = self.blockchain().get_caller();
        let token = EgldOrEsdtTokenIdentifier::egld();

        self.do_transfer_and_update_balance(&caller, &caller, &token, amount);
    }

    #[payable("*")]
    #[endpoint(depositEsdt)]
    fn deposit_esdt(&self) {
        let caller = self.blockchain().get_caller();
        let transfers = self.call_value().all_esdt_transfers();

        for transfer in transfers.iter() {
            let amount = transfer.amount;
            let token = EgldOrEsdtTokenIdentifier::esdt(transfer.token_identifier);

            require!(self.is_token_whitelisted(&token), "Token is not whitelisted");

            self.account_balance(&caller, &token).update(|balance| *balance += &amount);
        }
    }

    #[endpoint(withdrawEsdt)]
    fn withdraw_esdt(&self, token: &EgldOrEsdtTokenIdentifier, amount: &BigUint) {
        let caller = self.blockchain().get_caller();
        self.do_transfer_and_update_balance(&caller, &caller, token, amount);
    }
}
