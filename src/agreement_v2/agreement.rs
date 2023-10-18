multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{Agreement, AgreementType, AgreementAmountType, Amount};

#[multiversx_sc::module]
pub trait AgreementsModule:
    crate::events::EventsModule +
    crate::storage::StorageModule +
    crate::transfers::TransfersModule +
    crate::validation::ValidationModule
{
    #[endpoint(createPaymentAgreement)]
    fn create_payment_agreement(
        &self,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
        agreement_type: AgreementType,
        amount_type: AgreementAmountType,
        frequency: u64,
        _amount: Option<Amount<Self::Api>>,
        _trial_period: Option<u64>,
    ) {
        let caller = self.blockchain().get_caller();

        self.require_token_is_whitelisted(&token_identifier);
        self.require_address_is_whitelisted(&caller);

        let amount = self.construct_agreement_amount(agreement_type, amount_type, _amount);
        let agreement_identifier = self.create_agreement_identifier();

        let agreement = Agreement {
            id: agreement_identifier.clone(),
            creator: caller.clone(),

            token_nonce: 0,
            token_identifier,

            agreement_type,
            amount_type,
            amount
        };

        self.agreement_ids().insert(agreement_identifier);
        self.agreement_by_id(agreement_identifier).set(&agreement);
        self.account_created_agreements_list(&caller).insert(agreement_identifier);
    }

    //
    // #[endpoint(createRecurringPaymentAgreementToSend)]
    // fn create_recurring_payment_agreement_to_send(
    //     &self,
    //     token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
    //     amount_type: AgreementAmountType,
    //     frequency: u64
    // ) {
    //     let caller = self.blockchain().get_caller();
    //
    //     self.require_token_is_whitelisted(&token_identifier);
    //     self.require_address_is_whitelisted(&caller);
    //
    //     // Validate the amount type
    //     match amount_type {
    //         AgreementAmountType::FixedAmount |
    //         AgreementAmountType::CreatorDefinedFixedAmountPerReceiver => {}
    //         _ => panic!("Invalid amount type")
    //     }
    //
    //     let agreement_type = AgreementType::RecurringPayoutToSend;
    //
    //     self.create_recurring_agreement(&caller, agreement_type, token_identifier);
    // }

    // #[endpoint(createRecurringPaymentAgreementToReceive)]
    // fn create_recurring_payment_agreement_to_receive(
    //     &self,
    //     token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
    //     amount_type: AgreementAmountType,
    //     frequency: u64
    // ) {
    //     let caller = self.blockchain().get_caller();
    //
    //     self.require_token_is_whitelisted(&token_identifier);
    //     self.require_address_is_whitelisted(&caller);
    //
    //     // Validate the amount type
    //     match amount_type {
    //         AgreementAmountType::FixedAmount |
    //         AgreementAmountType::SenderDefinedFixedAmount => {},
    //         _ => panic!("Invalid amount type")
    //     }
    // }

    // TODO: Implement this
    // #[endpoint(addAgreementReceiver)]
    // fn add_agreement_receiver(&self, agreement_id: u64) {
    //     self.require_existing_agreement_id(agreement_id);
    //
    //     let caller = self.blockchain().get_caller();
    //     self.require_agreement_created_by_account(&caller, agreement_id);
    //
    //     let mut agreement = self.agreement_by_id(agreement_id).get();
    //
    //     match agreement.agreement_type {
    //         AgreementType::RecurringPayoutToSend {..} => {
    //             self.agreement_receivers(agreement_id).insert(caller.clone());
    //         },
    //
    //         AgreementType::TimeBoundPayoutToSend {..} => {
    //             self.agreement_receivers(agreement_id).insert(caller.clone());
    //         },
    //
    //         _ => panic!("Invalid agreement type")
    //     }
    //
    //     self.account_signed_agreements_list(&caller).insert(agreement_id);
    // }

    // /**
    //  * Unsubscribe from an agreement, only allowed to unsubscribe as sender from RecurringPayoutToReceive and TimeBoundPayoutToReceive
    //  */
    // #[endpoint(cancelAgreement)]
    // fn cancel_agreement(&self, agreement_id: u64) {
    //     self.require_existing_agreement_id(agreement_id);
    //
    //     let caller = self.blockchain().get_caller();
    //
    //     self.require_agreement_signed_by_account(agreement_id, &caller);
    //
    //     let agreement = self.agreement_by_id(agreement_id).get();
    //
    //     // self.charge_agreement_sender(&agreement, &caller);
    //
    //     let timestamp = self.blockchain().get_block_timestamp();
    //
    //     match agreement.agreement_type {
    //         AgreementType::RecurringPayoutToReceive {..} => {},
    //
    //         AgreementType::TermRestrictedPayoutToReceive {..} => {},
    //
    //         _ => panic!("You cannot cancel this agreement")
    //     }
    //
    //     self.agreement_current_accounts(agreement_id).swap_remove(&caller);
    //     self.agreement_cancel_time_per_account(agreement_id, &caller).set(timestamp);
    //     self.account_signed_agreements_list(&caller).swap_remove(&agreement_id);
    // }

    // #[inline]
    // fn create_recurring_agreement(
    //     &self,
    //     owner: &ManagedAddress,
    //     agreement_type: AgreementType,
    //     token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>
    // ) -> Agreement<Self::Api> {
    //     let agreement_identifier = self.create_agreement_identifier();
    //
    //     let agreement = Agreement {
    //         id: agreement_identifier.clone(),
    //         creator: owner.clone(),
    //
    //         token_nonce: 0,
    //         token_identifier,
    //
    //         agreement_type,
    //     };
    //
    //     self.agreement_ids().insert(agreement_identifier);
    //     self.agreement_by_id(agreement_identifier).set(&agreement);
    //     self.account_created_agreements_list(&owner).insert(agreement_identifier);
    //
    //     return agreement;
    // }

    #[inline]
    fn create_agreement_identifier(&self) -> u64 {
        self.last_agreement_id().update(|id| *id += 1);
        self.last_agreement_id().get()
    }
}
