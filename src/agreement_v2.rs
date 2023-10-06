multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{Agreement, AgreementAmountType, AgreementType, FrequencyType};

#[multiversx_sc::module]
pub trait AgreementV2Module:
    crate::storage::StorageModule +
    crate::validation::ValidationModule +
    crate::charges::ChargesModule
{
    #[endpoint(createRecuringPaymentAgreementToSend)]
    fn create_recurring_payment_agreement_to_send(
        &self,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
        amount_type: AgreementAmountType<Self::Api>,
        frequency: FrequencyType,
        _whitelisted_addresses: Option<ManagedVec<ManagedAddress<Self::Api>>>
    ) {
        let caller = self.blockchain().get_caller();
        self.require_address_is_whitelisted(&caller);

        let agreement_type = AgreementType::RecurringPayoutToSend {
            amount_type,
            sender: caller.clone(),
            frequency,
            receivers: ManagedVec::new(),
        };

        self.create_recurring_agreement(&caller, agreement_type, token_identifier, _whitelisted_addresses);
    }

    #[endpoint(createRecuringPaymentAgreementToReceive)]
    fn create_recurring_payment_agreement_to_receive(
        &self,
        token_identifier: EgldOrEsdtTokenIdentifier<Self::Api>,
        amount_type: AgreementAmountType<Self::Api>,
        frequency: FrequencyType,
        _whitelisted_addresses: Option<ManagedVec<ManagedAddress<Self::Api>>>
    ) {
        let caller = self.blockchain().get_caller();
        self.require_address_is_whitelisted(&caller);

        let agreement_type = AgreementType::RecurringPayoutToReceive {
            amount_type: amount_type,
            receiver: caller.clone(),
            frequency: frequency,
        };

        let agreement = self.create_recurring_agreement(
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
     *  Subscribe to an agreement, only allowed to subscribe as sender to RecurringPayoutToReceive and TimeBoundPayoutToReceive
     */
    #[endpoint(signAgreement)]
    fn sign_agreement(&self, agreement_id: u64) {
        self.require_existing_agreement_id(agreement_id);

        let caller = self.blockchain().get_caller();

        self.require_agreement_not_created_by_account(&caller, agreement_id);
        self.require_agreement_not_signed_by_account(agreement_id, &caller);

        let agreement = self.agreement_by_id(agreement_id).get();
        let timestamp = self.blockchain().get_block_timestamp();
        match agreement.agreement_type {
            AgreementType::RecurringPayoutToReceive {..} => {},

            AgreementType::TimeBoundPayoutToReceive {..} => {},

            _ => panic!("You cannot sign this agreement.")
        }

        self.agreement_all_senders(agreement_id).insert(caller.clone());
        self.agreement_current_senders(agreement_id).insert(caller.clone());
        self.agreement_sender_sign_time(agreement_id, &caller).set(timestamp);
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

        self.agreement_current_senders(agreement_id).swap_remove(&caller);
        self.agreement_sender_cancel_time(agreement_id, &caller).set(timestamp);
        self.account_signed_agreements_list(&caller).swap_remove(&agreement_id);
    }

    #[endpoint(claimAgreement)]
    fn claim_agreement(&self, agreement_id: u64) {
        self.require_existing_agreement_id(agreement_id);

        let caller = self.blockchain().get_caller();
        self.require_agreement_signed_by_account(agreement_id, &caller);

        let mut _agreement = self.agreement_by_id(agreement_id).get();

        match _agreement.agreement_type {
            // Claim tokens for an agreement that this account is a receiver of **/
            AgreementType::RecurringPayoutToSend {..} => {
                self.require_agreement_signed_by_account(agreement_id, &caller);
            },

            // Claim tokens for an agreement that this account is a receiver of **/
            AgreementType::TimeBoundPayoutToSend {..} => {
                self.require_agreement_signed_by_account(agreement_id, &caller);
            },

            _ => panic!("You cannot claim tokens for this agreement")
        }

        self.agreement_by_id(agreement_id).set(_agreement);
        self.account_signed_agreements_list(&caller).insert(agreement_id);
    }

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
    fn create_recurring_agreement(
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
            claimed_amount: BigUint::zero()
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
