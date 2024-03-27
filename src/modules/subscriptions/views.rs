multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait ViewsModule:
crate::modules::subscriptions::amount::AmountModule +
crate::modules::subscriptions::storage::StorageModule +
{
    /**
     * It returns the subscription token outflow for a given account
     */
    #[view(getUserSubscriptionsOutflow)]
    fn get_user_subscriptions_outflow(&self, address: &ManagedAddress){

    }


    //
    // /**
    //  * It returns the subscription token inflow for a given account
    //  */
    // #[view(getUserSubscriptionsInflow)]
    // fn get_user_subscriptions_inflow(&self, address: &ManagedAddress) -> MultiValueEncoded<(EgldOrEsdtTokenIdentifier, BigUint)> {
    //     let memberships = self.account_subscriptions_membership_list(&address);
    //
    //     let mut tokens_vec = MultiValueEncoded::new();
    //
    //     // memberships = [{EGLD, 1}, {USDC, 15}, {EGLD, 3}]
    //
    //     for membership_id in memberships.iter() {
    //         let subscription = self.subscription_by_id(membership_id).get();
    //
    //         let token: EgldOrEsdtTokenIdentifier<Self::Api> = subscription.token_identifier;
    //         let amount: BigUint = self.get_subscription_amount_agreed_by_parties(subscription.id, &address);
    //
    //         // Here I need to update existing token value if token exists instead of inserting a new one
    //         tokens_vec.push((token, amount))
    //     }
    //
    //     // Want to return [{EGLD, 4}, {USDC, 15}]
    //     tokens_vec
    // }
}
