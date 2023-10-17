multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::types::{AgreementType, AgreementAmountType, Amount};

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
            // TODO: We allow only RecurringPayoutToReceive for the xDay Hackathon
            AgreementType::RecurringPayoutToReceive => {
                match amount_type {
                    // TODO: We allow only FixedAmount for the xDay Hackathon
                    AgreementAmountType::FixedAmount => {
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
        let amount_struct = amount.expect("Amount should be provided for this agreement type");
        let fixed_amount = amount_struct.fixed_amount.expect("Fixed amount should be provided");

        require!(fixed_amount > 0, "Fixed amount value should be higher than 0");

        Amount {
            fixed_amount: Some(fixed_amount),
            minimum_amount: None,
            maximum_amount: None,
        }
    }

    fn construct_bounded_amount(&self, amount: Option<Amount<Self::Api>>) -> Amount<Self::Api> {
        let amount_struct = amount.expect("Amount should be provided for this agreement type");
        let minimum_amount = amount_struct.minimum_amount.expect("Minimum amount should be provided");
        let maximum_amount = amount_struct.maximum_amount.expect("Maximum amount should be provided");

        require!(maximum_amount > minimum_amount, "Maximum amount should be higher than minimum amount");

        Amount {
            fixed_amount: None,
            minimum_amount: Some(minimum_amount),
            maximum_amount: Some(maximum_amount),
        }
    }

    fn get_amount_agreed_by_parties(&self, agreement_id: u64, amount_type: AgreementAmountType, address: &ManagedAddress<Self::Api>) -> Amount<Self::Api> {
        match amount_type {
            AgreementAmountType::FixedAmount |
            AgreementAmountType::BoundedAmount => {
                self.agreement_amount(agreement_id).get()
            },

            AgreementAmountType::SenderDefinedFixedAmount |
            AgreementAmountType::CreatorDefinedFixedAmountPerReceiver |
            AgreementAmountType::SenderDefinedBoundedAmount |
            AgreementAmountType::CreatorDefinedBoundedAmountPerReceiver => {
                self.agreement_defined_amount_per_account(agreement_id, address).get()
            },

            _ => panic!("Invalid agreement amount type")
        }
    }

    fn get_charge_value(&self, agreement_id: u64, amount_type: AgreementAmountType, address: &ManagedAddress<Self::Api>) -> BigUint {
        // TODO: Check for bounds when new agreement types are implemented. Until then, we they only amount type we have is fixed_amount.
        self.get_amount_agreed_by_parties(agreement_id, amount_type, &address).fixed_amount.unwrap()
    }
}
