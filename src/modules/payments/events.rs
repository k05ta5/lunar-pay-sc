multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("payment")]
    fn payment_event(
        &self,
        #[indexed] sender: &ManagedAddress,
        #[indexed] receiver: &ManagedAddress,
        #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
        #[indexed] token_nonce: u64,
        #[indexed] amount: &BigUint,
        metadata: Option<ManagedBuffer>,
    );
}