multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait AccountModule:
    crate::events::EventsModule +
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

        if !self.used_token_ids().contains(&token) {
            self.used_token_ids().insert(token.clone());
        }

        self.deposit_event(&caller, &token, 0, &payment_value);
        self.account_balance(&caller, &token).update(|balance| *balance += &payment_value);
    }

    #[endpoint(withdrawEgld)]
    fn withdraw_egld(&self, amount: &BigUint) {
        let caller = self.blockchain().get_caller();
        let token = EgldOrEsdtTokenIdentifier::egld();

        self.withdraw_event(&caller, &token, 0, &amount);
        self.do_transfer_and_update_balance(&caller, &caller, &token, amount);
    }

    #[payable("*")]
    #[endpoint(depositEsdt)]
    fn deposit_esdt(&self) {
        let caller = self.blockchain().get_caller();
        let transfer = self.call_value().single_esdt();

        let amount = transfer.amount;
        let token = EgldOrEsdtTokenIdentifier::esdt(transfer.token_identifier);

        require!(self.is_token_whitelisted(&token), "Token is not whitelisted");

        if !self.used_token_ids().contains(&token) {
            self.used_token_ids().insert(token.clone());
        }

        self.deposit_event(&caller, &token, 0, &amount);
        self.account_balance(&caller, &token).update(|balance| *balance += &amount);
    }

    #[endpoint(withdrawEsdt)]
    fn withdraw_esdt(&self, token: &EgldOrEsdtTokenIdentifier, amount: &BigUint) {
        let caller = self.blockchain().get_caller();
        self.withdraw_event(&caller, &token, 0, &amount);
        self.do_transfer_and_update_balance(&caller, &caller, token, amount);
    }
}
