// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           27
// Async Callback (empty):               1
// Total number of exported functions:  29

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    lunarpay
    (
        init => init
        upgrade => upgrade
        isUserAdmin => is_user_admin
        getWhitelistedTokenIds => whitelisted_token_ids
        getUsedTokenIds => used_token_ids
        getWhitelistedAddresses => whitelisted_addresses
        whitelistToken => whitelist_token
        removeWhitelistedToken => remove_whitelisted_token
        whitelistAddress => whitelist_address
        removeWhitelistedAddress => reomve_whitelisted_address
        setAdmin => set_admin
        removeAdmin => remove_admin
        getAccountBalances => get_account_balances
        depositEgld => deposit_egld
        withdrawEgld => withdraw_egld
        depositEsdt => deposit_esdt
        withdrawEsdt => withdraw_esdt
        getLastAgreementId => last_agreement_id
        transferTokens => transfer
        pay => pay
        getSubscriptionIds => subscription_ids
        getAccountSubscriptionsCreatedList => account_subscriptions_created_list
        getAccountSubscriptionsMembershipList => account_subscriptions_membership_list
        createSubscription => create_subscription
        addSubscriptionMember => add_subscription_member
        cancelSubscriptionMembership => cancel_subscription_membership
        createSubscriptionMembership => create_subscription_membership
        triggerSubscription => trigger_subscription
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
