multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Eq, TypeAbi, Clone)]
pub enum AgreementType<M: ManagedTypeApi> {
    // For payments that follow a fixed interval and can be claimed retroactively for multiple periods
    RecurringPayment {
        amount: BigUint<M>,
        frequency: u64, // Number of seconds between claims
        start_time: u64, // When the agreement starts
        total_subscribers: Option<u64>,
        maximum_subscribers: Option<u64>,
        end_time: Option<u64>, // When the agreement ends, if ever
        last_claim_time: Option<u64>, // Last time the funds were claimed
        last_claim_period: Option<u64>, // Last period for which the funds were claimed
    },
    // Can be accessed periodically but can't be claimed retroactively
    // E.g. Create a $100 allowance for employees/contractors
    TimedEntitlement {
        amount: BigUint<M>,
        frequency: u64, // Number of seconds between claims
        start_time: u64, // When the agreement starts
        end_time: Option<u64>, // When the agreement ends, if ever
        last_claim_time: Option<u64>, // Last time the funds were claimed
    },
    // Any amount can be withdrawn between the start and end times
    FlexibleAmount {
        start_time: u64,
        maximum_amount: Option<BigUint<M>>,
        end_time: Option<u64>, // When the agreement ends, if ever
    },
}

#[derive(TopEncode, TopDecode, TypeAbi)]
pub struct Agreement<M: ManagedTypeApi> {
    pub uac: ManagedBuffer<M>, // Unique agreement code
    pub sender: ManagedAddress<M>,
    pub recipient: ManagedAddress<M>,

    pub token: EgldOrEsdtTokenIdentifier<M>,
    pub agreement_type: AgreementType<M>,
    pub claimed_amount: BigUint<M>, // Total claimed so far

    pub cancel_time: Option<u64>, // Time when the agreement was cancelled, if ever
}

#[multiversx_sc::module]
pub trait StorageModule {
    /** Protocol Storage */

    /*
     * Stores the current whitelisted token identifiers
     */
    #[view(getWhitelistedTokenIds)]
    #[storage_mapper("whitelisted_token_ids")]
    fn whitelisted_token_ids(&self) -> UnorderedSetMapper<EgldOrEsdtTokenIdentifier<Self::Api>>;

    /*
     * Stores the used token identifiers
     * A token might get removed from the whitelist but the account might still have balance.
     */
    #[view(getUsedTokenIds)]
    #[storage_mapper("used_token_ids")]
    fn used_token_ids(&self) -> UnorderedSetMapper<EgldOrEsdtTokenIdentifier<Self::Api>>;

    #[view(getWhitelistedAddresses)]
    #[storage_mapper("whitelisted_addresses")]
    fn whitelisted_addresses(&self) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;

    /** Account Storage */

    /*
    * Stores the accounts
    */
    #[storage_mapper("accounts")]
    fn accounts(&self) -> UnorderedSetMapper<ManagedAddress<Self::Api>>;

    /*
     * Stores the total account balance for each token identifier
     */
    #[storage_mapper("account_balance")]
    fn account_balance(&self, address: &ManagedAddress, token: &EgldOrEsdtTokenIdentifier) -> SingleValueMapper<BigUint<Self::Api>>;

    /** Agreement Storage */

    /** Stores the last ID assigned to an agreement **/
    #[view(getLastAgreementId)]
    #[storage_mapper("last_agreement_id")]
    fn last_agreement_id(&self) -> SingleValueMapper<u64>;

    /** Stores the last ID assigned to an UAC **/
    #[view(getLastUniversalAgreementCode)]
    #[storage_mapper("last_uac")]
    fn last_uac(&self) -> SingleValueMapper<u64>;

    /** Stores the agreement by ID **/
    #[storage_mapper("agreement_by_id")]
    fn agreement_by_id(&self, agreement_id: u64) -> SingleValueMapper<Agreement<Self::Api>>;

    /** Stores all the UACs that belong to an account **/
    #[storage_mapper("account_uac_list")]
    fn account_uac_list(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;

    /** Stores all the agreement IDs that belong to an UAC **/
    #[storage_mapper("uac_agreement_ids_list")]
    fn uac_agreement_ids_list(&self, uac: &ManagedBuffer) -> UnorderedSetMapper<u64>;

    /** Stores all the agreement IDs that belong to an account **/
    #[view(getAgreementsListByAddress)]
    #[storage_mapper("account_agreements_list")]
    fn account_agreements_list(&self, address: &ManagedAddress) -> UnorderedSetMapper<u64>;
}
