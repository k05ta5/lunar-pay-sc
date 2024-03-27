multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, PartialEq, Eq,Clone)]
pub struct Amount<M: ManagedTypeApi> {
    pub fixed_amount: Option<BigUint<M>>,
    pub minimum_amount: Option<BigUint<M>>,
    pub maximum_amount: Option<BigUint<M>>,
}