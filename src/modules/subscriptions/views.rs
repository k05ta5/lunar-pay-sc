multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use core::ops::Deref;

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

    /**
     * It returns the subscription token inflow for a given account
     */
    #[view(getUserSubscriptionsInflow)]
    fn get_user_subscriptions_inflow(&self, address: &ManagedAddress) -> MultiValueEncoded<(EgldOrEsdtTokenIdentifier, BigUint)> {
        let memberships = self.account_subscriptions_membership_list(&address);
        
        let mut tokens: ManagedVec<EgldOrEsdtTokenIdentifier> = ManagedVec::new();
        let mut amounts: ManagedVec<BigUint> = ManagedVec::new();

        // memberships = [{EGLD, 1}, {USDC, 15}, {EGLD, 3}]    
        for membership_id in memberships.iter() {
            let subscription = self.subscription_by_id(membership_id).get();
            let token: EgldOrEsdtTokenIdentifier<Self::Api> = subscription.token_identifier;
            let amount: BigUint = self.get_subscription_amount_agreed_by_parties(subscription.id, &address);

            let token_index_option = tokens.find(&token);

            if token_index_option.is_some() {
                // token already exists
                let token_index = token_index_option.unwrap();
                let _result = amounts.set(token_index, &(amounts.get(token_index).deref().clone() + amount));
            } else {
                // first time this token
                tokens.push(token);
                amounts.push(amount);
            }
        }

        let mut final_list: MultiValueEncoded<(EgldOrEsdtTokenIdentifier, BigUint)> = MultiValueEncoded::new();
        for (token, final_amount) in tokens.iter().zip(amounts.iter()) {
            final_list.push((
                token,
                final_amount.deref().clone()
            ));
        }

        final_list
    }
}
