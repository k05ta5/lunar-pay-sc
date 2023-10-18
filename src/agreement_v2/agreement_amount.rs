multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{Agreement, AgreementAmountType, AgreementType, Amount};

#[multiversx_sc::module]
pub trait AgreementAmountModule:
    crate::storage::StorageModule +
    crate::validation::ValidationModule
{
    fn construct_agreement_amount(
        &self,
        agreement_type: AgreementType,
        amount_type: AgreementAmountType,
        amount: Option<Amount<Self::Api>>
    ) -> Amount<Self::Api> {
        match agreement_type {
            AgreementType::RecurringPayoutToReceive |
            AgreementType::TermRestrictedPayoutToReceive => {
                match amount_type {
                    AgreementAmountType::FixedAmount |
                    AgreementAmountType::BoundedAmount |
                    AgreementAmountType::SenderDefinedFixedAmount |
                    AgreementAmountType::SenderDefinedBoundedAmount => {
                        self.construct_defined_amount(amount_type, amount)
                    },
                    _ => panic!("Invalid amount type")
                }
            },
            AgreementType::RecurringPayoutToSend |
            AgreementType::TermRestrictedPayoutToSend => {
                match amount_type {
                    AgreementAmountType::FixedAmount |
                    AgreementAmountType::BoundedAmount |
                    AgreementAmountType::CreatorDefinedFixedAmountPerReceiver |
                    AgreementAmountType::CreatorDefinedBoundedAmountPerReceiver => {
                        self.construct_defined_amount(amount_type, amount)
                    },
                    _ => panic!("Invalid amount type")
                }
            },
            _ => panic!("Invalid agreement type")
        }
    }

    fn construct_defined_amount(&self, amount_type: AgreementAmountType, amount: Option<Amount<Self::Api>>) -> Amount<Self::Api> {
        match amount_type {
            AgreementAmountType::FixedAmount |
            AgreementAmountType::SenderDefinedFixedAmount |
            AgreementAmountType::CreatorDefinedFixedAmountPerReceiver => {
                self.construct_fixed_amount(amount)
            },

            AgreementAmountType::BoundedAmount |
            AgreementAmountType::SenderDefinedBoundedAmount |
            AgreementAmountType::CreatorDefinedBoundedAmountPerReceiver => {
                self.construct_bounded_amount(amount)
            },

            _ => panic!("Invalid agreement amount type")
        }
    }

    fn construct_fixed_amount(&self, amount: Option<Amount<Self::Api>>) -> Amount<Self::Api> {
        let amountStruct = amount.expect("Amount should be provided for this agreement type");

        let fixed_amount = amountStruct.fixed_amount.expect("Fixed amount should be provided");

        Amount {
            fixed_amount: amountStruct.fixed_amount,
            minimum_amount: None,
            maximum_amount: None,
        }
    }

    fn construct_bounded_amount(&self, amount: Option<Amount<Self::Api>>) -> Amount<Self::Api> {
        let amountStruct = amount.expect("Amount should be provided for this agreement type");

        let minimum_amount = amountStruct.minimum_amount.expect("Minimum amount should be provided");
        let maximum_amount = amountStruct.maximum_amount.expect("Maximum amount should be provided");

        Amount {
            fixed_amount: None,
            minimum_amount: amountStruct.minimum_amount,
            maximum_amount: amountStruct.maximum_amount,
        }
    }

    fn get_amount_agreed_by_parties(&self, agreement: Agreement<Self::Api>, address: ManagedAddress<Self::Api>) -> Amount<Self::Api> {
        match agreement.amount_type {
            AgreementAmountType::FixedAmount |
            AgreementAmountType::BoundedAmount => {
                agreement.amount.unwrap()
            },

            AgreementAmountType::SenderDefinedFixedAmount |
            AgreementAmountType::CreatorDefinedFixedAmountPerReceiver |
            AgreementAmountType::SenderDefinedBoundedAmount |
            AgreementAmountType::CreatorDefinedBoundedAmountPerReceiver => {
                self.agreement_defined_amount_per_account(agreement.id, &address).get()
            },

            _ => panic!("Invalid agreement amount type")
        }
    }

    fn is_amount_in_agreed_bounds(&self, amount_type: AgreementAmountType, agreed_amount: Amount<Self::Api>, amount: BigUint) -> bool {
        match amount_type {
            AgreementAmountType::FixedAmount |
            AgreementAmountType::SenderDefinedFixedAmount |
            AgreementAmountType::CreatorDefinedFixedAmountPerReceiver => {
                amount == agreed_amount.fixed_amount.unwrap()
            },

            AgreementAmountType::BoundedAmount |
            AgreementAmountType::SenderDefinedBoundedAmount |
            AgreementAmountType::CreatorDefinedBoundedAmountPerReceiver => {
                amount <= agreed_amount.maximum_amount.unwrap() && amount >= agreed_amount.minimum_amount.unwrap()
            },

            _ => panic!("Invalid agreement amount type")
        }
    }
}
