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

pub struct AgreementChargeResult<M: ManagedTypeApi> {
    pub accounts: ManagedVec<M, ManagedAddress<M>>,
    pub amounts: ManagedVec<M, BigUint<M>>,
    pub cycles: ManagedVec<M, u64>
}

impl<M: ManagedTypeApi>  AgreementChargeResult<M> {
    pub fn new() -> Self {
        AgreementChargeResult {
            accounts: ManagedVec::new(),
            amounts: ManagedVec::new(),
            cycles: ManagedVec::new()
        }
    }

    pub fn push(&mut self, account: ManagedAddress<M>, amount: BigUint<M>, cycle: u64) {
        self.accounts.push(account);
        self.amounts.push(amount);
        self.cycles.push(cycle);
    }
}