#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod modules;
pub mod types;

#[multiversx_sc::contract]
pub trait LunarPay:
    // Protocol Module
    modules::protocol::events::EventsModule +
    modules::protocol::storage::StorageModule +
    modules::protocol::endpoints::EndpointsModule +
    modules::protocol::validation::ValidationModule +

    // Accounts Module
    modules::accounts::views::ViewsModule +
    modules::accounts::events::EventsModule +
    modules::accounts::storage::StorageModule +
    modules::accounts::validation::ValidationModule +
    modules::accounts::endpoints::EndpointsModule +

    // Agreements Module
    modules::agreements::storage::StorageModule +

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
    modules::subscriptions::cycles::CyclesModule +
    modules::subscriptions::storage::StorageModule +
    modules::subscriptions::validation::ValidationModule +
    modules::subscriptions::owner_endpoints::OwnerEndpoints +
    modules::subscriptions::member_endpoints::MemberEndpoints +
    modules::subscriptions::public_endpoints::PublicEndpoints +
{
    #[init]
    fn init(&self) {}

    #[endpoint(upgrade)]
    fn upgrade(&self) {}
}
