#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod account;
pub mod protocol;
pub mod storage;
pub mod transfers;
pub mod agreement;
pub mod validation;
pub mod events;

#[multiversx_sc::contract]
pub trait LunarPay:
    events::EventsModule +
    storage::StorageModule +
    account::AccountModule +
    protocol::ProtocolModule +
    transfers::TransfersModule +
    agreement::AgreementModule +
    validation::ValidationModule
{
    #[init]
    fn init(&self) {}
}
