#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod account;
pub mod protocol;
pub mod storage;
pub mod transfers;
pub mod validation;
pub mod events;

pub mod payments;

pub mod agreement;
pub mod agreement_signing;
pub mod agreement_cycles;
pub mod agreement_amount;
pub mod agreement_triggers;

pub mod types;

#[multiversx_sc::contract]
pub trait LunarPay:
    events::EventsModule +
    storage::StorageModule +
    validation::ValidationModule +

    account::AccountModule +
    protocol::ProtocolModule +
    transfers::TransfersModule +

    payments::PaymentsModule +

    agreement::AgreementsModule +
    agreement_signing::SignAgreementModule +
    agreement_triggers::AgreementTriggersModule +
    agreement_cycles::AgreementCyclesModule +
    agreement_amount::AgreementAmountModule
{
    #[init]
    fn init(&self) {}
}
