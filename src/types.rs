multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum FrequencyType {
    SECOND,
    MINUTE,
    HOUR,
    DAY,
    WEEK,
    MONTH,
    YEAR,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, TypeAbi, Clone)]
pub struct BoundedAmount<M: ManagedTypeApi> {
    minimum_amount: BigUint<M>,
    maximum_amount: BigUint<M>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, TypeAbi, Clone)]
pub enum AmountType<M: ManagedTypeApi> {
    AnyAmount,
    FixedAmount(BigUint<M>),
    BoundedAmount(BoundedAmount<M>),
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, TypeAbi, Clone)]
pub enum RecurringPayoutToSendAmountType<M: ManagedTypeApi> {
    FixedAmount(BigUint<M>),
    CreatorDefinedAmountPerSubscriber,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, TypeAbi, Clone)]
pub enum PayoutToReceiveAmountType<M: ManagedTypeApi> {
    FixedAmount(BigUint<M>),
    SubscriberDefinedAmount,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, TypeAbi, Clone)]
pub enum TimeBoundPayoutToSendAmountType<M: ManagedTypeApi> {
    FixedAmount(BigUint<M>),
    CreatorDefinedAmountPerSubscriber,
    BoundedAmount(BoundedAmount<M>) // will be used in claim with amount
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum AgreementType<M: ManagedTypeApi> {
    RecurringPayoutToSend {
        sender: ManagedAddress<M>,
        amount_type: RecurringPayoutToSendAmountType<M>,
        frequency: FrequencyType,
    },
    RecurringPayoutToReceive {
        receiver: ManagedAddress<M>,
        amount_type: PayoutToReceiveAmountType<M>,

        frequency: FrequencyType,
    },
    // Only for current period can be claimed
    TimeBoundPayoutToSend {
        sender: ManagedAddress<M>,
        amount_type: TimeBoundPayoutToSendAmountType<M>,

        frequency: FrequencyType,
    },
    // Only for current period can be charged
    TimeBoundPayoutToReceive {
        receiver: ManagedAddress<M>,
        amount_type: PayoutToReceiveAmountType<M>,

        frequency: FrequencyType,
    }
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Agreement<M: ManagedTypeApi> {
    pub id: u64,
    
    pub creator: ManagedAddress<M>,

    pub token_identifier: EgldOrEsdtTokenIdentifier<M>,
    pub token_nonce: u64,

    pub agreement_type: AgreementType<M>,
    pub claimed_amount: BigUint<M>, // Total claimed so far
}