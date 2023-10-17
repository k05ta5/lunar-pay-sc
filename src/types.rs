multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Eq,Clone)]
pub struct Amount<M: ManagedTypeApi> {
    pub fixed_amount: Option<BigUint<M>>,
    pub minimum_amount: Option<BigUint<M>>,
    pub maximum_amount: Option<BigUint<M>>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone, Copy, PartialEq, Eq)]
pub enum AgreementAmountType {
    AnyAmount,
    FixedAmount,
    BoundedAmount,
    SenderDefinedFixedAmount,
    SenderDefinedBoundedAmount,
    CreatorDefinedFixedAmountPerReceiver,
    CreatorDefinedBoundedAmountPerReceiver,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone, Copy, PartialEq, Eq)]
pub enum AgreementType {
    RecurringPayoutToSend,
    RecurringPayoutToReceive,
    // Can be triggered only for the current cycle
    TermRestrictedPayoutToSend,
    TermRestrictedPayoutToReceive
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi,  Clone, PartialEq, Eq)]
pub struct Agreement<M: ManagedTypeApi> {
    pub id: u64,
    pub owner: ManagedAddress<M>,
    pub time_created: u64,

    pub frequency: u64,
    pub agreement_type: AgreementType,
    pub amount_type: AgreementAmountType,

    pub token_nonce: u64,
    pub token_identifier: EgldOrEsdtTokenIdentifier<M>,
}