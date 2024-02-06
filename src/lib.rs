#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod modules;

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

    protocol::ProtocolModule +

    // Accounts Module
    modules::accounts::views::ViewsModule +
    modules::accounts::events::EventsModule +
    modules::accounts::storage::StorageModule +
    modules::accounts::validation::ValidationModule +
    modules::accounts::endpoints::EndpointsModule +

    // Transfers Module
    modules::transfers::events::EventsModule +
    modules::transfers::user_endpoints::UserEndpointsModule +
    modules::transfers::balance_transfer::BalanceTransferModule +

    // Payments Module
    modules::payments::events::EventsModule +
    modules::payments::user_endpoints::UserEndpointsModule +

    // Subscriptions Module
    modules::subscriptions::events::EventsModule +
    modules::subscriptions::amount::AmountModule +
    modules::subscriptions::storage::StorageModule +
    modules::subscriptions::validation::ValidationModule +
    modules::subscriptions::owner_endpoints::OwnerEndpoints +
    modules::subscriptions::member_endpoints::MemberEndpoints +

    // Agreements Module
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
