#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod modules;

pub mod account;
pub mod protocol;
pub mod storage;
pub mod validation;
pub mod events;

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

    // Transfers Module
    modules::transfers::events::EventsModule +
    modules::transfers::user_endpoints::UserEndpointsModule +
    modules::transfers::balance_transfer::BalanceTransferModule +

    // Payments Module
    modules::payments::events::EventsModule +
    modules::payments::user_endpoints::UserEndpointsModule +

    agreement::AgreementsModule +
    agreement_signing::SignAgreementModule +
    agreement_triggers::AgreementTriggersModule +
    agreement_cycles::AgreementCyclesModule +
    agreement_amount::AgreementAmountModule
{
    #[init]
    fn init(&self) {}

    #[endpoint(upgrade)]
    fn upgrade(&self) {}
}
