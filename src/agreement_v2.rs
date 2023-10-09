multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::*;

#[multiversx_sc::module]
pub trait AgreementV2Module:
    crate::storage::StorageModule +
    crate::transfers::TransfersModule +
    crate::charges::ChargesModule +
    crate::validation::ValidationModule
{
    #[endpoint(createRecuringPaymentAgreementToSend)]
    fn create_recurring_payment_agreement_to_send(
        &self,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
        amount_type: RecurringPayoutToSendAmountType<Self::Api>,
        frequency: FrequencyType,
        _whitelisted_addresses: Option<ManagedVec<ManagedAddress<Self::Api>>>
    ) {
        let caller = self.blockchain().get_caller();
        self.require_address_is_whitelisted(&caller);

        let agreement_type = AgreementType::RecurringPayoutToSend {
            amount_type,
            sender: caller.clone(),
            frequency
        };

        self.create_agreement(&caller, agreement_type, token_identifier, _whitelisted_addresses);
    }

    #[endpoint(createRecuringPaymentAgreementToReceive)]
    fn create_recurring_payment_agreement_to_receive(
        &self,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
        amount_type: PayoutToReceiveAmountType<Self::Api>,
        frequency: FrequencyType,
        _whitelisted_addresses: Option<ManagedVec<ManagedAddress<Self::Api>>>
    ) {
        let caller = self.blockchain().get_caller();
        self.require_address_is_whitelisted(&caller);

        let agreement_type = AgreementType::RecurringPayoutToReceive {
            amount_type,
            receiver: caller.clone(),
            frequency,
        };

        self.create_agreement(
            &caller,
            agreement_type,
            token_identifier,
            _whitelisted_addresses,
        );
    }

    #[endpoint(createTimeBoundPaymentAgreementToSend)]
    fn create_time_bound_payment_agreement_to_send(
        &self,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
        amount_type: TimeBoundPayoutToSendAmountType<Self::Api>,
        frequency: FrequencyType,
        _whitelisted_addresses: Option<ManagedVec<ManagedAddress<Self::Api>>>
    ) {
        let caller = self.blockchain().get_caller();
        self.require_address_is_whitelisted(&caller);

        let agreement_type = AgreementType::TimeBoundPayoutToSend {
            amount_type,
            sender: caller.clone(),
            frequency
        };

        self.create_agreement(&caller, agreement_type, token_identifier, _whitelisted_addresses);
    }

    #[endpoint(createTimeBoundPaymentAgreementToReceive)]
    fn create_time_bound_payment_agreement_to_receive(
        &self,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
        amount_type: PayoutToReceiveAmountType<Self::Api>,
        frequency: FrequencyType,
        _whitelisted_addresses: Option<ManagedVec<ManagedAddress<Self::Api>>>
    ) {
        let caller = self.blockchain().get_caller();
        self.require_address_is_whitelisted(&caller);

        let agreement_type = AgreementType::TimeBoundPayoutToReceive {
            amount_type,
            receiver: caller.clone(),
            frequency,
        };

        self.create_agreement(
            &caller,
            agreement_type,
            token_identifier,
            _whitelisted_addresses,
        );
    }

    // TODO: Implement this
    // #[endpoint(addAgreementBeneficiary)]
    // fn add_agreement_beneficiary(&self, agreement_id: u64) {
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

    /**
     *  Subscribe to an agreement, allowed to subscribe as sender to RecurringPayoutToReceive and TimeBoundPayoutToReceive
     */
    #[endpoint(signAgreement)]
    fn sign_agreement(&self, agreement_id: u64) {
        self.require_existing_agreement_id(agreement_id);
        let agreement = self.agreement_by_id(agreement_id).get();
        match agreement.agreement_type {
            AgreementType::RecurringPayoutToReceive {amount_type, ..} => {
                match amount_type {
                    PayoutToReceiveAmountType::SubscriberDefinedAmount => panic!("You cannot sign this agreement without providing an amount, use signAgreementWithAmount instead"),
                    _ => {}
                }
            },

            AgreementType::TimeBoundPayoutToReceive {amount_type, ..} => {
                match amount_type {
                    PayoutToReceiveAmountType::SubscriberDefinedAmount => panic!("You cannot sign this agreement without providing an amount, use signAgreementWithAmount instead"),
                    _ => {}
                }
            }

            _ => panic!("You cannot sign this agreement type.")
        }

        let caller = self.blockchain().get_caller();

        self.require_agreement_not_created_by_account(&caller, agreement_id);
        self.require_agreement_not_signed_by_account(agreement_id, &caller);
        self.require_address_is_whitelisted_for_agreeement(&caller, agreement_id);

        self.set_storage_for_sign_agreement(agreement_id, &caller);
    }

    /**
     *  Subscribe to an agreement, only allowed to subscribe as sender to RecurringPayoutToReceive and TimeBoundPayoutToReceive
     */
    #[endpoint(signAgreementWithAmount)]
    fn sign_agreement_with_amount(&self, agreement_id: u64, amount: BigUint) {
        self.require_existing_agreement_id(agreement_id);
        let agreement = self.agreement_by_id(agreement_id).get();
        match agreement.agreement_type {
            AgreementType::RecurringPayoutToReceive {amount_type, ..} => {
                match amount_type {
                    PayoutToReceiveAmountType::SubscriberDefinedAmount => {},
                    _ => panic!("You cannot sign this agreement with amount, use signAgreement instead")
                }
            },

            AgreementType::TimeBoundPayoutToReceive {..} => {},

            _ => panic!("You cannot sign this agreement type.")
        }

        let caller = self.blockchain().get_caller();

        self.require_agreement_not_created_by_account(&caller, agreement_id);
        self.require_agreement_not_signed_by_account(agreement_id, &caller);
        self.require_address_is_whitelisted_for_agreeement(&caller, agreement_id);

        self.agreement_subscriber_defined_amount(agreement_id, &caller).set(amount);

        self.set_storage_for_sign_agreement(agreement_id, &caller);
        
    }

    #[inline]
    fn set_storage_for_sign_agreement(&self, agreement_id: u64, caller: &ManagedAddress<Self::Api>) {
        let timestamp = self.blockchain().get_block_timestamp();

        self.agreement_all_signers(agreement_id).insert(caller.clone());
        self.agreement_current_signers(agreement_id).insert(caller.clone());
        self.agreement_signer_sign_time(agreement_id, &caller).set(timestamp);
        self.account_signed_agreements_list(&caller).insert(agreement_id);
    }


    /** 
     * Unsubscribe from an agreement, only allowed to unsubscribe as sender from RecurringPayoutToReceive and TimeBoundPayoutToReceive
     */
    #[endpoint(cancelAgreement)]
    fn cancel_agreement(&self, agreement_id: u64) {
        self.require_existing_agreement_id(agreement_id);

        let caller = self.blockchain().get_caller();

        self.require_agreement_signed_by_account(agreement_id, &caller);
        
        let agreement = self.agreement_by_id(agreement_id).get();

        self.charge_agreement_sender(&agreement, &caller);

        let timestamp = self.blockchain().get_block_timestamp();
    
        match agreement.agreement_type {
            AgreementType::RecurringPayoutToReceive {..} => {},
    
            AgreementType::TimeBoundPayoutToReceive {..} => {},
    
            _ => panic!("You cannot cancel this agreement")
        }

        self.agreement_current_signers(agreement_id).swap_remove(&caller);
        self.agreement_signer_cancel_time(agreement_id, &caller).set(timestamp);
        self.account_signed_agreements_list(&caller).swap_remove(&agreement_id);
    }

    // #[endpoint(claimAgreement)]
    // fn claim_agreement(&self, agreement_id: u64) {
    //     self.require_existing_agreement_id(agreement_id);

    //     let caller = self.blockchain().get_caller();
    //     self.require_agreement_signed_by_account(agreement_id, &caller);

    //     let mut _agreement = self.agreement_by_id(agreement_id).get();

    //     match _agreement.agreement_type {
    //         // Claim tokens for an agreement that this account is a receiver of **/
    //         AgreementType::RecurringPayoutToSend {..} => {},

    //         // Claim tokens for an agreement that this account is a receiver of **/
    //         AgreementType::TimeBoundPayoutToSend {..} => {},

    //         _ => panic!("You cannot claim tokens for this agreement")
    //     }

    //     self.agreement_by_id(agreement_id).set(_agreement);
    //     self.account_signed_agreements_list(&caller).insert(agreement_id);
    // }

    // TODO: Implement this

    // #[endpoint(payAgreement)]
    // fn pay_agreement(&self, agreement_id: u64, address: Option<ManagedAddress<Self::Api>>) {
    //     self.require_existing_agreement_id(agreement_id);
    //
    //     let caller = self.blockchain().get_caller();
    //
    //     let mut agreement = self.agreement_by_id(agreement_id).get();
    //
    //     match agreement.agreement_type {
    //         AgreementType::RecurringPayoutToReceive {..} => {
    //             self.require_agreement_not_created_by_account(&caller, agreement_id);
    //             self.require_agreement_signed_by_account(&caller, agreement_id);
    //         },
    //
    //         AgreementType::TimeBoundPayoutToReceive {..} => {
    //             self.require_agreement_not_created_by_account(&caller, agreement_id);
    //             self.require_agreement_signed_by_account(&caller, agreement_id);
    //         },
    //
    //         AgreementType::RecurringPayoutToSend {..} => {
    //             self.require_agreement_created_by_account(&caller, agreement_id);
    //         },
    //
    //         AgreementType::TimeBoundPayoutToSend {..} => {
    //             self.require_agreement_created_by_account(&caller, agreement_id);
    //         },
    //     }
    //
    //     self.agreement_by_id(agreement_id).set(agreement);
    //     self.account_signed_agreements_list(&caller).insert(agreement_id);
    // }

    #[inline]
    fn create_agreement(
        &self,
        owner: &ManagedAddress,
        agreement_type: AgreementType<Self::Api>,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
        _whitelisted_addresses: Option<ManagedVec<ManagedAddress<Self::Api>>>
    ) -> Agreement<Self::Api> {
        let agreement_number = self.create_agreement_identifier();

        let agreement = Agreement {
            id: agreement_number.clone(),
            creator: owner.clone(),

            token_nonce: 0,
            token_identifier,

            agreement_type,
            total_transfered_amount: BigUint::zero()
        };

        self.agreement_ids().insert(agreement_number);
        self.agreement_by_id(agreement_number).set(&agreement);
        self.account_created_agreements_list(&owner).insert(agreement_number);

        let whitelisted_addresses = _whitelisted_addresses.unwrap_or_default();
        if !whitelisted_addresses.is_empty() {
            self.agreement_whitelist_enabled(agreement_number).set(true);
            for address in whitelisted_addresses.iter() {
                self.agreement_whitelist(agreement_number).insert(address.clone_value());
            }
        }

        return agreement;
    }

    #[inline]
    fn create_agreement_identifier(&self) -> u64 {
        self.last_agreement_id().update(|uac| *uac += 1);
        self.last_agreement_id().get()
    }
}
