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
pub enum AgreementAmountType<M: ManagedTypeApi> {
    AnyAmount,
    FixedAmount(BigUint<M>),
    BoundedAmount(BoundedAmount<M>),
    SubscriberDefinedAmount,
    CreatorDefinedAmountPerSubscriber,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum AgreementType<M: ManagedTypeApi> {
    RecurringPayoutToSend {
        sender: ManagedAddress<M>,
        receivers: ManagedVec<M, ManagedAddress<M>>,
        amount_type: AgreementAmountType<M>,
        frequency: FrequencyType,
    },
    RecurringPayoutToReceive {
        receiver: ManagedAddress<M>,
        senders: ManagedVec<M, ManagedAddress<M>>,
        amount_type: AgreementAmountType<M>,

        frequency: FrequencyType,

        whitelist_enabled: Option<bool>,
        whitelisted_addresses: Option<ManagedVec<M, ManagedAddress<M>>>
    },
    TimeBoundPayoutToSend {
        sender: ManagedAddress<M>,
        receivers: ManagedVec<M, ManagedAddress<M>>,
        amount_type: AgreementAmountType<M>,

        frequency: FrequencyType,
    },
    TimeBoundPayoutToReceive {
        receiver: ManagedAddress<M>,
        senders: ManagedVec<M, ManagedAddress<M>>,

        frequency: FrequencyType,
    }
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Agreement<M: ManagedTypeApi> {
    pub creator: ManagedAddress<M>,

    pub token_identifier: EgldOrEsdtTokenIdentifier<M>,
    pub token_nonce: u64,

    pub agreement_type: AgreementType<M>,
    pub claimed_amount: BigUint<M>, // Total claimed so far
}