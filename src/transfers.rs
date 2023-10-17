multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait TransfersModule:
    crate::events::EventsModule +
    crate::storage::StorageModule +
    crate::validation::ValidationModule
{
    #[endpoint(transferTokens)]
    fn transfer(&self, token: EgldOrEsdtTokenIdentifier, amount: BigUint,  receiver: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        require!(caller != receiver, "Sender and receiver must be different");

        self.do_transfer_and_update_balance(&caller, &receiver, &token, &amount);
        self.transfer_event(&caller, &receiver, &token, 0, &amount, false);
    }

    #[inline]
    fn do_transfer_and_update_balance(
        &self,
        sender: &ManagedAddress<Self::Api>,
        receiver: &ManagedAddress<Self::Api>,
        token: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
    ) {
        self.require_account_has_sufficient_balance(&sender, &token, &amount.clone());

        self.account_balance(&sender, &token).update(|balance| *balance -= &amount.clone());
        self.send().direct(&receiver, &token, 0, &amount.clone());
    }

    #[inline]
    fn do_agreement_transfer_and_update_balances(
        &self,
        sender: &ManagedAddress<Self::Api>,
        receiver: &ManagedAddress<Self::Api>,
        token: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
    ) -> bool {
        self.require_account_has_sufficient_balance(&sender, &token, &amount.clone());

        // if !self.account_has_sufficient_balance(&sender, &token, &amount.clone()) {
        //     let transfer = AgreementTransfer {
        //         amount: amount.clone(),
        //         transfer_time: time.clone(),
        //         status: AgreementTransferStatus::FAILED,
        //         reason: AgreementTransferReason::InsufficientFunds
        //     };
        //
        //     self.agreement_sender_transfers(agreement_id, &sender).insert(transfer.clone());
        //     self.agreement_receiver_transfers(agreement_id, &receiver).insert(transfer.clone());
        //     return false;
        // }

        self.account_balance(&sender, &token).update(|balance| *balance -= &amount.clone());
        self.account_balance(&receiver, &token).update(|balance| *balance += &amount.clone());
        // let transfer = AgreementTransfer {
        //     amount: amount.clone(),
        //     transfer_time: time.clone(),
        //     status: AgreementTransferStatus::SUCCESS,
        //     reason: AgreementTransferReason::None
        // };
        // self.agreement_sender_transfers(agreement_id, &sender).insert(transfer.clone());
        // self.agreement_receiver_transfers(agreement_id, &receiver).insert(transfer.clone());
        // self.agreement_last_successful_transfer_time(agreement_id, &sender, &receiver).set(time.clone());
        return true;
    }
}
