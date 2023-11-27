pub mod my_types {
    use super::my_types;
    pub mod bitvec {
        use super::my_types;
        pub mod order {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct Lsb0;
        }
    }
    pub mod bounded_collections {
        use super::my_types;
        pub mod bounded_btree_map {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct BoundedBTreeMap<_0, _1>(pub ::std::collections::BTreeMap<_0, _1>);
        }
        pub mod bounded_vec {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
        }
        pub mod weak_bounded_vec {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
        }
    }
    pub mod finality_grandpa {
        use super::my_types;
        #[derive(Clone, Debug)]
        pub struct Equivocation<_0, _1, _2> {
            pub round_number: ::core::primitive::u64,
            pub identity: _0,
            pub first: (_1, _2),
            pub second: (_1, _2),
        }
        #[derive(Clone, Debug)]
        pub struct Precommit<_0, _1> {
            pub target_hash: _0,
            pub target_number: _1,
        }
        #[derive(Clone, Debug)]
        pub struct Prevote<_0, _1> {
            pub target_hash: _0,
            pub target_number: _1,
        }
    }
    pub mod frame_support {
        use super::my_types;
        pub mod dispatch {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub enum DispatchClass {
                Normal,
                Operational,
                Mandatory,
            }
            #[derive(Clone, Debug)]
            pub struct DispatchInfo {
                pub weight: my_types::sp_weights::weight_v2::Weight,
                pub class: my_types::frame_support::dispatch::DispatchClass,
                pub pays_fee: my_types::frame_support::dispatch::Pays,
            }
            #[derive(Clone, Debug)]
            pub enum Pays {
                Yes,
                No,
            }
            #[derive(Clone, Debug)]
            pub struct PerDispatchClass<_0> {
                pub normal: _0,
                pub operational: _0,
                pub mandatory: _0,
            }
            #[derive(Clone, Debug)]
            pub struct PostDispatchInfo {
                pub actual_weight: ::core::option::Option<
                    my_types::sp_weights::weight_v2::Weight,
                >,
                pub pays_fee: my_types::frame_support::dispatch::Pays,
            }
            #[derive(Clone, Debug)]
            pub enum RawOrigin<_0> {
                Root,
                Signed(_0),
                None,
            }
        }
        pub mod traits {
            use super::my_types;
            pub mod messages {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub enum ProcessMessageError {
                    BadFormat,
                    Corrupt,
                    Unsupported,
                    Overweight(my_types::sp_weights::weight_v2::Weight),
                    Yield,
                }
            }
            pub mod preimages {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub enum Bounded<_0> {
                    Legacy { hash: my_types::primitive_types::H256 },
                    Inline(
                        my_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    ),
                    Lookup {
                        hash: my_types::primitive_types::H256,
                        len: ::core::primitive::u32,
                    },
                    __Ignore(::core::marker::PhantomData<_0>),
                }
            }
            pub mod schedule {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub enum DispatchTime<_0> {
                    At(_0),
                    After(_0),
                }
            }
            pub mod tokens {
                use super::my_types;
                pub mod misc {
                    use super::my_types;
                    #[derive(Clone, Debug)]
                    pub enum BalanceStatus {
                        Free,
                        Reserved,
                    }
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct PalletId(pub [::core::primitive::u8; 8usize]);
    }
    pub mod frame_system {
        use super::my_types;
        pub mod extensions {
            use super::my_types;
            pub mod check_genesis {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct CheckGenesis;
            }
            pub mod check_mortality {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct CheckMortality(pub my_types::sp_runtime::generic::era::Era);
            }
            pub mod check_non_zero_sender {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct CheckNonZeroSender;
            }
            pub mod check_nonce {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct CheckNonce(pub ::core::primitive::u32);
            }
            pub mod check_spec_version {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct CheckSpecVersion;
            }
            pub mod check_tx_version {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct CheckTxVersion;
            }
            pub mod check_weight {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct CheckWeight;
            }
        }
        pub mod limits {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct BlockLength {
                pub max: my_types::frame_support::dispatch::PerDispatchClass<
                    ::core::primitive::u32,
                >,
            }
            #[derive(Clone, Debug)]
            pub struct BlockWeights {
                pub base_block: my_types::sp_weights::weight_v2::Weight,
                pub max_block: my_types::sp_weights::weight_v2::Weight,
                pub per_class: my_types::frame_support::dispatch::PerDispatchClass<
                    my_types::frame_system::limits::WeightsPerClass,
                >,
            }
            #[derive(Clone, Debug)]
            pub struct WeightsPerClass {
                pub base_extrinsic: my_types::sp_weights::weight_v2::Weight,
                pub max_extrinsic: ::core::option::Option<
                    my_types::sp_weights::weight_v2::Weight,
                >,
                pub max_total: ::core::option::Option<
                    my_types::sp_weights::weight_v2::Weight,
                >,
                pub reserved: ::core::option::Option<
                    my_types::sp_weights::weight_v2::Weight,
                >,
            }
        }
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::remark`].
                remark { remark: ::std::vec::Vec<::core::primitive::u8> },
                ///See [`Pallet::set_heap_pages`].
                set_heap_pages { pages: ::core::primitive::u64 },
                ///See [`Pallet::set_code`].
                set_code { code: ::std::vec::Vec<::core::primitive::u8> },
                ///See [`Pallet::set_code_without_checks`].
                set_code_without_checks { code: ::std::vec::Vec<::core::primitive::u8> },
                ///See [`Pallet::set_storage`].
                set_storage {
                    items: ::std::vec::Vec<
                        (
                            ::std::vec::Vec<::core::primitive::u8>,
                            ::std::vec::Vec<::core::primitive::u8>,
                        ),
                    >,
                },
                ///See [`Pallet::kill_storage`].
                kill_storage {
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                },
                ///See [`Pallet::kill_prefix`].
                kill_prefix {
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                },
                ///See [`Pallet::remark_with_event`].
                remark_with_event { remark: ::std::vec::Vec<::core::primitive::u8> },
            }
            #[derive(Clone, Debug)]
            ///Error for the System pallet
            pub enum Error {
                ///The name of specification does not match between the current runtime
                ///and the new runtime.
                InvalidSpecName,
                ///The specification version is not allowed to decrease between the current runtime
                ///and the new runtime.
                SpecVersionNeedsToIncrease,
                ///Failed to extract the runtime version from the new runtime.
                ///
                ///Either calling `Core_version` or decoding `RuntimeVersion` failed.
                FailedToExtractRuntimeVersion,
                ///Suicide called when the account has non-default composite data.
                NonDefaultComposite,
                ///There is a non-zero reference count preventing the account from being purged.
                NonZeroRefCount,
                ///The origin filter prevent the call to be dispatched.
                CallFiltered,
            }
            #[derive(Clone, Debug)]
            ///Event for the System pallet.
            pub enum Event {
                ///An extrinsic completed successfully.
                ExtrinsicSuccess {
                    dispatch_info: my_types::frame_support::dispatch::DispatchInfo,
                },
                ///An extrinsic failed.
                ExtrinsicFailed {
                    dispatch_error: my_types::sp_runtime::DispatchError,
                    dispatch_info: my_types::frame_support::dispatch::DispatchInfo,
                },
                ///`:code` was updated.
                CodeUpdated,
                ///A new account was created.
                NewAccount { account: my_types::sp_core::crypto::AccountId32 },
                ///An account was reaped.
                KilledAccount { account: my_types::sp_core::crypto::AccountId32 },
                ///On on-chain remark happened.
                Remarked {
                    sender: my_types::sp_core::crypto::AccountId32,
                    hash: my_types::primitive_types::H256,
                },
            }
        }
        #[derive(Clone, Debug)]
        pub struct AccountInfo<_0, _1> {
            pub nonce: _0,
            pub consumers: ::core::primitive::u32,
            pub providers: ::core::primitive::u32,
            pub sufficients: ::core::primitive::u32,
            pub data: _1,
        }
        #[derive(Clone, Debug)]
        pub struct EventRecord<_0, _1> {
            pub phase: my_types::frame_system::Phase,
            pub event: _0,
            pub topics: ::std::vec::Vec<_1>,
        }
        #[derive(Clone, Debug)]
        pub struct LastRuntimeUpgradeInfo {
            pub spec_version: ::core::primitive::u32,
            pub spec_name: ::std::string::String,
        }
        #[derive(Clone, Debug)]
        pub enum Phase {
            ApplyExtrinsic(::core::primitive::u32),
            Finalization,
            Initialization,
        }
    }
    pub mod pallet_babe {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::report_equivocation`].
                report_equivocation {
                    equivocation_proof: ::std::boxed::Box<
                        my_types::sp_consensus_slots::EquivocationProof<
                            my_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                my_types::sp_runtime::traits::BlakeTwo256,
                            >,
                            my_types::sp_consensus_babe::app::Public,
                        >,
                    >,
                    key_owner_proof: my_types::sp_session::MembershipProof,
                },
                ///See [`Pallet::report_equivocation_unsigned`].
                report_equivocation_unsigned {
                    equivocation_proof: ::std::boxed::Box<
                        my_types::sp_consensus_slots::EquivocationProof<
                            my_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                my_types::sp_runtime::traits::BlakeTwo256,
                            >,
                            my_types::sp_consensus_babe::app::Public,
                        >,
                    >,
                    key_owner_proof: my_types::sp_session::MembershipProof,
                },
                ///See [`Pallet::plan_config_change`].
                plan_config_change {
                    config: my_types::sp_consensus_babe::digests::NextConfigDescriptor,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///An equivocation proof provided as part of an equivocation report is invalid.
                InvalidEquivocationProof,
                ///A key ownership proof provided as part of an equivocation report is invalid.
                InvalidKeyOwnershipProof,
                ///A given equivocation report is valid but already previously reported.
                DuplicateOffenceReport,
                ///Submitted configuration is invalid.
                InvalidConfiguration,
            }
        }
    }
    pub mod pallet_bags_list {
        use super::my_types;
        pub mod list {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct Bag {
                pub head: ::core::option::Option<my_types::sp_core::crypto::AccountId32>,
                pub tail: ::core::option::Option<my_types::sp_core::crypto::AccountId32>,
            }
            #[derive(Clone, Debug)]
            pub enum ListError {
                Duplicate,
                NotHeavier,
                NotInSameBag,
                NodeNotFound,
            }
            #[derive(Clone, Debug)]
            pub struct Node {
                pub id: my_types::sp_core::crypto::AccountId32,
                pub prev: ::core::option::Option<my_types::sp_core::crypto::AccountId32>,
                pub next: ::core::option::Option<my_types::sp_core::crypto::AccountId32>,
                pub bag_upper: ::core::primitive::u64,
                pub score: ::core::primitive::u64,
            }
        }
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::rebag`].
                rebag {
                    dislocated: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::put_in_front_of`].
                put_in_front_of {
                    lighter: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///A error in the list interface implementation.
                List(my_types::pallet_bags_list::list::ListError),
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///Moved an account from one bag to another.
                Rebagged {
                    who: my_types::sp_core::crypto::AccountId32,
                    from: ::core::primitive::u64,
                    to: ::core::primitive::u64,
                },
                ///Updated the score of some account to the given amount.
                ScoreUpdated {
                    who: my_types::sp_core::crypto::AccountId32,
                    new_score: ::core::primitive::u64,
                },
            }
        }
    }
    pub mod pallet_balances {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::transfer_allow_death`].
                transfer_allow_death {
                    dest: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                },
                ///See [`Pallet::set_balance_deprecated`].
                set_balance_deprecated {
                    who: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: ::core::primitive::u128,
                    old_reserved: ::core::primitive::u128,
                },
                ///See [`Pallet::force_transfer`].
                force_transfer {
                    source: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    dest: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                },
                ///See [`Pallet::transfer_keep_alive`].
                transfer_keep_alive {
                    dest: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                },
                ///See [`Pallet::transfer_all`].
                transfer_all {
                    dest: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    keep_alive: ::core::primitive::bool,
                },
                ///See [`Pallet::force_unreserve`].
                force_unreserve {
                    who: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    amount: ::core::primitive::u128,
                },
                ///See [`Pallet::upgrade_accounts`].
                upgrade_accounts {
                    who: ::std::vec::Vec<my_types::sp_core::crypto::AccountId32>,
                },
                ///See [`Pallet::transfer`].
                transfer {
                    dest: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    value: ::core::primitive::u128,
                },
                ///See [`Pallet::force_set_balance`].
                force_set_balance {
                    who: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    new_free: ::core::primitive::u128,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Vesting balance too high to send value.
                VestingBalance,
                ///Account liquidity restrictions prevent withdrawal.
                LiquidityRestrictions,
                ///Balance too low to send value.
                InsufficientBalance,
                ///Value too low to create account due to existential deposit.
                ExistentialDeposit,
                ///Transfer/payment would kill account.
                Expendability,
                ///A vesting schedule already exists for this account.
                ExistingVestingSchedule,
                ///Beneficiary account must pre-exist.
                DeadAccount,
                ///Number of named reserves exceed `MaxReserves`.
                TooManyReserves,
                ///Number of holds exceed `MaxHolds`.
                TooManyHolds,
                ///Number of freezes exceed `MaxFreezes`.
                TooManyFreezes,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///An account was created with some free balance.
                Endowed {
                    account: my_types::sp_core::crypto::AccountId32,
                    free_balance: ::core::primitive::u128,
                },
                ///An account was removed whose balance was non-zero but below ExistentialDeposit,
                ///resulting in an outright loss.
                DustLost {
                    account: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///Transfer succeeded.
                Transfer {
                    from: my_types::sp_core::crypto::AccountId32,
                    to: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///A balance was set by root.
                BalanceSet {
                    who: my_types::sp_core::crypto::AccountId32,
                    free: ::core::primitive::u128,
                },
                ///Some balance was reserved (moved from free to reserved).
                Reserved {
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///Some balance was unreserved (moved from reserved to free).
                Unreserved {
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///Some balance was moved from the reserve of the first account to the second account.
                ///Final argument indicates the destination balance type.
                ReserveRepatriated {
                    from: my_types::sp_core::crypto::AccountId32,
                    to: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                    destination_status: my_types::frame_support::traits::tokens::misc::BalanceStatus,
                },
                ///Some amount was deposited (e.g. for transaction fees).
                Deposit {
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///Some amount was withdrawn from the account (e.g. for transaction fees).
                Withdraw {
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///Some amount was removed from the account (e.g. for misbehavior).
                Slashed {
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///Some amount was minted into an account.
                Minted {
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///Some amount was burned from an account.
                Burned {
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///Some amount was suspended from an account (it can be restored later).
                Suspended {
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///Some amount was restored into an account.
                Restored {
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///An account was upgraded.
                Upgraded { who: my_types::sp_core::crypto::AccountId32 },
                ///Total issuance was increased by `amount`, creating a credit to be balanced.
                Issued { amount: ::core::primitive::u128 },
                ///Total issuance was decreased by `amount`, creating a debt to be balanced.
                Rescinded { amount: ::core::primitive::u128 },
                ///Some balance was locked.
                Locked {
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///Some balance was unlocked.
                Unlocked {
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///Some balance was frozen.
                Frozen {
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///Some balance was thawed.
                Thawed {
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
            }
        }
        pub mod types {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub frozen: _0,
                pub flags: my_types::pallet_balances::types::ExtraFlags,
            }
            #[derive(Clone, Debug)]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: my_types::pallet_balances::types::Reasons,
            }
            #[derive(Clone, Debug, parity_scale_codec::CompactAs)]
            pub struct ExtraFlags(pub ::core::primitive::u128);
            #[derive(Clone, Debug)]
            pub struct IdAmount<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
            #[derive(Clone, Debug)]
            pub enum Reasons {
                Fee,
                Misc,
                All,
            }
            #[derive(Clone, Debug)]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
    }
    pub mod pallet_bounties {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::propose_bounty`].
                propose_bounty {
                    value: ::core::primitive::u128,
                    description: ::std::vec::Vec<::core::primitive::u8>,
                },
                ///See [`Pallet::approve_bounty`].
                approve_bounty { bounty_id: ::core::primitive::u32 },
                ///See [`Pallet::propose_curator`].
                propose_curator {
                    bounty_id: ::core::primitive::u32,
                    curator: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    fee: ::core::primitive::u128,
                },
                ///See [`Pallet::unassign_curator`].
                unassign_curator { bounty_id: ::core::primitive::u32 },
                ///See [`Pallet::accept_curator`].
                accept_curator { bounty_id: ::core::primitive::u32 },
                ///See [`Pallet::award_bounty`].
                award_bounty {
                    bounty_id: ::core::primitive::u32,
                    beneficiary: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::claim_bounty`].
                claim_bounty { bounty_id: ::core::primitive::u32 },
                ///See [`Pallet::close_bounty`].
                close_bounty { bounty_id: ::core::primitive::u32 },
                ///See [`Pallet::extend_bounty_expiry`].
                extend_bounty_expiry {
                    bounty_id: ::core::primitive::u32,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Proposer's balance is too low.
                InsufficientProposersBalance,
                ///No proposal or bounty at that index.
                InvalidIndex,
                ///The reason given is just too big.
                ReasonTooBig,
                ///The bounty status is unexpected.
                UnexpectedStatus,
                ///Require bounty curator.
                RequireCurator,
                ///Invalid bounty value.
                InvalidValue,
                ///Invalid bounty fee.
                InvalidFee,
                ///A bounty payout is pending.
                ///To cancel the bounty, you must unassign and slash the curator.
                PendingPayout,
                ///The bounties cannot be claimed/closed because it's still in the countdown period.
                Premature,
                ///The bounty cannot be closed because it has active child bounties.
                HasActiveChildBounty,
                ///Too many approvals are already queued.
                TooManyQueued,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///New bounty proposal.
                BountyProposed { index: ::core::primitive::u32 },
                ///A bounty proposal was rejected; funds were slashed.
                BountyRejected {
                    index: ::core::primitive::u32,
                    bond: ::core::primitive::u128,
                },
                ///A bounty proposal is funded and became active.
                BountyBecameActive { index: ::core::primitive::u32 },
                ///A bounty is awarded to a beneficiary.
                BountyAwarded {
                    index: ::core::primitive::u32,
                    beneficiary: my_types::sp_core::crypto::AccountId32,
                },
                ///A bounty is claimed by beneficiary.
                BountyClaimed {
                    index: ::core::primitive::u32,
                    payout: ::core::primitive::u128,
                    beneficiary: my_types::sp_core::crypto::AccountId32,
                },
                ///A bounty is cancelled.
                BountyCanceled { index: ::core::primitive::u32 },
                ///A bounty expiry is extended.
                BountyExtended { index: ::core::primitive::u32 },
            }
        }
        #[derive(Clone, Debug)]
        pub struct Bounty<_0, _1, _2> {
            pub proposer: _0,
            pub value: _1,
            pub fee: _1,
            pub curator_deposit: _1,
            pub bond: _1,
            pub status: my_types::pallet_bounties::BountyStatus<_0, _2>,
        }
        #[derive(Clone, Debug)]
        pub enum BountyStatus<_0, _1> {
            Proposed,
            Approved,
            Funded,
            CuratorProposed { curator: _0 },
            Active { curator: _0, update_due: _1 },
            PendingPayout { curator: _0, beneficiary: _0, unlock_at: _1 },
        }
    }
    pub mod pallet_child_bounties {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::add_child_bounty`].
                add_child_bounty {
                    parent_bounty_id: ::core::primitive::u32,
                    value: ::core::primitive::u128,
                    description: ::std::vec::Vec<::core::primitive::u8>,
                },
                ///See [`Pallet::propose_curator`].
                propose_curator {
                    parent_bounty_id: ::core::primitive::u32,
                    child_bounty_id: ::core::primitive::u32,
                    curator: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    fee: ::core::primitive::u128,
                },
                ///See [`Pallet::accept_curator`].
                accept_curator {
                    parent_bounty_id: ::core::primitive::u32,
                    child_bounty_id: ::core::primitive::u32,
                },
                ///See [`Pallet::unassign_curator`].
                unassign_curator {
                    parent_bounty_id: ::core::primitive::u32,
                    child_bounty_id: ::core::primitive::u32,
                },
                ///See [`Pallet::award_child_bounty`].
                award_child_bounty {
                    parent_bounty_id: ::core::primitive::u32,
                    child_bounty_id: ::core::primitive::u32,
                    beneficiary: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::claim_child_bounty`].
                claim_child_bounty {
                    parent_bounty_id: ::core::primitive::u32,
                    child_bounty_id: ::core::primitive::u32,
                },
                ///See [`Pallet::close_child_bounty`].
                close_child_bounty {
                    parent_bounty_id: ::core::primitive::u32,
                    child_bounty_id: ::core::primitive::u32,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///The parent bounty is not in active state.
                ParentBountyNotActive,
                ///The bounty balance is not enough to add new child-bounty.
                InsufficientBountyBalance,
                ///Number of child bounties exceeds limit `MaxActiveChildBountyCount`.
                TooManyChildBounties,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///A child-bounty is added.
                Added {
                    index: ::core::primitive::u32,
                    child_index: ::core::primitive::u32,
                },
                ///A child-bounty is awarded to a beneficiary.
                Awarded {
                    index: ::core::primitive::u32,
                    child_index: ::core::primitive::u32,
                    beneficiary: my_types::sp_core::crypto::AccountId32,
                },
                ///A child-bounty is claimed by beneficiary.
                Claimed {
                    index: ::core::primitive::u32,
                    child_index: ::core::primitive::u32,
                    payout: ::core::primitive::u128,
                    beneficiary: my_types::sp_core::crypto::AccountId32,
                },
                ///A child-bounty is cancelled.
                Canceled {
                    index: ::core::primitive::u32,
                    child_index: ::core::primitive::u32,
                },
            }
        }
        #[derive(Clone, Debug)]
        pub struct ChildBounty<_0, _1, _2> {
            pub parent_bounty: ::core::primitive::u32,
            pub value: _1,
            pub fee: _1,
            pub curator_deposit: _1,
            pub status: my_types::pallet_child_bounties::ChildBountyStatus<_0, _2>,
        }
        #[derive(Clone, Debug)]
        pub enum ChildBountyStatus<_0, _1> {
            Added,
            CuratorProposed { curator: _0 },
            Active { curator: _0 },
            PendingPayout { curator: _0, beneficiary: _0, unlock_at: _1 },
        }
    }
    pub mod pallet_collective {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::set_members`].
                set_members {
                    new_members: ::std::vec::Vec<my_types::sp_core::crypto::AccountId32>,
                    prime: ::core::option::Option<
                        my_types::sp_core::crypto::AccountId32,
                    >,
                    old_count: ::core::primitive::u32,
                },
                ///See [`Pallet::execute`].
                execute {
                    proposal: ::std::boxed::Box<my_types::polkadot_runtime::RuntimeCall>,
                    length_bound: ::core::primitive::u32,
                },
                ///See [`Pallet::propose`].
                propose {
                    threshold: ::core::primitive::u32,
                    proposal: ::std::boxed::Box<my_types::polkadot_runtime::RuntimeCall>,
                    length_bound: ::core::primitive::u32,
                },
                ///See [`Pallet::vote`].
                vote {
                    proposal: my_types::primitive_types::H256,
                    index: ::core::primitive::u32,
                    approve: ::core::primitive::bool,
                },
                ///See [`Pallet::disapprove_proposal`].
                disapprove_proposal { proposal_hash: my_types::primitive_types::H256 },
                ///See [`Pallet::close`].
                close {
                    proposal_hash: my_types::primitive_types::H256,
                    index: ::core::primitive::u32,
                    proposal_weight_bound: my_types::sp_weights::weight_v2::Weight,
                    length_bound: ::core::primitive::u32,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Account is not a member
                NotMember,
                ///Duplicate proposals not allowed
                DuplicateProposal,
                ///Proposal must exist
                ProposalMissing,
                ///Mismatched index
                WrongIndex,
                ///Duplicate vote ignored
                DuplicateVote,
                ///Members are already initialized!
                AlreadyInitialized,
                ///The close call was made too early, before the end of the voting.
                TooEarly,
                ///There can only be a maximum of `MaxProposals` active proposals.
                TooManyProposals,
                ///The given weight bound for the proposal was too low.
                WrongProposalWeight,
                ///The given length bound for the proposal was too low.
                WrongProposalLength,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///A motion (given hash) has been proposed (by given account) with a threshold (given
                ///`MemberCount`).
                Proposed {
                    account: my_types::sp_core::crypto::AccountId32,
                    proposal_index: ::core::primitive::u32,
                    proposal_hash: my_types::primitive_types::H256,
                    threshold: ::core::primitive::u32,
                },
                ///A motion (given hash) has been voted on by given account, leaving
                ///a tally (yes votes and no votes given respectively as `MemberCount`).
                Voted {
                    account: my_types::sp_core::crypto::AccountId32,
                    proposal_hash: my_types::primitive_types::H256,
                    voted: ::core::primitive::bool,
                    yes: ::core::primitive::u32,
                    no: ::core::primitive::u32,
                },
                ///A motion was approved by the required threshold.
                Approved { proposal_hash: my_types::primitive_types::H256 },
                ///A motion was not approved by the required threshold.
                Disapproved { proposal_hash: my_types::primitive_types::H256 },
                ///A motion was executed; result will be `Ok` if it returned without error.
                Executed {
                    proposal_hash: my_types::primitive_types::H256,
                    result: ::core::result::Result<
                        (),
                        my_types::sp_runtime::DispatchError,
                    >,
                },
                ///A single member did some action; result will be `Ok` if it returned without error.
                MemberExecuted {
                    proposal_hash: my_types::primitive_types::H256,
                    result: ::core::result::Result<
                        (),
                        my_types::sp_runtime::DispatchError,
                    >,
                },
                ///A proposal was closed because its threshold was reached or after its duration was up.
                Closed {
                    proposal_hash: my_types::primitive_types::H256,
                    yes: ::core::primitive::u32,
                    no: ::core::primitive::u32,
                },
            }
        }
        #[derive(Clone, Debug)]
        pub enum RawOrigin<_0> {
            Members(::core::primitive::u32, ::core::primitive::u32),
            Member(_0),
            _Phantom,
        }
        #[derive(Clone, Debug)]
        pub struct Votes<_0, _1> {
            pub index: ::core::primitive::u32,
            pub threshold: ::core::primitive::u32,
            pub ayes: ::std::vec::Vec<_0>,
            pub nays: ::std::vec::Vec<_0>,
            pub end: _1,
        }
    }
    pub mod pallet_conviction_voting {
        use super::my_types;
        pub mod conviction {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub enum Conviction {
                None,
                Locked1x,
                Locked2x,
                Locked3x,
                Locked4x,
                Locked5x,
                Locked6x,
            }
        }
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::vote`].
                vote {
                    poll_index: ::core::primitive::u32,
                    vote: my_types::pallet_conviction_voting::vote::AccountVote<
                        ::core::primitive::u128,
                    >,
                },
                ///See [`Pallet::delegate`].
                delegate {
                    class: ::core::primitive::u16,
                    to: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    conviction: my_types::pallet_conviction_voting::conviction::Conviction,
                    balance: ::core::primitive::u128,
                },
                ///See [`Pallet::undelegate`].
                undelegate { class: ::core::primitive::u16 },
                ///See [`Pallet::unlock`].
                unlock {
                    class: ::core::primitive::u16,
                    target: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::remove_vote`].
                remove_vote {
                    class: ::core::option::Option<::core::primitive::u16>,
                    index: ::core::primitive::u32,
                },
                ///See [`Pallet::remove_other_vote`].
                remove_other_vote {
                    target: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    class: ::core::primitive::u16,
                    index: ::core::primitive::u32,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Poll is not ongoing.
                NotOngoing,
                ///The given account did not vote on the poll.
                NotVoter,
                ///The actor has no permission to conduct the action.
                NoPermission,
                ///The actor has no permission to conduct the action right now but will do in the future.
                NoPermissionYet,
                ///The account is already delegating.
                AlreadyDelegating,
                ///The account currently has votes attached to it and the operation cannot succeed until
                ///these are removed, either through `unvote` or `reap_vote`.
                AlreadyVoting,
                ///Too high a balance was provided that the account cannot afford.
                InsufficientFunds,
                ///The account is not currently delegating.
                NotDelegating,
                ///Delegation to oneself makes no sense.
                Nonsense,
                ///Maximum number of votes reached.
                MaxVotesReached,
                ///The class must be supplied since it is not easily determinable from the state.
                ClassNeeded,
                ///The class ID supplied is invalid.
                BadClass,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///An account has delegated their vote to another account. \[who, target\]
                Delegated(
                    my_types::sp_core::crypto::AccountId32,
                    my_types::sp_core::crypto::AccountId32,
                ),
                ///An \[account\] has cancelled a previous delegation operation.
                Undelegated(my_types::sp_core::crypto::AccountId32),
            }
        }
        pub mod types {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct Delegations<_0> {
                pub votes: _0,
                pub capital: _0,
            }
            #[derive(Clone, Debug)]
            pub struct Tally<_0> {
                pub ayes: _0,
                pub nays: _0,
                pub support: _0,
            }
        }
        pub mod vote {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub enum AccountVote<_0> {
                Standard {
                    vote: my_types::pallet_conviction_voting::vote::Vote,
                    balance: _0,
                },
                Split { aye: _0, nay: _0 },
                SplitAbstain { aye: _0, nay: _0, abstain: _0 },
            }
            #[derive(Clone, Debug)]
            pub struct Casting<_0, _1, _2> {
                pub votes: my_types::bounded_collections::bounded_vec::BoundedVec<
                    (_1, my_types::pallet_conviction_voting::vote::AccountVote<_0>),
                >,
                pub delegations: my_types::pallet_conviction_voting::types::Delegations<
                    _0,
                >,
                pub prior: my_types::pallet_conviction_voting::vote::PriorLock<_1, _0>,
                pub __ignore: ::core::marker::PhantomData<_2>,
            }
            #[derive(Clone, Debug)]
            pub struct Delegating<_0, _1, _2> {
                pub balance: _0,
                pub target: _1,
                pub conviction: my_types::pallet_conviction_voting::conviction::Conviction,
                pub delegations: my_types::pallet_conviction_voting::types::Delegations<
                    _0,
                >,
                pub prior: my_types::pallet_conviction_voting::vote::PriorLock<_2, _0>,
            }
            #[derive(Clone, Debug)]
            pub struct PriorLock<_0, _1>(pub _0, pub _1);
            #[derive(Clone, Debug, parity_scale_codec::CompactAs)]
            pub struct Vote(pub ::core::primitive::u8);
            #[derive(Clone, Debug)]
            pub enum Voting<_0, _1, _2, _3> {
                Casting(my_types::pallet_conviction_voting::vote::Casting<_0, _2, _2>),
                Delegating(
                    my_types::pallet_conviction_voting::vote::Delegating<_0, _1, _2>,
                ),
                __Ignore(::core::marker::PhantomData<_3>),
            }
        }
    }
    pub mod pallet_democracy {
        use super::my_types;
        pub mod conviction {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub enum Conviction {
                None,
                Locked1x,
                Locked2x,
                Locked3x,
                Locked4x,
                Locked5x,
                Locked6x,
            }
        }
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::propose`].
                propose {
                    proposal: my_types::frame_support::traits::preimages::Bounded<
                        my_types::polkadot_runtime::RuntimeCall,
                    >,
                    value: ::core::primitive::u128,
                },
                ///See [`Pallet::second`].
                second { proposal: ::core::primitive::u32 },
                ///See [`Pallet::vote`].
                vote {
                    ref_index: ::core::primitive::u32,
                    vote: my_types::pallet_democracy::vote::AccountVote<
                        ::core::primitive::u128,
                    >,
                },
                ///See [`Pallet::emergency_cancel`].
                emergency_cancel { ref_index: ::core::primitive::u32 },
                ///See [`Pallet::external_propose`].
                external_propose {
                    proposal: my_types::frame_support::traits::preimages::Bounded<
                        my_types::polkadot_runtime::RuntimeCall,
                    >,
                },
                ///See [`Pallet::external_propose_majority`].
                external_propose_majority {
                    proposal: my_types::frame_support::traits::preimages::Bounded<
                        my_types::polkadot_runtime::RuntimeCall,
                    >,
                },
                ///See [`Pallet::external_propose_default`].
                external_propose_default {
                    proposal: my_types::frame_support::traits::preimages::Bounded<
                        my_types::polkadot_runtime::RuntimeCall,
                    >,
                },
                ///See [`Pallet::fast_track`].
                fast_track {
                    proposal_hash: my_types::primitive_types::H256,
                    voting_period: ::core::primitive::u32,
                    delay: ::core::primitive::u32,
                },
                ///See [`Pallet::veto_external`].
                veto_external { proposal_hash: my_types::primitive_types::H256 },
                ///See [`Pallet::cancel_referendum`].
                cancel_referendum { ref_index: ::core::primitive::u32 },
                ///See [`Pallet::delegate`].
                delegate {
                    to: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    conviction: my_types::pallet_democracy::conviction::Conviction,
                    balance: ::core::primitive::u128,
                },
                ///See [`Pallet::undelegate`].
                undelegate,
                ///See [`Pallet::clear_public_proposals`].
                clear_public_proposals,
                ///See [`Pallet::unlock`].
                unlock {
                    target: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::remove_vote`].
                remove_vote { index: ::core::primitive::u32 },
                ///See [`Pallet::remove_other_vote`].
                remove_other_vote {
                    target: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    index: ::core::primitive::u32,
                },
                ///See [`Pallet::blacklist`].
                blacklist {
                    proposal_hash: my_types::primitive_types::H256,
                    maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
                },
                ///See [`Pallet::cancel_proposal`].
                cancel_proposal { prop_index: ::core::primitive::u32 },
                ///See [`Pallet::set_metadata`].
                set_metadata {
                    owner: my_types::pallet_democracy::types::MetadataOwner,
                    maybe_hash: ::core::option::Option<my_types::primitive_types::H256>,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Value too low
                ValueLow,
                ///Proposal does not exist
                ProposalMissing,
                ///Cannot cancel the same proposal twice
                AlreadyCanceled,
                ///Proposal already made
                DuplicateProposal,
                ///Proposal still blacklisted
                ProposalBlacklisted,
                ///Next external proposal not simple majority
                NotSimpleMajority,
                ///Invalid hash
                InvalidHash,
                ///No external proposal
                NoProposal,
                ///Identity may not veto a proposal twice
                AlreadyVetoed,
                ///Vote given for invalid referendum
                ReferendumInvalid,
                ///No proposals waiting
                NoneWaiting,
                ///The given account did not vote on the referendum.
                NotVoter,
                ///The actor has no permission to conduct the action.
                NoPermission,
                ///The account is already delegating.
                AlreadyDelegating,
                ///Too high a balance was provided that the account cannot afford.
                InsufficientFunds,
                ///The account is not currently delegating.
                NotDelegating,
                ///The account currently has votes attached to it and the operation cannot succeed until
                ///these are removed, either through `unvote` or `reap_vote`.
                VotesExist,
                ///The instant referendum origin is currently disallowed.
                InstantNotAllowed,
                ///Delegation to oneself makes no sense.
                Nonsense,
                ///Invalid upper bound.
                WrongUpperBound,
                ///Maximum number of votes reached.
                MaxVotesReached,
                ///Maximum number of items reached.
                TooMany,
                ///Voting period too low
                VotingPeriodLow,
                ///The preimage does not exist.
                PreimageNotExist,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///A motion has been proposed by a public account.
                Proposed {
                    proposal_index: ::core::primitive::u32,
                    deposit: ::core::primitive::u128,
                },
                ///A public proposal has been tabled for referendum vote.
                Tabled {
                    proposal_index: ::core::primitive::u32,
                    deposit: ::core::primitive::u128,
                },
                ///An external proposal has been tabled.
                ExternalTabled,
                ///A referendum has begun.
                Started {
                    ref_index: ::core::primitive::u32,
                    threshold: my_types::pallet_democracy::vote_threshold::VoteThreshold,
                },
                ///A proposal has been approved by referendum.
                Passed { ref_index: ::core::primitive::u32 },
                ///A proposal has been rejected by referendum.
                NotPassed { ref_index: ::core::primitive::u32 },
                ///A referendum has been cancelled.
                Cancelled { ref_index: ::core::primitive::u32 },
                ///An account has delegated their vote to another account.
                Delegated {
                    who: my_types::sp_core::crypto::AccountId32,
                    target: my_types::sp_core::crypto::AccountId32,
                },
                ///An account has cancelled a previous delegation operation.
                Undelegated { account: my_types::sp_core::crypto::AccountId32 },
                ///An external proposal has been vetoed.
                Vetoed {
                    who: my_types::sp_core::crypto::AccountId32,
                    proposal_hash: my_types::primitive_types::H256,
                    until: ::core::primitive::u32,
                },
                ///A proposal_hash has been blacklisted permanently.
                Blacklisted { proposal_hash: my_types::primitive_types::H256 },
                ///An account has voted in a referendum
                Voted {
                    voter: my_types::sp_core::crypto::AccountId32,
                    ref_index: ::core::primitive::u32,
                    vote: my_types::pallet_democracy::vote::AccountVote<
                        ::core::primitive::u128,
                    >,
                },
                ///An account has secconded a proposal
                Seconded {
                    seconder: my_types::sp_core::crypto::AccountId32,
                    prop_index: ::core::primitive::u32,
                },
                ///A proposal got canceled.
                ProposalCanceled { prop_index: ::core::primitive::u32 },
                ///Metadata for a proposal or a referendum has been set.
                MetadataSet {
                    owner: my_types::pallet_democracy::types::MetadataOwner,
                    hash: my_types::primitive_types::H256,
                },
                ///Metadata for a proposal or a referendum has been cleared.
                MetadataCleared {
                    owner: my_types::pallet_democracy::types::MetadataOwner,
                    hash: my_types::primitive_types::H256,
                },
                ///Metadata has been transferred to new owner.
                MetadataTransferred {
                    prev_owner: my_types::pallet_democracy::types::MetadataOwner,
                    owner: my_types::pallet_democracy::types::MetadataOwner,
                    hash: my_types::primitive_types::H256,
                },
            }
        }
        pub mod types {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct Delegations<_0> {
                pub votes: _0,
                pub capital: _0,
            }
            #[derive(Clone, Debug)]
            pub enum MetadataOwner {
                External,
                Proposal(::core::primitive::u32),
                Referendum(::core::primitive::u32),
            }
            #[derive(Clone, Debug)]
            pub enum ReferendumInfo<_0, _1, _2> {
                Ongoing(my_types::pallet_democracy::types::ReferendumStatus<_0, _1, _2>),
                Finished { approved: ::core::primitive::bool, end: _0 },
            }
            #[derive(Clone, Debug)]
            pub struct ReferendumStatus<_0, _1, _2> {
                pub end: _0,
                pub proposal: _1,
                pub threshold: my_types::pallet_democracy::vote_threshold::VoteThreshold,
                pub delay: _0,
                pub tally: my_types::pallet_democracy::types::Tally<_2>,
            }
            #[derive(Clone, Debug)]
            pub struct Tally<_0> {
                pub ayes: _0,
                pub nays: _0,
                pub turnout: _0,
            }
        }
        pub mod vote {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub enum AccountVote<_0> {
                Standard { vote: my_types::pallet_democracy::vote::Vote, balance: _0 },
                Split { aye: _0, nay: _0 },
            }
            #[derive(Clone, Debug)]
            pub struct PriorLock<_0, _1>(pub _0, pub _1);
            #[derive(Clone, Debug, parity_scale_codec::CompactAs)]
            pub struct Vote(pub ::core::primitive::u8);
            #[derive(Clone, Debug)]
            pub enum Voting<_0, _1, _2> {
                Direct {
                    votes: my_types::bounded_collections::bounded_vec::BoundedVec<
                        (_2, my_types::pallet_democracy::vote::AccountVote<_0>),
                    >,
                    delegations: my_types::pallet_democracy::types::Delegations<_0>,
                    prior: my_types::pallet_democracy::vote::PriorLock<_2, _0>,
                },
                Delegating {
                    balance: _0,
                    target: _1,
                    conviction: my_types::pallet_democracy::conviction::Conviction,
                    delegations: my_types::pallet_democracy::types::Delegations<_0>,
                    prior: my_types::pallet_democracy::vote::PriorLock<_2, _0>,
                },
            }
        }
        pub mod vote_threshold {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub enum VoteThreshold {
                SuperMajorityApprove,
                SuperMajorityAgainst,
                SimpleMajority,
            }
        }
    }
    pub mod pallet_election_provider_multi_phase {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::submit_unsigned`].
                submit_unsigned {
                    raw_solution: ::std::boxed::Box<
                        my_types::pallet_election_provider_multi_phase::RawSolution<
                            my_types::polkadot_runtime::NposCompactSolution16,
                        >,
                    >,
                    witness: my_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
                },
                ///See [`Pallet::set_minimum_untrusted_score`].
                set_minimum_untrusted_score {
                    maybe_next_score: ::core::option::Option<
                        my_types::sp_npos_elections::ElectionScore,
                    >,
                },
                ///See [`Pallet::set_emergency_election_result`].
                set_emergency_election_result {
                    supports: ::std::vec::Vec<
                        (
                            my_types::sp_core::crypto::AccountId32,
                            my_types::sp_npos_elections::Support<
                                my_types::sp_core::crypto::AccountId32,
                            >,
                        ),
                    >,
                },
                ///See [`Pallet::submit`].
                submit {
                    raw_solution: ::std::boxed::Box<
                        my_types::pallet_election_provider_multi_phase::RawSolution<
                            my_types::polkadot_runtime::NposCompactSolution16,
                        >,
                    >,
                },
                ///See [`Pallet::governance_fallback`].
                governance_fallback {
                    maybe_max_voters: ::core::option::Option<::core::primitive::u32>,
                    maybe_max_targets: ::core::option::Option<::core::primitive::u32>,
                },
            }
            #[derive(Clone, Debug)]
            ///Error of the pallet that can be returned in response to dispatches.
            pub enum Error {
                ///Submission was too early.
                PreDispatchEarlySubmission,
                ///Wrong number of winners presented.
                PreDispatchWrongWinnerCount,
                ///Submission was too weak, score-wise.
                PreDispatchWeakSubmission,
                ///The queue was full, and the solution was not better than any of the existing ones.
                SignedQueueFull,
                ///The origin failed to pay the deposit.
                SignedCannotPayDeposit,
                ///Witness data to dispatchable is invalid.
                SignedInvalidWitness,
                ///The signed submission consumes too much weight
                SignedTooMuchWeight,
                ///OCW submitted solution for wrong round
                OcwCallWrongEra,
                ///Snapshot metadata should exist but didn't.
                MissingSnapshotMetadata,
                ///`Self::insert_submission` returned an invalid index.
                InvalidSubmissionIndex,
                ///The call is not allowed at this point.
                CallNotAllowed,
                ///The fallback failed
                FallbackFailed,
                ///Some bound not met
                BoundNotMet,
                ///Submitted solution has too many winners
                TooManyWinners,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///A solution was stored with the given compute.
                ///
                ///The `origin` indicates the origin of the solution. If `origin` is `Some(AccountId)`,
                ///the stored solution was submited in the signed phase by a miner with the `AccountId`.
                ///Otherwise, the solution was stored either during the unsigned phase or by
                ///`T::ForceOrigin`. The `bool` is `true` when a previous solution was ejected to make
                ///room for this one.
                SolutionStored {
                    compute: my_types::pallet_election_provider_multi_phase::ElectionCompute,
                    origin: ::core::option::Option<
                        my_types::sp_core::crypto::AccountId32,
                    >,
                    prev_ejected: ::core::primitive::bool,
                },
                ///The election has been finalized, with the given computation and score.
                ElectionFinalized {
                    compute: my_types::pallet_election_provider_multi_phase::ElectionCompute,
                    score: my_types::sp_npos_elections::ElectionScore,
                },
                ///An election failed.
                ///
                ///Not much can be said about which computes failed in the process.
                ElectionFailed,
                ///An account has been rewarded for their signed submission being finalized.
                Rewarded {
                    account: my_types::sp_core::crypto::AccountId32,
                    value: ::core::primitive::u128,
                },
                ///An account has been slashed for submitting an invalid signed submission.
                Slashed {
                    account: my_types::sp_core::crypto::AccountId32,
                    value: ::core::primitive::u128,
                },
                ///There was a phase transition in a given round.
                PhaseTransitioned {
                    from: my_types::pallet_election_provider_multi_phase::Phase<
                        ::core::primitive::u32,
                    >,
                    to: my_types::pallet_election_provider_multi_phase::Phase<
                        ::core::primitive::u32,
                    >,
                    round: ::core::primitive::u32,
                },
            }
        }
        pub mod signed {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct SignedSubmission<_0, _1, _2> {
                pub who: _0,
                pub deposit: _1,
                pub raw_solution: my_types::pallet_election_provider_multi_phase::RawSolution<
                    _2,
                >,
                pub call_fee: _1,
            }
        }
        #[derive(Clone, Debug)]
        pub enum ElectionCompute {
            OnChain,
            Signed,
            Unsigned,
            Fallback,
            Emergency,
        }
        #[derive(Clone, Debug)]
        pub enum Phase<_0> {
            Off,
            Signed,
            Unsigned((::core::primitive::bool, _0)),
            Emergency,
        }
        #[derive(Clone, Debug)]
        pub struct RawSolution<_0> {
            pub solution: _0,
            pub score: my_types::sp_npos_elections::ElectionScore,
            pub round: ::core::primitive::u32,
        }
        #[derive(Clone, Debug)]
        pub struct ReadySolution {
            pub supports: my_types::bounded_collections::bounded_vec::BoundedVec<
                (
                    my_types::sp_core::crypto::AccountId32,
                    my_types::sp_npos_elections::Support<
                        my_types::sp_core::crypto::AccountId32,
                    >,
                ),
            >,
            pub score: my_types::sp_npos_elections::ElectionScore,
            pub compute: my_types::pallet_election_provider_multi_phase::ElectionCompute,
        }
        #[derive(Clone, Debug)]
        pub struct RoundSnapshot<_0, _1> {
            pub voters: ::std::vec::Vec<_1>,
            pub targets: ::std::vec::Vec<_0>,
        }
        #[derive(Clone, Debug)]
        pub struct SolutionOrSnapshotSize {
            pub voters: ::core::primitive::u32,
            pub targets: ::core::primitive::u32,
        }
    }
    pub mod pallet_elections_phragmen {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::vote`].
                vote {
                    votes: ::std::vec::Vec<my_types::sp_core::crypto::AccountId32>,
                    value: ::core::primitive::u128,
                },
                ///See [`Pallet::remove_voter`].
                remove_voter,
                ///See [`Pallet::submit_candidacy`].
                submit_candidacy { candidate_count: ::core::primitive::u32 },
                ///See [`Pallet::renounce_candidacy`].
                renounce_candidacy {
                    renouncing: my_types::pallet_elections_phragmen::Renouncing,
                },
                ///See [`Pallet::remove_member`].
                remove_member {
                    who: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    slash_bond: ::core::primitive::bool,
                    rerun_election: ::core::primitive::bool,
                },
                ///See [`Pallet::clean_defunct_voters`].
                clean_defunct_voters {
                    num_voters: ::core::primitive::u32,
                    num_defunct: ::core::primitive::u32,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Cannot vote when no candidates or members exist.
                UnableToVote,
                ///Must vote for at least one candidate.
                NoVotes,
                ///Cannot vote more than candidates.
                TooManyVotes,
                ///Cannot vote more than maximum allowed.
                MaximumVotesExceeded,
                ///Cannot vote with stake less than minimum balance.
                LowBalance,
                ///Voter can not pay voting bond.
                UnableToPayBond,
                ///Must be a voter.
                MustBeVoter,
                ///Duplicated candidate submission.
                DuplicatedCandidate,
                ///Too many candidates have been created.
                TooManyCandidates,
                ///Member cannot re-submit candidacy.
                MemberSubmit,
                ///Runner cannot re-submit candidacy.
                RunnerUpSubmit,
                ///Candidate does not have enough funds.
                InsufficientCandidateFunds,
                ///Not a member.
                NotMember,
                ///The provided count of number of candidates is incorrect.
                InvalidWitnessData,
                ///The provided count of number of votes is incorrect.
                InvalidVoteCount,
                ///The renouncing origin presented a wrong `Renouncing` parameter.
                InvalidRenouncing,
                ///Prediction regarding replacement after member removal is wrong.
                InvalidReplacement,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///A new term with new_members. This indicates that enough candidates existed to run
                ///the election, not that enough have has been elected. The inner value must be examined
                ///for this purpose. A `NewTerm(\[\])` indicates that some candidates got their bond
                ///slashed and none were elected, whilst `EmptyTerm` means that no candidates existed to
                ///begin with.
                NewTerm {
                    new_members: ::std::vec::Vec<
                        (my_types::sp_core::crypto::AccountId32, ::core::primitive::u128),
                    >,
                },
                ///No (or not enough) candidates existed for this round. This is different from
                ///`NewTerm(\[\])`. See the description of `NewTerm`.
                EmptyTerm,
                ///Internal error happened while trying to perform election.
                ElectionError,
                ///A member has been removed. This should always be followed by either `NewTerm` or
                ///`EmptyTerm`.
                MemberKicked { member: my_types::sp_core::crypto::AccountId32 },
                ///Someone has renounced their candidacy.
                Renounced { candidate: my_types::sp_core::crypto::AccountId32 },
                ///A candidate was slashed by amount due to failing to obtain a seat as member or
                ///runner-up.
                ///
                ///Note that old members and runners-up are also candidates.
                CandidateSlashed {
                    candidate: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///A seat holder was slashed by amount by being forcefully removed from the set.
                SeatHolderSlashed {
                    seat_holder: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
            }
        }
        #[derive(Clone, Debug)]
        pub enum Renouncing {
            Member,
            RunnerUp,
            Candidate(::core::primitive::u32),
        }
        #[derive(Clone, Debug)]
        pub struct SeatHolder<_0, _1> {
            pub who: _0,
            pub stake: _1,
            pub deposit: _1,
        }
        #[derive(Clone, Debug)]
        pub struct Voter<_0, _1> {
            pub votes: ::std::vec::Vec<_0>,
            pub stake: _1,
            pub deposit: _1,
        }
    }
    pub mod pallet_fast_unstake {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::register_fast_unstake`].
                register_fast_unstake,
                ///See [`Pallet::deregister`].
                deregister,
                ///See [`Pallet::control`].
                control { eras_to_check: ::core::primitive::u32 },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///The provided Controller account was not found.
                ///
                ///This means that the given account is not bonded.
                NotController,
                ///The bonded account has already been queued.
                AlreadyQueued,
                ///The bonded account has active unlocking chunks.
                NotFullyBonded,
                ///The provided un-staker is not in the `Queue`.
                NotQueued,
                ///The provided un-staker is already in Head, and cannot deregister.
                AlreadyHead,
                ///The call is not allowed at this point because the pallet is not active.
                CallNotAllowed,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///A staker was unstaked.
                Unstaked {
                    stash: my_types::sp_core::crypto::AccountId32,
                    result: ::core::result::Result<
                        (),
                        my_types::sp_runtime::DispatchError,
                    >,
                },
                ///A staker was slashed for requesting fast-unstake whilst being exposed.
                Slashed {
                    stash: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///A batch was partially checked for the given eras, but the process did not finish.
                BatchChecked { eras: ::std::vec::Vec<::core::primitive::u32> },
                ///A batch of a given size was terminated.
                ///
                ///This is always follows by a number of `Unstaked` or `Slashed` events, marking the end
                ///of the batch. A new batch will be created upon next block.
                BatchFinished { size: ::core::primitive::u32 },
                ///An internal error happened. Operations will be paused now.
                InternalError,
            }
        }
        pub mod types {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct UnstakeRequest {
                pub stashes: my_types::bounded_collections::bounded_vec::BoundedVec<
                    (my_types::sp_core::crypto::AccountId32, ::core::primitive::u128),
                >,
                pub checked: my_types::bounded_collections::bounded_vec::BoundedVec<
                    ::core::primitive::u32,
                >,
            }
        }
    }
    pub mod pallet_grandpa {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::report_equivocation`].
                report_equivocation {
                    equivocation_proof: ::std::boxed::Box<
                        my_types::sp_consensus_grandpa::EquivocationProof<
                            my_types::primitive_types::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    key_owner_proof: my_types::sp_session::MembershipProof,
                },
                ///See [`Pallet::report_equivocation_unsigned`].
                report_equivocation_unsigned {
                    equivocation_proof: ::std::boxed::Box<
                        my_types::sp_consensus_grandpa::EquivocationProof<
                            my_types::primitive_types::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    key_owner_proof: my_types::sp_session::MembershipProof,
                },
                ///See [`Pallet::note_stalled`].
                note_stalled {
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Attempt to signal GRANDPA pause when the authority set isn't live
                ///(either paused or already pending pause).
                PauseFailed,
                ///Attempt to signal GRANDPA resume when the authority set isn't paused
                ///(either live or already pending resume).
                ResumeFailed,
                ///Attempt to signal GRANDPA change with one already pending.
                ChangePending,
                ///Cannot signal forced change so soon after last.
                TooSoon,
                ///A key ownership proof provided as part of an equivocation report is invalid.
                InvalidKeyOwnershipProof,
                ///An equivocation proof provided as part of an equivocation report is invalid.
                InvalidEquivocationProof,
                ///A given equivocation report is valid but already previously reported.
                DuplicateOffenceReport,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///New authority set has been applied.
                NewAuthorities {
                    authority_set: ::std::vec::Vec<
                        (
                            my_types::sp_consensus_grandpa::app::Public,
                            ::core::primitive::u64,
                        ),
                    >,
                },
                ///Current authority set has been paused.
                Paused,
                ///Current authority set has been resumed.
                Resumed,
            }
        }
        #[derive(Clone, Debug)]
        pub struct StoredPendingChange<_0> {
            pub scheduled_at: _0,
            pub delay: _0,
            pub next_authorities: my_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                (my_types::sp_consensus_grandpa::app::Public, ::core::primitive::u64),
            >,
            pub forced: ::core::option::Option<_0>,
        }
        #[derive(Clone, Debug)]
        pub enum StoredState<_0> {
            Live,
            PendingPause { scheduled_at: _0, delay: _0 },
            Paused,
            PendingResume { scheduled_at: _0, delay: _0 },
        }
    }
    pub mod pallet_identity {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Identity pallet declaration.
            pub enum Call {
                ///See [`Pallet::add_registrar`].
                add_registrar {
                    account: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::set_identity`].
                set_identity {
                    info: ::std::boxed::Box<
                        my_types::pallet_identity::types::IdentityInfo,
                    >,
                },
                ///See [`Pallet::set_subs`].
                set_subs {
                    subs: ::std::vec::Vec<
                        (
                            my_types::sp_core::crypto::AccountId32,
                            my_types::pallet_identity::types::Data,
                        ),
                    >,
                },
                ///See [`Pallet::clear_identity`].
                clear_identity,
                ///See [`Pallet::request_judgement`].
                request_judgement {
                    reg_index: ::core::primitive::u32,
                    max_fee: ::core::primitive::u128,
                },
                ///See [`Pallet::cancel_request`].
                cancel_request { reg_index: ::core::primitive::u32 },
                ///See [`Pallet::set_fee`].
                set_fee { index: ::core::primitive::u32, fee: ::core::primitive::u128 },
                ///See [`Pallet::set_account_id`].
                set_account_id {
                    index: ::core::primitive::u32,
                    new: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::set_fields`].
                set_fields {
                    index: ::core::primitive::u32,
                    fields: my_types::pallet_identity::types::BitFlags<
                        my_types::pallet_identity::types::IdentityField,
                    >,
                },
                ///See [`Pallet::provide_judgement`].
                provide_judgement {
                    reg_index: ::core::primitive::u32,
                    target: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    judgement: my_types::pallet_identity::types::Judgement<
                        ::core::primitive::u128,
                    >,
                    identity: my_types::primitive_types::H256,
                },
                ///See [`Pallet::kill_identity`].
                kill_identity {
                    target: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::add_sub`].
                add_sub {
                    sub: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    data: my_types::pallet_identity::types::Data,
                },
                ///See [`Pallet::rename_sub`].
                rename_sub {
                    sub: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    data: my_types::pallet_identity::types::Data,
                },
                ///See [`Pallet::remove_sub`].
                remove_sub {
                    sub: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::quit_sub`].
                quit_sub,
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Too many subs-accounts.
                TooManySubAccounts,
                ///Account isn't found.
                NotFound,
                ///Account isn't named.
                NotNamed,
                ///Empty index.
                EmptyIndex,
                ///Fee is changed.
                FeeChanged,
                ///No identity found.
                NoIdentity,
                ///Sticky judgement.
                StickyJudgement,
                ///Judgement given.
                JudgementGiven,
                ///Invalid judgement.
                InvalidJudgement,
                ///The index is invalid.
                InvalidIndex,
                ///The target is invalid.
                InvalidTarget,
                ///Too many additional fields.
                TooManyFields,
                ///Maximum amount of registrars reached. Cannot add any more.
                TooManyRegistrars,
                ///Account ID is already named.
                AlreadyClaimed,
                ///Sender is not a sub-account.
                NotSub,
                ///Sub-account isn't owned by sender.
                NotOwned,
                ///The provided judgement was for a different identity.
                JudgementForDifferentIdentity,
                ///Error that occurs when there is an issue paying for judgement.
                JudgementPaymentFailed,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///A name was set or reset (which will remove all judgements).
                IdentitySet { who: my_types::sp_core::crypto::AccountId32 },
                ///A name was cleared, and the given balance returned.
                IdentityCleared {
                    who: my_types::sp_core::crypto::AccountId32,
                    deposit: ::core::primitive::u128,
                },
                ///A name was removed and the given balance slashed.
                IdentityKilled {
                    who: my_types::sp_core::crypto::AccountId32,
                    deposit: ::core::primitive::u128,
                },
                ///A judgement was asked from a registrar.
                JudgementRequested {
                    who: my_types::sp_core::crypto::AccountId32,
                    registrar_index: ::core::primitive::u32,
                },
                ///A judgement request was retracted.
                JudgementUnrequested {
                    who: my_types::sp_core::crypto::AccountId32,
                    registrar_index: ::core::primitive::u32,
                },
                ///A judgement was given by a registrar.
                JudgementGiven {
                    target: my_types::sp_core::crypto::AccountId32,
                    registrar_index: ::core::primitive::u32,
                },
                ///A registrar was added.
                RegistrarAdded { registrar_index: ::core::primitive::u32 },
                ///A sub-identity was added to an identity and the deposit paid.
                SubIdentityAdded {
                    sub: my_types::sp_core::crypto::AccountId32,
                    main: my_types::sp_core::crypto::AccountId32,
                    deposit: ::core::primitive::u128,
                },
                ///A sub-identity was removed from an identity and the deposit freed.
                SubIdentityRemoved {
                    sub: my_types::sp_core::crypto::AccountId32,
                    main: my_types::sp_core::crypto::AccountId32,
                    deposit: ::core::primitive::u128,
                },
                ///A sub-identity was cleared, and the given deposit repatriated from the
                ///main identity account to the sub-identity account.
                SubIdentityRevoked {
                    sub: my_types::sp_core::crypto::AccountId32,
                    main: my_types::sp_core::crypto::AccountId32,
                    deposit: ::core::primitive::u128,
                },
            }
        }
        pub mod types {
            use super::my_types;
            #[derive(Clone, Debug, parity_scale_codec::CompactAs)]
            pub struct BitFlags<_0>(
                pub ::core::primitive::u64,
                pub ::core::marker::PhantomData<_0>,
            );
            #[derive(Clone, Debug)]
            pub enum Data {
                None,
                Raw0([::core::primitive::u8; 0usize]),
                Raw1([::core::primitive::u8; 1usize]),
                Raw2([::core::primitive::u8; 2usize]),
                Raw3([::core::primitive::u8; 3usize]),
                Raw4([::core::primitive::u8; 4usize]),
                Raw5([::core::primitive::u8; 5usize]),
                Raw6([::core::primitive::u8; 6usize]),
                Raw7([::core::primitive::u8; 7usize]),
                Raw8([::core::primitive::u8; 8usize]),
                Raw9([::core::primitive::u8; 9usize]),
                Raw10([::core::primitive::u8; 10usize]),
                Raw11([::core::primitive::u8; 11usize]),
                Raw12([::core::primitive::u8; 12usize]),
                Raw13([::core::primitive::u8; 13usize]),
                Raw14([::core::primitive::u8; 14usize]),
                Raw15([::core::primitive::u8; 15usize]),
                Raw16([::core::primitive::u8; 16usize]),
                Raw17([::core::primitive::u8; 17usize]),
                Raw18([::core::primitive::u8; 18usize]),
                Raw19([::core::primitive::u8; 19usize]),
                Raw20([::core::primitive::u8; 20usize]),
                Raw21([::core::primitive::u8; 21usize]),
                Raw22([::core::primitive::u8; 22usize]),
                Raw23([::core::primitive::u8; 23usize]),
                Raw24([::core::primitive::u8; 24usize]),
                Raw25([::core::primitive::u8; 25usize]),
                Raw26([::core::primitive::u8; 26usize]),
                Raw27([::core::primitive::u8; 27usize]),
                Raw28([::core::primitive::u8; 28usize]),
                Raw29([::core::primitive::u8; 29usize]),
                Raw30([::core::primitive::u8; 30usize]),
                Raw31([::core::primitive::u8; 31usize]),
                Raw32([::core::primitive::u8; 32usize]),
                BlakeTwo256([::core::primitive::u8; 32usize]),
                Sha256([::core::primitive::u8; 32usize]),
                Keccak256([::core::primitive::u8; 32usize]),
                ShaThree256([::core::primitive::u8; 32usize]),
            }
            #[derive(Clone, Debug)]
            pub enum IdentityField {
                Display,
                Legal,
                Web,
                Riot,
                Email,
                PgpFingerprint,
                Image,
                Twitter,
            }
            #[derive(Clone, Debug)]
            pub struct IdentityInfo {
                pub additional: my_types::bounded_collections::bounded_vec::BoundedVec<
                    (
                        my_types::pallet_identity::types::Data,
                        my_types::pallet_identity::types::Data,
                    ),
                >,
                pub display: my_types::pallet_identity::types::Data,
                pub legal: my_types::pallet_identity::types::Data,
                pub web: my_types::pallet_identity::types::Data,
                pub riot: my_types::pallet_identity::types::Data,
                pub email: my_types::pallet_identity::types::Data,
                pub pgp_fingerprint: ::core::option::Option<
                    [::core::primitive::u8; 20usize],
                >,
                pub image: my_types::pallet_identity::types::Data,
                pub twitter: my_types::pallet_identity::types::Data,
            }
            #[derive(Clone, Debug)]
            pub enum Judgement<_0> {
                Unknown,
                FeePaid(_0),
                Reasonable,
                KnownGood,
                OutOfDate,
                LowQuality,
                Erroneous,
            }
            #[derive(Clone, Debug)]
            pub struct RegistrarInfo<_0, _1> {
                pub account: _1,
                pub fee: _0,
                pub fields: my_types::pallet_identity::types::BitFlags<
                    my_types::pallet_identity::types::IdentityField,
                >,
            }
            #[derive(Clone, Debug)]
            pub struct Registration<_0> {
                pub judgements: my_types::bounded_collections::bounded_vec::BoundedVec<
                    (
                        ::core::primitive::u32,
                        my_types::pallet_identity::types::Judgement<_0>,
                    ),
                >,
                pub deposit: _0,
                pub info: my_types::pallet_identity::types::IdentityInfo,
            }
        }
    }
    pub mod pallet_im_online {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::heartbeat`].
                heartbeat {
                    heartbeat: my_types::pallet_im_online::Heartbeat<
                        ::core::primitive::u32,
                    >,
                    signature: my_types::pallet_im_online::sr25519::app_sr25519::Signature,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Non existent public key.
                InvalidKey,
                ///Duplicated heartbeat.
                DuplicatedHeartbeat,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///A new heartbeat was received from `AuthorityId`.
                HeartbeatReceived {
                    authority_id: my_types::pallet_im_online::sr25519::app_sr25519::Public,
                },
                ///At the end of the session, no offence was committed.
                AllGood,
                ///At the end of the session, at least one validator was found to be offline.
                SomeOffline {
                    offline: ::std::vec::Vec<
                        (
                            my_types::sp_core::crypto::AccountId32,
                            my_types::pallet_staking::Exposure<
                                my_types::sp_core::crypto::AccountId32,
                                ::core::primitive::u128,
                            >,
                        ),
                    >,
                },
            }
        }
        pub mod sr25519 {
            use super::my_types;
            pub mod app_sr25519 {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct Public(pub my_types::sp_core::sr25519::Public);
                #[derive(Clone, Debug)]
                pub struct Signature(pub my_types::sp_core::sr25519::Signature);
            }
        }
        #[derive(Clone, Debug)]
        pub struct Heartbeat<_0> {
            pub block_number: _0,
            pub session_index: ::core::primitive::u32,
            pub authority_index: ::core::primitive::u32,
            pub validators_len: ::core::primitive::u32,
        }
    }
    pub mod pallet_indices {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::claim`].
                claim { index: ::core::primitive::u32 },
                ///See [`Pallet::transfer`].
                transfer {
                    new: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    index: ::core::primitive::u32,
                },
                ///See [`Pallet::free`].
                free { index: ::core::primitive::u32 },
                ///See [`Pallet::force_transfer`].
                force_transfer {
                    new: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    index: ::core::primitive::u32,
                    freeze: ::core::primitive::bool,
                },
                ///See [`Pallet::freeze`].
                freeze { index: ::core::primitive::u32 },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///The index was not already assigned.
                NotAssigned,
                ///The index is assigned to another account.
                NotOwner,
                ///The index was not available.
                InUse,
                ///The source and destination accounts are identical.
                NotTransfer,
                ///The index is permanent and may not be freed/changed.
                Permanent,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///A account index was assigned.
                IndexAssigned {
                    who: my_types::sp_core::crypto::AccountId32,
                    index: ::core::primitive::u32,
                },
                ///A account index has been freed up (unassigned).
                IndexFreed { index: ::core::primitive::u32 },
                ///A account index has been frozen to its current account ID.
                IndexFrozen {
                    index: ::core::primitive::u32,
                    who: my_types::sp_core::crypto::AccountId32,
                },
            }
        }
    }
    pub mod pallet_membership {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::add_member`].
                add_member {
                    who: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::remove_member`].
                remove_member {
                    who: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::swap_member`].
                swap_member {
                    remove: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    add: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::reset_members`].
                reset_members {
                    members: ::std::vec::Vec<my_types::sp_core::crypto::AccountId32>,
                },
                ///See [`Pallet::change_key`].
                change_key {
                    new: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::set_prime`].
                set_prime {
                    who: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::clear_prime`].
                clear_prime,
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Already a member.
                AlreadyMember,
                ///Not a member.
                NotMember,
                ///Too many members.
                TooManyMembers,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///The given member was added; see the transaction for who.
                MemberAdded,
                ///The given member was removed; see the transaction for who.
                MemberRemoved,
                ///Two members were swapped; see the transaction for who.
                MembersSwapped,
                ///The membership was reset; see the transaction for who the new set is.
                MembersReset,
                ///One of the members' keys changed.
                KeyChanged,
                ///Phantom member, never used.
                Dummy,
            }
        }
    }
    pub mod pallet_message_queue {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::reap_page`].
                reap_page {
                    message_origin: my_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin,
                    page_index: ::core::primitive::u32,
                },
                ///See [`Pallet::execute_overweight`].
                execute_overweight {
                    message_origin: my_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin,
                    page: ::core::primitive::u32,
                    index: ::core::primitive::u32,
                    weight_limit: my_types::sp_weights::weight_v2::Weight,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Page is not reapable because it has items remaining to be processed and is not old
                ///enough.
                NotReapable,
                ///Page to be reaped does not exist.
                NoPage,
                ///The referenced message could not be found.
                NoMessage,
                ///The message was already processed and cannot be processed again.
                AlreadyProcessed,
                ///The message is queued for future execution.
                Queued,
                ///There is temporarily not enough weight to continue servicing messages.
                InsufficientWeight,
                ///This message is temporarily unprocessable.
                ///
                ///Such errors are expected, but not guaranteed, to resolve themselves eventually through
                ///retrying.
                TemporarilyUnprocessable,
                ///The queue is paused and no message can be executed from it.
                ///
                ///This can change at any time and may resolve in the future by re-trying.
                QueuePaused,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///Message discarded due to an error in the `MessageProcessor` (usually a format error).
                ProcessingFailed {
                    id: [::core::primitive::u8; 32usize],
                    origin: my_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin,
                    error: my_types::frame_support::traits::messages::ProcessMessageError,
                },
                ///Message is processed.
                Processed {
                    id: [::core::primitive::u8; 32usize],
                    origin: my_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin,
                    weight_used: my_types::sp_weights::weight_v2::Weight,
                    success: ::core::primitive::bool,
                },
                ///Message placed in overweight queue.
                OverweightEnqueued {
                    id: [::core::primitive::u8; 32usize],
                    origin: my_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin,
                    page_index: ::core::primitive::u32,
                    message_index: ::core::primitive::u32,
                },
                ///This page was reaped.
                PageReaped {
                    origin: my_types::polkadot_runtime_parachains::inclusion::AggregateMessageOrigin,
                    index: ::core::primitive::u32,
                },
            }
        }
        #[derive(Clone, Debug)]
        pub struct BookState<_0> {
            pub begin: ::core::primitive::u32,
            pub end: ::core::primitive::u32,
            pub count: ::core::primitive::u32,
            pub ready_neighbours: ::core::option::Option<
                my_types::pallet_message_queue::Neighbours<_0>,
            >,
            pub message_count: ::core::primitive::u64,
            pub size: ::core::primitive::u64,
        }
        #[derive(Clone, Debug)]
        pub struct Neighbours<_0> {
            pub prev: _0,
            pub next: _0,
        }
        #[derive(Clone, Debug)]
        pub struct Page<_0> {
            pub remaining: _0,
            pub remaining_size: _0,
            pub first_index: _0,
            pub first: _0,
            pub last: _0,
            pub heap: my_types::bounded_collections::bounded_vec::BoundedVec<
                ::core::primitive::u8,
            >,
        }
    }
    pub mod pallet_multisig {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::as_multi_threshold_1`].
                as_multi_threshold_1 {
                    other_signatories: ::std::vec::Vec<
                        my_types::sp_core::crypto::AccountId32,
                    >,
                    call: ::std::boxed::Box<my_types::polkadot_runtime::RuntimeCall>,
                },
                ///See [`Pallet::as_multi`].
                as_multi {
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<
                        my_types::sp_core::crypto::AccountId32,
                    >,
                    maybe_timepoint: ::core::option::Option<
                        my_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call: ::std::boxed::Box<my_types::polkadot_runtime::RuntimeCall>,
                    max_weight: my_types::sp_weights::weight_v2::Weight,
                },
                ///See [`Pallet::approve_as_multi`].
                approve_as_multi {
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<
                        my_types::sp_core::crypto::AccountId32,
                    >,
                    maybe_timepoint: ::core::option::Option<
                        my_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call_hash: [::core::primitive::u8; 32usize],
                    max_weight: my_types::sp_weights::weight_v2::Weight,
                },
                ///See [`Pallet::cancel_as_multi`].
                cancel_as_multi {
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<
                        my_types::sp_core::crypto::AccountId32,
                    >,
                    timepoint: my_types::pallet_multisig::Timepoint<
                        ::core::primitive::u32,
                    >,
                    call_hash: [::core::primitive::u8; 32usize],
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Threshold must be 2 or greater.
                MinimumThreshold,
                ///Call is already approved by this signatory.
                AlreadyApproved,
                ///Call doesn't need any (more) approvals.
                NoApprovalsNeeded,
                ///There are too few signatories in the list.
                TooFewSignatories,
                ///There are too many signatories in the list.
                TooManySignatories,
                ///The signatories were provided out of order; they should be ordered.
                SignatoriesOutOfOrder,
                ///The sender was contained in the other signatories; it shouldn't be.
                SenderInSignatories,
                ///Multisig operation not found when attempting to cancel.
                NotFound,
                ///Only the account that originally created the multisig is able to cancel it.
                NotOwner,
                ///No timepoint was given, yet the multisig operation is already underway.
                NoTimepoint,
                ///A different timepoint was given to the multisig operation that is underway.
                WrongTimepoint,
                ///A timepoint was given, yet no multisig operation is underway.
                UnexpectedTimepoint,
                ///The maximum weight information provided was too low.
                MaxWeightTooLow,
                ///The data to be stored is already stored.
                AlreadyStored,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///A new multisig operation has begun.
                NewMultisig {
                    approving: my_types::sp_core::crypto::AccountId32,
                    multisig: my_types::sp_core::crypto::AccountId32,
                    call_hash: [::core::primitive::u8; 32usize],
                },
                ///A multisig operation has been approved by someone.
                MultisigApproval {
                    approving: my_types::sp_core::crypto::AccountId32,
                    timepoint: my_types::pallet_multisig::Timepoint<
                        ::core::primitive::u32,
                    >,
                    multisig: my_types::sp_core::crypto::AccountId32,
                    call_hash: [::core::primitive::u8; 32usize],
                },
                ///A multisig operation has been executed.
                MultisigExecuted {
                    approving: my_types::sp_core::crypto::AccountId32,
                    timepoint: my_types::pallet_multisig::Timepoint<
                        ::core::primitive::u32,
                    >,
                    multisig: my_types::sp_core::crypto::AccountId32,
                    call_hash: [::core::primitive::u8; 32usize],
                    result: ::core::result::Result<
                        (),
                        my_types::sp_runtime::DispatchError,
                    >,
                },
                ///A multisig operation has been cancelled.
                MultisigCancelled {
                    cancelling: my_types::sp_core::crypto::AccountId32,
                    timepoint: my_types::pallet_multisig::Timepoint<
                        ::core::primitive::u32,
                    >,
                    multisig: my_types::sp_core::crypto::AccountId32,
                    call_hash: [::core::primitive::u8; 32usize],
                },
            }
        }
        #[derive(Clone, Debug)]
        pub struct Multisig<_0, _1, _2> {
            pub when: my_types::pallet_multisig::Timepoint<_0>,
            pub deposit: _1,
            pub depositor: _2,
            pub approvals: my_types::bounded_collections::bounded_vec::BoundedVec<_2>,
        }
        #[derive(Clone, Debug)]
        pub struct Timepoint<_0> {
            pub height: _0,
            pub index: ::core::primitive::u32,
        }
    }
    pub mod pallet_nomination_pools {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::join`].
                join {
                    amount: ::core::primitive::u128,
                    pool_id: ::core::primitive::u32,
                },
                ///See [`Pallet::bond_extra`].
                bond_extra {
                    extra: my_types::pallet_nomination_pools::BondExtra<
                        ::core::primitive::u128,
                    >,
                },
                ///See [`Pallet::claim_payout`].
                claim_payout,
                ///See [`Pallet::unbond`].
                unbond {
                    member_account: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    unbonding_points: ::core::primitive::u128,
                },
                ///See [`Pallet::pool_withdraw_unbonded`].
                pool_withdraw_unbonded {
                    pool_id: ::core::primitive::u32,
                    num_slashing_spans: ::core::primitive::u32,
                },
                ///See [`Pallet::withdraw_unbonded`].
                withdraw_unbonded {
                    member_account: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    num_slashing_spans: ::core::primitive::u32,
                },
                ///See [`Pallet::create`].
                create {
                    amount: ::core::primitive::u128,
                    root: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    nominator: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    bouncer: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::create_with_pool_id`].
                create_with_pool_id {
                    amount: ::core::primitive::u128,
                    root: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    nominator: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    bouncer: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    pool_id: ::core::primitive::u32,
                },
                ///See [`Pallet::nominate`].
                nominate {
                    pool_id: ::core::primitive::u32,
                    validators: ::std::vec::Vec<my_types::sp_core::crypto::AccountId32>,
                },
                ///See [`Pallet::set_state`].
                set_state {
                    pool_id: ::core::primitive::u32,
                    state: my_types::pallet_nomination_pools::PoolState,
                },
                ///See [`Pallet::set_metadata`].
                set_metadata {
                    pool_id: ::core::primitive::u32,
                    metadata: ::std::vec::Vec<::core::primitive::u8>,
                },
                ///See [`Pallet::set_configs`].
                set_configs {
                    min_join_bond: my_types::pallet_nomination_pools::ConfigOp<
                        ::core::primitive::u128,
                    >,
                    min_create_bond: my_types::pallet_nomination_pools::ConfigOp<
                        ::core::primitive::u128,
                    >,
                    max_pools: my_types::pallet_nomination_pools::ConfigOp<
                        ::core::primitive::u32,
                    >,
                    max_members: my_types::pallet_nomination_pools::ConfigOp<
                        ::core::primitive::u32,
                    >,
                    max_members_per_pool: my_types::pallet_nomination_pools::ConfigOp<
                        ::core::primitive::u32,
                    >,
                    global_max_commission: my_types::pallet_nomination_pools::ConfigOp<
                        my_types::sp_arithmetic::per_things::Perbill,
                    >,
                },
                ///See [`Pallet::update_roles`].
                update_roles {
                    pool_id: ::core::primitive::u32,
                    new_root: my_types::pallet_nomination_pools::ConfigOp<
                        my_types::sp_core::crypto::AccountId32,
                    >,
                    new_nominator: my_types::pallet_nomination_pools::ConfigOp<
                        my_types::sp_core::crypto::AccountId32,
                    >,
                    new_bouncer: my_types::pallet_nomination_pools::ConfigOp<
                        my_types::sp_core::crypto::AccountId32,
                    >,
                },
                ///See [`Pallet::chill`].
                chill { pool_id: ::core::primitive::u32 },
                ///See [`Pallet::bond_extra_other`].
                bond_extra_other {
                    member: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    extra: my_types::pallet_nomination_pools::BondExtra<
                        ::core::primitive::u128,
                    >,
                },
                ///See [`Pallet::set_claim_permission`].
                set_claim_permission {
                    permission: my_types::pallet_nomination_pools::ClaimPermission,
                },
                ///See [`Pallet::claim_payout_other`].
                claim_payout_other { other: my_types::sp_core::crypto::AccountId32 },
                ///See [`Pallet::set_commission`].
                set_commission {
                    pool_id: ::core::primitive::u32,
                    new_commission: ::core::option::Option<
                        (
                            my_types::sp_arithmetic::per_things::Perbill,
                            my_types::sp_core::crypto::AccountId32,
                        ),
                    >,
                },
                ///See [`Pallet::set_commission_max`].
                set_commission_max {
                    pool_id: ::core::primitive::u32,
                    max_commission: my_types::sp_arithmetic::per_things::Perbill,
                },
                ///See [`Pallet::set_commission_change_rate`].
                set_commission_change_rate {
                    pool_id: ::core::primitive::u32,
                    change_rate: my_types::pallet_nomination_pools::CommissionChangeRate<
                        ::core::primitive::u32,
                    >,
                },
                ///See [`Pallet::claim_commission`].
                claim_commission { pool_id: ::core::primitive::u32 },
            }
            #[derive(Clone, Debug)]
            pub enum DefensiveError {
                NotEnoughSpaceInUnbondPool,
                PoolNotFound,
                RewardPoolNotFound,
                SubPoolsNotFound,
                BondedStashKilledPrematurely,
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///A (bonded) pool id does not exist.
                PoolNotFound,
                ///An account is not a member.
                PoolMemberNotFound,
                ///A reward pool does not exist. In all cases this is a system logic error.
                RewardPoolNotFound,
                ///A sub pool does not exist.
                SubPoolsNotFound,
                ///An account is already delegating in another pool. An account may only belong to one
                ///pool at a time.
                AccountBelongsToOtherPool,
                ///The member is fully unbonded (and thus cannot access the bonded and reward pool
                ///anymore to, for example, collect rewards).
                FullyUnbonding,
                ///The member cannot unbond further chunks due to reaching the limit.
                MaxUnbondingLimit,
                ///None of the funds can be withdrawn yet because the bonding duration has not passed.
                CannotWithdrawAny,
                ///The amount does not meet the minimum bond to either join or create a pool.
                ///
                ///The depositor can never unbond to a value less than
                ///`Pallet::depositor_min_bond`. The caller does not have nominating
                ///permissions for the pool. Members can never unbond to a value below `MinJoinBond`.
                MinimumBondNotMet,
                ///The transaction could not be executed due to overflow risk for the pool.
                OverflowRisk,
                ///A pool must be in [`PoolState::Destroying`] in order for the depositor to unbond or for
                ///other members to be permissionlessly unbonded.
                NotDestroying,
                ///The caller does not have nominating permissions for the pool.
                NotNominator,
                ///Either a) the caller cannot make a valid kick or b) the pool is not destroying.
                NotKickerOrDestroying,
                ///The pool is not open to join
                NotOpen,
                ///The system is maxed out on pools.
                MaxPools,
                ///Too many members in the pool or system.
                MaxPoolMembers,
                ///The pools state cannot be changed.
                CanNotChangeState,
                ///The caller does not have adequate permissions.
                DoesNotHavePermission,
                ///Metadata exceeds [`Config::MaxMetadataLen`]
                MetadataExceedsMaxLen,
                ///Some error occurred that should never happen. This should be reported to the
                ///maintainers.
                Defensive(my_types::pallet_nomination_pools::pallet::DefensiveError),
                ///Partial unbonding now allowed permissionlessly.
                PartialUnbondNotAllowedPermissionlessly,
                ///The pool's max commission cannot be set higher than the existing value.
                MaxCommissionRestricted,
                ///The supplied commission exceeds the max allowed commission.
                CommissionExceedsMaximum,
                ///Not enough blocks have surpassed since the last commission update.
                CommissionChangeThrottled,
                ///The submitted changes to commission change rate are not allowed.
                CommissionChangeRateNotAllowed,
                ///There is no pending commission to claim.
                NoPendingCommission,
                ///No commission current has been set.
                NoCommissionCurrentSet,
                ///Pool id currently in use.
                PoolIdInUse,
                ///Pool id provided is not correct/usable.
                InvalidPoolId,
                ///Bonding extra is restricted to the exact pending reward amount.
                BondExtraRestricted,
            }
            #[derive(Clone, Debug)]
            ///Events of this pallet.
            pub enum Event {
                ///A pool has been created.
                Created {
                    depositor: my_types::sp_core::crypto::AccountId32,
                    pool_id: ::core::primitive::u32,
                },
                ///A member has became bonded in a pool.
                Bonded {
                    member: my_types::sp_core::crypto::AccountId32,
                    pool_id: ::core::primitive::u32,
                    bonded: ::core::primitive::u128,
                    joined: ::core::primitive::bool,
                },
                ///A payout has been made to a member.
                PaidOut {
                    member: my_types::sp_core::crypto::AccountId32,
                    pool_id: ::core::primitive::u32,
                    payout: ::core::primitive::u128,
                },
                ///A member has unbonded from their pool.
                ///
                ///- `balance` is the corresponding balance of the number of points that has been
                ///  requested to be unbonded (the argument of the `unbond` transaction) from the bonded
                ///  pool.
                ///- `points` is the number of points that are issued as a result of `balance` being
                ///dissolved into the corresponding unbonding pool.
                ///- `era` is the era in which the balance will be unbonded.
                ///In the absence of slashing, these values will match. In the presence of slashing, the
                ///number of points that are issued in the unbonding pool will be less than the amount
                ///requested to be unbonded.
                Unbonded {
                    member: my_types::sp_core::crypto::AccountId32,
                    pool_id: ::core::primitive::u32,
                    balance: ::core::primitive::u128,
                    points: ::core::primitive::u128,
                    era: ::core::primitive::u32,
                },
                ///A member has withdrawn from their pool.
                ///
                ///The given number of `points` have been dissolved in return of `balance`.
                ///
                ///Similar to `Unbonded` event, in the absence of slashing, the ratio of point to balance
                ///will be 1.
                Withdrawn {
                    member: my_types::sp_core::crypto::AccountId32,
                    pool_id: ::core::primitive::u32,
                    balance: ::core::primitive::u128,
                    points: ::core::primitive::u128,
                },
                ///A pool has been destroyed.
                Destroyed { pool_id: ::core::primitive::u32 },
                ///The state of a pool has changed
                StateChanged {
                    pool_id: ::core::primitive::u32,
                    new_state: my_types::pallet_nomination_pools::PoolState,
                },
                ///A member has been removed from a pool.
                ///
                ///The removal can be voluntary (withdrawn all unbonded funds) or involuntary (kicked).
                MemberRemoved {
                    pool_id: ::core::primitive::u32,
                    member: my_types::sp_core::crypto::AccountId32,
                },
                ///The roles of a pool have been updated to the given new roles. Note that the depositor
                ///can never change.
                RolesUpdated {
                    root: ::core::option::Option<my_types::sp_core::crypto::AccountId32>,
                    bouncer: ::core::option::Option<
                        my_types::sp_core::crypto::AccountId32,
                    >,
                    nominator: ::core::option::Option<
                        my_types::sp_core::crypto::AccountId32,
                    >,
                },
                ///The active balance of pool `pool_id` has been slashed to `balance`.
                PoolSlashed {
                    pool_id: ::core::primitive::u32,
                    balance: ::core::primitive::u128,
                },
                ///The unbond pool at `era` of pool `pool_id` has been slashed to `balance`.
                UnbondingPoolSlashed {
                    pool_id: ::core::primitive::u32,
                    era: ::core::primitive::u32,
                    balance: ::core::primitive::u128,
                },
                ///A pool's commission setting has been changed.
                PoolCommissionUpdated {
                    pool_id: ::core::primitive::u32,
                    current: ::core::option::Option<
                        (
                            my_types::sp_arithmetic::per_things::Perbill,
                            my_types::sp_core::crypto::AccountId32,
                        ),
                    >,
                },
                ///A pool's maximum commission setting has been changed.
                PoolMaxCommissionUpdated {
                    pool_id: ::core::primitive::u32,
                    max_commission: my_types::sp_arithmetic::per_things::Perbill,
                },
                ///A pool's commission `change_rate` has been changed.
                PoolCommissionChangeRateUpdated {
                    pool_id: ::core::primitive::u32,
                    change_rate: my_types::pallet_nomination_pools::CommissionChangeRate<
                        ::core::primitive::u32,
                    >,
                },
                ///Pool commission has been claimed.
                PoolCommissionClaimed {
                    pool_id: ::core::primitive::u32,
                    commission: ::core::primitive::u128,
                },
            }
        }
        #[derive(Clone, Debug)]
        pub enum BondExtra<_0> {
            FreeBalance(_0),
            Rewards,
        }
        #[derive(Clone, Debug)]
        pub struct BondedPoolInner {
            pub commission: my_types::pallet_nomination_pools::Commission,
            pub member_counter: ::core::primitive::u32,
            pub points: ::core::primitive::u128,
            pub roles: my_types::pallet_nomination_pools::PoolRoles<
                my_types::sp_core::crypto::AccountId32,
            >,
            pub state: my_types::pallet_nomination_pools::PoolState,
        }
        #[derive(Clone, Debug)]
        pub enum ClaimPermission {
            Permissioned,
            PermissionlessCompound,
            PermissionlessWithdraw,
            PermissionlessAll,
        }
        #[derive(Clone, Debug)]
        pub struct Commission {
            pub current: ::core::option::Option<
                (
                    my_types::sp_arithmetic::per_things::Perbill,
                    my_types::sp_core::crypto::AccountId32,
                ),
            >,
            pub max: ::core::option::Option<
                my_types::sp_arithmetic::per_things::Perbill,
            >,
            pub change_rate: ::core::option::Option<
                my_types::pallet_nomination_pools::CommissionChangeRate<
                    ::core::primitive::u32,
                >,
            >,
            pub throttle_from: ::core::option::Option<::core::primitive::u32>,
        }
        #[derive(Clone, Debug)]
        pub struct CommissionChangeRate<_0> {
            pub max_increase: my_types::sp_arithmetic::per_things::Perbill,
            pub min_delay: _0,
        }
        #[derive(Clone, Debug)]
        pub enum ConfigOp<_0> {
            Noop,
            Set(_0),
            Remove,
        }
        #[derive(Clone, Debug)]
        pub struct PoolMember {
            pub pool_id: ::core::primitive::u32,
            pub points: ::core::primitive::u128,
            pub last_recorded_reward_counter: my_types::sp_arithmetic::fixed_point::FixedU128,
            pub unbonding_eras: my_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
                ::core::primitive::u32,
                ::core::primitive::u128,
            >,
        }
        #[derive(Clone, Debug)]
        pub struct PoolRoles<_0> {
            pub depositor: _0,
            pub root: ::core::option::Option<_0>,
            pub nominator: ::core::option::Option<_0>,
            pub bouncer: ::core::option::Option<_0>,
        }
        #[derive(Clone, Debug)]
        pub enum PoolState {
            Open,
            Blocked,
            Destroying,
        }
        #[derive(Clone, Debug)]
        pub struct RewardPool {
            pub last_recorded_reward_counter: my_types::sp_arithmetic::fixed_point::FixedU128,
            pub last_recorded_total_payouts: ::core::primitive::u128,
            pub total_rewards_claimed: ::core::primitive::u128,
            pub total_commission_pending: ::core::primitive::u128,
            pub total_commission_claimed: ::core::primitive::u128,
        }
        #[derive(Clone, Debug)]
        pub struct SubPools {
            pub no_era: my_types::pallet_nomination_pools::UnbondPool,
            pub with_era: my_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
                ::core::primitive::u32,
                my_types::pallet_nomination_pools::UnbondPool,
            >,
        }
        #[derive(Clone, Debug)]
        pub struct UnbondPool {
            pub points: ::core::primitive::u128,
            pub balance: ::core::primitive::u128,
        }
    }
    pub mod pallet_offences {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Events type.
            pub enum Event {
                ///There is an offence reported of the given `kind` happened at the `session_index` and
                ///(kind-specific) time slot. This event is not deposited for duplicate slashes.
                ///\[kind, timeslot\].
                Offence {
                    kind: [::core::primitive::u8; 16usize],
                    timeslot: ::std::vec::Vec<::core::primitive::u8>,
                },
            }
        }
    }
    pub mod pallet_preimage {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::note_preimage`].
                note_preimage { bytes: ::std::vec::Vec<::core::primitive::u8> },
                ///See [`Pallet::unnote_preimage`].
                unnote_preimage { hash: my_types::primitive_types::H256 },
                ///See [`Pallet::request_preimage`].
                request_preimage { hash: my_types::primitive_types::H256 },
                ///See [`Pallet::unrequest_preimage`].
                unrequest_preimage { hash: my_types::primitive_types::H256 },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Preimage is too large to store on-chain.
                TooBig,
                ///Preimage has already been noted on-chain.
                AlreadyNoted,
                ///The user is not authorized to perform this action.
                NotAuthorized,
                ///The preimage cannot be removed since it has not yet been noted.
                NotNoted,
                ///A preimage may not be removed when there are outstanding requests.
                Requested,
                ///The preimage request cannot be removed since no outstanding requests exist.
                NotRequested,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///A preimage has been noted.
                Noted { hash: my_types::primitive_types::H256 },
                ///A preimage has been requested.
                Requested { hash: my_types::primitive_types::H256 },
                ///A preimage has ben cleared.
                Cleared { hash: my_types::primitive_types::H256 },
            }
        }
        #[derive(Clone, Debug)]
        pub enum RequestStatus<_0, _1> {
            Unrequested { deposit: (_0, _1), len: ::core::primitive::u32 },
            Requested {
                deposit: ::core::option::Option<(_0, _1)>,
                count: ::core::primitive::u32,
                len: ::core::option::Option<::core::primitive::u32>,
            },
        }
    }
    pub mod pallet_proxy {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::proxy`].
                proxy {
                    real: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    force_proxy_type: ::core::option::Option<
                        my_types::polkadot_runtime::ProxyType,
                    >,
                    call: ::std::boxed::Box<my_types::polkadot_runtime::RuntimeCall>,
                },
                ///See [`Pallet::add_proxy`].
                add_proxy {
                    delegate: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    proxy_type: my_types::polkadot_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                },
                ///See [`Pallet::remove_proxy`].
                remove_proxy {
                    delegate: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    proxy_type: my_types::polkadot_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                },
                ///See [`Pallet::remove_proxies`].
                remove_proxies,
                ///See [`Pallet::create_pure`].
                create_pure {
                    proxy_type: my_types::polkadot_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                    index: ::core::primitive::u16,
                },
                ///See [`Pallet::kill_pure`].
                kill_pure {
                    spawner: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    proxy_type: my_types::polkadot_runtime::ProxyType,
                    index: ::core::primitive::u16,
                    height: ::core::primitive::u32,
                    ext_index: ::core::primitive::u32,
                },
                ///See [`Pallet::announce`].
                announce {
                    real: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    call_hash: my_types::primitive_types::H256,
                },
                ///See [`Pallet::remove_announcement`].
                remove_announcement {
                    real: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    call_hash: my_types::primitive_types::H256,
                },
                ///See [`Pallet::reject_announcement`].
                reject_announcement {
                    delegate: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    call_hash: my_types::primitive_types::H256,
                },
                ///See [`Pallet::proxy_announced`].
                proxy_announced {
                    delegate: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    real: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    force_proxy_type: ::core::option::Option<
                        my_types::polkadot_runtime::ProxyType,
                    >,
                    call: ::std::boxed::Box<my_types::polkadot_runtime::RuntimeCall>,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///There are too many proxies registered or too many announcements pending.
                TooMany,
                ///Proxy registration not found.
                NotFound,
                ///Sender is not a proxy of the account to be proxied.
                NotProxy,
                ///A call which is incompatible with the proxy type's filter was attempted.
                Unproxyable,
                ///Account is already a proxy.
                Duplicate,
                ///Call may not be made by proxy because it may escalate its privileges.
                NoPermission,
                ///Announcement, if made at all, was made too recently.
                Unannounced,
                ///Cannot add self as proxy.
                NoSelfProxy,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///A proxy was executed correctly, with the given.
                ProxyExecuted {
                    result: ::core::result::Result<
                        (),
                        my_types::sp_runtime::DispatchError,
                    >,
                },
                ///A pure account has been created by new proxy with given
                ///disambiguation index and proxy type.
                PureCreated {
                    pure: my_types::sp_core::crypto::AccountId32,
                    who: my_types::sp_core::crypto::AccountId32,
                    proxy_type: my_types::polkadot_runtime::ProxyType,
                    disambiguation_index: ::core::primitive::u16,
                },
                ///An announcement was placed to make a call in the future.
                Announced {
                    real: my_types::sp_core::crypto::AccountId32,
                    proxy: my_types::sp_core::crypto::AccountId32,
                    call_hash: my_types::primitive_types::H256,
                },
                ///A proxy was added.
                ProxyAdded {
                    delegator: my_types::sp_core::crypto::AccountId32,
                    delegatee: my_types::sp_core::crypto::AccountId32,
                    proxy_type: my_types::polkadot_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                },
                ///A proxy was removed.
                ProxyRemoved {
                    delegator: my_types::sp_core::crypto::AccountId32,
                    delegatee: my_types::sp_core::crypto::AccountId32,
                    proxy_type: my_types::polkadot_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                },
            }
        }
        #[derive(Clone, Debug)]
        pub struct Announcement<_0, _1, _2> {
            pub real: _0,
            pub call_hash: _1,
            pub height: _2,
        }
        #[derive(Clone, Debug)]
        pub struct ProxyDefinition<_0, _1, _2> {
            pub delegate: _0,
            pub proxy_type: _1,
            pub delay: _2,
        }
    }
    pub mod pallet_referenda {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::submit`].
                submit {
                    proposal_origin: ::std::boxed::Box<
                        my_types::polkadot_runtime::OriginCaller,
                    >,
                    proposal: my_types::frame_support::traits::preimages::Bounded<
                        my_types::polkadot_runtime::RuntimeCall,
                    >,
                    enactment_moment: my_types::frame_support::traits::schedule::DispatchTime<
                        ::core::primitive::u32,
                    >,
                },
                ///See [`Pallet::place_decision_deposit`].
                place_decision_deposit { index: ::core::primitive::u32 },
                ///See [`Pallet::refund_decision_deposit`].
                refund_decision_deposit { index: ::core::primitive::u32 },
                ///See [`Pallet::cancel`].
                cancel { index: ::core::primitive::u32 },
                ///See [`Pallet::kill`].
                kill { index: ::core::primitive::u32 },
                ///See [`Pallet::nudge_referendum`].
                nudge_referendum { index: ::core::primitive::u32 },
                ///See [`Pallet::one_fewer_deciding`].
                one_fewer_deciding { track: ::core::primitive::u16 },
                ///See [`Pallet::refund_submission_deposit`].
                refund_submission_deposit { index: ::core::primitive::u32 },
                ///See [`Pallet::set_metadata`].
                set_metadata {
                    index: ::core::primitive::u32,
                    maybe_hash: ::core::option::Option<my_types::primitive_types::H256>,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Referendum is not ongoing.
                NotOngoing,
                ///Referendum's decision deposit is already paid.
                HasDeposit,
                ///The track identifier given was invalid.
                BadTrack,
                ///There are already a full complement of referenda in progress for this track.
                Full,
                ///The queue of the track is empty.
                QueueEmpty,
                ///The referendum index provided is invalid in this context.
                BadReferendum,
                ///There was nothing to do in the advancement.
                NothingToDo,
                ///No track exists for the proposal origin.
                NoTrack,
                ///Any deposit cannot be refunded until after the decision is over.
                Unfinished,
                ///The deposit refunder is not the depositor.
                NoPermission,
                ///The deposit cannot be refunded since none was made.
                NoDeposit,
                ///The referendum status is invalid for this operation.
                BadStatus,
                ///The preimage does not exist.
                PreimageNotExist,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///A referendum has been submitted.
                Submitted {
                    index: ::core::primitive::u32,
                    track: ::core::primitive::u16,
                    proposal: my_types::frame_support::traits::preimages::Bounded<
                        my_types::polkadot_runtime::RuntimeCall,
                    >,
                },
                ///The decision deposit has been placed.
                DecisionDepositPlaced {
                    index: ::core::primitive::u32,
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///The decision deposit has been refunded.
                DecisionDepositRefunded {
                    index: ::core::primitive::u32,
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///A deposit has been slashaed.
                DepositSlashed {
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///A referendum has moved into the deciding phase.
                DecisionStarted {
                    index: ::core::primitive::u32,
                    track: ::core::primitive::u16,
                    proposal: my_types::frame_support::traits::preimages::Bounded<
                        my_types::polkadot_runtime::RuntimeCall,
                    >,
                    tally: my_types::pallet_conviction_voting::types::Tally<
                        ::core::primitive::u128,
                    >,
                },
                ConfirmStarted { index: ::core::primitive::u32 },
                ConfirmAborted { index: ::core::primitive::u32 },
                ///A referendum has ended its confirmation phase and is ready for approval.
                Confirmed {
                    index: ::core::primitive::u32,
                    tally: my_types::pallet_conviction_voting::types::Tally<
                        ::core::primitive::u128,
                    >,
                },
                ///A referendum has been approved and its proposal has been scheduled.
                Approved { index: ::core::primitive::u32 },
                ///A proposal has been rejected by referendum.
                Rejected {
                    index: ::core::primitive::u32,
                    tally: my_types::pallet_conviction_voting::types::Tally<
                        ::core::primitive::u128,
                    >,
                },
                ///A referendum has been timed out without being decided.
                TimedOut {
                    index: ::core::primitive::u32,
                    tally: my_types::pallet_conviction_voting::types::Tally<
                        ::core::primitive::u128,
                    >,
                },
                ///A referendum has been cancelled.
                Cancelled {
                    index: ::core::primitive::u32,
                    tally: my_types::pallet_conviction_voting::types::Tally<
                        ::core::primitive::u128,
                    >,
                },
                ///A referendum has been killed.
                Killed {
                    index: ::core::primitive::u32,
                    tally: my_types::pallet_conviction_voting::types::Tally<
                        ::core::primitive::u128,
                    >,
                },
                ///The submission deposit has been refunded.
                SubmissionDepositRefunded {
                    index: ::core::primitive::u32,
                    who: my_types::sp_core::crypto::AccountId32,
                    amount: ::core::primitive::u128,
                },
                ///Metadata for a referendum has been set.
                MetadataSet {
                    index: ::core::primitive::u32,
                    hash: my_types::primitive_types::H256,
                },
                ///Metadata for a referendum has been cleared.
                MetadataCleared {
                    index: ::core::primitive::u32,
                    hash: my_types::primitive_types::H256,
                },
            }
        }
        pub mod types {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub enum Curve {
                LinearDecreasing {
                    length: my_types::sp_arithmetic::per_things::Perbill,
                    floor: my_types::sp_arithmetic::per_things::Perbill,
                    ceil: my_types::sp_arithmetic::per_things::Perbill,
                },
                SteppedDecreasing {
                    begin: my_types::sp_arithmetic::per_things::Perbill,
                    end: my_types::sp_arithmetic::per_things::Perbill,
                    step: my_types::sp_arithmetic::per_things::Perbill,
                    period: my_types::sp_arithmetic::per_things::Perbill,
                },
                Reciprocal {
                    factor: my_types::sp_arithmetic::fixed_point::FixedI64,
                    x_offset: my_types::sp_arithmetic::fixed_point::FixedI64,
                    y_offset: my_types::sp_arithmetic::fixed_point::FixedI64,
                },
            }
            #[derive(Clone, Debug)]
            pub struct DecidingStatus<_0> {
                pub since: _0,
                pub confirming: ::core::option::Option<_0>,
            }
            #[derive(Clone, Debug)]
            pub struct Deposit<_0, _1> {
                pub who: _0,
                pub amount: _1,
            }
            #[derive(Clone, Debug)]
            pub enum ReferendumInfo<_0, _1, _2, _3, _4, _5, _6, _7> {
                Ongoing(
                    my_types::pallet_referenda::types::ReferendumStatus<
                        _0,
                        _1,
                        _2,
                        _3,
                        _4,
                        _5,
                        _6,
                        _7,
                    >,
                ),
                Approved(
                    _2,
                    ::core::option::Option<
                        my_types::pallet_referenda::types::Deposit<_6, _4>,
                    >,
                    ::core::option::Option<
                        my_types::pallet_referenda::types::Deposit<_6, _4>,
                    >,
                ),
                Rejected(
                    _2,
                    ::core::option::Option<
                        my_types::pallet_referenda::types::Deposit<_6, _4>,
                    >,
                    ::core::option::Option<
                        my_types::pallet_referenda::types::Deposit<_6, _4>,
                    >,
                ),
                Cancelled(
                    _2,
                    ::core::option::Option<
                        my_types::pallet_referenda::types::Deposit<_6, _4>,
                    >,
                    ::core::option::Option<
                        my_types::pallet_referenda::types::Deposit<_6, _4>,
                    >,
                ),
                TimedOut(
                    _2,
                    ::core::option::Option<
                        my_types::pallet_referenda::types::Deposit<_6, _4>,
                    >,
                    ::core::option::Option<
                        my_types::pallet_referenda::types::Deposit<_6, _4>,
                    >,
                ),
                Killed(_2),
            }
            #[derive(Clone, Debug)]
            pub struct ReferendumStatus<_0, _1, _2, _3, _4, _5, _6, _7> {
                pub track: _0,
                pub origin: _1,
                pub proposal: _3,
                pub enactment: my_types::frame_support::traits::schedule::DispatchTime<
                    _2,
                >,
                pub submitted: _2,
                pub submission_deposit: my_types::pallet_referenda::types::Deposit<
                    _6,
                    _4,
                >,
                pub decision_deposit: ::core::option::Option<
                    my_types::pallet_referenda::types::Deposit<_6, _4>,
                >,
                pub deciding: ::core::option::Option<
                    my_types::pallet_referenda::types::DecidingStatus<_2>,
                >,
                pub tally: _5,
                pub in_queue: ::core::primitive::bool,
                pub alarm: ::core::option::Option<(_2, _7)>,
            }
            #[derive(Clone, Debug)]
            pub struct TrackInfo<_0, _1> {
                pub name: ::std::string::String,
                pub max_deciding: ::core::primitive::u32,
                pub decision_deposit: _0,
                pub prepare_period: _1,
                pub decision_period: _1,
                pub confirm_period: _1,
                pub min_enactment_period: _1,
                pub min_approval: my_types::pallet_referenda::types::Curve,
                pub min_support: my_types::pallet_referenda::types::Curve,
            }
        }
    }
    pub mod pallet_scheduler {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::schedule`].
                schedule {
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<
                        (::core::primitive::u32, ::core::primitive::u32),
                    >,
                    priority: ::core::primitive::u8,
                    call: ::std::boxed::Box<my_types::polkadot_runtime::RuntimeCall>,
                },
                ///See [`Pallet::cancel`].
                cancel { when: ::core::primitive::u32, index: ::core::primitive::u32 },
                ///See [`Pallet::schedule_named`].
                schedule_named {
                    id: [::core::primitive::u8; 32usize],
                    when: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<
                        (::core::primitive::u32, ::core::primitive::u32),
                    >,
                    priority: ::core::primitive::u8,
                    call: ::std::boxed::Box<my_types::polkadot_runtime::RuntimeCall>,
                },
                ///See [`Pallet::cancel_named`].
                cancel_named { id: [::core::primitive::u8; 32usize] },
                ///See [`Pallet::schedule_after`].
                schedule_after {
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<
                        (::core::primitive::u32, ::core::primitive::u32),
                    >,
                    priority: ::core::primitive::u8,
                    call: ::std::boxed::Box<my_types::polkadot_runtime::RuntimeCall>,
                },
                ///See [`Pallet::schedule_named_after`].
                schedule_named_after {
                    id: [::core::primitive::u8; 32usize],
                    after: ::core::primitive::u32,
                    maybe_periodic: ::core::option::Option<
                        (::core::primitive::u32, ::core::primitive::u32),
                    >,
                    priority: ::core::primitive::u8,
                    call: ::std::boxed::Box<my_types::polkadot_runtime::RuntimeCall>,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Failed to schedule a call
                FailedToSchedule,
                ///Cannot find the scheduled call.
                NotFound,
                ///Given target block number is in the past.
                TargetBlockNumberInPast,
                ///Reschedule failed because it does not change scheduled time.
                RescheduleNoChange,
                ///Attempt to use a non-named function on a named task.
                Named,
            }
            #[derive(Clone, Debug)]
            ///Events type.
            pub enum Event {
                ///Scheduled some task.
                Scheduled {
                    when: ::core::primitive::u32,
                    index: ::core::primitive::u32,
                },
                ///Canceled some task.
                Canceled { when: ::core::primitive::u32, index: ::core::primitive::u32 },
                ///Dispatched some task.
                Dispatched {
                    task: (::core::primitive::u32, ::core::primitive::u32),
                    id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    result: ::core::result::Result<
                        (),
                        my_types::sp_runtime::DispatchError,
                    >,
                },
                ///The call for the provided hash was not found so the task has been aborted.
                CallUnavailable {
                    task: (::core::primitive::u32, ::core::primitive::u32),
                    id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                },
                ///The given task was unable to be renewed since the agenda is full at that block.
                PeriodicFailed {
                    task: (::core::primitive::u32, ::core::primitive::u32),
                    id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                },
                ///The given task can never be executed since it is overweight.
                PermanentlyOverweight {
                    task: (::core::primitive::u32, ::core::primitive::u32),
                    id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                },
            }
        }
        #[derive(Clone, Debug)]
        pub struct Scheduled<_0, _1, _2, _3, _4> {
            pub maybe_id: ::core::option::Option<_0>,
            pub priority: ::core::primitive::u8,
            pub call: _1,
            pub maybe_periodic: ::core::option::Option<(_2, _2)>,
            pub origin: _3,
            pub __ignore: ::core::marker::PhantomData<_4>,
        }
    }
    pub mod pallet_session {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::set_keys`].
                set_keys {
                    keys: my_types::polkadot_runtime::SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                },
                ///See [`Pallet::purge_keys`].
                purge_keys,
            }
            #[derive(Clone, Debug)]
            ///Error for the session pallet.
            pub enum Error {
                ///Invalid ownership proof.
                InvalidProof,
                ///No associated validator ID for account.
                NoAssociatedValidatorId,
                ///Registered duplicate key.
                DuplicatedKey,
                ///No keys are associated with this account.
                NoKeys,
                ///Key setting account is not live, so it's impossible to associate keys.
                NoAccount,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///New session has happened. Note that the argument is the session index, not the
                ///block number as the type might suggest.
                NewSession { session_index: ::core::primitive::u32 },
            }
        }
    }
    pub mod pallet_staking {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            pub mod pallet {
                use super::my_types;
                #[derive(Clone, Debug)]
                ///Contains a variant per dispatchable extrinsic that this pallet has.
                pub enum Call {
                    ///See [`Pallet::bond`].
                    bond {
                        value: ::core::primitive::u128,
                        payee: my_types::pallet_staking::RewardDestination<
                            my_types::sp_core::crypto::AccountId32,
                        >,
                    },
                    ///See [`Pallet::bond_extra`].
                    bond_extra { max_additional: ::core::primitive::u128 },
                    ///See [`Pallet::unbond`].
                    unbond { value: ::core::primitive::u128 },
                    ///See [`Pallet::withdraw_unbonded`].
                    withdraw_unbonded { num_slashing_spans: ::core::primitive::u32 },
                    ///See [`Pallet::validate`].
                    validate { prefs: my_types::pallet_staking::ValidatorPrefs },
                    ///See [`Pallet::nominate`].
                    nominate {
                        targets: ::std::vec::Vec<
                            my_types::sp_runtime::multiaddress::MultiAddress<
                                my_types::sp_core::crypto::AccountId32,
                                (),
                            >,
                        >,
                    },
                    ///See [`Pallet::chill`].
                    chill,
                    ///See [`Pallet::set_payee`].
                    set_payee {
                        payee: my_types::pallet_staking::RewardDestination<
                            my_types::sp_core::crypto::AccountId32,
                        >,
                    },
                    ///See [`Pallet::set_controller`].
                    set_controller,
                    ///See [`Pallet::set_validator_count`].
                    set_validator_count { new: ::core::primitive::u32 },
                    ///See [`Pallet::increase_validator_count`].
                    increase_validator_count { additional: ::core::primitive::u32 },
                    ///See [`Pallet::scale_validator_count`].
                    scale_validator_count {
                        factor: my_types::sp_arithmetic::per_things::Percent,
                    },
                    ///See [`Pallet::force_no_eras`].
                    force_no_eras,
                    ///See [`Pallet::force_new_era`].
                    force_new_era,
                    ///See [`Pallet::set_invulnerables`].
                    set_invulnerables {
                        invulnerables: ::std::vec::Vec<
                            my_types::sp_core::crypto::AccountId32,
                        >,
                    },
                    ///See [`Pallet::force_unstake`].
                    force_unstake {
                        stash: my_types::sp_core::crypto::AccountId32,
                        num_slashing_spans: ::core::primitive::u32,
                    },
                    ///See [`Pallet::force_new_era_always`].
                    force_new_era_always,
                    ///See [`Pallet::cancel_deferred_slash`].
                    cancel_deferred_slash {
                        era: ::core::primitive::u32,
                        slash_indices: ::std::vec::Vec<::core::primitive::u32>,
                    },
                    ///See [`Pallet::payout_stakers`].
                    payout_stakers {
                        validator_stash: my_types::sp_core::crypto::AccountId32,
                        era: ::core::primitive::u32,
                    },
                    ///See [`Pallet::rebond`].
                    rebond { value: ::core::primitive::u128 },
                    ///See [`Pallet::reap_stash`].
                    reap_stash {
                        stash: my_types::sp_core::crypto::AccountId32,
                        num_slashing_spans: ::core::primitive::u32,
                    },
                    ///See [`Pallet::kick`].
                    kick {
                        who: ::std::vec::Vec<
                            my_types::sp_runtime::multiaddress::MultiAddress<
                                my_types::sp_core::crypto::AccountId32,
                                (),
                            >,
                        >,
                    },
                    ///See [`Pallet::set_staking_configs`].
                    set_staking_configs {
                        min_nominator_bond: my_types::pallet_staking::pallet::pallet::ConfigOp<
                            ::core::primitive::u128,
                        >,
                        min_validator_bond: my_types::pallet_staking::pallet::pallet::ConfigOp<
                            ::core::primitive::u128,
                        >,
                        max_nominator_count: my_types::pallet_staking::pallet::pallet::ConfigOp<
                            ::core::primitive::u32,
                        >,
                        max_validator_count: my_types::pallet_staking::pallet::pallet::ConfigOp<
                            ::core::primitive::u32,
                        >,
                        chill_threshold: my_types::pallet_staking::pallet::pallet::ConfigOp<
                            my_types::sp_arithmetic::per_things::Percent,
                        >,
                        min_commission: my_types::pallet_staking::pallet::pallet::ConfigOp<
                            my_types::sp_arithmetic::per_things::Perbill,
                        >,
                    },
                    ///See [`Pallet::chill_other`].
                    chill_other { controller: my_types::sp_core::crypto::AccountId32 },
                    ///See [`Pallet::force_apply_min_commission`].
                    force_apply_min_commission {
                        validator_stash: my_types::sp_core::crypto::AccountId32,
                    },
                    ///See [`Pallet::set_min_commission`].
                    set_min_commission {
                        new: my_types::sp_arithmetic::per_things::Perbill,
                    },
                }
                #[derive(Clone, Debug)]
                pub enum ConfigOp<_0> {
                    Noop,
                    Set(_0),
                    Remove,
                }
                #[derive(Clone, Debug)]
                ///The `Error` enum of this pallet.
                pub enum Error {
                    ///Not a controller account.
                    NotController,
                    ///Not a stash account.
                    NotStash,
                    ///Stash is already bonded.
                    AlreadyBonded,
                    ///Controller is already paired.
                    AlreadyPaired,
                    ///Targets cannot be empty.
                    EmptyTargets,
                    ///Duplicate index.
                    DuplicateIndex,
                    ///Slash record index out of bounds.
                    InvalidSlashIndex,
                    ///Cannot have a validator or nominator role, with value less than the minimum defined by
                    ///governance (see `MinValidatorBond` and `MinNominatorBond`). If unbonding is the
                    ///intention, `chill` first to remove one's role as validator/nominator.
                    InsufficientBond,
                    ///Can not schedule more unlock chunks.
                    NoMoreChunks,
                    ///Can not rebond without unlocking chunks.
                    NoUnlockChunk,
                    ///Attempting to target a stash that still has funds.
                    FundedTarget,
                    ///Invalid era to reward.
                    InvalidEraToReward,
                    ///Invalid number of nominations.
                    InvalidNumberOfNominations,
                    ///Items are not sorted and unique.
                    NotSortedAndUnique,
                    ///Rewards for this era have already been claimed for this validator.
                    AlreadyClaimed,
                    ///Incorrect previous history depth input provided.
                    IncorrectHistoryDepth,
                    ///Incorrect number of slashing spans provided.
                    IncorrectSlashingSpans,
                    ///Internal state has become somehow corrupted and the operation cannot continue.
                    BadState,
                    ///Too many nomination targets supplied.
                    TooManyTargets,
                    ///A nomination target was supplied that was blocked or otherwise not a validator.
                    BadTarget,
                    ///The user has enough bond and thus cannot be chilled forcefully by an external person.
                    CannotChillOther,
                    ///There are too many nominators in the system. Governance needs to adjust the staking
                    ///settings to keep things safe for the runtime.
                    TooManyNominators,
                    ///There are too many validator candidates in the system. Governance needs to adjust the
                    ///staking settings to keep things safe for the runtime.
                    TooManyValidators,
                    ///Commission is too low. Must be at least `MinCommission`.
                    CommissionTooLow,
                    ///Some bound is not met.
                    BoundNotMet,
                }
                #[derive(Clone, Debug)]
                ///The `Event` enum of this pallet
                pub enum Event {
                    ///The era payout has been set; the first balance is the validator-payout; the second is
                    ///the remainder from the maximum amount of reward.
                    EraPaid {
                        era_index: ::core::primitive::u32,
                        validator_payout: ::core::primitive::u128,
                        remainder: ::core::primitive::u128,
                    },
                    ///The nominator has been rewarded by this amount.
                    Rewarded {
                        stash: my_types::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    ///A staker (validator or nominator) has been slashed by the given amount.
                    Slashed {
                        staker: my_types::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    ///A slash for the given validator, for the given percentage of their stake, at the given
                    ///era as been reported.
                    SlashReported {
                        validator: my_types::sp_core::crypto::AccountId32,
                        fraction: my_types::sp_arithmetic::per_things::Perbill,
                        slash_era: ::core::primitive::u32,
                    },
                    ///An old slashing report from a prior era was discarded because it could
                    ///not be processed.
                    OldSlashingReportDiscarded { session_index: ::core::primitive::u32 },
                    ///A new set of stakers was elected.
                    StakersElected,
                    ///An account has bonded this amount. \[stash, amount\]
                    ///
                    ///NOTE: This event is only emitted when funds are bonded via a dispatchable. Notably,
                    ///it will not be emitted for staking rewards when they are added to stake.
                    Bonded {
                        stash: my_types::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    ///An account has unbonded this amount.
                    Unbonded {
                        stash: my_types::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    ///An account has called `withdraw_unbonded` and removed unbonding chunks worth `Balance`
                    ///from the unlocking queue.
                    Withdrawn {
                        stash: my_types::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    ///A nominator has been kicked from a validator.
                    Kicked {
                        nominator: my_types::sp_core::crypto::AccountId32,
                        stash: my_types::sp_core::crypto::AccountId32,
                    },
                    ///The election failed. No new era is planned.
                    StakingElectionFailed,
                    ///An account has stopped participating as either a validator or nominator.
                    Chilled { stash: my_types::sp_core::crypto::AccountId32 },
                    ///The stakers' rewards are getting paid.
                    PayoutStarted {
                        era_index: ::core::primitive::u32,
                        validator_stash: my_types::sp_core::crypto::AccountId32,
                    },
                    ///A validator has set their preferences.
                    ValidatorPrefsSet {
                        stash: my_types::sp_core::crypto::AccountId32,
                        prefs: my_types::pallet_staking::ValidatorPrefs,
                    },
                    ///A new force era mode was set.
                    ForceEra { mode: my_types::pallet_staking::Forcing },
                }
            }
        }
        pub mod slashing {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct SlashingSpans {
                pub span_index: ::core::primitive::u32,
                pub last_start: ::core::primitive::u32,
                pub last_nonzero_slash: ::core::primitive::u32,
                pub prior: ::std::vec::Vec<::core::primitive::u32>,
            }
            #[derive(Clone, Debug)]
            pub struct SpanRecord<_0> {
                pub slashed: _0,
                pub paid_out: _0,
            }
        }
        #[derive(Clone, Debug)]
        pub struct ActiveEraInfo {
            pub index: ::core::primitive::u32,
            pub start: ::core::option::Option<::core::primitive::u64>,
        }
        #[derive(Clone, Debug)]
        pub struct EraRewardPoints<_0> {
            pub total: ::core::primitive::u32,
            pub individual: ::std::collections::BTreeMap<_0, ::core::primitive::u32>,
        }
        #[derive(Clone, Debug)]
        pub struct Exposure<_0, _1> {
            pub total: _1,
            pub own: _1,
            pub others: ::std::vec::Vec<
                my_types::pallet_staking::IndividualExposure<_0, _1>,
            >,
        }
        #[derive(Clone, Debug)]
        pub enum Forcing {
            NotForcing,
            ForceNew,
            ForceNone,
            ForceAlways,
        }
        #[derive(Clone, Debug)]
        pub struct IndividualExposure<_0, _1> {
            pub who: _0,
            pub value: _1,
        }
        #[derive(Clone, Debug)]
        pub struct Nominations {
            pub targets: my_types::bounded_collections::bounded_vec::BoundedVec<
                my_types::sp_core::crypto::AccountId32,
            >,
            pub submitted_in: ::core::primitive::u32,
            pub suppressed: ::core::primitive::bool,
        }
        #[derive(Clone, Debug)]
        pub enum RewardDestination<_0> {
            Staked,
            Stash,
            Controller,
            Account(_0),
            None,
        }
        #[derive(Clone, Debug)]
        pub struct StakingLedger {
            pub stash: my_types::sp_core::crypto::AccountId32,
            pub total: ::core::primitive::u128,
            pub active: ::core::primitive::u128,
            pub unlocking: my_types::bounded_collections::bounded_vec::BoundedVec<
                my_types::pallet_staking::UnlockChunk<::core::primitive::u128>,
            >,
            pub claimed_rewards: my_types::bounded_collections::bounded_vec::BoundedVec<
                ::core::primitive::u32,
            >,
        }
        #[derive(Clone, Debug)]
        pub struct UnappliedSlash<_0, _1> {
            pub validator: _0,
            pub own: _1,
            pub others: ::std::vec::Vec<(_0, _1)>,
            pub reporters: ::std::vec::Vec<_0>,
            pub payout: _1,
        }
        #[derive(Clone, Debug)]
        pub struct UnlockChunk<_0> {
            pub value: _0,
            pub era: ::core::primitive::u32,
        }
        #[derive(Clone, Debug)]
        pub struct ValidatorPrefs {
            pub commission: my_types::sp_arithmetic::per_things::Perbill,
            pub blocked: ::core::primitive::bool,
        }
    }
    pub mod pallet_timestamp {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::set`].
                set { now: ::core::primitive::u64 },
            }
        }
    }
    pub mod pallet_tips {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::report_awesome`].
                report_awesome {
                    reason: ::std::vec::Vec<::core::primitive::u8>,
                    who: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::retract_tip`].
                retract_tip { hash: my_types::primitive_types::H256 },
                ///See [`Pallet::tip_new`].
                tip_new {
                    reason: ::std::vec::Vec<::core::primitive::u8>,
                    who: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    tip_value: ::core::primitive::u128,
                },
                ///See [`Pallet::tip`].
                tip {
                    hash: my_types::primitive_types::H256,
                    tip_value: ::core::primitive::u128,
                },
                ///See [`Pallet::close_tip`].
                close_tip { hash: my_types::primitive_types::H256 },
                ///See [`Pallet::slash_tip`].
                slash_tip { hash: my_types::primitive_types::H256 },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///The reason given is just too big.
                ReasonTooBig,
                ///The tip was already found/started.
                AlreadyKnown,
                ///The tip hash is unknown.
                UnknownTip,
                ///The account attempting to retract the tip is not the finder of the tip.
                NotFinder,
                ///The tip cannot be claimed/closed because there are not enough tippers yet.
                StillOpen,
                ///The tip cannot be claimed/closed because it's still in the countdown period.
                Premature,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///A new tip suggestion has been opened.
                NewTip { tip_hash: my_types::primitive_types::H256 },
                ///A tip suggestion has reached threshold and is closing.
                TipClosing { tip_hash: my_types::primitive_types::H256 },
                ///A tip suggestion has been closed.
                TipClosed {
                    tip_hash: my_types::primitive_types::H256,
                    who: my_types::sp_core::crypto::AccountId32,
                    payout: ::core::primitive::u128,
                },
                ///A tip suggestion has been retracted.
                TipRetracted { tip_hash: my_types::primitive_types::H256 },
                ///A tip suggestion has been slashed.
                TipSlashed {
                    tip_hash: my_types::primitive_types::H256,
                    finder: my_types::sp_core::crypto::AccountId32,
                    deposit: ::core::primitive::u128,
                },
            }
        }
        #[derive(Clone, Debug)]
        pub struct OpenTip<_0, _1, _2, _3> {
            pub reason: _3,
            pub who: _0,
            pub finder: _0,
            pub deposit: _1,
            pub closes: ::core::option::Option<_2>,
            pub tips: ::std::vec::Vec<(_0, _1)>,
            pub finders_fee: ::core::primitive::bool,
        }
    }
    pub mod pallet_transaction_payment {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,
                ///has been paid by `who`.
                TransactionFeePaid {
                    who: my_types::sp_core::crypto::AccountId32,
                    actual_fee: ::core::primitive::u128,
                    tip: ::core::primitive::u128,
                },
            }
        }
        pub mod types {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct FeeDetails<_0> {
                pub inclusion_fee: ::core::option::Option<
                    my_types::pallet_transaction_payment::types::InclusionFee<_0>,
                >,
                pub tip: _0,
            }
            #[derive(Clone, Debug)]
            pub struct InclusionFee<_0> {
                pub base_fee: _0,
                pub len_fee: _0,
                pub adjusted_weight_fee: _0,
            }
            #[derive(Clone, Debug)]
            pub struct RuntimeDispatchInfo<_0, _1> {
                pub weight: _1,
                pub class: my_types::frame_support::dispatch::DispatchClass,
                pub partial_fee: _0,
            }
        }
        #[derive(Clone, Debug)]
        pub struct ChargeTransactionPayment(pub ::core::primitive::u128);
        #[derive(Clone, Debug)]
        pub enum Releases {
            V1Ancient,
            V2,
        }
    }
    pub mod pallet_treasury {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::propose_spend`].
                propose_spend {
                    value: ::core::primitive::u128,
                    beneficiary: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::reject_proposal`].
                reject_proposal { proposal_id: ::core::primitive::u32 },
                ///See [`Pallet::approve_proposal`].
                approve_proposal { proposal_id: ::core::primitive::u32 },
                ///See [`Pallet::spend`].
                spend {
                    amount: ::core::primitive::u128,
                    beneficiary: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::remove_approval`].
                remove_approval { proposal_id: ::core::primitive::u32 },
            }
            #[derive(Clone, Debug)]
            ///Error for the treasury pallet.
            pub enum Error {
                ///Proposer's balance is too low.
                InsufficientProposersBalance,
                ///No proposal or bounty at that index.
                InvalidIndex,
                ///Too many approvals in the queue.
                TooManyApprovals,
                ///The spend origin is valid but the amount it is allowed to spend is lower than the
                ///amount to be spent.
                InsufficientPermission,
                ///Proposal has not been approved.
                ProposalNotApproved,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///New proposal.
                Proposed { proposal_index: ::core::primitive::u32 },
                ///We have ended a spend period and will now allocate funds.
                Spending { budget_remaining: ::core::primitive::u128 },
                ///Some funds have been allocated.
                Awarded {
                    proposal_index: ::core::primitive::u32,
                    award: ::core::primitive::u128,
                    account: my_types::sp_core::crypto::AccountId32,
                },
                ///A proposal was rejected; funds were slashed.
                Rejected {
                    proposal_index: ::core::primitive::u32,
                    slashed: ::core::primitive::u128,
                },
                ///Some of our funds have been burnt.
                Burnt { burnt_funds: ::core::primitive::u128 },
                ///Spending has finished; this is the amount that rolls over until next spend.
                Rollover { rollover_balance: ::core::primitive::u128 },
                ///Some funds have been deposited.
                Deposit { value: ::core::primitive::u128 },
                ///A new spend proposal has been approved.
                SpendApproved {
                    proposal_index: ::core::primitive::u32,
                    amount: ::core::primitive::u128,
                    beneficiary: my_types::sp_core::crypto::AccountId32,
                },
                ///The inactive funds of the pallet have been updated.
                UpdatedInactive {
                    reactivated: ::core::primitive::u128,
                    deactivated: ::core::primitive::u128,
                },
            }
        }
        #[derive(Clone, Debug)]
        pub struct Proposal<_0, _1> {
            pub proposer: _0,
            pub value: _1,
            pub beneficiary: _0,
            pub bond: _1,
        }
    }
    pub mod pallet_utility {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::batch`].
                batch {
                    calls: ::std::vec::Vec<my_types::polkadot_runtime::RuntimeCall>,
                },
                ///See [`Pallet::as_derivative`].
                as_derivative {
                    index: ::core::primitive::u16,
                    call: ::std::boxed::Box<my_types::polkadot_runtime::RuntimeCall>,
                },
                ///See [`Pallet::batch_all`].
                batch_all {
                    calls: ::std::vec::Vec<my_types::polkadot_runtime::RuntimeCall>,
                },
                ///See [`Pallet::dispatch_as`].
                dispatch_as {
                    as_origin: ::std::boxed::Box<
                        my_types::polkadot_runtime::OriginCaller,
                    >,
                    call: ::std::boxed::Box<my_types::polkadot_runtime::RuntimeCall>,
                },
                ///See [`Pallet::force_batch`].
                force_batch {
                    calls: ::std::vec::Vec<my_types::polkadot_runtime::RuntimeCall>,
                },
                ///See [`Pallet::with_weight`].
                with_weight {
                    call: ::std::boxed::Box<my_types::polkadot_runtime::RuntimeCall>,
                    weight: my_types::sp_weights::weight_v2::Weight,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///Too many calls batched.
                TooManyCalls,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///Batch of dispatches did not complete fully. Index of first failing dispatch given, as
                ///well as the error.
                BatchInterrupted {
                    index: ::core::primitive::u32,
                    error: my_types::sp_runtime::DispatchError,
                },
                ///Batch of dispatches completed fully with no error.
                BatchCompleted,
                ///Batch of dispatches completed but has errors.
                BatchCompletedWithErrors,
                ///A single item within a Batch of dispatches has completed with no error.
                ItemCompleted,
                ///A single item within a Batch of dispatches has completed with error.
                ItemFailed { error: my_types::sp_runtime::DispatchError },
                ///A call was dispatched.
                DispatchedAs {
                    result: ::core::result::Result<
                        (),
                        my_types::sp_runtime::DispatchError,
                    >,
                },
            }
        }
    }
    pub mod pallet_vesting {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::vest`].
                vest,
                ///See [`Pallet::vest_other`].
                vest_other {
                    target: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                },
                ///See [`Pallet::vested_transfer`].
                vested_transfer {
                    target: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    schedule: my_types::pallet_vesting::vesting_info::VestingInfo<
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                },
                ///See [`Pallet::force_vested_transfer`].
                force_vested_transfer {
                    source: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    target: my_types::sp_runtime::multiaddress::MultiAddress<
                        my_types::sp_core::crypto::AccountId32,
                        (),
                    >,
                    schedule: my_types::pallet_vesting::vesting_info::VestingInfo<
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                },
                ///See [`Pallet::merge_schedules`].
                merge_schedules {
                    schedule1_index: ::core::primitive::u32,
                    schedule2_index: ::core::primitive::u32,
                },
            }
            #[derive(Clone, Debug)]
            ///Error for the vesting pallet.
            pub enum Error {
                ///The account given is not vesting.
                NotVesting,
                ///The account already has `MaxVestingSchedules` count of schedules and thus
                ///cannot add another one. Consider merging existing schedules in order to add another.
                AtMaxVestingSchedules,
                ///Amount being transferred is too low to create a vesting schedule.
                AmountLow,
                ///An index was out of bounds of the vesting schedules.
                ScheduleIndexOutOfBounds,
                ///Failed to create a new schedule because some parameter was invalid.
                InvalidScheduleParams,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///The amount vested has been updated. This could indicate a change in funds available.
                ///The balance given is the amount which is left unvested (and thus locked).
                VestingUpdated {
                    account: my_types::sp_core::crypto::AccountId32,
                    unvested: ::core::primitive::u128,
                },
                ///An \[account\] has become fully vested.
                VestingCompleted { account: my_types::sp_core::crypto::AccountId32 },
            }
        }
        pub mod vesting_info {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct VestingInfo<_0, _1> {
                pub locked: _0,
                pub per_block: _0,
                pub starting_block: _1,
            }
        }
        #[derive(Clone, Debug)]
        pub enum Releases {
            V0,
            V1,
        }
    }
    pub mod pallet_whitelist {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::whitelist_call`].
                whitelist_call { call_hash: my_types::primitive_types::H256 },
                ///See [`Pallet::remove_whitelisted_call`].
                remove_whitelisted_call { call_hash: my_types::primitive_types::H256 },
                ///See [`Pallet::dispatch_whitelisted_call`].
                dispatch_whitelisted_call {
                    call_hash: my_types::primitive_types::H256,
                    call_encoded_len: ::core::primitive::u32,
                    call_weight_witness: my_types::sp_weights::weight_v2::Weight,
                },
                ///See [`Pallet::dispatch_whitelisted_call_with_preimage`].
                dispatch_whitelisted_call_with_preimage {
                    call: ::std::boxed::Box<my_types::polkadot_runtime::RuntimeCall>,
                },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///The preimage of the call hash could not be loaded.
                UnavailablePreImage,
                ///The call could not be decoded.
                UndecodableCall,
                ///The weight of the decoded call was higher than the witness.
                InvalidCallWeightWitness,
                ///The call was not whitelisted.
                CallIsNotWhitelisted,
                ///The call was already whitelisted; No-Op.
                CallAlreadyWhitelisted,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                CallWhitelisted { call_hash: my_types::primitive_types::H256 },
                WhitelistedCallRemoved { call_hash: my_types::primitive_types::H256 },
                WhitelistedCallDispatched {
                    call_hash: my_types::primitive_types::H256,
                    result: ::core::result::Result<
                        my_types::frame_support::dispatch::PostDispatchInfo,
                        my_types::sp_runtime::DispatchErrorWithPostInfo<
                            my_types::frame_support::dispatch::PostDispatchInfo,
                        >,
                    >,
                },
            }
        }
    }
    pub mod pallet_xcm {
        use super::my_types;
        pub mod pallet {
            use super::my_types;
            #[derive(Clone, Debug)]
            ///Contains a variant per dispatchable extrinsic that this pallet has.
            pub enum Call {
                ///See [`Pallet::send`].
                send {
                    dest: ::std::boxed::Box<my_types::xcm::VersionedMultiLocation>,
                    message: ::std::boxed::Box<my_types::xcm::VersionedXcm>,
                },
                ///See [`Pallet::teleport_assets`].
                teleport_assets {
                    dest: ::std::boxed::Box<my_types::xcm::VersionedMultiLocation>,
                    beneficiary: ::std::boxed::Box<
                        my_types::xcm::VersionedMultiLocation,
                    >,
                    assets: ::std::boxed::Box<my_types::xcm::VersionedMultiAssets>,
                    fee_asset_item: ::core::primitive::u32,
                },
                ///See [`Pallet::reserve_transfer_assets`].
                reserve_transfer_assets {
                    dest: ::std::boxed::Box<my_types::xcm::VersionedMultiLocation>,
                    beneficiary: ::std::boxed::Box<
                        my_types::xcm::VersionedMultiLocation,
                    >,
                    assets: ::std::boxed::Box<my_types::xcm::VersionedMultiAssets>,
                    fee_asset_item: ::core::primitive::u32,
                },
                ///See [`Pallet::execute`].
                execute {
                    message: ::std::boxed::Box<my_types::xcm::VersionedXcm>,
                    max_weight: my_types::sp_weights::weight_v2::Weight,
                },
                ///See [`Pallet::force_xcm_version`].
                force_xcm_version {
                    location: ::std::boxed::Box<
                        my_types::xcm::v3::multilocation::MultiLocation,
                    >,
                    version: ::core::primitive::u32,
                },
                ///See [`Pallet::force_default_xcm_version`].
                force_default_xcm_version {
                    maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
                },
                ///See [`Pallet::force_subscribe_version_notify`].
                force_subscribe_version_notify {
                    location: ::std::boxed::Box<my_types::xcm::VersionedMultiLocation>,
                },
                ///See [`Pallet::force_unsubscribe_version_notify`].
                force_unsubscribe_version_notify {
                    location: ::std::boxed::Box<my_types::xcm::VersionedMultiLocation>,
                },
                ///See [`Pallet::limited_reserve_transfer_assets`].
                limited_reserve_transfer_assets {
                    dest: ::std::boxed::Box<my_types::xcm::VersionedMultiLocation>,
                    beneficiary: ::std::boxed::Box<
                        my_types::xcm::VersionedMultiLocation,
                    >,
                    assets: ::std::boxed::Box<my_types::xcm::VersionedMultiAssets>,
                    fee_asset_item: ::core::primitive::u32,
                    weight_limit: my_types::xcm::v3::WeightLimit,
                },
                ///See [`Pallet::limited_teleport_assets`].
                limited_teleport_assets {
                    dest: ::std::boxed::Box<my_types::xcm::VersionedMultiLocation>,
                    beneficiary: ::std::boxed::Box<
                        my_types::xcm::VersionedMultiLocation,
                    >,
                    assets: ::std::boxed::Box<my_types::xcm::VersionedMultiAssets>,
                    fee_asset_item: ::core::primitive::u32,
                    weight_limit: my_types::xcm::v3::WeightLimit,
                },
                ///See [`Pallet::force_suspension`].
                force_suspension { suspended: ::core::primitive::bool },
            }
            #[derive(Clone, Debug)]
            ///The `Error` enum of this pallet.
            pub enum Error {
                ///The desired destination was unreachable, generally because there is a no way of routing
                ///to it.
                Unreachable,
                ///There was some other issue (i.e. not to do with routing) in sending the message. Perhaps
                ///a lack of space for buffering the message.
                SendFailure,
                ///The message execution fails the filter.
                Filtered,
                ///The message's weight could not be determined.
                UnweighableMessage,
                ///The destination `MultiLocation` provided cannot be inverted.
                DestinationNotInvertible,
                ///The assets to be sent are empty.
                Empty,
                ///Could not re-anchor the assets to declare the fees for the destination chain.
                CannotReanchor,
                ///Too many assets have been attempted for transfer.
                TooManyAssets,
                ///Origin is invalid for sending.
                InvalidOrigin,
                ///The version of the `Versioned` value used is not able to be interpreted.
                BadVersion,
                ///The given location could not be used (e.g. because it cannot be expressed in the
                ///desired version of XCM).
                BadLocation,
                ///The referenced subscription could not be found.
                NoSubscription,
                ///The location is invalid since it already has a subscription from us.
                AlreadySubscribed,
                ///Invalid asset for the operation.
                InvalidAsset,
                ///The owner does not own (all) of the asset that they wish to do the operation on.
                LowBalance,
                ///The asset owner has too many locks on the asset.
                TooManyLocks,
                ///The given account is not an identifiable sovereign account for any location.
                AccountNotSovereign,
                ///The operation required fees to be paid which the initiator could not meet.
                FeesNotMet,
                ///A remote lock with the corresponding data could not be found.
                LockNotFound,
                ///The unlock operation cannot succeed because there are still consumers of the lock.
                InUse,
            }
            #[derive(Clone, Debug)]
            ///The `Event` enum of this pallet
            pub enum Event {
                ///Execution of an XCM message was attempted.
                Attempted { outcome: my_types::xcm::v3::traits::Outcome },
                ///A XCM message was sent.
                Sent {
                    origin: my_types::xcm::v3::multilocation::MultiLocation,
                    destination: my_types::xcm::v3::multilocation::MultiLocation,
                    message: my_types::xcm::v3::Xcm,
                    message_id: [::core::primitive::u8; 32usize],
                },
                ///Query response received which does not match a registered query. This may be because a
                ///matching query was never registered, it may be because it is a duplicate response, or
                ///because the query timed out.
                UnexpectedResponse {
                    origin: my_types::xcm::v3::multilocation::MultiLocation,
                    query_id: ::core::primitive::u64,
                },
                ///Query response has been received and is ready for taking with `take_response`. There is
                ///no registered notification call.
                ResponseReady {
                    query_id: ::core::primitive::u64,
                    response: my_types::xcm::v3::Response,
                },
                ///Query response has been received and query is removed. The registered notification has
                ///been dispatched and executed successfully.
                Notified {
                    query_id: ::core::primitive::u64,
                    pallet_index: ::core::primitive::u8,
                    call_index: ::core::primitive::u8,
                },
                ///Query response has been received and query is removed. The registered notification could
                ///not be dispatched because the dispatch weight is greater than the maximum weight
                ///originally budgeted by this runtime for the query result.
                NotifyOverweight {
                    query_id: ::core::primitive::u64,
                    pallet_index: ::core::primitive::u8,
                    call_index: ::core::primitive::u8,
                    actual_weight: my_types::sp_weights::weight_v2::Weight,
                    max_budgeted_weight: my_types::sp_weights::weight_v2::Weight,
                },
                ///Query response has been received and query is removed. There was a general error with
                ///dispatching the notification call.
                NotifyDispatchError {
                    query_id: ::core::primitive::u64,
                    pallet_index: ::core::primitive::u8,
                    call_index: ::core::primitive::u8,
                },
                ///Query response has been received and query is removed. The dispatch was unable to be
                ///decoded into a `Call`; this might be due to dispatch function having a signature which
                ///is not `(origin, QueryId, Response)`.
                NotifyDecodeFailed {
                    query_id: ::core::primitive::u64,
                    pallet_index: ::core::primitive::u8,
                    call_index: ::core::primitive::u8,
                },
                ///Expected query response has been received but the origin location of the response does
                ///not match that expected. The query remains registered for a later, valid, response to
                ///be received and acted upon.
                InvalidResponder {
                    origin: my_types::xcm::v3::multilocation::MultiLocation,
                    query_id: ::core::primitive::u64,
                    expected_location: ::core::option::Option<
                        my_types::xcm::v3::multilocation::MultiLocation,
                    >,
                },
                ///Expected query response has been received but the expected origin location placed in
                ///storage by this runtime previously cannot be decoded. The query remains registered.
                ///
                ///This is unexpected (since a location placed in storage in a previously executing
                ///runtime should be readable prior to query timeout) and dangerous since the possibly
                ///valid response will be dropped. Manual governance intervention is probably going to be
                ///needed.
                InvalidResponderVersion {
                    origin: my_types::xcm::v3::multilocation::MultiLocation,
                    query_id: ::core::primitive::u64,
                },
                ///Received query response has been read and removed.
                ResponseTaken { query_id: ::core::primitive::u64 },
                ///Some assets have been placed in an asset trap.
                AssetsTrapped {
                    hash: my_types::primitive_types::H256,
                    origin: my_types::xcm::v3::multilocation::MultiLocation,
                    assets: my_types::xcm::VersionedMultiAssets,
                },
                ///An XCM version change notification message has been attempted to be sent.
                ///
                ///The cost of sending it (borne by the chain) is included.
                VersionChangeNotified {
                    destination: my_types::xcm::v3::multilocation::MultiLocation,
                    result: ::core::primitive::u32,
                    cost: my_types::xcm::v3::multiasset::MultiAssets,
                    message_id: [::core::primitive::u8; 32usize],
                },
                ///The supported version of a location has been changed. This might be through an
                ///automatic notification or a manual intervention.
                SupportedVersionChanged {
                    location: my_types::xcm::v3::multilocation::MultiLocation,
                    version: ::core::primitive::u32,
                },
                ///A given location which had a version change subscription was dropped owing to an error
                ///sending the notification to it.
                NotifyTargetSendFail {
                    location: my_types::xcm::v3::multilocation::MultiLocation,
                    query_id: ::core::primitive::u64,
                    error: my_types::xcm::v3::traits::Error,
                },
                ///A given location which had a version change subscription was dropped owing to an error
                ///migrating the location to our new XCM format.
                NotifyTargetMigrationFail {
                    location: my_types::xcm::VersionedMultiLocation,
                    query_id: ::core::primitive::u64,
                },
                ///Expected query response has been received but the expected querier location placed in
                ///storage by this runtime previously cannot be decoded. The query remains registered.
                ///
                ///This is unexpected (since a location placed in storage in a previously executing
                ///runtime should be readable prior to query timeout) and dangerous since the possibly
                ///valid response will be dropped. Manual governance intervention is probably going to be
                ///needed.
                InvalidQuerierVersion {
                    origin: my_types::xcm::v3::multilocation::MultiLocation,
                    query_id: ::core::primitive::u64,
                },
                ///Expected query response has been received but the querier location of the response does
                ///not match the expected. The query remains registered for a later, valid, response to
                ///be received and acted upon.
                InvalidQuerier {
                    origin: my_types::xcm::v3::multilocation::MultiLocation,
                    query_id: ::core::primitive::u64,
                    expected_querier: my_types::xcm::v3::multilocation::MultiLocation,
                    maybe_actual_querier: ::core::option::Option<
                        my_types::xcm::v3::multilocation::MultiLocation,
                    >,
                },
                ///A remote has requested XCM version change notification from us and we have honored it.
                ///A version information message is sent to them and its cost is included.
                VersionNotifyStarted {
                    destination: my_types::xcm::v3::multilocation::MultiLocation,
                    cost: my_types::xcm::v3::multiasset::MultiAssets,
                    message_id: [::core::primitive::u8; 32usize],
                },
                ///We have requested that a remote chain send us XCM version change notifications.
                VersionNotifyRequested {
                    destination: my_types::xcm::v3::multilocation::MultiLocation,
                    cost: my_types::xcm::v3::multiasset::MultiAssets,
                    message_id: [::core::primitive::u8; 32usize],
                },
                ///We have requested that a remote chain stops sending us XCM version change notifications.
                VersionNotifyUnrequested {
                    destination: my_types::xcm::v3::multilocation::MultiLocation,
                    cost: my_types::xcm::v3::multiasset::MultiAssets,
                    message_id: [::core::primitive::u8; 32usize],
                },
                ///Fees were paid from a location for an operation (often for using `SendXcm`).
                FeesPaid {
                    paying: my_types::xcm::v3::multilocation::MultiLocation,
                    fees: my_types::xcm::v3::multiasset::MultiAssets,
                },
                ///Some assets have been claimed from an asset trap
                AssetsClaimed {
                    hash: my_types::primitive_types::H256,
                    origin: my_types::xcm::v3::multilocation::MultiLocation,
                    assets: my_types::xcm::VersionedMultiAssets,
                },
            }
            #[derive(Clone, Debug)]
            pub enum Origin {
                Xcm(my_types::xcm::v3::multilocation::MultiLocation),
                Response(my_types::xcm::v3::multilocation::MultiLocation),
            }
            #[derive(Clone, Debug)]
            pub enum QueryStatus<_0> {
                Pending {
                    responder: my_types::xcm::VersionedMultiLocation,
                    maybe_match_querier: ::core::option::Option<
                        my_types::xcm::VersionedMultiLocation,
                    >,
                    maybe_notify: ::core::option::Option<
                        (::core::primitive::u8, ::core::primitive::u8),
                    >,
                    timeout: _0,
                },
                VersionNotifier {
                    origin: my_types::xcm::VersionedMultiLocation,
                    is_active: ::core::primitive::bool,
                },
                Ready { response: my_types::xcm::VersionedResponse, at: _0 },
            }
            #[derive(Clone, Debug)]
            pub struct RemoteLockedFungibleRecord<_0> {
                pub amount: ::core::primitive::u128,
                pub owner: my_types::xcm::VersionedMultiLocation,
                pub locker: my_types::xcm::VersionedMultiLocation,
                pub consumers: my_types::bounded_collections::bounded_vec::BoundedVec<
                    (_0, ::core::primitive::u128),
                >,
            }
            #[derive(Clone, Debug)]
            pub enum VersionMigrationStage {
                MigrateSupportedVersion,
                MigrateVersionNotifiers,
                NotifyCurrentTargets(
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                ),
                MigrateAndNotifyOldTargets,
            }
        }
    }
    pub mod polkadot_core_primitives {
        use super::my_types;
        #[derive(Clone, Debug)]
        pub struct CandidateHash(pub my_types::primitive_types::H256);
        #[derive(Clone, Debug)]
        pub struct InboundDownwardMessage<_0> {
            pub sent_at: _0,
            pub msg: ::std::vec::Vec<::core::primitive::u8>,
        }
        #[derive(Clone, Debug)]
        pub struct InboundHrmpMessage<_0> {
            pub sent_at: _0,
            pub data: ::std::vec::Vec<::core::primitive::u8>,
        }
        #[derive(Clone, Debug)]
        pub struct OutboundHrmpMessage<_0> {
            pub recipient: _0,
            pub data: ::std::vec::Vec<::core::primitive::u8>,
        }
    }
    pub mod polkadot_parachain {
        use super::my_types;
        pub mod primitives {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(Clone, Debug)]
            pub struct HrmpChannelId {
                pub sender: my_types::polkadot_parachain::primitives::Id,
                pub recipient: my_types::polkadot_parachain::primitives::Id,
            }
            #[derive(Clone, Debug, parity_scale_codec::CompactAs)]
            pub struct Id(pub ::core::primitive::u32);
            #[derive(Clone, Debug)]
            pub struct ValidationCode(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(Clone, Debug)]
            pub struct ValidationCodeHash(pub my_types::primitive_types::H256);
        }
    }
    pub mod polkadot_primitives {
        use super::my_types;
        pub mod v5 {
            use super::my_types;
            pub mod assignment_app {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct Public(pub my_types::sp_core::sr25519::Public);
            }
            pub mod collator_app {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct Public(pub my_types::sp_core::sr25519::Public);
                #[derive(Clone, Debug)]
                pub struct Signature(pub my_types::sp_core::sr25519::Signature);
            }
            pub mod executor_params {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub enum ExecutorParam {
                    MaxMemoryPages(::core::primitive::u32),
                    StackLogicalMax(::core::primitive::u32),
                    StackNativeMax(::core::primitive::u32),
                    PrecheckingMaxMemory(::core::primitive::u64),
                    PvfPrepTimeout(
                        my_types::polkadot_primitives::v5::PvfPrepTimeoutKind,
                        ::core::primitive::u64,
                    ),
                    PvfExecTimeout(
                        my_types::polkadot_primitives::v5::PvfExecTimeoutKind,
                        ::core::primitive::u64,
                    ),
                    WasmExtBulkMemory,
                }
                #[derive(Clone, Debug)]
                pub struct ExecutorParams(
                    pub ::std::vec::Vec<
                        my_types::polkadot_primitives::v5::executor_params::ExecutorParam,
                    >,
                );
            }
            pub mod signed {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct UncheckedSigned<_0, _1> {
                    pub payload: _0,
                    pub validator_index: my_types::polkadot_primitives::v5::ValidatorIndex,
                    pub signature: my_types::polkadot_primitives::v5::validator_app::Signature,
                    pub __ignore: ::core::marker::PhantomData<_1>,
                }
            }
            pub mod slashing {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct DisputeProof {
                    pub time_slot: my_types::polkadot_primitives::v5::slashing::DisputesTimeSlot,
                    pub kind: my_types::polkadot_primitives::v5::slashing::SlashingOffenceKind,
                    pub validator_index: my_types::polkadot_primitives::v5::ValidatorIndex,
                    pub validator_id: my_types::polkadot_primitives::v5::validator_app::Public,
                }
                #[derive(Clone, Debug)]
                pub struct DisputesTimeSlot {
                    pub session_index: ::core::primitive::u32,
                    pub candidate_hash: my_types::polkadot_core_primitives::CandidateHash,
                }
                #[derive(Clone, Debug)]
                pub struct OpaqueKeyOwnershipProof(
                    pub ::std::vec::Vec<::core::primitive::u8>,
                );
                #[derive(Clone, Debug)]
                pub struct PendingSlashes {
                    pub keys: ::std::collections::BTreeMap<
                        my_types::polkadot_primitives::v5::ValidatorIndex,
                        my_types::polkadot_primitives::v5::validator_app::Public,
                    >,
                    pub kind: my_types::polkadot_primitives::v5::slashing::SlashingOffenceKind,
                }
                #[derive(Clone, Debug)]
                pub enum SlashingOffenceKind {
                    ForInvalid,
                    AgainstValid,
                }
            }
            pub mod validator_app {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct Public(pub my_types::sp_core::sr25519::Public);
                #[derive(Clone, Debug)]
                pub struct Signature(pub my_types::sp_core::sr25519::Signature);
            }
            #[derive(Clone, Debug)]
            pub struct AvailabilityBitfield(
                pub DecodedBits<::core::primitive::u8, my_types::bitvec::order::Lsb0>,
            );
            #[derive(Clone, Debug)]
            pub struct BackedCandidate<_0> {
                pub candidate: my_types::polkadot_primitives::v5::CommittedCandidateReceipt<
                    _0,
                >,
                pub validity_votes: ::std::vec::Vec<
                    my_types::polkadot_primitives::v5::ValidityAttestation,
                >,
                pub validator_indices: DecodedBits<
                    ::core::primitive::u8,
                    my_types::bitvec::order::Lsb0,
                >,
            }
            #[derive(Clone, Debug)]
            pub struct CandidateCommitments<_0> {
                pub upward_messages: my_types::bounded_collections::bounded_vec::BoundedVec<
                    ::std::vec::Vec<::core::primitive::u8>,
                >,
                pub horizontal_messages: my_types::bounded_collections::bounded_vec::BoundedVec<
                    my_types::polkadot_core_primitives::OutboundHrmpMessage<
                        my_types::polkadot_parachain::primitives::Id,
                    >,
                >,
                pub new_validation_code: ::core::option::Option<
                    my_types::polkadot_parachain::primitives::ValidationCode,
                >,
                pub head_data: my_types::polkadot_parachain::primitives::HeadData,
                pub processed_downward_messages: ::core::primitive::u32,
                pub hrmp_watermark: _0,
            }
            #[derive(Clone, Debug)]
            pub struct CandidateDescriptor<_0> {
                pub para_id: my_types::polkadot_parachain::primitives::Id,
                pub relay_parent: _0,
                pub collator: my_types::polkadot_primitives::v5::collator_app::Public,
                pub persisted_validation_data_hash: my_types::primitive_types::H256,
                pub pov_hash: my_types::primitive_types::H256,
                pub erasure_root: my_types::primitive_types::H256,
                pub signature: my_types::polkadot_primitives::v5::collator_app::Signature,
                pub para_head: my_types::primitive_types::H256,
                pub validation_code_hash: my_types::polkadot_parachain::primitives::ValidationCodeHash,
            }
            #[derive(Clone, Debug)]
            pub enum CandidateEvent<_0> {
                CandidateBacked(
                    my_types::polkadot_primitives::v5::CandidateReceipt<_0>,
                    my_types::polkadot_parachain::primitives::HeadData,
                    my_types::polkadot_primitives::v5::CoreIndex,
                    my_types::polkadot_primitives::v5::GroupIndex,
                ),
                CandidateIncluded(
                    my_types::polkadot_primitives::v5::CandidateReceipt<_0>,
                    my_types::polkadot_parachain::primitives::HeadData,
                    my_types::polkadot_primitives::v5::CoreIndex,
                    my_types::polkadot_primitives::v5::GroupIndex,
                ),
                CandidateTimedOut(
                    my_types::polkadot_primitives::v5::CandidateReceipt<_0>,
                    my_types::polkadot_parachain::primitives::HeadData,
                    my_types::polkadot_primitives::v5::CoreIndex,
                ),
            }
            #[derive(Clone, Debug)]
            pub struct CandidateReceipt<_0> {
                pub descriptor: my_types::polkadot_primitives::v5::CandidateDescriptor<
                    _0,
                >,
                pub commitments_hash: my_types::primitive_types::H256,
            }
            #[derive(Clone, Debug)]
            pub struct CommittedCandidateReceipt<_0> {
                pub descriptor: my_types::polkadot_primitives::v5::CandidateDescriptor<
                    _0,
                >,
                pub commitments: my_types::polkadot_primitives::v5::CandidateCommitments<
                    ::core::primitive::u32,
                >,
            }
            #[derive(Clone, Debug, parity_scale_codec::CompactAs)]
            pub struct CoreIndex(pub ::core::primitive::u32);
            #[derive(Clone, Debug)]
            pub enum CoreOccupied {
                Parathread(my_types::polkadot_primitives::v5::ParathreadEntry),
                Parachain,
            }
            #[derive(Clone, Debug)]
            pub enum CoreState<_0, _1> {
                Occupied(my_types::polkadot_primitives::v5::OccupiedCore<_0, _1>),
                Scheduled(my_types::polkadot_primitives::v5::ScheduledCore),
                Free,
            }
            #[derive(Clone, Debug)]
            pub struct DisputeState<_0> {
                pub validators_for: DecodedBits<
                    ::core::primitive::u8,
                    my_types::bitvec::order::Lsb0,
                >,
                pub validators_against: DecodedBits<
                    ::core::primitive::u8,
                    my_types::bitvec::order::Lsb0,
                >,
                pub start: _0,
                pub concluded_at: ::core::option::Option<_0>,
            }
            #[derive(Clone, Debug)]
            pub enum DisputeStatement {
                Valid(my_types::polkadot_primitives::v5::ValidDisputeStatementKind),
                Invalid(my_types::polkadot_primitives::v5::InvalidDisputeStatementKind),
            }
            #[derive(Clone, Debug)]
            pub struct DisputeStatementSet {
                pub candidate_hash: my_types::polkadot_core_primitives::CandidateHash,
                pub session: ::core::primitive::u32,
                pub statements: ::std::vec::Vec<
                    (
                        my_types::polkadot_primitives::v5::DisputeStatement,
                        my_types::polkadot_primitives::v5::ValidatorIndex,
                        my_types::polkadot_primitives::v5::validator_app::Signature,
                    ),
                >,
            }
            #[derive(Clone, Debug, parity_scale_codec::CompactAs)]
            pub struct GroupIndex(pub ::core::primitive::u32);
            #[derive(Clone, Debug)]
            pub struct GroupRotationInfo<_0> {
                pub session_start_block: _0,
                pub group_rotation_frequency: _0,
                pub now: _0,
            }
            #[derive(Clone, Debug)]
            pub struct IndexedVec<_0, _1>(
                pub ::std::vec::Vec<_1>,
                pub ::core::marker::PhantomData<_0>,
            );
            #[derive(Clone, Debug)]
            pub struct InherentData<_0> {
                pub bitfields: ::std::vec::Vec<
                    my_types::polkadot_primitives::v5::signed::UncheckedSigned<
                        my_types::polkadot_primitives::v5::AvailabilityBitfield,
                        my_types::polkadot_primitives::v5::AvailabilityBitfield,
                    >,
                >,
                pub backed_candidates: ::std::vec::Vec<
                    my_types::polkadot_primitives::v5::BackedCandidate<
                        my_types::primitive_types::H256,
                    >,
                >,
                pub disputes: ::std::vec::Vec<
                    my_types::polkadot_primitives::v5::DisputeStatementSet,
                >,
                pub parent_header: _0,
            }
            #[derive(Clone, Debug)]
            pub enum InvalidDisputeStatementKind {
                Explicit,
            }
            #[derive(Clone, Debug)]
            pub struct OccupiedCore<_0, _1> {
                pub next_up_on_available: ::core::option::Option<
                    my_types::polkadot_primitives::v5::ScheduledCore,
                >,
                pub occupied_since: _1,
                pub time_out_at: _1,
                pub next_up_on_time_out: ::core::option::Option<
                    my_types::polkadot_primitives::v5::ScheduledCore,
                >,
                pub availability: DecodedBits<
                    ::core::primitive::u8,
                    my_types::bitvec::order::Lsb0,
                >,
                pub group_responsible: my_types::polkadot_primitives::v5::GroupIndex,
                pub candidate_hash: my_types::polkadot_core_primitives::CandidateHash,
                pub candidate_descriptor: my_types::polkadot_primitives::v5::CandidateDescriptor<
                    _0,
                >,
            }
            #[derive(Clone, Debug)]
            pub enum OccupiedCoreAssumption {
                Included,
                TimedOut,
                Free,
            }
            #[derive(Clone, Debug)]
            pub struct ParathreadClaim(
                pub my_types::polkadot_parachain::primitives::Id,
                pub my_types::polkadot_primitives::v5::collator_app::Public,
            );
            #[derive(Clone, Debug)]
            pub struct ParathreadEntry {
                pub claim: my_types::polkadot_primitives::v5::ParathreadClaim,
                pub retries: ::core::primitive::u32,
            }
            #[derive(Clone, Debug)]
            pub struct PersistedValidationData<_0, _1> {
                pub parent_head: my_types::polkadot_parachain::primitives::HeadData,
                pub relay_parent_number: _1,
                pub relay_parent_storage_root: _0,
                pub max_pov_size: ::core::primitive::u32,
            }
            #[derive(Clone, Debug)]
            pub struct PvfCheckStatement {
                pub accept: ::core::primitive::bool,
                pub subject: my_types::polkadot_parachain::primitives::ValidationCodeHash,
                pub session_index: ::core::primitive::u32,
                pub validator_index: my_types::polkadot_primitives::v5::ValidatorIndex,
            }
            #[derive(Clone, Debug)]
            pub enum PvfExecTimeoutKind {
                Backing,
                Approval,
            }
            #[derive(Clone, Debug)]
            pub enum PvfPrepTimeoutKind {
                Precheck,
                Lenient,
            }
            #[derive(Clone, Debug)]
            pub struct ScheduledCore {
                pub para_id: my_types::polkadot_parachain::primitives::Id,
                pub collator: ::core::option::Option<
                    my_types::polkadot_primitives::v5::collator_app::Public,
                >,
            }
            #[derive(Clone, Debug)]
            pub struct ScrapedOnChainVotes<_0> {
                pub session: ::core::primitive::u32,
                pub backing_validators_per_candidate: ::std::vec::Vec<
                    (
                        my_types::polkadot_primitives::v5::CandidateReceipt<_0>,
                        ::std::vec::Vec<
                            (
                                my_types::polkadot_primitives::v5::ValidatorIndex,
                                my_types::polkadot_primitives::v5::ValidityAttestation,
                            ),
                        >,
                    ),
                >,
                pub disputes: ::std::vec::Vec<
                    my_types::polkadot_primitives::v5::DisputeStatementSet,
                >,
            }
            #[derive(Clone, Debug)]
            pub struct SessionInfo {
                pub active_validator_indices: ::std::vec::Vec<
                    my_types::polkadot_primitives::v5::ValidatorIndex,
                >,
                pub random_seed: [::core::primitive::u8; 32usize],
                pub dispute_period: ::core::primitive::u32,
                pub validators: my_types::polkadot_primitives::v5::IndexedVec<
                    my_types::polkadot_primitives::v5::ValidatorIndex,
                    my_types::polkadot_primitives::v5::validator_app::Public,
                >,
                pub discovery_keys: ::std::vec::Vec<
                    my_types::sp_authority_discovery::app::Public,
                >,
                pub assignment_keys: ::std::vec::Vec<
                    my_types::polkadot_primitives::v5::assignment_app::Public,
                >,
                pub validator_groups: my_types::polkadot_primitives::v5::IndexedVec<
                    my_types::polkadot_primitives::v5::GroupIndex,
                    ::std::vec::Vec<my_types::polkadot_primitives::v5::ValidatorIndex>,
                >,
                pub n_cores: ::core::primitive::u32,
                pub zeroth_delay_tranche_width: ::core::primitive::u32,
                pub relay_vrf_modulo_samples: ::core::primitive::u32,
                pub n_delay_tranches: ::core::primitive::u32,
                pub no_show_slots: ::core::primitive::u32,
                pub needed_approvals: ::core::primitive::u32,
            }
            #[derive(Clone, Debug)]
            pub enum UpgradeGoAhead {
                Abort,
                GoAhead,
            }
            #[derive(Clone, Debug)]
            pub enum UpgradeRestriction {
                Present,
            }
            #[derive(Clone, Debug)]
            pub enum ValidDisputeStatementKind {
                Explicit,
                BackingSeconded(my_types::primitive_types::H256),
                BackingValid(my_types::primitive_types::H256),
                ApprovalChecking,
            }
            #[derive(Clone, Debug, parity_scale_codec::CompactAs)]
            pub struct ValidatorIndex(pub ::core::primitive::u32);
            #[derive(Clone, Debug)]
            pub enum ValidityAttestation {
                Implicit(my_types::polkadot_primitives::v5::validator_app::Signature),
                Explicit(my_types::polkadot_primitives::v5::validator_app::Signature),
            }
        }
        pub mod vstaging {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct AsyncBackingParams {
                pub max_candidate_depth: ::core::primitive::u32,
                pub allowed_ancestry_len: ::core::primitive::u32,
            }
        }
    }
    pub mod polkadot_runtime {
        use super::my_types;
        pub mod governance {
            use super::my_types;
            pub mod origins {
                use super::my_types;
                pub mod pallet_custom_origins {
                    use super::my_types;
                    #[derive(Clone, Debug)]
                    pub enum Origin {
                        StakingAdmin,
                        Treasurer,
                        FellowshipAdmin,
                        GeneralAdmin,
                        AuctionAdmin,
                        LeaseAdmin,
                        ReferendumCanceller,
                        ReferendumKiller,
                        SmallTipper,
                        BigTipper,
                        SmallSpender,
                        MediumSpender,
                        BigSpender,
                        WhitelistedCaller,
                    }
                }
            }
        }
        #[derive(Clone, Debug)]
        pub struct NposCompactSolution16 {
            pub votes1: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
            pub votes2: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    (
                        parity_scale_codec::Compact<::core::primitive::u16>,
                        parity_scale_codec::Compact<
                            my_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ),
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
            pub votes3: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    [(
                        parity_scale_codec::Compact<::core::primitive::u16>,
                        parity_scale_codec::Compact<
                            my_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 2usize],
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
            pub votes4: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    [(
                        parity_scale_codec::Compact<::core::primitive::u16>,
                        parity_scale_codec::Compact<
                            my_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 3usize],
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
            pub votes5: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    [(
                        parity_scale_codec::Compact<::core::primitive::u16>,
                        parity_scale_codec::Compact<
                            my_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 4usize],
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
            pub votes6: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    [(
                        parity_scale_codec::Compact<::core::primitive::u16>,
                        parity_scale_codec::Compact<
                            my_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 5usize],
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
            pub votes7: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    [(
                        parity_scale_codec::Compact<::core::primitive::u16>,
                        parity_scale_codec::Compact<
                            my_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 6usize],
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
            pub votes8: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    [(
                        parity_scale_codec::Compact<::core::primitive::u16>,
                        parity_scale_codec::Compact<
                            my_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 7usize],
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
            pub votes9: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    [(
                        parity_scale_codec::Compact<::core::primitive::u16>,
                        parity_scale_codec::Compact<
                            my_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 8usize],
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
            pub votes10: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    [(
                        parity_scale_codec::Compact<::core::primitive::u16>,
                        parity_scale_codec::Compact<
                            my_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 9usize],
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
            pub votes11: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    [(
                        parity_scale_codec::Compact<::core::primitive::u16>,
                        parity_scale_codec::Compact<
                            my_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 10usize],
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
            pub votes12: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    [(
                        parity_scale_codec::Compact<::core::primitive::u16>,
                        parity_scale_codec::Compact<
                            my_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 11usize],
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
            pub votes13: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    [(
                        parity_scale_codec::Compact<::core::primitive::u16>,
                        parity_scale_codec::Compact<
                            my_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 12usize],
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
            pub votes14: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    [(
                        parity_scale_codec::Compact<::core::primitive::u16>,
                        parity_scale_codec::Compact<
                            my_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 13usize],
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
            pub votes15: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    [(
                        parity_scale_codec::Compact<::core::primitive::u16>,
                        parity_scale_codec::Compact<
                            my_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 14usize],
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
            pub votes16: ::std::vec::Vec<
                (
                    parity_scale_codec::Compact<::core::primitive::u32>,
                    [(
                        parity_scale_codec::Compact<::core::primitive::u16>,
                        parity_scale_codec::Compact<
                            my_types::sp_arithmetic::per_things::PerU16,
                        >,
                    ); 15usize],
                    parity_scale_codec::Compact<::core::primitive::u16>,
                ),
            >,
        }
        #[derive(Clone, Debug)]
        pub enum OriginCaller {
            system(
                my_types::frame_support::dispatch::RawOrigin<
                    my_types::sp_core::crypto::AccountId32,
                >,
            ),
            Council(
                my_types::pallet_collective::RawOrigin<
                    my_types::sp_core::crypto::AccountId32,
                >,
            ),
            TechnicalCommittee(
                my_types::pallet_collective::RawOrigin<
                    my_types::sp_core::crypto::AccountId32,
                >,
            ),
            Origins(
                my_types::polkadot_runtime::governance::origins::pallet_custom_origins::Origin,
            ),
            ParachainsOrigin(
                my_types::polkadot_runtime_parachains::origin::pallet::Origin,
            ),
            XcmPallet(my_types::pallet_xcm::pallet::Origin),
            Void(my_types::sp_core::Void),
        }
        #[derive(Clone, Debug)]
        pub enum ProxyType {
            Any,
            NonTransfer,
            Governance,
            Staking,
            IdentityJudgement,
            CancelProxy,
            Auction,
            NominationPools,
        }
        #[derive(Clone, Debug)]
        pub struct Runtime;
        #[derive(Clone, Debug)]
        pub enum RuntimeCall {
            System(my_types::frame_system::pallet::Call),
            Scheduler(my_types::pallet_scheduler::pallet::Call),
            Preimage(my_types::pallet_preimage::pallet::Call),
            Babe(my_types::pallet_babe::pallet::Call),
            Timestamp(my_types::pallet_timestamp::pallet::Call),
            Indices(my_types::pallet_indices::pallet::Call),
            Balances(my_types::pallet_balances::pallet::Call),
            Staking(my_types::pallet_staking::pallet::pallet::Call),
            Session(my_types::pallet_session::pallet::Call),
            Grandpa(my_types::pallet_grandpa::pallet::Call),
            ImOnline(my_types::pallet_im_online::pallet::Call),
            Democracy(my_types::pallet_democracy::pallet::Call),
            Council(my_types::pallet_collective::pallet::Call),
            TechnicalCommittee(my_types::pallet_collective::pallet::Call),
            PhragmenElection(my_types::pallet_elections_phragmen::pallet::Call),
            TechnicalMembership(my_types::pallet_membership::pallet::Call),
            Treasury(my_types::pallet_treasury::pallet::Call),
            ConvictionVoting(my_types::pallet_conviction_voting::pallet::Call),
            Referenda(my_types::pallet_referenda::pallet::Call),
            Whitelist(my_types::pallet_whitelist::pallet::Call),
            Claims(my_types::polkadot_runtime_common::claims::pallet::Call),
            Vesting(my_types::pallet_vesting::pallet::Call),
            Utility(my_types::pallet_utility::pallet::Call),
            Identity(my_types::pallet_identity::pallet::Call),
            Proxy(my_types::pallet_proxy::pallet::Call),
            Multisig(my_types::pallet_multisig::pallet::Call),
            Bounties(my_types::pallet_bounties::pallet::Call),
            ChildBounties(my_types::pallet_child_bounties::pallet::Call),
            Tips(my_types::pallet_tips::pallet::Call),
            ElectionProviderMultiPhase(
                my_types::pallet_election_provider_multi_phase::pallet::Call,
            ),
            VoterList(my_types::pallet_bags_list::pallet::Call),
            NominationPools(my_types::pallet_nomination_pools::pallet::Call),
            FastUnstake(my_types::pallet_fast_unstake::pallet::Call),
            Configuration(
                my_types::polkadot_runtime_parachains::configuration::pallet::Call,
            ),
            ParasShared(my_types::polkadot_runtime_parachains::shared::pallet::Call),
            ParaInclusion(
                my_types::polkadot_runtime_parachains::inclusion::pallet::Call,
            ),
            ParaInherent(
                my_types::polkadot_runtime_parachains::paras_inherent::pallet::Call,
            ),
            Paras(my_types::polkadot_runtime_parachains::paras::pallet::Call),
            Initializer(
                my_types::polkadot_runtime_parachains::initializer::pallet::Call,
            ),
            Hrmp(my_types::polkadot_runtime_parachains::hrmp::pallet::Call),
            ParasDisputes(my_types::polkadot_runtime_parachains::disputes::pallet::Call),
            ParasSlashing(
                my_types::polkadot_runtime_parachains::disputes::slashing::pallet::Call,
            ),
            Registrar(my_types::polkadot_runtime_common::paras_registrar::pallet::Call),
            Slots(my_types::polkadot_runtime_common::slots::pallet::Call),
            Auctions(my_types::polkadot_runtime_common::auctions::pallet::Call),
            Crowdloan(my_types::polkadot_runtime_common::crowdloan::pallet::Call),
            XcmPallet(my_types::pallet_xcm::pallet::Call),
            MessageQueue(my_types::pallet_message_queue::pallet::Call),
        }
        #[derive(Clone, Debug)]
        pub enum RuntimeError {
            System(my_types::frame_system::pallet::Error),
            Scheduler(my_types::pallet_scheduler::pallet::Error),
            Preimage(my_types::pallet_preimage::pallet::Error),
            Babe(my_types::pallet_babe::pallet::Error),
            Indices(my_types::pallet_indices::pallet::Error),
            Balances(my_types::pallet_balances::pallet::Error),
            Staking(my_types::pallet_staking::pallet::pallet::Error),
            Session(my_types::pallet_session::pallet::Error),
            Grandpa(my_types::pallet_grandpa::pallet::Error),
            ImOnline(my_types::pallet_im_online::pallet::Error),
            Democracy(my_types::pallet_democracy::pallet::Error),
            Council(my_types::pallet_collective::pallet::Error),
            TechnicalCommittee(my_types::pallet_collective::pallet::Error),
            PhragmenElection(my_types::pallet_elections_phragmen::pallet::Error),
            TechnicalMembership(my_types::pallet_membership::pallet::Error),
            Treasury(my_types::pallet_treasury::pallet::Error),
            ConvictionVoting(my_types::pallet_conviction_voting::pallet::Error),
            Referenda(my_types::pallet_referenda::pallet::Error),
            Whitelist(my_types::pallet_whitelist::pallet::Error),
            Claims(my_types::polkadot_runtime_common::claims::pallet::Error),
            Vesting(my_types::pallet_vesting::pallet::Error),
            Utility(my_types::pallet_utility::pallet::Error),
            Identity(my_types::pallet_identity::pallet::Error),
            Proxy(my_types::pallet_proxy::pallet::Error),
            Multisig(my_types::pallet_multisig::pallet::Error),
            Bounties(my_types::pallet_bounties::pallet::Error),
            ChildBounties(my_types::pallet_child_bounties::pallet::Error),
            Tips(my_types::pallet_tips::pallet::Error),
            ElectionProviderMultiPhase(
                my_types::pallet_election_provider_multi_phase::pallet::Error,
            ),
            VoterList(my_types::pallet_bags_list::pallet::Error),
            NominationPools(my_types::pallet_nomination_pools::pallet::Error),
            FastUnstake(my_types::pallet_fast_unstake::pallet::Error),
            Configuration(
                my_types::polkadot_runtime_parachains::configuration::pallet::Error,
            ),
            ParaInclusion(
                my_types::polkadot_runtime_parachains::inclusion::pallet::Error,
            ),
            ParaInherent(
                my_types::polkadot_runtime_parachains::paras_inherent::pallet::Error,
            ),
            Paras(my_types::polkadot_runtime_parachains::paras::pallet::Error),
            Hrmp(my_types::polkadot_runtime_parachains::hrmp::pallet::Error),
            ParasDisputes(
                my_types::polkadot_runtime_parachains::disputes::pallet::Error,
            ),
            ParasSlashing(
                my_types::polkadot_runtime_parachains::disputes::slashing::pallet::Error,
            ),
            Registrar(my_types::polkadot_runtime_common::paras_registrar::pallet::Error),
            Slots(my_types::polkadot_runtime_common::slots::pallet::Error),
            Auctions(my_types::polkadot_runtime_common::auctions::pallet::Error),
            Crowdloan(my_types::polkadot_runtime_common::crowdloan::pallet::Error),
            XcmPallet(my_types::pallet_xcm::pallet::Error),
            MessageQueue(my_types::pallet_message_queue::pallet::Error),
        }
        #[derive(Clone, Debug)]
        pub enum RuntimeEvent {
            System(my_types::frame_system::pallet::Event),
            Scheduler(my_types::pallet_scheduler::pallet::Event),
            Preimage(my_types::pallet_preimage::pallet::Event),
            Indices(my_types::pallet_indices::pallet::Event),
            Balances(my_types::pallet_balances::pallet::Event),
            TransactionPayment(my_types::pallet_transaction_payment::pallet::Event),
            Staking(my_types::pallet_staking::pallet::pallet::Event),
            Offences(my_types::pallet_offences::pallet::Event),
            Session(my_types::pallet_session::pallet::Event),
            Grandpa(my_types::pallet_grandpa::pallet::Event),
            ImOnline(my_types::pallet_im_online::pallet::Event),
            Democracy(my_types::pallet_democracy::pallet::Event),
            Council(my_types::pallet_collective::pallet::Event),
            TechnicalCommittee(my_types::pallet_collective::pallet::Event),
            PhragmenElection(my_types::pallet_elections_phragmen::pallet::Event),
            TechnicalMembership(my_types::pallet_membership::pallet::Event),
            Treasury(my_types::pallet_treasury::pallet::Event),
            ConvictionVoting(my_types::pallet_conviction_voting::pallet::Event),
            Referenda(my_types::pallet_referenda::pallet::Event),
            Whitelist(my_types::pallet_whitelist::pallet::Event),
            Claims(my_types::polkadot_runtime_common::claims::pallet::Event),
            Vesting(my_types::pallet_vesting::pallet::Event),
            Utility(my_types::pallet_utility::pallet::Event),
            Identity(my_types::pallet_identity::pallet::Event),
            Proxy(my_types::pallet_proxy::pallet::Event),
            Multisig(my_types::pallet_multisig::pallet::Event),
            Bounties(my_types::pallet_bounties::pallet::Event),
            ChildBounties(my_types::pallet_child_bounties::pallet::Event),
            Tips(my_types::pallet_tips::pallet::Event),
            ElectionProviderMultiPhase(
                my_types::pallet_election_provider_multi_phase::pallet::Event,
            ),
            VoterList(my_types::pallet_bags_list::pallet::Event),
            NominationPools(my_types::pallet_nomination_pools::pallet::Event),
            FastUnstake(my_types::pallet_fast_unstake::pallet::Event),
            ParaInclusion(
                my_types::polkadot_runtime_parachains::inclusion::pallet::Event,
            ),
            Paras(my_types::polkadot_runtime_parachains::paras::pallet::Event),
            Hrmp(my_types::polkadot_runtime_parachains::hrmp::pallet::Event),
            ParasDisputes(
                my_types::polkadot_runtime_parachains::disputes::pallet::Event,
            ),
            Registrar(my_types::polkadot_runtime_common::paras_registrar::pallet::Event),
            Slots(my_types::polkadot_runtime_common::slots::pallet::Event),
            Auctions(my_types::polkadot_runtime_common::auctions::pallet::Event),
            Crowdloan(my_types::polkadot_runtime_common::crowdloan::pallet::Event),
            XcmPallet(my_types::pallet_xcm::pallet::Event),
            MessageQueue(my_types::pallet_message_queue::pallet::Event),
        }
        #[derive(Clone, Debug)]
        pub enum RuntimeHoldReason {}
        #[derive(Clone, Debug)]
        pub struct SessionKeys {
            pub grandpa: my_types::sp_consensus_grandpa::app::Public,
            pub babe: my_types::sp_consensus_babe::app::Public,
            pub im_online: my_types::pallet_im_online::sr25519::app_sr25519::Public,
            pub para_validator: my_types::polkadot_primitives::v5::validator_app::Public,
            pub para_assignment: my_types::polkadot_primitives::v5::assignment_app::Public,
            pub authority_discovery: my_types::sp_authority_discovery::app::Public,
        }
    }
    pub mod polkadot_runtime_common {
        use super::my_types;
        pub mod auctions {
            use super::my_types;
            pub mod pallet {
                use super::my_types;
                #[derive(Clone, Debug)]
                ///Contains a variant per dispatchable extrinsic that this pallet has.
                pub enum Call {
                    ///See [`Pallet::new_auction`].
                    new_auction {
                        duration: ::core::primitive::u32,
                        lease_period_index: ::core::primitive::u32,
                    },
                    ///See [`Pallet::bid`].
                    bid {
                        para: my_types::polkadot_parachain::primitives::Id,
                        auction_index: ::core::primitive::u32,
                        first_slot: ::core::primitive::u32,
                        last_slot: ::core::primitive::u32,
                        amount: ::core::primitive::u128,
                    },
                    ///See [`Pallet::cancel_auction`].
                    cancel_auction,
                }
                #[derive(Clone, Debug)]
                ///The `Error` enum of this pallet.
                pub enum Error {
                    ///This auction is already in progress.
                    AuctionInProgress,
                    ///The lease period is in the past.
                    LeasePeriodInPast,
                    ///Para is not registered
                    ParaNotRegistered,
                    ///Not a current auction.
                    NotCurrentAuction,
                    ///Not an auction.
                    NotAuction,
                    ///Auction has already ended.
                    AuctionEnded,
                    ///The para is already leased out for part of this range.
                    AlreadyLeasedOut,
                }
                #[derive(Clone, Debug)]
                ///The `Event` enum of this pallet
                pub enum Event {
                    ///An auction started. Provides its index and the block number where it will begin to
                    ///close and the first lease period of the quadruplet that is auctioned.
                    AuctionStarted {
                        auction_index: ::core::primitive::u32,
                        lease_period: ::core::primitive::u32,
                        ending: ::core::primitive::u32,
                    },
                    ///An auction ended. All funds become unreserved.
                    AuctionClosed { auction_index: ::core::primitive::u32 },
                    ///Funds were reserved for a winning bid. First balance is the extra amount reserved.
                    ///Second is the total.
                    Reserved {
                        bidder: my_types::sp_core::crypto::AccountId32,
                        extra_reserved: ::core::primitive::u128,
                        total_amount: ::core::primitive::u128,
                    },
                    ///Funds were unreserved since bidder is no longer active. `[bidder, amount]`
                    Unreserved {
                        bidder: my_types::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    ///Someone attempted to lease the same slot twice for a parachain. The amount is held in reserve
                    ///but no parachain slot has been leased.
                    ReserveConfiscated {
                        para_id: my_types::polkadot_parachain::primitives::Id,
                        leaser: my_types::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    ///A new bid has been accepted as the current winner.
                    BidAccepted {
                        bidder: my_types::sp_core::crypto::AccountId32,
                        para_id: my_types::polkadot_parachain::primitives::Id,
                        amount: ::core::primitive::u128,
                        first_slot: ::core::primitive::u32,
                        last_slot: ::core::primitive::u32,
                    },
                    ///The winning offset was chosen for an auction. This will map into the `Winning` storage map.
                    WinningOffset {
                        auction_index: ::core::primitive::u32,
                        block_number: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod claims {
            use super::my_types;
            pub mod pallet {
                use super::my_types;
                #[derive(Clone, Debug)]
                ///Contains a variant per dispatchable extrinsic that this pallet has.
                pub enum Call {
                    ///See [`Pallet::claim`].
                    claim {
                        dest: my_types::sp_core::crypto::AccountId32,
                        ethereum_signature: my_types::polkadot_runtime_common::claims::EcdsaSignature,
                    },
                    ///See [`Pallet::mint_claim`].
                    mint_claim {
                        who: my_types::polkadot_runtime_common::claims::EthereumAddress,
                        value: ::core::primitive::u128,
                        vesting_schedule: ::core::option::Option<
                            (
                                ::core::primitive::u128,
                                ::core::primitive::u128,
                                ::core::primitive::u32,
                            ),
                        >,
                        statement: ::core::option::Option<
                            my_types::polkadot_runtime_common::claims::StatementKind,
                        >,
                    },
                    ///See [`Pallet::claim_attest`].
                    claim_attest {
                        dest: my_types::sp_core::crypto::AccountId32,
                        ethereum_signature: my_types::polkadot_runtime_common::claims::EcdsaSignature,
                        statement: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    ///See [`Pallet::attest`].
                    attest { statement: ::std::vec::Vec<::core::primitive::u8> },
                    ///See [`Pallet::move_claim`].
                    move_claim {
                        old: my_types::polkadot_runtime_common::claims::EthereumAddress,
                        new: my_types::polkadot_runtime_common::claims::EthereumAddress,
                        maybe_preclaim: ::core::option::Option<
                            my_types::sp_core::crypto::AccountId32,
                        >,
                    },
                }
                #[derive(Clone, Debug)]
                ///The `Error` enum of this pallet.
                pub enum Error {
                    ///Invalid Ethereum signature.
                    InvalidEthereumSignature,
                    ///Ethereum address has no claim.
                    SignerHasNoClaim,
                    ///Account ID sending transaction has no claim.
                    SenderHasNoClaim,
                    ///There's not enough in the pot to pay out some unvested amount. Generally implies a logic
                    ///error.
                    PotUnderflow,
                    ///A needed statement was not included.
                    InvalidStatement,
                    ///The account already has a vested balance.
                    VestedBalanceExists,
                }
                #[derive(Clone, Debug)]
                ///The `Event` enum of this pallet
                pub enum Event {
                    ///Someone claimed some DOTs.
                    Claimed {
                        who: my_types::sp_core::crypto::AccountId32,
                        ethereum_address: my_types::polkadot_runtime_common::claims::EthereumAddress,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            #[derive(Clone, Debug)]
            pub struct EcdsaSignature(pub [::core::primitive::u8; 65usize]);
            #[derive(Clone, Debug)]
            pub struct EthereumAddress(pub [::core::primitive::u8; 20usize]);
            #[derive(Clone, Debug)]
            pub struct PrevalidateAttests;
            #[derive(Clone, Debug)]
            pub enum StatementKind {
                Regular,
                Saft,
            }
        }
        pub mod crowdloan {
            use super::my_types;
            pub mod pallet {
                use super::my_types;
                #[derive(Clone, Debug)]
                ///Contains a variant per dispatchable extrinsic that this pallet has.
                pub enum Call {
                    ///See [`Pallet::create`].
                    create {
                        index: my_types::polkadot_parachain::primitives::Id,
                        cap: ::core::primitive::u128,
                        first_period: ::core::primitive::u32,
                        last_period: ::core::primitive::u32,
                        end: ::core::primitive::u32,
                        verifier: ::core::option::Option<
                            my_types::sp_runtime::MultiSigner,
                        >,
                    },
                    ///See [`Pallet::contribute`].
                    contribute {
                        index: my_types::polkadot_parachain::primitives::Id,
                        value: ::core::primitive::u128,
                        signature: ::core::option::Option<
                            my_types::sp_runtime::MultiSignature,
                        >,
                    },
                    ///See [`Pallet::withdraw`].
                    withdraw {
                        who: my_types::sp_core::crypto::AccountId32,
                        index: my_types::polkadot_parachain::primitives::Id,
                    },
                    ///See [`Pallet::refund`].
                    refund { index: my_types::polkadot_parachain::primitives::Id },
                    ///See [`Pallet::dissolve`].
                    dissolve { index: my_types::polkadot_parachain::primitives::Id },
                    ///See [`Pallet::edit`].
                    edit {
                        index: my_types::polkadot_parachain::primitives::Id,
                        cap: ::core::primitive::u128,
                        first_period: ::core::primitive::u32,
                        last_period: ::core::primitive::u32,
                        end: ::core::primitive::u32,
                        verifier: ::core::option::Option<
                            my_types::sp_runtime::MultiSigner,
                        >,
                    },
                    ///See [`Pallet::add_memo`].
                    add_memo {
                        index: my_types::polkadot_parachain::primitives::Id,
                        memo: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    ///See [`Pallet::poke`].
                    poke { index: my_types::polkadot_parachain::primitives::Id },
                    ///See [`Pallet::contribute_all`].
                    contribute_all {
                        index: my_types::polkadot_parachain::primitives::Id,
                        signature: ::core::option::Option<
                            my_types::sp_runtime::MultiSignature,
                        >,
                    },
                }
                #[derive(Clone, Debug)]
                ///The `Error` enum of this pallet.
                pub enum Error {
                    ///The current lease period is more than the first lease period.
                    FirstPeriodInPast,
                    ///The first lease period needs to at least be less than 3 `max_value`.
                    FirstPeriodTooFarInFuture,
                    ///Last lease period must be greater than first lease period.
                    LastPeriodBeforeFirstPeriod,
                    ///The last lease period cannot be more than 3 periods after the first period.
                    LastPeriodTooFarInFuture,
                    ///The campaign ends before the current block number. The end must be in the future.
                    CannotEndInPast,
                    ///The end date for this crowdloan is not sensible.
                    EndTooFarInFuture,
                    ///There was an overflow.
                    Overflow,
                    ///The contribution was below the minimum, `MinContribution`.
                    ContributionTooSmall,
                    ///Invalid fund index.
                    InvalidParaId,
                    ///Contributions exceed maximum amount.
                    CapExceeded,
                    ///The contribution period has already ended.
                    ContributionPeriodOver,
                    ///The origin of this call is invalid.
                    InvalidOrigin,
                    ///This crowdloan does not correspond to a parachain.
                    NotParachain,
                    ///This parachain lease is still active and retirement cannot yet begin.
                    LeaseActive,
                    ///This parachain's bid or lease is still active and withdraw cannot yet begin.
                    BidOrLeaseActive,
                    ///The crowdloan has not yet ended.
                    FundNotEnded,
                    ///There are no contributions stored in this crowdloan.
                    NoContributions,
                    ///The crowdloan is not ready to dissolve. Potentially still has a slot or in retirement period.
                    NotReadyToDissolve,
                    ///Invalid signature.
                    InvalidSignature,
                    ///The provided memo is too large.
                    MemoTooLarge,
                    ///The fund is already in `NewRaise`
                    AlreadyInNewRaise,
                    ///No contributions allowed during the VRF delay
                    VrfDelayInProgress,
                    ///A lease period has not started yet, due to an offset in the starting block.
                    NoLeasePeriod,
                }
                #[derive(Clone, Debug)]
                ///The `Event` enum of this pallet
                pub enum Event {
                    ///Create a new crowdloaning campaign.
                    Created { para_id: my_types::polkadot_parachain::primitives::Id },
                    ///Contributed to a crowd sale.
                    Contributed {
                        who: my_types::sp_core::crypto::AccountId32,
                        fund_index: my_types::polkadot_parachain::primitives::Id,
                        amount: ::core::primitive::u128,
                    },
                    ///Withdrew full balance of a contributor.
                    Withdrew {
                        who: my_types::sp_core::crypto::AccountId32,
                        fund_index: my_types::polkadot_parachain::primitives::Id,
                        amount: ::core::primitive::u128,
                    },
                    ///The loans in a fund have been partially dissolved, i.e. there are some left
                    ///over child keys that still need to be killed.
                    PartiallyRefunded {
                        para_id: my_types::polkadot_parachain::primitives::Id,
                    },
                    ///All loans in a fund have been refunded.
                    AllRefunded {
                        para_id: my_types::polkadot_parachain::primitives::Id,
                    },
                    ///Fund is dissolved.
                    Dissolved { para_id: my_types::polkadot_parachain::primitives::Id },
                    ///The result of trying to submit a new bid to the Slots pallet.
                    HandleBidResult {
                        para_id: my_types::polkadot_parachain::primitives::Id,
                        result: ::core::result::Result<
                            (),
                            my_types::sp_runtime::DispatchError,
                        >,
                    },
                    ///The configuration to a crowdloan has been edited.
                    Edited { para_id: my_types::polkadot_parachain::primitives::Id },
                    ///A memo has been updated.
                    MemoUpdated {
                        who: my_types::sp_core::crypto::AccountId32,
                        para_id: my_types::polkadot_parachain::primitives::Id,
                        memo: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    ///A parachain has been moved to `NewRaise`
                    AddedToNewRaise {
                        para_id: my_types::polkadot_parachain::primitives::Id,
                    },
                }
            }
            #[derive(Clone, Debug)]
            pub struct FundInfo<_0, _1, _2, _3> {
                pub depositor: _0,
                pub verifier: ::core::option::Option<my_types::sp_runtime::MultiSigner>,
                pub deposit: _1,
                pub raised: _1,
                pub end: _2,
                pub cap: _1,
                pub last_contribution: my_types::polkadot_runtime_common::crowdloan::LastContribution<
                    _2,
                >,
                pub first_period: _3,
                pub last_period: _3,
                pub fund_index: ::core::primitive::u32,
            }
            #[derive(Clone, Debug)]
            pub enum LastContribution<_0> {
                Never,
                PreEnding(::core::primitive::u32),
                Ending(_0),
            }
        }
        pub mod paras_registrar {
            use super::my_types;
            pub mod pallet {
                use super::my_types;
                #[derive(Clone, Debug)]
                ///Contains a variant per dispatchable extrinsic that this pallet has.
                pub enum Call {
                    ///See [`Pallet::register`].
                    register {
                        id: my_types::polkadot_parachain::primitives::Id,
                        genesis_head: my_types::polkadot_parachain::primitives::HeadData,
                        validation_code: my_types::polkadot_parachain::primitives::ValidationCode,
                    },
                    ///See [`Pallet::force_register`].
                    force_register {
                        who: my_types::sp_core::crypto::AccountId32,
                        deposit: ::core::primitive::u128,
                        id: my_types::polkadot_parachain::primitives::Id,
                        genesis_head: my_types::polkadot_parachain::primitives::HeadData,
                        validation_code: my_types::polkadot_parachain::primitives::ValidationCode,
                    },
                    ///See [`Pallet::deregister`].
                    deregister { id: my_types::polkadot_parachain::primitives::Id },
                    ///See [`Pallet::swap`].
                    swap {
                        id: my_types::polkadot_parachain::primitives::Id,
                        other: my_types::polkadot_parachain::primitives::Id,
                    },
                    ///See [`Pallet::remove_lock`].
                    remove_lock { para: my_types::polkadot_parachain::primitives::Id },
                    ///See [`Pallet::reserve`].
                    reserve,
                    ///See [`Pallet::add_lock`].
                    add_lock { para: my_types::polkadot_parachain::primitives::Id },
                    ///See [`Pallet::schedule_code_upgrade`].
                    schedule_code_upgrade {
                        para: my_types::polkadot_parachain::primitives::Id,
                        new_code: my_types::polkadot_parachain::primitives::ValidationCode,
                    },
                    ///See [`Pallet::set_current_head`].
                    set_current_head {
                        para: my_types::polkadot_parachain::primitives::Id,
                        new_head: my_types::polkadot_parachain::primitives::HeadData,
                    },
                }
                #[derive(Clone, Debug)]
                ///The `Error` enum of this pallet.
                pub enum Error {
                    ///The ID is not registered.
                    NotRegistered,
                    ///The ID is already registered.
                    AlreadyRegistered,
                    ///The caller is not the owner of this Id.
                    NotOwner,
                    ///Invalid para code size.
                    CodeTooLarge,
                    ///Invalid para head data size.
                    HeadDataTooLarge,
                    ///Para is not a Parachain.
                    NotParachain,
                    ///Para is not a Parathread.
                    NotParathread,
                    ///Cannot deregister para
                    CannotDeregister,
                    ///Cannot schedule downgrade of parachain to parathread
                    CannotDowngrade,
                    ///Cannot schedule upgrade of parathread to parachain
                    CannotUpgrade,
                    ///Para is locked from manipulation by the manager. Must use parachain or relay chain governance.
                    ParaLocked,
                    ///The ID given for registration has not been reserved.
                    NotReserved,
                    ///Registering parachain with empty code is not allowed.
                    EmptyCode,
                    ///Cannot perform a parachain slot / lifecycle swap. Check that the state of both paras are
                    ///correct for the swap to work.
                    CannotSwap,
                }
                #[derive(Clone, Debug)]
                ///The `Event` enum of this pallet
                pub enum Event {
                    Registered {
                        para_id: my_types::polkadot_parachain::primitives::Id,
                        manager: my_types::sp_core::crypto::AccountId32,
                    },
                    Deregistered {
                        para_id: my_types::polkadot_parachain::primitives::Id,
                    },
                    Reserved {
                        para_id: my_types::polkadot_parachain::primitives::Id,
                        who: my_types::sp_core::crypto::AccountId32,
                    },
                    Swapped {
                        para_id: my_types::polkadot_parachain::primitives::Id,
                        other_id: my_types::polkadot_parachain::primitives::Id,
                    },
                }
            }
            #[derive(Clone, Debug)]
            pub struct ParaInfo<_0, _1> {
                pub manager: _0,
                pub deposit: _1,
                pub locked: ::core::primitive::bool,
            }
        }
        pub mod slots {
            use super::my_types;
            pub mod pallet {
                use super::my_types;
                #[derive(Clone, Debug)]
                ///Contains a variant per dispatchable extrinsic that this pallet has.
                pub enum Call {
                    ///See [`Pallet::force_lease`].
                    force_lease {
                        para: my_types::polkadot_parachain::primitives::Id,
                        leaser: my_types::sp_core::crypto::AccountId32,
                        amount: ::core::primitive::u128,
                        period_begin: ::core::primitive::u32,
                        period_count: ::core::primitive::u32,
                    },
                    ///See [`Pallet::clear_all_leases`].
                    clear_all_leases {
                        para: my_types::polkadot_parachain::primitives::Id,
                    },
                    ///See [`Pallet::trigger_onboard`].
                    trigger_onboard {
                        para: my_types::polkadot_parachain::primitives::Id,
                    },
                }
                #[derive(Clone, Debug)]
                ///The `Error` enum of this pallet.
                pub enum Error {
                    ///The parachain ID is not onboarding.
                    ParaNotOnboarding,
                    ///There was an error with the lease.
                    LeaseError,
                }
                #[derive(Clone, Debug)]
                ///The `Event` enum of this pallet
                pub enum Event {
                    ///A new `[lease_period]` is beginning.
                    NewLeasePeriod { lease_period: ::core::primitive::u32 },
                    ///A para has won the right to a continuous set of lease periods as a parachain.
                    ///First balance is any extra amount reserved on top of the para's existing deposit.
                    ///Second balance is the total amount reserved.
                    Leased {
                        para_id: my_types::polkadot_parachain::primitives::Id,
                        leaser: my_types::sp_core::crypto::AccountId32,
                        period_begin: ::core::primitive::u32,
                        period_count: ::core::primitive::u32,
                        extra_reserved: ::core::primitive::u128,
                        total_amount: ::core::primitive::u128,
                    },
                }
            }
        }
    }
    pub mod polkadot_runtime_parachains {
        use super::my_types;
        pub mod configuration {
            use super::my_types;
            pub mod pallet {
                use super::my_types;
                #[derive(Clone, Debug)]
                ///Contains a variant per dispatchable extrinsic that this pallet has.
                pub enum Call {
                    ///See [`Pallet::set_validation_upgrade_cooldown`].
                    set_validation_upgrade_cooldown { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_validation_upgrade_delay`].
                    set_validation_upgrade_delay { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_code_retention_period`].
                    set_code_retention_period { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_max_code_size`].
                    set_max_code_size { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_max_pov_size`].
                    set_max_pov_size { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_max_head_data_size`].
                    set_max_head_data_size { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_parathread_cores`].
                    set_parathread_cores { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_parathread_retries`].
                    set_parathread_retries { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_group_rotation_frequency`].
                    set_group_rotation_frequency { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_chain_availability_period`].
                    set_chain_availability_period { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_thread_availability_period`].
                    set_thread_availability_period { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_scheduling_lookahead`].
                    set_scheduling_lookahead { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_max_validators_per_core`].
                    set_max_validators_per_core {
                        new: ::core::option::Option<::core::primitive::u32>,
                    },
                    ///See [`Pallet::set_max_validators`].
                    set_max_validators {
                        new: ::core::option::Option<::core::primitive::u32>,
                    },
                    ///See [`Pallet::set_dispute_period`].
                    set_dispute_period { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_dispute_post_conclusion_acceptance_period`].
                    set_dispute_post_conclusion_acceptance_period {
                        new: ::core::primitive::u32,
                    },
                    ///See [`Pallet::set_no_show_slots`].
                    set_no_show_slots { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_n_delay_tranches`].
                    set_n_delay_tranches { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_zeroth_delay_tranche_width`].
                    set_zeroth_delay_tranche_width { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_needed_approvals`].
                    set_needed_approvals { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_relay_vrf_modulo_samples`].
                    set_relay_vrf_modulo_samples { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_max_upward_queue_count`].
                    set_max_upward_queue_count { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_max_upward_queue_size`].
                    set_max_upward_queue_size { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_max_downward_message_size`].
                    set_max_downward_message_size { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_max_upward_message_size`].
                    set_max_upward_message_size { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_max_upward_message_num_per_candidate`].
                    set_max_upward_message_num_per_candidate {
                        new: ::core::primitive::u32,
                    },
                    ///See [`Pallet::set_hrmp_open_request_ttl`].
                    set_hrmp_open_request_ttl { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_hrmp_sender_deposit`].
                    set_hrmp_sender_deposit { new: ::core::primitive::u128 },
                    ///See [`Pallet::set_hrmp_recipient_deposit`].
                    set_hrmp_recipient_deposit { new: ::core::primitive::u128 },
                    ///See [`Pallet::set_hrmp_channel_max_capacity`].
                    set_hrmp_channel_max_capacity { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_hrmp_channel_max_total_size`].
                    set_hrmp_channel_max_total_size { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_hrmp_max_parachain_inbound_channels`].
                    set_hrmp_max_parachain_inbound_channels {
                        new: ::core::primitive::u32,
                    },
                    ///See [`Pallet::set_hrmp_max_parathread_inbound_channels`].
                    set_hrmp_max_parathread_inbound_channels {
                        new: ::core::primitive::u32,
                    },
                    ///See [`Pallet::set_hrmp_channel_max_message_size`].
                    set_hrmp_channel_max_message_size { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_hrmp_max_parachain_outbound_channels`].
                    set_hrmp_max_parachain_outbound_channels {
                        new: ::core::primitive::u32,
                    },
                    ///See [`Pallet::set_hrmp_max_parathread_outbound_channels`].
                    set_hrmp_max_parathread_outbound_channels {
                        new: ::core::primitive::u32,
                    },
                    ///See [`Pallet::set_hrmp_max_message_num_per_candidate`].
                    set_hrmp_max_message_num_per_candidate {
                        new: ::core::primitive::u32,
                    },
                    ///See [`Pallet::set_pvf_checking_enabled`].
                    set_pvf_checking_enabled { new: ::core::primitive::bool },
                    ///See [`Pallet::set_pvf_voting_ttl`].
                    set_pvf_voting_ttl { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_minimum_validation_upgrade_delay`].
                    set_minimum_validation_upgrade_delay { new: ::core::primitive::u32 },
                    ///See [`Pallet::set_bypass_consistency_check`].
                    set_bypass_consistency_check { new: ::core::primitive::bool },
                    ///See [`Pallet::set_async_backing_params`].
                    set_async_backing_params {
                        new: my_types::polkadot_primitives::vstaging::AsyncBackingParams,
                    },
                    ///See [`Pallet::set_executor_params`].
                    set_executor_params {
                        new: my_types::polkadot_primitives::v5::executor_params::ExecutorParams,
                    },
                }
                #[derive(Clone, Debug)]
                ///The `Error` enum of this pallet.
                pub enum Error {
                    ///The new value for a configuration parameter is invalid.
                    InvalidNewValue,
                }
            }
            #[derive(Clone, Debug)]
            pub struct HostConfiguration<_0> {
                pub max_code_size: ::core::primitive::u32,
                pub max_head_data_size: ::core::primitive::u32,
                pub max_upward_queue_count: ::core::primitive::u32,
                pub max_upward_queue_size: ::core::primitive::u32,
                pub max_upward_message_size: ::core::primitive::u32,
                pub max_upward_message_num_per_candidate: ::core::primitive::u32,
                pub hrmp_max_message_num_per_candidate: ::core::primitive::u32,
                pub validation_upgrade_cooldown: _0,
                pub validation_upgrade_delay: _0,
                pub async_backing_params: my_types::polkadot_primitives::vstaging::AsyncBackingParams,
                pub max_pov_size: ::core::primitive::u32,
                pub max_downward_message_size: ::core::primitive::u32,
                pub hrmp_max_parachain_outbound_channels: ::core::primitive::u32,
                pub hrmp_max_parathread_outbound_channels: ::core::primitive::u32,
                pub hrmp_sender_deposit: ::core::primitive::u128,
                pub hrmp_recipient_deposit: ::core::primitive::u128,
                pub hrmp_channel_max_capacity: ::core::primitive::u32,
                pub hrmp_channel_max_total_size: ::core::primitive::u32,
                pub hrmp_max_parachain_inbound_channels: ::core::primitive::u32,
                pub hrmp_max_parathread_inbound_channels: ::core::primitive::u32,
                pub hrmp_channel_max_message_size: ::core::primitive::u32,
                pub executor_params: my_types::polkadot_primitives::v5::executor_params::ExecutorParams,
                pub code_retention_period: _0,
                pub parathread_cores: ::core::primitive::u32,
                pub parathread_retries: ::core::primitive::u32,
                pub group_rotation_frequency: _0,
                pub chain_availability_period: _0,
                pub thread_availability_period: _0,
                pub scheduling_lookahead: ::core::primitive::u32,
                pub max_validators_per_core: ::core::option::Option<_0>,
                pub max_validators: ::core::option::Option<_0>,
                pub dispute_period: ::core::primitive::u32,
                pub dispute_post_conclusion_acceptance_period: _0,
                pub no_show_slots: ::core::primitive::u32,
                pub n_delay_tranches: ::core::primitive::u32,
                pub zeroth_delay_tranche_width: ::core::primitive::u32,
                pub needed_approvals: ::core::primitive::u32,
                pub relay_vrf_modulo_samples: ::core::primitive::u32,
                pub pvf_checking_enabled: ::core::primitive::bool,
                pub pvf_voting_ttl: ::core::primitive::u32,
                pub minimum_validation_upgrade_delay: _0,
            }
        }
        pub mod disputes {
            use super::my_types;
            pub mod pallet {
                use super::my_types;
                #[derive(Clone, Debug)]
                ///Contains a variant per dispatchable extrinsic that this pallet has.
                pub enum Call {
                    ///See [`Pallet::force_unfreeze`].
                    force_unfreeze,
                }
                #[derive(Clone, Debug)]
                ///The `Error` enum of this pallet.
                pub enum Error {
                    ///Duplicate dispute statement sets provided.
                    DuplicateDisputeStatementSets,
                    ///Ancient dispute statement provided.
                    AncientDisputeStatement,
                    ///Validator index on statement is out of bounds for session.
                    ValidatorIndexOutOfBounds,
                    ///Invalid signature on statement.
                    InvalidSignature,
                    ///Validator vote submitted more than once to dispute.
                    DuplicateStatement,
                    ///A dispute where there are only votes on one side.
                    SingleSidedDispute,
                    ///A dispute vote from a malicious backer.
                    MaliciousBacker,
                    ///No backing votes were provides along dispute statements.
                    MissingBackingVotes,
                    ///Unconfirmed dispute statement sets provided.
                    UnconfirmedDispute,
                }
                #[derive(Clone, Debug)]
                ///The `Event` enum of this pallet
                pub enum Event {
                    ///A dispute has been initiated. \[candidate hash, dispute location\]
                    DisputeInitiated(
                        my_types::polkadot_core_primitives::CandidateHash,
                        my_types::polkadot_runtime_parachains::disputes::DisputeLocation,
                    ),
                    ///A dispute has concluded for or against a candidate.
                    ///`\[para id, candidate hash, dispute result\]`
                    DisputeConcluded(
                        my_types::polkadot_core_primitives::CandidateHash,
                        my_types::polkadot_runtime_parachains::disputes::DisputeResult,
                    ),
                    ///A dispute has concluded with supermajority against a candidate.
                    ///Block authors should no longer build on top of this head and should
                    ///instead revert the block at the given height. This should be the
                    ///number of the child of the last known valid block in the chain.
                    Revert(::core::primitive::u32),
                }
            }
            pub mod slashing {
                use super::my_types;
                pub mod pallet {
                    use super::my_types;
                    #[derive(Clone, Debug)]
                    ///Contains a variant per dispatchable extrinsic that this pallet has.
                    pub enum Call {
                        ///See [`Pallet::report_dispute_lost_unsigned`].
                        report_dispute_lost_unsigned {
                            dispute_proof: ::std::boxed::Box<
                                my_types::polkadot_primitives::v5::slashing::DisputeProof,
                            >,
                            key_owner_proof: my_types::sp_session::MembershipProof,
                        },
                    }
                    #[derive(Clone, Debug)]
                    ///The `Error` enum of this pallet.
                    pub enum Error {
                        ///The key ownership proof is invalid.
                        InvalidKeyOwnershipProof,
                        ///The session index is too old or invalid.
                        InvalidSessionIndex,
                        ///The candidate hash is invalid.
                        InvalidCandidateHash,
                        ///There is no pending slash for the given validator index and time
                        ///slot.
                        InvalidValidatorIndex,
                        ///The validator index does not match the validator id.
                        ValidatorIndexIdMismatch,
                        ///The given slashing report is valid but already previously reported.
                        DuplicateSlashingReport,
                    }
                }
            }
            #[derive(Clone, Debug)]
            pub enum DisputeLocation {
                Local,
                Remote,
            }
            #[derive(Clone, Debug)]
            pub enum DisputeResult {
                Valid,
                Invalid,
            }
        }
        pub mod hrmp {
            use super::my_types;
            pub mod pallet {
                use super::my_types;
                #[derive(Clone, Debug)]
                ///Contains a variant per dispatchable extrinsic that this pallet has.
                pub enum Call {
                    ///See [`Pallet::hrmp_init_open_channel`].
                    hrmp_init_open_channel {
                        recipient: my_types::polkadot_parachain::primitives::Id,
                        proposed_max_capacity: ::core::primitive::u32,
                        proposed_max_message_size: ::core::primitive::u32,
                    },
                    ///See [`Pallet::hrmp_accept_open_channel`].
                    hrmp_accept_open_channel {
                        sender: my_types::polkadot_parachain::primitives::Id,
                    },
                    ///See [`Pallet::hrmp_close_channel`].
                    hrmp_close_channel {
                        channel_id: my_types::polkadot_parachain::primitives::HrmpChannelId,
                    },
                    ///See [`Pallet::force_clean_hrmp`].
                    force_clean_hrmp {
                        para: my_types::polkadot_parachain::primitives::Id,
                        inbound: ::core::primitive::u32,
                        outbound: ::core::primitive::u32,
                    },
                    ///See [`Pallet::force_process_hrmp_open`].
                    force_process_hrmp_open { channels: ::core::primitive::u32 },
                    ///See [`Pallet::force_process_hrmp_close`].
                    force_process_hrmp_close { channels: ::core::primitive::u32 },
                    ///See [`Pallet::hrmp_cancel_open_request`].
                    hrmp_cancel_open_request {
                        channel_id: my_types::polkadot_parachain::primitives::HrmpChannelId,
                        open_requests: ::core::primitive::u32,
                    },
                    ///See [`Pallet::force_open_hrmp_channel`].
                    force_open_hrmp_channel {
                        sender: my_types::polkadot_parachain::primitives::Id,
                        recipient: my_types::polkadot_parachain::primitives::Id,
                        max_capacity: ::core::primitive::u32,
                        max_message_size: ::core::primitive::u32,
                    },
                }
                #[derive(Clone, Debug)]
                ///The `Error` enum of this pallet.
                pub enum Error {
                    ///The sender tried to open a channel to themselves.
                    OpenHrmpChannelToSelf,
                    ///The recipient is not a valid para.
                    OpenHrmpChannelInvalidRecipient,
                    ///The requested capacity is zero.
                    OpenHrmpChannelZeroCapacity,
                    ///The requested capacity exceeds the global limit.
                    OpenHrmpChannelCapacityExceedsLimit,
                    ///The requested maximum message size is 0.
                    OpenHrmpChannelZeroMessageSize,
                    ///The open request requested the message size that exceeds the global limit.
                    OpenHrmpChannelMessageSizeExceedsLimit,
                    ///The channel already exists
                    OpenHrmpChannelAlreadyExists,
                    ///There is already a request to open the same channel.
                    OpenHrmpChannelAlreadyRequested,
                    ///The sender already has the maximum number of allowed outbound channels.
                    OpenHrmpChannelLimitExceeded,
                    ///The channel from the sender to the origin doesn't exist.
                    AcceptHrmpChannelDoesntExist,
                    ///The channel is already confirmed.
                    AcceptHrmpChannelAlreadyConfirmed,
                    ///The recipient already has the maximum number of allowed inbound channels.
                    AcceptHrmpChannelLimitExceeded,
                    ///The origin tries to close a channel where it is neither the sender nor the recipient.
                    CloseHrmpChannelUnauthorized,
                    ///The channel to be closed doesn't exist.
                    CloseHrmpChannelDoesntExist,
                    ///The channel close request is already requested.
                    CloseHrmpChannelAlreadyUnderway,
                    ///Canceling is requested by neither the sender nor recipient of the open channel request.
                    CancelHrmpOpenChannelUnauthorized,
                    ///The open request doesn't exist.
                    OpenHrmpChannelDoesntExist,
                    ///Cannot cancel an HRMP open channel request because it is already confirmed.
                    OpenHrmpChannelAlreadyConfirmed,
                    ///The provided witness data is wrong.
                    WrongWitness,
                }
                #[derive(Clone, Debug)]
                ///The `Event` enum of this pallet
                pub enum Event {
                    ///Open HRMP channel requested.
                    ///`[sender, recipient, proposed_max_capacity, proposed_max_message_size]`
                    OpenChannelRequested(
                        my_types::polkadot_parachain::primitives::Id,
                        my_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    ),
                    ///An HRMP channel request sent by the receiver was canceled by either party.
                    ///`[by_parachain, channel_id]`
                    OpenChannelCanceled(
                        my_types::polkadot_parachain::primitives::Id,
                        my_types::polkadot_parachain::primitives::HrmpChannelId,
                    ),
                    ///Open HRMP channel accepted. `[sender, recipient]`
                    OpenChannelAccepted(
                        my_types::polkadot_parachain::primitives::Id,
                        my_types::polkadot_parachain::primitives::Id,
                    ),
                    ///HRMP channel closed. `[by_parachain, channel_id]`
                    ChannelClosed(
                        my_types::polkadot_parachain::primitives::Id,
                        my_types::polkadot_parachain::primitives::HrmpChannelId,
                    ),
                    ///An HRMP channel was opened via Root origin.
                    ///`[sender, recipient, proposed_max_capacity, proposed_max_message_size]`
                    HrmpChannelForceOpened(
                        my_types::polkadot_parachain::primitives::Id,
                        my_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    ),
                }
            }
            #[derive(Clone, Debug)]
            pub struct HrmpChannel {
                pub max_capacity: ::core::primitive::u32,
                pub max_total_size: ::core::primitive::u32,
                pub max_message_size: ::core::primitive::u32,
                pub msg_count: ::core::primitive::u32,
                pub total_size: ::core::primitive::u32,
                pub mqc_head: ::core::option::Option<my_types::primitive_types::H256>,
                pub sender_deposit: ::core::primitive::u128,
                pub recipient_deposit: ::core::primitive::u128,
            }
            #[derive(Clone, Debug)]
            pub struct HrmpOpenChannelRequest {
                pub confirmed: ::core::primitive::bool,
                pub _age: ::core::primitive::u32,
                pub sender_deposit: ::core::primitive::u128,
                pub max_message_size: ::core::primitive::u32,
                pub max_capacity: ::core::primitive::u32,
                pub max_total_size: ::core::primitive::u32,
            }
        }
        pub mod inclusion {
            use super::my_types;
            pub mod pallet {
                use super::my_types;
                #[derive(Clone, Debug)]
                ///Contains a variant per dispatchable extrinsic that this pallet has.
                pub enum Call {}
                #[derive(Clone, Debug)]
                ///The `Error` enum of this pallet.
                pub enum Error {
                    ///Validator indices are out of order or contains duplicates.
                    UnsortedOrDuplicateValidatorIndices,
                    ///Dispute statement sets are out of order or contain duplicates.
                    UnsortedOrDuplicateDisputeStatementSet,
                    ///Backed candidates are out of order (core index) or contain duplicates.
                    UnsortedOrDuplicateBackedCandidates,
                    ///A different relay parent was provided compared to the on-chain stored one.
                    UnexpectedRelayParent,
                    ///Availability bitfield has unexpected size.
                    WrongBitfieldSize,
                    ///Bitfield consists of zeros only.
                    BitfieldAllZeros,
                    ///Multiple bitfields submitted by same validator or validators out of order by index.
                    BitfieldDuplicateOrUnordered,
                    ///Validator index out of bounds.
                    ValidatorIndexOutOfBounds,
                    ///Invalid signature
                    InvalidBitfieldSignature,
                    ///Candidate submitted but para not scheduled.
                    UnscheduledCandidate,
                    ///Candidate scheduled despite pending candidate already existing for the para.
                    CandidateScheduledBeforeParaFree,
                    ///Candidate included with the wrong collator.
                    WrongCollator,
                    ///Scheduled cores out of order.
                    ScheduledOutOfOrder,
                    ///Head data exceeds the configured maximum.
                    HeadDataTooLarge,
                    ///Code upgrade prematurely.
                    PrematureCodeUpgrade,
                    ///Output code is too large
                    NewCodeTooLarge,
                    ///Candidate not in parent context.
                    CandidateNotInParentContext,
                    ///Invalid group index in core assignment.
                    InvalidGroupIndex,
                    ///Insufficient (non-majority) backing.
                    InsufficientBacking,
                    ///Invalid (bad signature, unknown validator, etc.) backing.
                    InvalidBacking,
                    ///Collator did not sign PoV.
                    NotCollatorSigned,
                    ///The validation data hash does not match expected.
                    ValidationDataHashMismatch,
                    ///The downward message queue is not processed correctly.
                    IncorrectDownwardMessageHandling,
                    ///At least one upward message sent does not pass the acceptance criteria.
                    InvalidUpwardMessages,
                    ///The candidate didn't follow the rules of HRMP watermark advancement.
                    HrmpWatermarkMishandling,
                    ///The HRMP messages sent by the candidate is not valid.
                    InvalidOutboundHrmp,
                    ///The validation code hash of the candidate is not valid.
                    InvalidValidationCodeHash,
                    ///The `para_head` hash in the candidate descriptor doesn't match the hash of the actual para head in the
                    ///commitments.
                    ParaHeadMismatch,
                    ///A bitfield that references a freed core,
                    ///either intentionally or as part of a concluded
                    ///invalid dispute.
                    BitfieldReferencesFreedCore,
                }
                #[derive(Clone, Debug)]
                ///The `Event` enum of this pallet
                pub enum Event {
                    ///A candidate was backed. `[candidate, head_data]`
                    CandidateBacked(
                        my_types::polkadot_primitives::v5::CandidateReceipt<
                            my_types::primitive_types::H256,
                        >,
                        my_types::polkadot_parachain::primitives::HeadData,
                        my_types::polkadot_primitives::v5::CoreIndex,
                        my_types::polkadot_primitives::v5::GroupIndex,
                    ),
                    ///A candidate was included. `[candidate, head_data]`
                    CandidateIncluded(
                        my_types::polkadot_primitives::v5::CandidateReceipt<
                            my_types::primitive_types::H256,
                        >,
                        my_types::polkadot_parachain::primitives::HeadData,
                        my_types::polkadot_primitives::v5::CoreIndex,
                        my_types::polkadot_primitives::v5::GroupIndex,
                    ),
                    ///A candidate timed out. `[candidate, head_data]`
                    CandidateTimedOut(
                        my_types::polkadot_primitives::v5::CandidateReceipt<
                            my_types::primitive_types::H256,
                        >,
                        my_types::polkadot_parachain::primitives::HeadData,
                        my_types::polkadot_primitives::v5::CoreIndex,
                    ),
                    ///Some upward messages have been received and will be processed.
                    UpwardMessagesReceived {
                        from: my_types::polkadot_parachain::primitives::Id,
                        count: ::core::primitive::u32,
                    },
                }
            }
            #[derive(Clone, Debug)]
            pub enum AggregateMessageOrigin {
                Ump(my_types::polkadot_runtime_parachains::inclusion::UmpQueueId),
            }
            #[derive(Clone, Debug)]
            pub struct AvailabilityBitfieldRecord<_0> {
                pub bitfield: my_types::polkadot_primitives::v5::AvailabilityBitfield,
                pub submitted_at: _0,
            }
            #[derive(Clone, Debug)]
            pub struct CandidatePendingAvailability<_0, _1> {
                pub core: my_types::polkadot_primitives::v5::CoreIndex,
                pub hash: my_types::polkadot_core_primitives::CandidateHash,
                pub descriptor: my_types::polkadot_primitives::v5::CandidateDescriptor<
                    _0,
                >,
                pub availability_votes: DecodedBits<
                    ::core::primitive::u8,
                    my_types::bitvec::order::Lsb0,
                >,
                pub backers: DecodedBits<
                    ::core::primitive::u8,
                    my_types::bitvec::order::Lsb0,
                >,
                pub relay_parent_number: _1,
                pub backed_in_number: _1,
                pub backing_group: my_types::polkadot_primitives::v5::GroupIndex,
            }
            #[derive(Clone, Debug)]
            pub enum UmpQueueId {
                Para(my_types::polkadot_parachain::primitives::Id),
            }
        }
        pub mod initializer {
            use super::my_types;
            pub mod pallet {
                use super::my_types;
                #[derive(Clone, Debug)]
                ///Contains a variant per dispatchable extrinsic that this pallet has.
                pub enum Call {
                    ///See [`Pallet::force_approve`].
                    force_approve { up_to: ::core::primitive::u32 },
                }
            }
            #[derive(Clone, Debug)]
            pub struct BufferedSessionChange {
                pub validators: ::std::vec::Vec<
                    my_types::polkadot_primitives::v5::validator_app::Public,
                >,
                pub queued: ::std::vec::Vec<
                    my_types::polkadot_primitives::v5::validator_app::Public,
                >,
                pub session_index: ::core::primitive::u32,
            }
        }
        pub mod origin {
            use super::my_types;
            pub mod pallet {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub enum Origin {
                    Parachain(my_types::polkadot_parachain::primitives::Id),
                }
            }
        }
        pub mod paras {
            use super::my_types;
            pub mod pallet {
                use super::my_types;
                #[derive(Clone, Debug)]
                ///Contains a variant per dispatchable extrinsic that this pallet has.
                pub enum Call {
                    ///See [`Pallet::force_set_current_code`].
                    force_set_current_code {
                        para: my_types::polkadot_parachain::primitives::Id,
                        new_code: my_types::polkadot_parachain::primitives::ValidationCode,
                    },
                    ///See [`Pallet::force_set_current_head`].
                    force_set_current_head {
                        para: my_types::polkadot_parachain::primitives::Id,
                        new_head: my_types::polkadot_parachain::primitives::HeadData,
                    },
                    ///See [`Pallet::force_schedule_code_upgrade`].
                    force_schedule_code_upgrade {
                        para: my_types::polkadot_parachain::primitives::Id,
                        new_code: my_types::polkadot_parachain::primitives::ValidationCode,
                        relay_parent_number: ::core::primitive::u32,
                    },
                    ///See [`Pallet::force_note_new_head`].
                    force_note_new_head {
                        para: my_types::polkadot_parachain::primitives::Id,
                        new_head: my_types::polkadot_parachain::primitives::HeadData,
                    },
                    ///See [`Pallet::force_queue_action`].
                    force_queue_action {
                        para: my_types::polkadot_parachain::primitives::Id,
                    },
                    ///See [`Pallet::add_trusted_validation_code`].
                    add_trusted_validation_code {
                        validation_code: my_types::polkadot_parachain::primitives::ValidationCode,
                    },
                    ///See [`Pallet::poke_unused_validation_code`].
                    poke_unused_validation_code {
                        validation_code_hash: my_types::polkadot_parachain::primitives::ValidationCodeHash,
                    },
                    ///See [`Pallet::include_pvf_check_statement`].
                    include_pvf_check_statement {
                        stmt: my_types::polkadot_primitives::v5::PvfCheckStatement,
                        signature: my_types::polkadot_primitives::v5::validator_app::Signature,
                    },
                }
                #[derive(Clone, Debug)]
                ///The `Error` enum of this pallet.
                pub enum Error {
                    ///Para is not registered in our system.
                    NotRegistered,
                    ///Para cannot be onboarded because it is already tracked by our system.
                    CannotOnboard,
                    ///Para cannot be offboarded at this time.
                    CannotOffboard,
                    ///Para cannot be upgraded to a parachain.
                    CannotUpgrade,
                    ///Para cannot be downgraded to a parathread.
                    CannotDowngrade,
                    ///The statement for PVF pre-checking is stale.
                    PvfCheckStatementStale,
                    ///The statement for PVF pre-checking is for a future session.
                    PvfCheckStatementFuture,
                    ///Claimed validator index is out of bounds.
                    PvfCheckValidatorIndexOutOfBounds,
                    ///The signature for the PVF pre-checking is invalid.
                    PvfCheckInvalidSignature,
                    ///The given validator already has cast a vote.
                    PvfCheckDoubleVote,
                    ///The given PVF does not exist at the moment of process a vote.
                    PvfCheckSubjectInvalid,
                    ///Parachain cannot currently schedule a code upgrade.
                    CannotUpgradeCode,
                }
                #[derive(Clone, Debug)]
                ///The `Event` enum of this pallet
                pub enum Event {
                    ///Current code has been updated for a Para. `para_id`
                    CurrentCodeUpdated(my_types::polkadot_parachain::primitives::Id),
                    ///Current head has been updated for a Para. `para_id`
                    CurrentHeadUpdated(my_types::polkadot_parachain::primitives::Id),
                    ///A code upgrade has been scheduled for a Para. `para_id`
                    CodeUpgradeScheduled(my_types::polkadot_parachain::primitives::Id),
                    ///A new head has been noted for a Para. `para_id`
                    NewHeadNoted(my_types::polkadot_parachain::primitives::Id),
                    ///A para has been queued to execute pending actions. `para_id`
                    ActionQueued(
                        my_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                    ),
                    ///The given para either initiated or subscribed to a PVF check for the given validation
                    ///code. `code_hash` `para_id`
                    PvfCheckStarted(
                        my_types::polkadot_parachain::primitives::ValidationCodeHash,
                        my_types::polkadot_parachain::primitives::Id,
                    ),
                    ///The given validation code was accepted by the PVF pre-checking vote.
                    ///`code_hash` `para_id`
                    PvfCheckAccepted(
                        my_types::polkadot_parachain::primitives::ValidationCodeHash,
                        my_types::polkadot_parachain::primitives::Id,
                    ),
                    ///The given validation code was rejected by the PVF pre-checking vote.
                    ///`code_hash` `para_id`
                    PvfCheckRejected(
                        my_types::polkadot_parachain::primitives::ValidationCodeHash,
                        my_types::polkadot_parachain::primitives::Id,
                    ),
                }
            }
            #[derive(Clone, Debug)]
            pub struct ParaGenesisArgs {
                pub genesis_head: my_types::polkadot_parachain::primitives::HeadData,
                pub validation_code: my_types::polkadot_parachain::primitives::ValidationCode,
                pub para_kind: ::core::primitive::bool,
            }
            #[derive(Clone, Debug)]
            pub enum ParaLifecycle {
                Onboarding,
                Parathread,
                Parachain,
                UpgradingParathread,
                DowngradingParachain,
                OffboardingParathread,
                OffboardingParachain,
            }
            #[derive(Clone, Debug)]
            pub struct ParaPastCodeMeta<_0> {
                pub upgrade_times: ::std::vec::Vec<
                    my_types::polkadot_runtime_parachains::paras::ReplacementTimes<_0>,
                >,
                pub last_pruned: ::core::option::Option<_0>,
            }
            #[derive(Clone, Debug)]
            pub struct PvfCheckActiveVoteState<_0> {
                pub votes_accept: DecodedBits<
                    ::core::primitive::u8,
                    my_types::bitvec::order::Lsb0,
                >,
                pub votes_reject: DecodedBits<
                    ::core::primitive::u8,
                    my_types::bitvec::order::Lsb0,
                >,
                pub age: ::core::primitive::u32,
                pub created_at: _0,
                pub causes: ::std::vec::Vec<
                    my_types::polkadot_runtime_parachains::paras::PvfCheckCause<_0>,
                >,
            }
            #[derive(Clone, Debug)]
            pub enum PvfCheckCause<_0> {
                Onboarding(my_types::polkadot_parachain::primitives::Id),
                Upgrade {
                    id: my_types::polkadot_parachain::primitives::Id,
                    relay_parent_number: _0,
                },
            }
            #[derive(Clone, Debug)]
            pub struct ReplacementTimes<_0> {
                pub expected_at: _0,
                pub activated_at: _0,
            }
        }
        pub mod paras_inherent {
            use super::my_types;
            pub mod pallet {
                use super::my_types;
                #[derive(Clone, Debug)]
                ///Contains a variant per dispatchable extrinsic that this pallet has.
                pub enum Call {
                    ///See [`Pallet::enter`].
                    enter {
                        data: my_types::polkadot_primitives::v5::InherentData<
                            my_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                my_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                    },
                }
                #[derive(Clone, Debug)]
                ///The `Error` enum of this pallet.
                pub enum Error {
                    ///Inclusion inherent called more than once per block.
                    TooManyInclusionInherents,
                    ///The hash of the submitted parent header doesn't correspond to the saved block hash of
                    ///the parent.
                    InvalidParentHeader,
                    ///Disputed candidate that was concluded invalid.
                    CandidateConcludedInvalid,
                    ///The data given to the inherent will result in an overweight block.
                    InherentOverweight,
                    ///The ordering of dispute statements was invalid.
                    DisputeStatementsUnsortedOrDuplicates,
                    ///A dispute statement was invalid.
                    DisputeInvalid,
                }
            }
        }
        pub mod scheduler {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub enum AssignmentKind {
                Parachain,
                Parathread(
                    my_types::polkadot_primitives::v5::collator_app::Public,
                    ::core::primitive::u32,
                ),
            }
            #[derive(Clone, Debug)]
            pub struct CoreAssignment {
                pub core: my_types::polkadot_primitives::v5::CoreIndex,
                pub para_id: my_types::polkadot_parachain::primitives::Id,
                pub kind: my_types::polkadot_runtime_parachains::scheduler::AssignmentKind,
                pub group_idx: my_types::polkadot_primitives::v5::GroupIndex,
            }
            #[derive(Clone, Debug)]
            pub struct ParathreadClaimQueue {
                pub queue: ::std::vec::Vec<
                    my_types::polkadot_runtime_parachains::scheduler::QueuedParathread,
                >,
                pub next_core_offset: ::core::primitive::u32,
            }
            #[derive(Clone, Debug)]
            pub struct QueuedParathread {
                pub claim: my_types::polkadot_primitives::v5::ParathreadEntry,
                pub core_offset: ::core::primitive::u32,
            }
        }
        pub mod shared {
            use super::my_types;
            pub mod pallet {
                use super::my_types;
                #[derive(Clone, Debug)]
                ///Contains a variant per dispatchable extrinsic that this pallet has.
                pub enum Call {}
            }
        }
    }
    pub mod primitive_types {
        use super::my_types;
        #[derive(Clone, Debug)]
        pub struct H256(pub [::core::primitive::u8; 32usize]);
    }
    pub mod sp_arithmetic {
        use super::my_types;
        pub mod fixed_point {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct FixedI64(pub ::core::primitive::i64);
            #[derive(Clone, Debug, parity_scale_codec::CompactAs)]
            pub struct FixedU128(pub ::core::primitive::u128);
        }
        pub mod per_things {
            use super::my_types;
            #[derive(Clone, Debug, parity_scale_codec::CompactAs)]
            pub struct PerU16(pub ::core::primitive::u16);
            #[derive(Clone, Debug, parity_scale_codec::CompactAs)]
            pub struct Perbill(pub ::core::primitive::u32);
            #[derive(Clone, Debug, parity_scale_codec::CompactAs)]
            pub struct Percent(pub ::core::primitive::u8);
            #[derive(Clone, Debug, parity_scale_codec::CompactAs)]
            pub struct Permill(pub ::core::primitive::u32);
        }
        #[derive(Clone, Debug)]
        pub enum ArithmeticError {
            Underflow,
            Overflow,
            DivisionByZero,
        }
    }
    pub mod sp_authority_discovery {
        use super::my_types;
        pub mod app {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct Public(pub my_types::sp_core::sr25519::Public);
        }
    }
    pub mod sp_consensus_babe {
        use super::my_types;
        pub mod app {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct Public(pub my_types::sp_core::sr25519::Public);
        }
        pub mod digests {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub enum NextConfigDescriptor {
                V1 {
                    c: (::core::primitive::u64, ::core::primitive::u64),
                    allowed_slots: my_types::sp_consensus_babe::AllowedSlots,
                },
            }
            #[derive(Clone, Debug)]
            pub enum PreDigest {
                Primary(my_types::sp_consensus_babe::digests::PrimaryPreDigest),
                SecondaryPlain(
                    my_types::sp_consensus_babe::digests::SecondaryPlainPreDigest,
                ),
                SecondaryVRF(
                    my_types::sp_consensus_babe::digests::SecondaryVRFPreDigest,
                ),
            }
            #[derive(Clone, Debug)]
            pub struct PrimaryPreDigest {
                pub authority_index: ::core::primitive::u32,
                pub slot: my_types::sp_consensus_slots::Slot,
                pub vrf_signature: my_types::sp_core::sr25519::vrf::VrfSignature,
            }
            #[derive(Clone, Debug)]
            pub struct SecondaryPlainPreDigest {
                pub authority_index: ::core::primitive::u32,
                pub slot: my_types::sp_consensus_slots::Slot,
            }
            #[derive(Clone, Debug)]
            pub struct SecondaryVRFPreDigest {
                pub authority_index: ::core::primitive::u32,
                pub slot: my_types::sp_consensus_slots::Slot,
                pub vrf_signature: my_types::sp_core::sr25519::vrf::VrfSignature,
            }
        }
        #[derive(Clone, Debug)]
        pub enum AllowedSlots {
            PrimarySlots,
            PrimaryAndSecondaryPlainSlots,
            PrimaryAndSecondaryVRFSlots,
        }
        #[derive(Clone, Debug)]
        pub struct BabeConfiguration {
            pub slot_duration: ::core::primitive::u64,
            pub epoch_length: ::core::primitive::u64,
            pub c: (::core::primitive::u64, ::core::primitive::u64),
            pub authorities: ::std::vec::Vec<
                (my_types::sp_consensus_babe::app::Public, ::core::primitive::u64),
            >,
            pub randomness: [::core::primitive::u8; 32usize],
            pub allowed_slots: my_types::sp_consensus_babe::AllowedSlots,
        }
        #[derive(Clone, Debug)]
        pub struct BabeEpochConfiguration {
            pub c: (::core::primitive::u64, ::core::primitive::u64),
            pub allowed_slots: my_types::sp_consensus_babe::AllowedSlots,
        }
        #[derive(Clone, Debug)]
        pub struct Epoch {
            pub epoch_index: ::core::primitive::u64,
            pub start_slot: my_types::sp_consensus_slots::Slot,
            pub duration: ::core::primitive::u64,
            pub authorities: ::std::vec::Vec<
                (my_types::sp_consensus_babe::app::Public, ::core::primitive::u64),
            >,
            pub randomness: [::core::primitive::u8; 32usize],
            pub config: my_types::sp_consensus_babe::BabeEpochConfiguration,
        }
        #[derive(Clone, Debug)]
        pub struct OpaqueKeyOwnershipProof(pub ::std::vec::Vec<::core::primitive::u8>);
    }
    pub mod sp_consensus_beefy {
        use super::my_types;
        pub mod commitment {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct Commitment<_0> {
                pub payload: my_types::sp_consensus_beefy::payload::Payload,
                pub block_number: _0,
                pub validator_set_id: ::core::primitive::u64,
            }
        }
        pub mod crypto {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct Public(pub my_types::sp_core::ecdsa::Public);
            #[derive(Clone, Debug)]
            pub struct Signature(pub my_types::sp_core::ecdsa::Signature);
        }
        pub mod payload {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct Payload(
                pub ::std::vec::Vec<
                    (
                        [::core::primitive::u8; 2usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                >,
            );
        }
        #[derive(Clone, Debug)]
        pub struct EquivocationProof<_0, _1, _2> {
            pub first: my_types::sp_consensus_beefy::VoteMessage<_0, _1, _2>,
            pub second: my_types::sp_consensus_beefy::VoteMessage<_0, _1, _2>,
        }
        #[derive(Clone, Debug)]
        pub struct OpaqueKeyOwnershipProof(pub ::std::vec::Vec<::core::primitive::u8>);
        #[derive(Clone, Debug)]
        pub struct ValidatorSet<_0> {
            pub validators: ::std::vec::Vec<_0>,
            pub id: ::core::primitive::u64,
        }
        #[derive(Clone, Debug)]
        pub struct VoteMessage<_0, _1, _2> {
            pub commitment: my_types::sp_consensus_beefy::commitment::Commitment<_0>,
            pub id: _1,
            pub signature: _2,
        }
    }
    pub mod sp_consensus_grandpa {
        use super::my_types;
        pub mod app {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct Public(pub my_types::sp_core::ed25519::Public);
            #[derive(Clone, Debug)]
            pub struct Signature(pub my_types::sp_core::ed25519::Signature);
        }
        #[derive(Clone, Debug)]
        pub enum Equivocation<_0, _1> {
            Prevote(
                my_types::finality_grandpa::Equivocation<
                    my_types::sp_consensus_grandpa::app::Public,
                    my_types::finality_grandpa::Prevote<_0, _1>,
                    my_types::sp_consensus_grandpa::app::Signature,
                >,
            ),
            Precommit(
                my_types::finality_grandpa::Equivocation<
                    my_types::sp_consensus_grandpa::app::Public,
                    my_types::finality_grandpa::Precommit<_0, _1>,
                    my_types::sp_consensus_grandpa::app::Signature,
                >,
            ),
        }
        #[derive(Clone, Debug)]
        pub struct EquivocationProof<_0, _1> {
            pub set_id: ::core::primitive::u64,
            pub equivocation: my_types::sp_consensus_grandpa::Equivocation<_0, _1>,
        }
        #[derive(Clone, Debug)]
        pub struct OpaqueKeyOwnershipProof(pub ::std::vec::Vec<::core::primitive::u8>);
    }
    pub mod sp_consensus_slots {
        use super::my_types;
        #[derive(Clone, Debug)]
        pub struct EquivocationProof<_0, _1> {
            pub offender: _1,
            pub slot: my_types::sp_consensus_slots::Slot,
            pub first_header: _0,
            pub second_header: _0,
        }
        #[derive(Clone, Debug, parity_scale_codec::CompactAs)]
        pub struct Slot(pub ::core::primitive::u64);
    }
    pub mod sp_core {
        use super::my_types;
        pub mod crypto {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct AccountId32(pub [::core::primitive::u8; 32usize]);
            #[derive(Clone, Debug)]
            pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
        }
        pub mod ecdsa {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct Public(pub [::core::primitive::u8; 33usize]);
            #[derive(Clone, Debug)]
            pub struct Signature(pub [::core::primitive::u8; 65usize]);
        }
        pub mod ed25519 {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct Public(pub [::core::primitive::u8; 32usize]);
            #[derive(Clone, Debug)]
            pub struct Signature(pub [::core::primitive::u8; 64usize]);
        }
        pub mod sr25519 {
            use super::my_types;
            pub mod vrf {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct VrfSignature {
                    pub output: [::core::primitive::u8; 32usize],
                    pub proof: [::core::primitive::u8; 64usize],
                }
            }
            #[derive(Clone, Debug)]
            pub struct Public(pub [::core::primitive::u8; 32usize]);
            #[derive(Clone, Debug)]
            pub struct Signature(pub [::core::primitive::u8; 64usize]);
        }
        #[derive(Clone, Debug)]
        pub struct OpaqueMetadata(pub ::std::vec::Vec<::core::primitive::u8>);
        #[derive(Clone, Debug)]
        pub enum Void {}
    }
    pub mod sp_inherents {
        use super::my_types;
        #[derive(Clone, Debug)]
        pub struct CheckInherentsResult {
            pub okay: ::core::primitive::bool,
            pub fatal_error: ::core::primitive::bool,
            pub errors: my_types::sp_inherents::InherentData,
        }
        #[derive(Clone, Debug)]
        pub struct InherentData {
            pub data: ::std::collections::BTreeMap<
                [::core::primitive::u8; 8usize],
                ::std::vec::Vec<::core::primitive::u8>,
            >,
        }
    }
    pub mod sp_mmr_primitives {
        use super::my_types;
        #[derive(Clone, Debug)]
        pub struct EncodableOpaqueLeaf(pub ::std::vec::Vec<::core::primitive::u8>);
        #[derive(Clone, Debug)]
        pub enum Error {
            InvalidNumericOp,
            Push,
            GetRoot,
            Commit,
            GenerateProof,
            Verify,
            LeafNotFound,
            PalletNotIncluded,
            InvalidLeafIndex,
            InvalidBestKnownBlock,
        }
        #[derive(Clone, Debug)]
        pub struct Proof<_0> {
            pub leaf_indices: ::std::vec::Vec<::core::primitive::u64>,
            pub leaf_count: ::core::primitive::u64,
            pub items: ::std::vec::Vec<_0>,
        }
    }
    pub mod sp_npos_elections {
        use super::my_types;
        #[derive(Clone, Debug)]
        pub struct ElectionScore {
            pub minimal_stake: ::core::primitive::u128,
            pub sum_stake: ::core::primitive::u128,
            pub sum_stake_squared: ::core::primitive::u128,
        }
        #[derive(Clone, Debug)]
        pub struct Support<_0> {
            pub total: ::core::primitive::u128,
            pub voters: ::std::vec::Vec<(_0, ::core::primitive::u128)>,
        }
    }
    pub mod sp_runtime {
        use super::my_types;
        pub mod generic {
            use super::my_types;
            pub mod block {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct Block<_0, _1> {
                    pub header: _0,
                    pub extrinsics: ::std::vec::Vec<_1>,
                }
            }
            pub mod digest {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct Digest {
                    pub logs: ::std::vec::Vec<
                        my_types::sp_runtime::generic::digest::DigestItem,
                    >,
                }
                #[derive(Clone, Debug)]
                pub enum DigestItem {
                    PreRuntime(
                        [::core::primitive::u8; 4usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    Consensus(
                        [::core::primitive::u8; 4usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    Seal(
                        [::core::primitive::u8; 4usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    Other(::std::vec::Vec<::core::primitive::u8>),
                    RuntimeEnvironmentUpdated,
                }
            }
            pub mod era {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub enum Era {
                    Immortal,
                    Mortal1(::core::primitive::u8),
                    Mortal2(::core::primitive::u8),
                    Mortal3(::core::primitive::u8),
                    Mortal4(::core::primitive::u8),
                    Mortal5(::core::primitive::u8),
                    Mortal6(::core::primitive::u8),
                    Mortal7(::core::primitive::u8),
                    Mortal8(::core::primitive::u8),
                    Mortal9(::core::primitive::u8),
                    Mortal10(::core::primitive::u8),
                    Mortal11(::core::primitive::u8),
                    Mortal12(::core::primitive::u8),
                    Mortal13(::core::primitive::u8),
                    Mortal14(::core::primitive::u8),
                    Mortal15(::core::primitive::u8),
                    Mortal16(::core::primitive::u8),
                    Mortal17(::core::primitive::u8),
                    Mortal18(::core::primitive::u8),
                    Mortal19(::core::primitive::u8),
                    Mortal20(::core::primitive::u8),
                    Mortal21(::core::primitive::u8),
                    Mortal22(::core::primitive::u8),
                    Mortal23(::core::primitive::u8),
                    Mortal24(::core::primitive::u8),
                    Mortal25(::core::primitive::u8),
                    Mortal26(::core::primitive::u8),
                    Mortal27(::core::primitive::u8),
                    Mortal28(::core::primitive::u8),
                    Mortal29(::core::primitive::u8),
                    Mortal30(::core::primitive::u8),
                    Mortal31(::core::primitive::u8),
                    Mortal32(::core::primitive::u8),
                    Mortal33(::core::primitive::u8),
                    Mortal34(::core::primitive::u8),
                    Mortal35(::core::primitive::u8),
                    Mortal36(::core::primitive::u8),
                    Mortal37(::core::primitive::u8),
                    Mortal38(::core::primitive::u8),
                    Mortal39(::core::primitive::u8),
                    Mortal40(::core::primitive::u8),
                    Mortal41(::core::primitive::u8),
                    Mortal42(::core::primitive::u8),
                    Mortal43(::core::primitive::u8),
                    Mortal44(::core::primitive::u8),
                    Mortal45(::core::primitive::u8),
                    Mortal46(::core::primitive::u8),
                    Mortal47(::core::primitive::u8),
                    Mortal48(::core::primitive::u8),
                    Mortal49(::core::primitive::u8),
                    Mortal50(::core::primitive::u8),
                    Mortal51(::core::primitive::u8),
                    Mortal52(::core::primitive::u8),
                    Mortal53(::core::primitive::u8),
                    Mortal54(::core::primitive::u8),
                    Mortal55(::core::primitive::u8),
                    Mortal56(::core::primitive::u8),
                    Mortal57(::core::primitive::u8),
                    Mortal58(::core::primitive::u8),
                    Mortal59(::core::primitive::u8),
                    Mortal60(::core::primitive::u8),
                    Mortal61(::core::primitive::u8),
                    Mortal62(::core::primitive::u8),
                    Mortal63(::core::primitive::u8),
                    Mortal64(::core::primitive::u8),
                    Mortal65(::core::primitive::u8),
                    Mortal66(::core::primitive::u8),
                    Mortal67(::core::primitive::u8),
                    Mortal68(::core::primitive::u8),
                    Mortal69(::core::primitive::u8),
                    Mortal70(::core::primitive::u8),
                    Mortal71(::core::primitive::u8),
                    Mortal72(::core::primitive::u8),
                    Mortal73(::core::primitive::u8),
                    Mortal74(::core::primitive::u8),
                    Mortal75(::core::primitive::u8),
                    Mortal76(::core::primitive::u8),
                    Mortal77(::core::primitive::u8),
                    Mortal78(::core::primitive::u8),
                    Mortal79(::core::primitive::u8),
                    Mortal80(::core::primitive::u8),
                    Mortal81(::core::primitive::u8),
                    Mortal82(::core::primitive::u8),
                    Mortal83(::core::primitive::u8),
                    Mortal84(::core::primitive::u8),
                    Mortal85(::core::primitive::u8),
                    Mortal86(::core::primitive::u8),
                    Mortal87(::core::primitive::u8),
                    Mortal88(::core::primitive::u8),
                    Mortal89(::core::primitive::u8),
                    Mortal90(::core::primitive::u8),
                    Mortal91(::core::primitive::u8),
                    Mortal92(::core::primitive::u8),
                    Mortal93(::core::primitive::u8),
                    Mortal94(::core::primitive::u8),
                    Mortal95(::core::primitive::u8),
                    Mortal96(::core::primitive::u8),
                    Mortal97(::core::primitive::u8),
                    Mortal98(::core::primitive::u8),
                    Mortal99(::core::primitive::u8),
                    Mortal100(::core::primitive::u8),
                    Mortal101(::core::primitive::u8),
                    Mortal102(::core::primitive::u8),
                    Mortal103(::core::primitive::u8),
                    Mortal104(::core::primitive::u8),
                    Mortal105(::core::primitive::u8),
                    Mortal106(::core::primitive::u8),
                    Mortal107(::core::primitive::u8),
                    Mortal108(::core::primitive::u8),
                    Mortal109(::core::primitive::u8),
                    Mortal110(::core::primitive::u8),
                    Mortal111(::core::primitive::u8),
                    Mortal112(::core::primitive::u8),
                    Mortal113(::core::primitive::u8),
                    Mortal114(::core::primitive::u8),
                    Mortal115(::core::primitive::u8),
                    Mortal116(::core::primitive::u8),
                    Mortal117(::core::primitive::u8),
                    Mortal118(::core::primitive::u8),
                    Mortal119(::core::primitive::u8),
                    Mortal120(::core::primitive::u8),
                    Mortal121(::core::primitive::u8),
                    Mortal122(::core::primitive::u8),
                    Mortal123(::core::primitive::u8),
                    Mortal124(::core::primitive::u8),
                    Mortal125(::core::primitive::u8),
                    Mortal126(::core::primitive::u8),
                    Mortal127(::core::primitive::u8),
                    Mortal128(::core::primitive::u8),
                    Mortal129(::core::primitive::u8),
                    Mortal130(::core::primitive::u8),
                    Mortal131(::core::primitive::u8),
                    Mortal132(::core::primitive::u8),
                    Mortal133(::core::primitive::u8),
                    Mortal134(::core::primitive::u8),
                    Mortal135(::core::primitive::u8),
                    Mortal136(::core::primitive::u8),
                    Mortal137(::core::primitive::u8),
                    Mortal138(::core::primitive::u8),
                    Mortal139(::core::primitive::u8),
                    Mortal140(::core::primitive::u8),
                    Mortal141(::core::primitive::u8),
                    Mortal142(::core::primitive::u8),
                    Mortal143(::core::primitive::u8),
                    Mortal144(::core::primitive::u8),
                    Mortal145(::core::primitive::u8),
                    Mortal146(::core::primitive::u8),
                    Mortal147(::core::primitive::u8),
                    Mortal148(::core::primitive::u8),
                    Mortal149(::core::primitive::u8),
                    Mortal150(::core::primitive::u8),
                    Mortal151(::core::primitive::u8),
                    Mortal152(::core::primitive::u8),
                    Mortal153(::core::primitive::u8),
                    Mortal154(::core::primitive::u8),
                    Mortal155(::core::primitive::u8),
                    Mortal156(::core::primitive::u8),
                    Mortal157(::core::primitive::u8),
                    Mortal158(::core::primitive::u8),
                    Mortal159(::core::primitive::u8),
                    Mortal160(::core::primitive::u8),
                    Mortal161(::core::primitive::u8),
                    Mortal162(::core::primitive::u8),
                    Mortal163(::core::primitive::u8),
                    Mortal164(::core::primitive::u8),
                    Mortal165(::core::primitive::u8),
                    Mortal166(::core::primitive::u8),
                    Mortal167(::core::primitive::u8),
                    Mortal168(::core::primitive::u8),
                    Mortal169(::core::primitive::u8),
                    Mortal170(::core::primitive::u8),
                    Mortal171(::core::primitive::u8),
                    Mortal172(::core::primitive::u8),
                    Mortal173(::core::primitive::u8),
                    Mortal174(::core::primitive::u8),
                    Mortal175(::core::primitive::u8),
                    Mortal176(::core::primitive::u8),
                    Mortal177(::core::primitive::u8),
                    Mortal178(::core::primitive::u8),
                    Mortal179(::core::primitive::u8),
                    Mortal180(::core::primitive::u8),
                    Mortal181(::core::primitive::u8),
                    Mortal182(::core::primitive::u8),
                    Mortal183(::core::primitive::u8),
                    Mortal184(::core::primitive::u8),
                    Mortal185(::core::primitive::u8),
                    Mortal186(::core::primitive::u8),
                    Mortal187(::core::primitive::u8),
                    Mortal188(::core::primitive::u8),
                    Mortal189(::core::primitive::u8),
                    Mortal190(::core::primitive::u8),
                    Mortal191(::core::primitive::u8),
                    Mortal192(::core::primitive::u8),
                    Mortal193(::core::primitive::u8),
                    Mortal194(::core::primitive::u8),
                    Mortal195(::core::primitive::u8),
                    Mortal196(::core::primitive::u8),
                    Mortal197(::core::primitive::u8),
                    Mortal198(::core::primitive::u8),
                    Mortal199(::core::primitive::u8),
                    Mortal200(::core::primitive::u8),
                    Mortal201(::core::primitive::u8),
                    Mortal202(::core::primitive::u8),
                    Mortal203(::core::primitive::u8),
                    Mortal204(::core::primitive::u8),
                    Mortal205(::core::primitive::u8),
                    Mortal206(::core::primitive::u8),
                    Mortal207(::core::primitive::u8),
                    Mortal208(::core::primitive::u8),
                    Mortal209(::core::primitive::u8),
                    Mortal210(::core::primitive::u8),
                    Mortal211(::core::primitive::u8),
                    Mortal212(::core::primitive::u8),
                    Mortal213(::core::primitive::u8),
                    Mortal214(::core::primitive::u8),
                    Mortal215(::core::primitive::u8),
                    Mortal216(::core::primitive::u8),
                    Mortal217(::core::primitive::u8),
                    Mortal218(::core::primitive::u8),
                    Mortal219(::core::primitive::u8),
                    Mortal220(::core::primitive::u8),
                    Mortal221(::core::primitive::u8),
                    Mortal222(::core::primitive::u8),
                    Mortal223(::core::primitive::u8),
                    Mortal224(::core::primitive::u8),
                    Mortal225(::core::primitive::u8),
                    Mortal226(::core::primitive::u8),
                    Mortal227(::core::primitive::u8),
                    Mortal228(::core::primitive::u8),
                    Mortal229(::core::primitive::u8),
                    Mortal230(::core::primitive::u8),
                    Mortal231(::core::primitive::u8),
                    Mortal232(::core::primitive::u8),
                    Mortal233(::core::primitive::u8),
                    Mortal234(::core::primitive::u8),
                    Mortal235(::core::primitive::u8),
                    Mortal236(::core::primitive::u8),
                    Mortal237(::core::primitive::u8),
                    Mortal238(::core::primitive::u8),
                    Mortal239(::core::primitive::u8),
                    Mortal240(::core::primitive::u8),
                    Mortal241(::core::primitive::u8),
                    Mortal242(::core::primitive::u8),
                    Mortal243(::core::primitive::u8),
                    Mortal244(::core::primitive::u8),
                    Mortal245(::core::primitive::u8),
                    Mortal246(::core::primitive::u8),
                    Mortal247(::core::primitive::u8),
                    Mortal248(::core::primitive::u8),
                    Mortal249(::core::primitive::u8),
                    Mortal250(::core::primitive::u8),
                    Mortal251(::core::primitive::u8),
                    Mortal252(::core::primitive::u8),
                    Mortal253(::core::primitive::u8),
                    Mortal254(::core::primitive::u8),
                    Mortal255(::core::primitive::u8),
                }
            }
            pub mod header {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct Header<_0, _1> {
                    pub parent_hash: my_types::primitive_types::H256,
                    pub number: _0,
                    pub state_root: my_types::primitive_types::H256,
                    pub extrinsics_root: my_types::primitive_types::H256,
                    pub digest: my_types::sp_runtime::generic::digest::Digest,
                    pub __ignore: ::core::marker::PhantomData<_1>,
                }
            }
            pub mod unchecked_extrinsic {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                    pub ::std::vec::Vec<::core::primitive::u8>,
                    pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
                );
            }
        }
        pub mod multiaddress {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub enum MultiAddress<_0, _1> {
                Id(_0),
                Index(_1),
                Raw(::std::vec::Vec<::core::primitive::u8>),
                Address32([::core::primitive::u8; 32usize]),
                Address20([::core::primitive::u8; 20usize]),
            }
        }
        pub mod traits {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct BlakeTwo256;
        }
        pub mod transaction_validity {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub enum InvalidTransaction {
                Call,
                Payment,
                Future,
                Stale,
                BadProof,
                AncientBirthBlock,
                ExhaustsResources,
                Custom(::core::primitive::u8),
                BadMandatory,
                MandatoryValidation,
                BadSigner,
            }
            #[derive(Clone, Debug)]
            pub enum TransactionSource {
                InBlock,
                Local,
                External,
            }
            #[derive(Clone, Debug)]
            pub enum TransactionValidityError {
                Invalid(my_types::sp_runtime::transaction_validity::InvalidTransaction),
                Unknown(my_types::sp_runtime::transaction_validity::UnknownTransaction),
            }
            #[derive(Clone, Debug)]
            pub enum UnknownTransaction {
                CannotLookup,
                NoUnsignedValidator,
                Custom(::core::primitive::u8),
            }
            #[derive(Clone, Debug)]
            pub struct ValidTransaction {
                pub priority: ::core::primitive::u64,
                pub requires: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                pub provides: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                pub longevity: ::core::primitive::u64,
                pub propagate: ::core::primitive::bool,
            }
        }
        #[derive(Clone, Debug)]
        pub enum DispatchError {
            Other,
            CannotLookup,
            BadOrigin,
            Module(my_types::sp_runtime::ModuleError),
            ConsumerRemaining,
            NoProviders,
            TooManyConsumers,
            Token(my_types::sp_runtime::TokenError),
            Arithmetic(my_types::sp_arithmetic::ArithmeticError),
            Transactional(my_types::sp_runtime::TransactionalError),
            Exhausted,
            Corruption,
            Unavailable,
            RootNotAllowed,
        }
        #[derive(Clone, Debug)]
        pub struct DispatchErrorWithPostInfo<_0> {
            pub post_info: _0,
            pub error: my_types::sp_runtime::DispatchError,
        }
        #[derive(Clone, Debug)]
        pub struct ModuleError {
            pub index: ::core::primitive::u8,
            pub error: [::core::primitive::u8; 4usize],
        }
        #[derive(Clone, Debug)]
        pub enum MultiSignature {
            Ed25519(my_types::sp_core::ed25519::Signature),
            Sr25519(my_types::sp_core::sr25519::Signature),
            Ecdsa(my_types::sp_core::ecdsa::Signature),
        }
        #[derive(Clone, Debug)]
        pub enum MultiSigner {
            Ed25519(my_types::sp_core::ed25519::Public),
            Sr25519(my_types::sp_core::sr25519::Public),
            Ecdsa(my_types::sp_core::ecdsa::Public),
        }
        #[derive(Clone, Debug)]
        pub enum TokenError {
            FundsUnavailable,
            OnlyProvider,
            BelowMinimum,
            CannotCreate,
            UnknownAsset,
            Frozen,
            Unsupported,
            CannotCreateHold,
            NotExpendable,
            Blocked,
        }
        #[derive(Clone, Debug)]
        pub enum TransactionalError {
            LimitReached,
            NoLayer,
        }
    }
    pub mod sp_session {
        use super::my_types;
        #[derive(Clone, Debug)]
        pub struct MembershipProof {
            pub session: ::core::primitive::u32,
            pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            pub validator_count: ::core::primitive::u32,
        }
    }
    pub mod sp_staking {
        use super::my_types;
        pub mod offence {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct OffenceDetails<_0, _1> {
                pub offender: _1,
                pub reporters: ::std::vec::Vec<_0>,
            }
        }
    }
    pub mod sp_version {
        use super::my_types;
        #[derive(Clone, Debug)]
        pub struct RuntimeVersion {
            pub spec_name: ::std::string::String,
            pub impl_name: ::std::string::String,
            pub authoring_version: ::core::primitive::u32,
            pub spec_version: ::core::primitive::u32,
            pub impl_version: ::core::primitive::u32,
            pub apis: ::std::vec::Vec<
                ([::core::primitive::u8; 8usize], ::core::primitive::u32),
            >,
            pub transaction_version: ::core::primitive::u32,
            pub state_version: ::core::primitive::u8,
        }
    }
    pub mod sp_weights {
        use super::my_types;
        pub mod weight_v2 {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct Weight {
                pub ref_time: ::core::primitive::u64,
                pub proof_size: ::core::primitive::u64,
            }
        }
        #[derive(Clone, Debug)]
        pub struct RuntimeDbWeight {
            pub read: ::core::primitive::u64,
            pub write: ::core::primitive::u64,
        }
    }
    pub mod xcm {
        use super::my_types;
        pub mod double_encoded {
            use super::my_types;
            #[derive(Clone, Debug)]
            pub struct DoubleEncoded {
                pub encoded: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod v2 {
            use super::my_types;
            pub mod junction {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub enum Junction {
                    Parachain(::core::primitive::u32),
                    AccountId32 {
                        network: my_types::xcm::v2::NetworkId,
                        id: [::core::primitive::u8; 32usize],
                    },
                    AccountIndex64 {
                        network: my_types::xcm::v2::NetworkId,
                        index: ::core::primitive::u64,
                    },
                    AccountKey20 {
                        network: my_types::xcm::v2::NetworkId,
                        key: [::core::primitive::u8; 20usize],
                    },
                    PalletInstance(::core::primitive::u8),
                    GeneralIndex(::core::primitive::u128),
                    GeneralKey(
                        my_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            ::core::primitive::u8,
                        >,
                    ),
                    OnlyChild,
                    Plurality {
                        id: my_types::xcm::v2::BodyId,
                        part: my_types::xcm::v2::BodyPart,
                    },
                }
            }
            pub mod multiasset {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub enum AssetId {
                    Concrete(my_types::xcm::v2::multilocation::MultiLocation),
                    Abstract(::std::vec::Vec<::core::primitive::u8>),
                }
                #[derive(Clone, Debug)]
                pub enum AssetInstance {
                    Undefined,
                    Index(::core::primitive::u128),
                    Array4([::core::primitive::u8; 4usize]),
                    Array8([::core::primitive::u8; 8usize]),
                    Array16([::core::primitive::u8; 16usize]),
                    Array32([::core::primitive::u8; 32usize]),
                    Blob(::std::vec::Vec<::core::primitive::u8>),
                }
                #[derive(Clone, Debug)]
                pub enum Fungibility {
                    Fungible(::core::primitive::u128),
                    NonFungible(my_types::xcm::v2::multiasset::AssetInstance),
                }
                #[derive(Clone, Debug)]
                pub struct MultiAsset {
                    pub id: my_types::xcm::v2::multiasset::AssetId,
                    pub fun: my_types::xcm::v2::multiasset::Fungibility,
                }
                #[derive(Clone, Debug)]
                pub enum MultiAssetFilter {
                    Definite(my_types::xcm::v2::multiasset::MultiAssets),
                    Wild(my_types::xcm::v2::multiasset::WildMultiAsset),
                }
                #[derive(Clone, Debug)]
                pub struct MultiAssets(
                    pub ::std::vec::Vec<my_types::xcm::v2::multiasset::MultiAsset>,
                );
                #[derive(Clone, Debug)]
                pub enum WildFungibility {
                    Fungible,
                    NonFungible,
                }
                #[derive(Clone, Debug)]
                pub enum WildMultiAsset {
                    All,
                    AllOf {
                        id: my_types::xcm::v2::multiasset::AssetId,
                        fun: my_types::xcm::v2::multiasset::WildFungibility,
                    },
                }
            }
            pub mod multilocation {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub enum Junctions {
                    Here,
                    X1(my_types::xcm::v2::junction::Junction),
                    X2(
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                    ),
                    X3(
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                    ),
                    X4(
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                    ),
                    X5(
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                    ),
                    X6(
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                    ),
                    X7(
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                    ),
                    X8(
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                        my_types::xcm::v2::junction::Junction,
                    ),
                }
                #[derive(Clone, Debug)]
                pub struct MultiLocation {
                    pub parents: ::core::primitive::u8,
                    pub interior: my_types::xcm::v2::multilocation::Junctions,
                }
            }
            pub mod traits {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub enum Error {
                    Overflow,
                    Unimplemented,
                    UntrustedReserveLocation,
                    UntrustedTeleportLocation,
                    MultiLocationFull,
                    MultiLocationNotInvertible,
                    BadOrigin,
                    InvalidLocation,
                    AssetNotFound,
                    FailedToTransactAsset,
                    NotWithdrawable,
                    LocationCannotHold,
                    ExceedsMaxMessageSize,
                    DestinationUnsupported,
                    Transport,
                    Unroutable,
                    UnknownClaim,
                    FailedToDecode,
                    MaxWeightInvalid,
                    NotHoldingFees,
                    TooExpensive,
                    Trap(::core::primitive::u64),
                    UnhandledXcmVersion,
                    WeightLimitReached(::core::primitive::u64),
                    Barrier,
                    WeightNotComputable,
                }
            }
            #[derive(Clone, Debug)]
            pub enum BodyId {
                Unit,
                Named(
                    my_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        ::core::primitive::u8,
                    >,
                ),
                Index(::core::primitive::u32),
                Executive,
                Technical,
                Legislative,
                Judicial,
                Defense,
                Administration,
                Treasury,
            }
            #[derive(Clone, Debug)]
            pub enum BodyPart {
                Voice,
                Members { count: ::core::primitive::u32 },
                Fraction { nom: ::core::primitive::u32, denom: ::core::primitive::u32 },
                AtLeastProportion {
                    nom: ::core::primitive::u32,
                    denom: ::core::primitive::u32,
                },
                MoreThanProportion {
                    nom: ::core::primitive::u32,
                    denom: ::core::primitive::u32,
                },
            }
            #[derive(Clone, Debug)]
            pub enum Instruction {
                WithdrawAsset(my_types::xcm::v2::multiasset::MultiAssets),
                ReserveAssetDeposited(my_types::xcm::v2::multiasset::MultiAssets),
                ReceiveTeleportedAsset(my_types::xcm::v2::multiasset::MultiAssets),
                QueryResponse {
                    query_id: ::core::primitive::u64,
                    response: my_types::xcm::v2::Response,
                    max_weight: ::core::primitive::u64,
                },
                TransferAsset {
                    assets: my_types::xcm::v2::multiasset::MultiAssets,
                    beneficiary: my_types::xcm::v2::multilocation::MultiLocation,
                },
                TransferReserveAsset {
                    assets: my_types::xcm::v2::multiasset::MultiAssets,
                    dest: my_types::xcm::v2::multilocation::MultiLocation,
                    xcm: my_types::xcm::v2::Xcm,
                },
                Transact {
                    origin_type: my_types::xcm::v2::OriginKind,
                    require_weight_at_most: ::core::primitive::u64,
                    call: my_types::xcm::double_encoded::DoubleEncoded,
                },
                HrmpNewChannelOpenRequest {
                    sender: ::core::primitive::u32,
                    max_message_size: ::core::primitive::u32,
                    max_capacity: ::core::primitive::u32,
                },
                HrmpChannelAccepted { recipient: ::core::primitive::u32 },
                HrmpChannelClosing {
                    initiator: ::core::primitive::u32,
                    sender: ::core::primitive::u32,
                    recipient: ::core::primitive::u32,
                },
                ClearOrigin,
                DescendOrigin(my_types::xcm::v2::multilocation::Junctions),
                ReportError {
                    query_id: ::core::primitive::u64,
                    dest: my_types::xcm::v2::multilocation::MultiLocation,
                    max_response_weight: ::core::primitive::u64,
                },
                DepositAsset {
                    assets: my_types::xcm::v2::multiasset::MultiAssetFilter,
                    max_assets: ::core::primitive::u32,
                    beneficiary: my_types::xcm::v2::multilocation::MultiLocation,
                },
                DepositReserveAsset {
                    assets: my_types::xcm::v2::multiasset::MultiAssetFilter,
                    max_assets: ::core::primitive::u32,
                    dest: my_types::xcm::v2::multilocation::MultiLocation,
                    xcm: my_types::xcm::v2::Xcm,
                },
                ExchangeAsset {
                    give: my_types::xcm::v2::multiasset::MultiAssetFilter,
                    receive: my_types::xcm::v2::multiasset::MultiAssets,
                },
                InitiateReserveWithdraw {
                    assets: my_types::xcm::v2::multiasset::MultiAssetFilter,
                    reserve: my_types::xcm::v2::multilocation::MultiLocation,
                    xcm: my_types::xcm::v2::Xcm,
                },
                InitiateTeleport {
                    assets: my_types::xcm::v2::multiasset::MultiAssetFilter,
                    dest: my_types::xcm::v2::multilocation::MultiLocation,
                    xcm: my_types::xcm::v2::Xcm,
                },
                QueryHolding {
                    query_id: ::core::primitive::u64,
                    dest: my_types::xcm::v2::multilocation::MultiLocation,
                    assets: my_types::xcm::v2::multiasset::MultiAssetFilter,
                    max_response_weight: ::core::primitive::u64,
                },
                BuyExecution {
                    fees: my_types::xcm::v2::multiasset::MultiAsset,
                    weight_limit: my_types::xcm::v2::WeightLimit,
                },
                RefundSurplus,
                SetErrorHandler(my_types::xcm::v2::Xcm),
                SetAppendix(my_types::xcm::v2::Xcm),
                ClearError,
                ClaimAsset {
                    assets: my_types::xcm::v2::multiasset::MultiAssets,
                    ticket: my_types::xcm::v2::multilocation::MultiLocation,
                },
                Trap(::core::primitive::u64),
                SubscribeVersion {
                    query_id: ::core::primitive::u64,
                    max_response_weight: ::core::primitive::u64,
                },
                UnsubscribeVersion,
            }
            #[derive(Clone, Debug)]
            pub enum NetworkId {
                Any,
                Named(
                    my_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        ::core::primitive::u8,
                    >,
                ),
                Polkadot,
                Kusama,
            }
            #[derive(Clone, Debug)]
            pub enum OriginKind {
                Native,
                SovereignAccount,
                Superuser,
                Xcm,
            }
            #[derive(Clone, Debug)]
            pub enum Response {
                Null,
                Assets(my_types::xcm::v2::multiasset::MultiAssets),
                ExecutionResult(
                    ::core::option::Option<
                        (::core::primitive::u32, my_types::xcm::v2::traits::Error),
                    >,
                ),
                Version(::core::primitive::u32),
            }
            #[derive(Clone, Debug)]
            pub enum WeightLimit {
                Unlimited,
                Limited(::core::primitive::u64),
            }
            #[derive(Clone, Debug)]
            pub struct Xcm(pub ::std::vec::Vec<my_types::xcm::v2::Instruction>);
        }
        pub mod v3 {
            use super::my_types;
            pub mod junction {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub enum BodyId {
                    Unit,
                    Moniker([::core::primitive::u8; 4usize]),
                    Index(::core::primitive::u32),
                    Executive,
                    Technical,
                    Legislative,
                    Judicial,
                    Defense,
                    Administration,
                    Treasury,
                }
                #[derive(Clone, Debug)]
                pub enum BodyPart {
                    Voice,
                    Members { count: ::core::primitive::u32 },
                    Fraction {
                        nom: ::core::primitive::u32,
                        denom: ::core::primitive::u32,
                    },
                    AtLeastProportion {
                        nom: ::core::primitive::u32,
                        denom: ::core::primitive::u32,
                    },
                    MoreThanProportion {
                        nom: ::core::primitive::u32,
                        denom: ::core::primitive::u32,
                    },
                }
                #[derive(Clone, Debug)]
                pub enum Junction {
                    Parachain(::core::primitive::u32),
                    AccountId32 {
                        network: ::core::option::Option<
                            my_types::xcm::v3::junction::NetworkId,
                        >,
                        id: [::core::primitive::u8; 32usize],
                    },
                    AccountIndex64 {
                        network: ::core::option::Option<
                            my_types::xcm::v3::junction::NetworkId,
                        >,
                        index: ::core::primitive::u64,
                    },
                    AccountKey20 {
                        network: ::core::option::Option<
                            my_types::xcm::v3::junction::NetworkId,
                        >,
                        key: [::core::primitive::u8; 20usize],
                    },
                    PalletInstance(::core::primitive::u8),
                    GeneralIndex(::core::primitive::u128),
                    GeneralKey {
                        length: ::core::primitive::u8,
                        data: [::core::primitive::u8; 32usize],
                    },
                    OnlyChild,
                    Plurality {
                        id: my_types::xcm::v3::junction::BodyId,
                        part: my_types::xcm::v3::junction::BodyPart,
                    },
                    GlobalConsensus(my_types::xcm::v3::junction::NetworkId),
                }
                #[derive(Clone, Debug)]
                pub enum NetworkId {
                    ByGenesis([::core::primitive::u8; 32usize]),
                    ByFork {
                        block_number: ::core::primitive::u64,
                        block_hash: [::core::primitive::u8; 32usize],
                    },
                    Polkadot,
                    Kusama,
                    Westend,
                    Rococo,
                    Wococo,
                    Ethereum { chain_id: ::core::primitive::u64 },
                    BitcoinCore,
                    BitcoinCash,
                }
            }
            pub mod junctions {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub enum Junctions {
                    Here,
                    X1(my_types::xcm::v3::junction::Junction),
                    X2(
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                    ),
                    X3(
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                    ),
                    X4(
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                    ),
                    X5(
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                    ),
                    X6(
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                    ),
                    X7(
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                    ),
                    X8(
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                        my_types::xcm::v3::junction::Junction,
                    ),
                }
            }
            pub mod multiasset {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub enum AssetId {
                    Concrete(my_types::xcm::v3::multilocation::MultiLocation),
                    Abstract([::core::primitive::u8; 32usize]),
                }
                #[derive(Clone, Debug)]
                pub enum AssetInstance {
                    Undefined,
                    Index(::core::primitive::u128),
                    Array4([::core::primitive::u8; 4usize]),
                    Array8([::core::primitive::u8; 8usize]),
                    Array16([::core::primitive::u8; 16usize]),
                    Array32([::core::primitive::u8; 32usize]),
                }
                #[derive(Clone, Debug)]
                pub enum Fungibility {
                    Fungible(::core::primitive::u128),
                    NonFungible(my_types::xcm::v3::multiasset::AssetInstance),
                }
                #[derive(Clone, Debug)]
                pub struct MultiAsset {
                    pub id: my_types::xcm::v3::multiasset::AssetId,
                    pub fun: my_types::xcm::v3::multiasset::Fungibility,
                }
                #[derive(Clone, Debug)]
                pub enum MultiAssetFilter {
                    Definite(my_types::xcm::v3::multiasset::MultiAssets),
                    Wild(my_types::xcm::v3::multiasset::WildMultiAsset),
                }
                #[derive(Clone, Debug)]
                pub struct MultiAssets(
                    pub ::std::vec::Vec<my_types::xcm::v3::multiasset::MultiAsset>,
                );
                #[derive(Clone, Debug)]
                pub enum WildFungibility {
                    Fungible,
                    NonFungible,
                }
                #[derive(Clone, Debug)]
                pub enum WildMultiAsset {
                    All,
                    AllOf {
                        id: my_types::xcm::v3::multiasset::AssetId,
                        fun: my_types::xcm::v3::multiasset::WildFungibility,
                    },
                    AllCounted(::core::primitive::u32),
                    AllOfCounted {
                        id: my_types::xcm::v3::multiasset::AssetId,
                        fun: my_types::xcm::v3::multiasset::WildFungibility,
                        count: ::core::primitive::u32,
                    },
                }
            }
            pub mod multilocation {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub struct MultiLocation {
                    pub parents: ::core::primitive::u8,
                    pub interior: my_types::xcm::v3::junctions::Junctions,
                }
            }
            pub mod traits {
                use super::my_types;
                #[derive(Clone, Debug)]
                pub enum Error {
                    Overflow,
                    Unimplemented,
                    UntrustedReserveLocation,
                    UntrustedTeleportLocation,
                    LocationFull,
                    LocationNotInvertible,
                    BadOrigin,
                    InvalidLocation,
                    AssetNotFound,
                    FailedToTransactAsset,
                    NotWithdrawable,
                    LocationCannotHold,
                    ExceedsMaxMessageSize,
                    DestinationUnsupported,
                    Transport,
                    Unroutable,
                    UnknownClaim,
                    FailedToDecode,
                    MaxWeightInvalid,
                    NotHoldingFees,
                    TooExpensive,
                    Trap(::core::primitive::u64),
                    ExpectationFalse,
                    PalletNotFound,
                    NameMismatch,
                    VersionIncompatible,
                    HoldingWouldOverflow,
                    ExportError,
                    ReanchorFailed,
                    NoDeal,
                    FeesNotMet,
                    LockError,
                    NoPermission,
                    Unanchored,
                    NotDepositable,
                    UnhandledXcmVersion,
                    WeightLimitReached(my_types::sp_weights::weight_v2::Weight),
                    Barrier,
                    WeightNotComputable,
                    ExceedsStackLimit,
                }
                #[derive(Clone, Debug)]
                pub enum Outcome {
                    Complete(my_types::sp_weights::weight_v2::Weight),
                    Incomplete(
                        my_types::sp_weights::weight_v2::Weight,
                        my_types::xcm::v3::traits::Error,
                    ),
                    Error(my_types::xcm::v3::traits::Error),
                }
            }
            #[derive(Clone, Debug)]
            pub enum Instruction {
                WithdrawAsset(my_types::xcm::v3::multiasset::MultiAssets),
                ReserveAssetDeposited(my_types::xcm::v3::multiasset::MultiAssets),
                ReceiveTeleportedAsset(my_types::xcm::v3::multiasset::MultiAssets),
                QueryResponse {
                    query_id: ::core::primitive::u64,
                    response: my_types::xcm::v3::Response,
                    max_weight: my_types::sp_weights::weight_v2::Weight,
                    querier: ::core::option::Option<
                        my_types::xcm::v3::multilocation::MultiLocation,
                    >,
                },
                TransferAsset {
                    assets: my_types::xcm::v3::multiasset::MultiAssets,
                    beneficiary: my_types::xcm::v3::multilocation::MultiLocation,
                },
                TransferReserveAsset {
                    assets: my_types::xcm::v3::multiasset::MultiAssets,
                    dest: my_types::xcm::v3::multilocation::MultiLocation,
                    xcm: my_types::xcm::v3::Xcm,
                },
                Transact {
                    origin_kind: my_types::xcm::v2::OriginKind,
                    require_weight_at_most: my_types::sp_weights::weight_v2::Weight,
                    call: my_types::xcm::double_encoded::DoubleEncoded,
                },
                HrmpNewChannelOpenRequest {
                    sender: ::core::primitive::u32,
                    max_message_size: ::core::primitive::u32,
                    max_capacity: ::core::primitive::u32,
                },
                HrmpChannelAccepted { recipient: ::core::primitive::u32 },
                HrmpChannelClosing {
                    initiator: ::core::primitive::u32,
                    sender: ::core::primitive::u32,
                    recipient: ::core::primitive::u32,
                },
                ClearOrigin,
                DescendOrigin(my_types::xcm::v3::junctions::Junctions),
                ReportError(my_types::xcm::v3::QueryResponseInfo),
                DepositAsset {
                    assets: my_types::xcm::v3::multiasset::MultiAssetFilter,
                    beneficiary: my_types::xcm::v3::multilocation::MultiLocation,
                },
                DepositReserveAsset {
                    assets: my_types::xcm::v3::multiasset::MultiAssetFilter,
                    dest: my_types::xcm::v3::multilocation::MultiLocation,
                    xcm: my_types::xcm::v3::Xcm,
                },
                ExchangeAsset {
                    give: my_types::xcm::v3::multiasset::MultiAssetFilter,
                    want: my_types::xcm::v3::multiasset::MultiAssets,
                    maximal: ::core::primitive::bool,
                },
                InitiateReserveWithdraw {
                    assets: my_types::xcm::v3::multiasset::MultiAssetFilter,
                    reserve: my_types::xcm::v3::multilocation::MultiLocation,
                    xcm: my_types::xcm::v3::Xcm,
                },
                InitiateTeleport {
                    assets: my_types::xcm::v3::multiasset::MultiAssetFilter,
                    dest: my_types::xcm::v3::multilocation::MultiLocation,
                    xcm: my_types::xcm::v3::Xcm,
                },
                ReportHolding {
                    response_info: my_types::xcm::v3::QueryResponseInfo,
                    assets: my_types::xcm::v3::multiasset::MultiAssetFilter,
                },
                BuyExecution {
                    fees: my_types::xcm::v3::multiasset::MultiAsset,
                    weight_limit: my_types::xcm::v3::WeightLimit,
                },
                RefundSurplus,
                SetErrorHandler(my_types::xcm::v3::Xcm),
                SetAppendix(my_types::xcm::v3::Xcm),
                ClearError,
                ClaimAsset {
                    assets: my_types::xcm::v3::multiasset::MultiAssets,
                    ticket: my_types::xcm::v3::multilocation::MultiLocation,
                },
                Trap(::core::primitive::u64),
                SubscribeVersion {
                    query_id: ::core::primitive::u64,
                    max_response_weight: my_types::sp_weights::weight_v2::Weight,
                },
                UnsubscribeVersion,
                BurnAsset(my_types::xcm::v3::multiasset::MultiAssets),
                ExpectAsset(my_types::xcm::v3::multiasset::MultiAssets),
                ExpectOrigin(
                    ::core::option::Option<
                        my_types::xcm::v3::multilocation::MultiLocation,
                    >,
                ),
                ExpectError(
                    ::core::option::Option<
                        (::core::primitive::u32, my_types::xcm::v3::traits::Error),
                    >,
                ),
                ExpectTransactStatus(my_types::xcm::v3::MaybeErrorCode),
                QueryPallet {
                    module_name: ::std::vec::Vec<::core::primitive::u8>,
                    response_info: my_types::xcm::v3::QueryResponseInfo,
                },
                ExpectPallet {
                    index: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    module_name: ::std::vec::Vec<::core::primitive::u8>,
                    crate_major: ::core::primitive::u32,
                    min_crate_minor: ::core::primitive::u32,
                },
                ReportTransactStatus(my_types::xcm::v3::QueryResponseInfo),
                ClearTransactStatus,
                UniversalOrigin(my_types::xcm::v3::junction::Junction),
                ExportMessage {
                    network: my_types::xcm::v3::junction::NetworkId,
                    destination: my_types::xcm::v3::junctions::Junctions,
                    xcm: my_types::xcm::v3::Xcm,
                },
                LockAsset {
                    asset: my_types::xcm::v3::multiasset::MultiAsset,
                    unlocker: my_types::xcm::v3::multilocation::MultiLocation,
                },
                UnlockAsset {
                    asset: my_types::xcm::v3::multiasset::MultiAsset,
                    target: my_types::xcm::v3::multilocation::MultiLocation,
                },
                NoteUnlockable {
                    asset: my_types::xcm::v3::multiasset::MultiAsset,
                    owner: my_types::xcm::v3::multilocation::MultiLocation,
                },
                RequestUnlock {
                    asset: my_types::xcm::v3::multiasset::MultiAsset,
                    locker: my_types::xcm::v3::multilocation::MultiLocation,
                },
                SetFeesMode { jit_withdraw: ::core::primitive::bool },
                SetTopic([::core::primitive::u8; 32usize]),
                ClearTopic,
                AliasOrigin(my_types::xcm::v3::multilocation::MultiLocation),
                UnpaidExecution {
                    weight_limit: my_types::xcm::v3::WeightLimit,
                    check_origin: ::core::option::Option<
                        my_types::xcm::v3::multilocation::MultiLocation,
                    >,
                },
            }
            #[derive(Clone, Debug)]
            pub enum MaybeErrorCode {
                Success,
                Error(
                    my_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ),
                TruncatedError(
                    my_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ),
            }
            #[derive(Clone, Debug)]
            pub struct PalletInfo {
                pub index: ::core::primitive::u32,
                pub name: my_types::bounded_collections::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub module_name: my_types::bounded_collections::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub major: ::core::primitive::u32,
                pub minor: ::core::primitive::u32,
                pub patch: ::core::primitive::u32,
            }
            #[derive(Clone, Debug)]
            pub struct QueryResponseInfo {
                pub destination: my_types::xcm::v3::multilocation::MultiLocation,
                pub query_id: ::core::primitive::u64,
                pub max_weight: my_types::sp_weights::weight_v2::Weight,
            }
            #[derive(Clone, Debug)]
            pub enum Response {
                Null,
                Assets(my_types::xcm::v3::multiasset::MultiAssets),
                ExecutionResult(
                    ::core::option::Option<
                        (::core::primitive::u32, my_types::xcm::v3::traits::Error),
                    >,
                ),
                Version(::core::primitive::u32),
                PalletsInfo(
                    my_types::bounded_collections::bounded_vec::BoundedVec<
                        my_types::xcm::v3::PalletInfo,
                    >,
                ),
                DispatchResult(my_types::xcm::v3::MaybeErrorCode),
            }
            #[derive(Clone, Debug)]
            pub enum WeightLimit {
                Unlimited,
                Limited(my_types::sp_weights::weight_v2::Weight),
            }
            #[derive(Clone, Debug)]
            pub struct Xcm(pub ::std::vec::Vec<my_types::xcm::v3::Instruction>);
        }
        #[derive(Clone, Debug)]
        pub enum VersionedAssetId {
            V3(my_types::xcm::v3::multiasset::AssetId),
        }
        #[derive(Clone, Debug)]
        pub enum VersionedMultiAssets {
            V2(my_types::xcm::v2::multiasset::MultiAssets),
            V3(my_types::xcm::v3::multiasset::MultiAssets),
        }
        #[derive(Clone, Debug)]
        pub enum VersionedMultiLocation {
            V2(my_types::xcm::v2::multilocation::MultiLocation),
            V3(my_types::xcm::v3::multilocation::MultiLocation),
        }
        #[derive(Clone, Debug)]
        pub enum VersionedResponse {
            V2(my_types::xcm::v2::Response),
            V3(my_types::xcm::v3::Response),
        }
        #[derive(Clone, Debug)]
        pub enum VersionedXcm {
            V2(my_types::xcm::v2::Xcm),
            V3(my_types::xcm::v3::Xcm),
        }
    }
}
