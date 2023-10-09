multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{Agreement, AgreementType, Amount, AgreementAmountType};

#[multiversx_sc::module]
pub trait AgreementTriggersModule:
    crate::storage::StorageModule +
    crate::transfers::TransfersModule +
    crate::validation::ValidationModule +
    crate::agreement_cycles::AgreementCyclesModule +
    crate::agreement_amount::AgreementAmountModule
{
    /**  **/
    #[endpoint(triggerRecurringAgreement)]
    fn trigger_recurring_agreement(&self, agreement_id: u64) {
        self.require_existing_agreement_id(agreement_id);

        let caller = self.blockchain().get_caller();
        let agreement = self.agreement_by_id(agreement_id).get();

        match agreement.agreement_type {
            AgreementType::RecurringPayoutToSend |
            AgreementType::RecurringPayoutToReceive => {
                let accounts_list: UnorderedSetMapper<ManagedAddress<Self::Api>> = self.agreement_accounts(agreement.id);

                require!(!accounts_list.is_empty(), "Nothing to send");

                for account in accounts_list.iter() {
                    self.trigger_agreement_for_account(agreement, account);
                }
            },

            _ => panic!("You cannot trigger this agreement")
        }
    }

    #[inline]
    fn trigger_agreement_for_account(&self, agreement: Agreement<Self::Api>, account: ManagedAddress<Self::Api>) {
        let amount_agreed: Amount<Self::Api> = self.get_amount_agreed_by_parties(agreement, account);

        let mut sender: ManagedAddress<Self::Api>;
        let mut receiver: ManagedAddress<Self::Api>;

        match agreement.agreement_type {
            AgreementType::RecurringPayoutToSend => {
                sender = agreement.creator;
                receiver = account.clone();

                //TODO: Implement this

                let cycles_to_charge = self.get_account_number_of_cycles_to_trigger(agreement.id, agreement.frequency, &account);
                let amount_per_cycle = amount_agreed.fixed_amount.unwrap();

                let total_amount = amount_per_cycle * cycles_to_charge;

                if(self.account_has_sufficient_balance(&sender, &agreement.token_identifier, &total_amount)) {
                    self.do_transfer_and_update_balance(&sender, &receiver, &agreement.token_identifier, &total_amount);
                }
            },
            AgreementType::RecurringPayoutToReceive => {
                sender = agreement.creator;
                receiver = account.clone();

                //TODO: Implement this

                let cycles_to_charge = self.get_account_number_of_cycles_to_trigger(agreement.id, agreement.frequency, &account);
                let amount_per_cycle = amount_agreed.fixed_amount.unwrap();

                let total_amount = amount_per_cycle * cycles_to_charge;

                if self.account_has_sufficient_balance(&sender, &agreement.token_identifier, &total_amount) {
                    self.do_transfer_and_update_balance(&sender, &receiver, &agreement.token_identifier, &total_amount);
                }
            }
            AgreementType::TermRestrictedPayoutToSend => {
                sender = agreement.creator;
                receiver = account.clone();

                //TODO: Implement this
                match agreement.amount_type {
                    AgreementAmountType::FixedAmount => {
                        let total_amount = amount_agreed.fixed_amount.unwrap();

                        if self.account_has_sufficient_balance(&sender, &agreement.token_identifier, &total_amount) {
                            self.do_transfer_and_update_balance(&sender, &receiver, &agreement.token_identifier, &total_amount);
                        }
                    },
                    AgreementAmountType::BoundedAmount => {},
                    AgreementAmountType::CreatorDefinedFixedAmountPerReceiver => {},
                    AgreementAmountType::CreatorDefinedBoundedAmountPerReceiver => {},
                    _ => panic!("Invalid amount type")
                }
            },
            AgreementType::TermRestrictedPayoutToReceive => {
                sender = account.clone();
                receiver = agreement.creator;

                //TODO: Implement this
                match agreement.amount_type {
                    AgreementAmountType::FixedAmount => {},
                    AgreementAmountType::BoundedAmount => {},
                    AgreementAmountType::SenderDefinedFixedAmount => {},
                    AgreementAmountType::SenderDefinedBoundedAmount => {},
                    _ => panic!("Invalid amount type")
                }
            },
            _ => panic!("You cannot trigger this agreement")
        }
    }
}
