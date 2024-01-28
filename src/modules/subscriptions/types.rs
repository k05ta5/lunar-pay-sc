multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionAmountType {
    FixedAmount,
    MemberDefinedAmount,
    OwnerDefinedAmountPerMember
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionType {
    RecurringPayoutToSend,
    RecurringPayoutToReceive,

    /* Can be triggered only for the current cycle */
    TermRestrictedPayoutToSend,
    TermRestrictedPayoutToReceive
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi,  Clone, PartialEq, Eq)]
pub struct Subscription<M: ManagedTypeApi> {
    pub id: u64,
    pub owner: ManagedAddress<M>,
    pub time_created: u64,

    pub frequency: u64,
    pub subscription_type: SubscriptionType,
    pub amount_type: SubscriptionAmountType,

    pub token_nonce: u64,
    pub token_identifier: EgldOrEsdtTokenIdentifier<M>,
}

pub type ChargeOperationValue<M: ManagedTypeApi> = Option<(BigUint<M>, u64)>;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone, PartialEq, Eq)]
pub struct SubscriptionChargeData<M: ManagedTypeApi> {
    pub successful: Option<(BigUint<M>, u64)>,
    pub failed: Option<(BigUint<M>, u64)>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone, PartialEq, Eq)]
pub struct SubscriptionMultiChargeResult<M: ManagedTypeApi> {
    pub acccount: ManagedAddress<M>,
    pub data: SubscriptionChargeData<M>,
}

// impl<M: ManagedTypeApi>  SubscriptionChargeResult<M> {
//     pub fn new(account: &ManagedAddress<M>) -> Self {
//         SubscriptionChargeResult {
//             acccount: account.clone(),
//             successful: None,
//             failed: None,
//         }
//     }
//
//     pub fn put_result(&mut self, success_value: ChargeOperationValue<M>, fail_value: ChargeOperationValue<M>) {
//         self.successful = success_value;
//         self.failed = fail_value;
//     }
// }