multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait StorageModule {
    /** Stores the last ID assigned to an agreement **/
    #[view(getLastAgreementId)]
    #[storage_mapper("last_agreement_id")]
    fn last_agreement_id(&self) -> SingleValueMapper<u64>;
}
