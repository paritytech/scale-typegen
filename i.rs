pub mod runtime_types {
    use super::runtime_types;
    pub mod bounded_collections {
        use super::runtime_types;
        pub mod bounded_btree_map {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct BoundedBTreeMap<_0, _1>(pub ::subxt::utils::KeyedVec<_0, _1>);
        }
        pub mod bounded_vec {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct BoundedVec<_0>(pub ::std::vec::Vec<_0>);
        }
        pub mod weak_bounded_vec {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct WeakBoundedVec<_0>(pub ::std::vec::Vec<_0>);
        }
    }
    pub mod finality_grandpa {
        use super::runtime_types;
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Equivocation<_0, _1, _2> {
            pub round_number: ::core::primitive::u64,
            pub identity: _0,
            pub first: (_1, _2),
            pub second: (_1, _2),
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Precommit<_0, _1> {
            pub target_hash: _0,
            pub target_number: _1,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Prevote<_0, _1> {
            pub target_hash: _0,
            pub target_number: _1,
        }
    }
    pub mod frame_support {
        use super::runtime_types;
        pub mod dispatch {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum DispatchClass {
                #[codec(index = 0)]
                Normal,
                #[codec(index = 1)]
                Operational,
                #[codec(index = 2)]
                Mandatory,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct DispatchInfo {
                pub weight: runtime_types::sp_weights::weight_v2::Weight,
                pub class: runtime_types::frame_support::dispatch::DispatchClass,
                pub pays_fee: runtime_types::frame_support::dispatch::Pays,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Pays {
                #[codec(index = 0)]
                Yes,
                #[codec(index = 1)]
                No,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PerDispatchClass<_0> {
                pub normal: _0,
                pub operational: _0,
                pub mandatory: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PostDispatchInfo {
                pub actual_weight:
                    ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                pub pays_fee: runtime_types::frame_support::dispatch::Pays,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum RawOrigin<_0> {
                #[codec(index = 0)]
                Root,
                #[codec(index = 1)]
                Signed(_0),
                #[codec(index = 2)]
                None,
            }
        }
        pub mod traits {
            use super::runtime_types;
            pub mod messages {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum ProcessMessageError {
                    #[codec(index = 0)]
                    BadFormat,
                    #[codec(index = 1)]
                    Corrupt,
                    #[codec(index = 2)]
                    Unsupported,
                    #[codec(index = 3)]
                    Overweight(runtime_types::sp_weights::weight_v2::Weight),
                    #[codec(index = 4)]
                    Yield,
                }
            }
            pub mod preimages {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Bounded<_0> {
                    #[codec(index = 0)]
                    Legacy {
                        hash: ::subxt::utils::H256,
                    },
                    #[codec(index = 1)]
                    Inline(
                        runtime_types::bounded_collections::bounded_vec::BoundedVec<
                            ::core::primitive::u8,
                        >,
                    ),
                    #[codec(index = 2)]
                    Lookup {
                        hash: ::subxt::utils::H256,
                        len: ::core::primitive::u32,
                    },
                    __Ignore(::core::marker::PhantomData<_0>),
                }
            }
            pub mod schedule {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum DispatchTime<_0> {
                    #[codec(index = 0)]
                    At(_0),
                    #[codec(index = 1)]
                    After(_0),
                }
            }
            pub mod tokens {
                use super::runtime_types;
                pub mod misc {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum BalanceStatus {
                        #[codec(index = 0)]
                        Free,
                        #[codec(index = 1)]
                        Reserved,
                    }
                }
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct PalletId(pub [::core::primitive::u8; 8usize]);
    }
    pub mod frame_system {
        use super::runtime_types;
        pub mod extensions {
            use super::runtime_types;
            pub mod check_genesis {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CheckGenesis;
            }
            pub mod check_mortality {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CheckMortality(pub runtime_types::sp_runtime::generic::era::Era);
            }
            pub mod check_non_zero_sender {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CheckNonZeroSender;
            }
            pub mod check_nonce {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CheckNonce(#[codec(compact)] pub ::core::primitive::u32);
            }
            pub mod check_spec_version {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CheckSpecVersion;
            }
            pub mod check_tx_version {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CheckTxVersion;
            }
            pub mod check_weight {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct CheckWeight;
            }
        }
        pub mod limits {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct BlockLength {
                pub max: runtime_types::frame_support::dispatch::PerDispatchClass<
                    ::core::primitive::u32,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct BlockWeights {
                pub base_block: runtime_types::sp_weights::weight_v2::Weight,
                pub max_block: runtime_types::sp_weights::weight_v2::Weight,
                pub per_class: runtime_types::frame_support::dispatch::PerDispatchClass<
                    runtime_types::frame_system::limits::WeightsPerClass,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct WeightsPerClass {
                pub base_extrinsic: runtime_types::sp_weights::weight_v2::Weight,
                pub max_extrinsic:
                    ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                pub max_total: ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
                pub reserved: ::core::option::Option<runtime_types::sp_weights::weight_v2::Weight>,
            }
        }
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::remark`]."]
                remark {
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::set_heap_pages`]."]
                set_heap_pages { pages: ::core::primitive::u64 },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::set_code`]."]
                set_code {
                    code: ::std::vec::Vec<::core::primitive::u8>,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::set_code_without_checks`]."]
                set_code_without_checks {
                    code: ::std::vec::Vec<::core::primitive::u8>,
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::set_storage`]."]
                set_storage {
                    items: ::std::vec::Vec<(
                        ::std::vec::Vec<::core::primitive::u8>,
                        ::std::vec::Vec<::core::primitive::u8>,
                    )>,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::kill_storage`]."]
                kill_storage {
                    keys: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                },
                #[codec(index = 6)]
                #[doc = "See [`Pallet::kill_prefix`]."]
                kill_prefix {
                    prefix: ::std::vec::Vec<::core::primitive::u8>,
                    subkeys: ::core::primitive::u32,
                },
                #[codec(index = 7)]
                #[doc = "See [`Pallet::remark_with_event`]."]
                remark_with_event {
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Error for the System pallet"]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "The name of specification does not match between the current runtime"]
                #[doc = "and the new runtime."]
                InvalidSpecName,
                #[codec(index = 1)]
                #[doc = "The specification version is not allowed to decrease between the current runtime"]
                #[doc = "and the new runtime."]
                SpecVersionNeedsToIncrease,
                #[codec(index = 2)]
                #[doc = "Failed to extract the runtime version from the new runtime."]
                #[doc = ""]
                #[doc = "Either calling `Core_version` or decoding `RuntimeVersion` failed."]
                FailedToExtractRuntimeVersion,
                #[codec(index = 3)]
                #[doc = "Suicide called when the account has non-default composite data."]
                NonDefaultComposite,
                #[codec(index = 4)]
                #[doc = "There is a non-zero reference count preventing the account from being purged."]
                NonZeroRefCount,
                #[codec(index = 5)]
                #[doc = "The origin filter prevent the call to be dispatched."]
                CallFiltered,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Event for the System pallet."]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "An extrinsic completed successfully."]
                ExtrinsicSuccess {
                    dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                },
                #[codec(index = 1)]
                #[doc = "An extrinsic failed."]
                ExtrinsicFailed {
                    dispatch_error: runtime_types::sp_runtime::DispatchError,
                    dispatch_info: runtime_types::frame_support::dispatch::DispatchInfo,
                },
                #[codec(index = 2)]
                #[doc = "`:code` was updated."]
                CodeUpdated,
                #[codec(index = 3)]
                #[doc = "A new account was created."]
                NewAccount {
                    account: ::subxt::utils::AccountId32,
                },
                #[codec(index = 4)]
                #[doc = "An account was reaped."]
                KilledAccount {
                    account: ::subxt::utils::AccountId32,
                },
                #[codec(index = 5)]
                #[doc = "On on-chain remark happened."]
                Remarked {
                    sender: ::subxt::utils::AccountId32,
                    hash: ::subxt::utils::H256,
                },
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct AccountInfo<_0, _1> {
            pub nonce: _0,
            pub consumers: ::core::primitive::u32,
            pub providers: ::core::primitive::u32,
            pub sufficients: ::core::primitive::u32,
            pub data: _1,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct EventRecord<_0, _1> {
            pub phase: runtime_types::frame_system::Phase,
            pub event: _0,
            pub topics: ::std::vec::Vec<_1>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct LastRuntimeUpgradeInfo {
            #[codec(compact)]
            pub spec_version: ::core::primitive::u32,
            pub spec_name: ::std::string::String,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum Phase {
            #[codec(index = 0)]
            ApplyExtrinsic(::core::primitive::u32),
            #[codec(index = 1)]
            Finalization,
            #[codec(index = 2)]
            Initialization,
        }
    }
    pub mod pallet_babe {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::report_equivocation`]."]
                report_equivocation {
                    equivocation_proof: ::std::boxed::Box<
                        runtime_types::sp_consensus_slots::EquivocationProof<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                            runtime_types::sp_consensus_babe::app::Public,
                        >,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::report_equivocation_unsigned`]."]
                report_equivocation_unsigned {
                    equivocation_proof: ::std::boxed::Box<
                        runtime_types::sp_consensus_slots::EquivocationProof<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                            runtime_types::sp_consensus_babe::app::Public,
                        >,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::plan_config_change`]."]
                plan_config_change {
                    config: runtime_types::sp_consensus_babe::digests::NextConfigDescriptor,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
                InvalidEquivocationProof,
                #[codec(index = 1)]
                #[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
                InvalidKeyOwnershipProof,
                #[codec(index = 2)]
                #[doc = "A given equivocation report is valid but already previously reported."]
                DuplicateOffenceReport,
                #[codec(index = 3)]
                #[doc = "Submitted configuration is invalid."]
                InvalidConfiguration,
            }
        }
    }
    pub mod pallet_bags_list {
        use super::runtime_types;
        pub mod list {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Bag {
                pub head: ::core::option::Option<::subxt::utils::AccountId32>,
                pub tail: ::core::option::Option<::subxt::utils::AccountId32>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum ListError {
                #[codec(index = 0)]
                Duplicate,
                #[codec(index = 1)]
                NotHeavier,
                #[codec(index = 2)]
                NotInSameBag,
                #[codec(index = 3)]
                NodeNotFound,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Node {
                pub id: ::subxt::utils::AccountId32,
                pub prev: ::core::option::Option<::subxt::utils::AccountId32>,
                pub next: ::core::option::Option<::subxt::utils::AccountId32>,
                pub bag_upper: ::core::primitive::u64,
                pub score: ::core::primitive::u64,
            }
        }
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::rebag`]."]
                rebag {
                    dislocated: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::put_in_front_of`]."]
                put_in_front_of {
                    lighter: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "A error in the list interface implementation."]
                List(runtime_types::pallet_bags_list::list::ListError),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "Moved an account from one bag to another."]
                Rebagged {
                    who: ::subxt::utils::AccountId32,
                    from: ::core::primitive::u64,
                    to: ::core::primitive::u64,
                },
                #[codec(index = 1)]
                #[doc = "Updated the score of some account to the given amount."]
                ScoreUpdated {
                    who: ::subxt::utils::AccountId32,
                    new_score: ::core::primitive::u64,
                },
            }
        }
    }
    pub mod pallet_balances {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::transfer_allow_death`]."]
                transfer_allow_death {
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    value: ::core::primitive::u128,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::set_balance_deprecated`]."]
                set_balance_deprecated {
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    new_free: ::core::primitive::u128,
                    #[codec(compact)]
                    old_reserved: ::core::primitive::u128,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::force_transfer`]."]
                force_transfer {
                    source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    value: ::core::primitive::u128,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::transfer_keep_alive`]."]
                transfer_keep_alive {
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    value: ::core::primitive::u128,
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::transfer_all`]."]
                transfer_all {
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    keep_alive: ::core::primitive::bool,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::force_unreserve`]."]
                force_unreserve {
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 6)]
                #[doc = "See [`Pallet::upgrade_accounts`]."]
                upgrade_accounts {
                    who: ::std::vec::Vec<::subxt::utils::AccountId32>,
                },
                #[codec(index = 7)]
                #[doc = "See [`Pallet::transfer`]."]
                transfer {
                    dest: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    value: ::core::primitive::u128,
                },
                #[codec(index = 8)]
                #[doc = "See [`Pallet::force_set_balance`]."]
                force_set_balance {
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    new_free: ::core::primitive::u128,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Vesting balance too high to send value."]
                VestingBalance,
                #[codec(index = 1)]
                #[doc = "Account liquidity restrictions prevent withdrawal."]
                LiquidityRestrictions,
                #[codec(index = 2)]
                #[doc = "Balance too low to send value."]
                InsufficientBalance,
                #[codec(index = 3)]
                #[doc = "Value too low to create account due to existential deposit."]
                ExistentialDeposit,
                #[codec(index = 4)]
                #[doc = "Transfer/payment would kill account."]
                Expendability,
                #[codec(index = 5)]
                #[doc = "A vesting schedule already exists for this account."]
                ExistingVestingSchedule,
                #[codec(index = 6)]
                #[doc = "Beneficiary account must pre-exist."]
                DeadAccount,
                #[codec(index = 7)]
                #[doc = "Number of named reserves exceed `MaxReserves`."]
                TooManyReserves,
                #[codec(index = 8)]
                #[doc = "Number of holds exceed `MaxHolds`."]
                TooManyHolds,
                #[codec(index = 9)]
                #[doc = "Number of freezes exceed `MaxFreezes`."]
                TooManyFreezes,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "An account was created with some free balance."]
                Endowed {
                    account: ::subxt::utils::AccountId32,
                    free_balance: ::core::primitive::u128,
                },
                #[codec(index = 1)]
                #[doc = "An account was removed whose balance was non-zero but below ExistentialDeposit,"]
                #[doc = "resulting in an outright loss."]
                DustLost {
                    account: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 2)]
                #[doc = "Transfer succeeded."]
                Transfer {
                    from: ::subxt::utils::AccountId32,
                    to: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 3)]
                #[doc = "A balance was set by root."]
                BalanceSet {
                    who: ::subxt::utils::AccountId32,
                    free: ::core::primitive::u128,
                },
                #[codec(index = 4)]
                #[doc = "Some balance was reserved (moved from free to reserved)."]
                Reserved {
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 5)]
                #[doc = "Some balance was unreserved (moved from reserved to free)."]
                Unreserved {
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 6)]
                #[doc = "Some balance was moved from the reserve of the first account to the second account."]
                #[doc = "Final argument indicates the destination balance type."]
                ReserveRepatriated {
                    from: ::subxt::utils::AccountId32,
                    to: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                    destination_status:
                        runtime_types::frame_support::traits::tokens::misc::BalanceStatus,
                },
                #[codec(index = 7)]
                #[doc = "Some amount was deposited (e.g. for transaction fees)."]
                Deposit {
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 8)]
                #[doc = "Some amount was withdrawn from the account (e.g. for transaction fees)."]
                Withdraw {
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 9)]
                #[doc = "Some amount was removed from the account (e.g. for misbehavior)."]
                Slashed {
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 10)]
                #[doc = "Some amount was minted into an account."]
                Minted {
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 11)]
                #[doc = "Some amount was burned from an account."]
                Burned {
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 12)]
                #[doc = "Some amount was suspended from an account (it can be restored later)."]
                Suspended {
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 13)]
                #[doc = "Some amount was restored into an account."]
                Restored {
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 14)]
                #[doc = "An account was upgraded."]
                Upgraded { who: ::subxt::utils::AccountId32 },
                #[codec(index = 15)]
                #[doc = "Total issuance was increased by `amount`, creating a credit to be balanced."]
                Issued { amount: ::core::primitive::u128 },
                #[codec(index = 16)]
                #[doc = "Total issuance was decreased by `amount`, creating a debt to be balanced."]
                Rescinded { amount: ::core::primitive::u128 },
                #[codec(index = 17)]
                #[doc = "Some balance was locked."]
                Locked {
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 18)]
                #[doc = "Some balance was unlocked."]
                Unlocked {
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 19)]
                #[doc = "Some balance was frozen."]
                Frozen {
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 20)]
                #[doc = "Some balance was thawed."]
                Thawed {
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
            }
        }
        pub mod types {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AccountData<_0> {
                pub free: _0,
                pub reserved: _0,
                pub frozen: _0,
                pub flags: runtime_types::pallet_balances::types::ExtraFlags,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct BalanceLock<_0> {
                pub id: [::core::primitive::u8; 8usize],
                pub amount: _0,
                pub reasons: runtime_types::pallet_balances::types::Reasons,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ExtraFlags(pub ::core::primitive::u128);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct IdAmount<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Reasons {
                #[codec(index = 0)]
                Fee,
                #[codec(index = 1)]
                Misc,
                #[codec(index = 2)]
                All,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ReserveData<_0, _1> {
                pub id: _0,
                pub amount: _1,
            }
        }
    }
    pub mod pallet_bounties {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::propose_bounty`]."]
                propose_bounty {
                    #[codec(compact)]
                    value: ::core::primitive::u128,
                    description: ::std::vec::Vec<::core::primitive::u8>,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::approve_bounty`]."]
                approve_bounty {
                    #[codec(compact)]
                    bounty_id: ::core::primitive::u32,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::propose_curator`]."]
                propose_curator {
                    #[codec(compact)]
                    bounty_id: ::core::primitive::u32,
                    curator: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    fee: ::core::primitive::u128,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::unassign_curator`]."]
                unassign_curator {
                    #[codec(compact)]
                    bounty_id: ::core::primitive::u32,
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::accept_curator`]."]
                accept_curator {
                    #[codec(compact)]
                    bounty_id: ::core::primitive::u32,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::award_bounty`]."]
                award_bounty {
                    #[codec(compact)]
                    bounty_id: ::core::primitive::u32,
                    beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 6)]
                #[doc = "See [`Pallet::claim_bounty`]."]
                claim_bounty {
                    #[codec(compact)]
                    bounty_id: ::core::primitive::u32,
                },
                #[codec(index = 7)]
                #[doc = "See [`Pallet::close_bounty`]."]
                close_bounty {
                    #[codec(compact)]
                    bounty_id: ::core::primitive::u32,
                },
                #[codec(index = 8)]
                #[doc = "See [`Pallet::extend_bounty_expiry`]."]
                extend_bounty_expiry {
                    #[codec(compact)]
                    bounty_id: ::core::primitive::u32,
                    remark: ::std::vec::Vec<::core::primitive::u8>,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Proposer's balance is too low."]
                InsufficientProposersBalance,
                #[codec(index = 1)]
                #[doc = "No proposal or bounty at that index."]
                InvalidIndex,
                #[codec(index = 2)]
                #[doc = "The reason given is just too big."]
                ReasonTooBig,
                #[codec(index = 3)]
                #[doc = "The bounty status is unexpected."]
                UnexpectedStatus,
                #[codec(index = 4)]
                #[doc = "Require bounty curator."]
                RequireCurator,
                #[codec(index = 5)]
                #[doc = "Invalid bounty value."]
                InvalidValue,
                #[codec(index = 6)]
                #[doc = "Invalid bounty fee."]
                InvalidFee,
                #[codec(index = 7)]
                #[doc = "A bounty payout is pending."]
                #[doc = "To cancel the bounty, you must unassign and slash the curator."]
                PendingPayout,
                #[codec(index = 8)]
                #[doc = "The bounties cannot be claimed/closed because it's still in the countdown period."]
                Premature,
                #[codec(index = 9)]
                #[doc = "The bounty cannot be closed because it has active child bounties."]
                HasActiveChildBounty,
                #[codec(index = 10)]
                #[doc = "Too many approvals are already queued."]
                TooManyQueued,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "New bounty proposal."]
                BountyProposed { index: ::core::primitive::u32 },
                #[codec(index = 1)]
                #[doc = "A bounty proposal was rejected; funds were slashed."]
                BountyRejected {
                    index: ::core::primitive::u32,
                    bond: ::core::primitive::u128,
                },
                #[codec(index = 2)]
                #[doc = "A bounty proposal is funded and became active."]
                BountyBecameActive { index: ::core::primitive::u32 },
                #[codec(index = 3)]
                #[doc = "A bounty is awarded to a beneficiary."]
                BountyAwarded {
                    index: ::core::primitive::u32,
                    beneficiary: ::subxt::utils::AccountId32,
                },
                #[codec(index = 4)]
                #[doc = "A bounty is claimed by beneficiary."]
                BountyClaimed {
                    index: ::core::primitive::u32,
                    payout: ::core::primitive::u128,
                    beneficiary: ::subxt::utils::AccountId32,
                },
                #[codec(index = 5)]
                #[doc = "A bounty is cancelled."]
                BountyCanceled { index: ::core::primitive::u32 },
                #[codec(index = 6)]
                #[doc = "A bounty expiry is extended."]
                BountyExtended { index: ::core::primitive::u32 },
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Bounty<_0, _1, _2> {
            pub proposer: _0,
            pub value: _1,
            pub fee: _1,
            pub curator_deposit: _1,
            pub bond: _1,
            pub status: runtime_types::pallet_bounties::BountyStatus<_0, _2>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum BountyStatus<_0, _1> {
            #[codec(index = 0)]
            Proposed,
            #[codec(index = 1)]
            Approved,
            #[codec(index = 2)]
            Funded,
            #[codec(index = 3)]
            CuratorProposed { curator: _0 },
            #[codec(index = 4)]
            Active { curator: _0, update_due: _1 },
            #[codec(index = 5)]
            PendingPayout {
                curator: _0,
                beneficiary: _0,
                unlock_at: _1,
            },
        }
    }
    pub mod pallet_child_bounties {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::add_child_bounty`]."]
                add_child_bounty {
                    #[codec(compact)]
                    parent_bounty_id: ::core::primitive::u32,
                    #[codec(compact)]
                    value: ::core::primitive::u128,
                    description: ::std::vec::Vec<::core::primitive::u8>,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::propose_curator`]."]
                propose_curator {
                    #[codec(compact)]
                    parent_bounty_id: ::core::primitive::u32,
                    #[codec(compact)]
                    child_bounty_id: ::core::primitive::u32,
                    curator: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    fee: ::core::primitive::u128,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::accept_curator`]."]
                accept_curator {
                    #[codec(compact)]
                    parent_bounty_id: ::core::primitive::u32,
                    #[codec(compact)]
                    child_bounty_id: ::core::primitive::u32,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::unassign_curator`]."]
                unassign_curator {
                    #[codec(compact)]
                    parent_bounty_id: ::core::primitive::u32,
                    #[codec(compact)]
                    child_bounty_id: ::core::primitive::u32,
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::award_child_bounty`]."]
                award_child_bounty {
                    #[codec(compact)]
                    parent_bounty_id: ::core::primitive::u32,
                    #[codec(compact)]
                    child_bounty_id: ::core::primitive::u32,
                    beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::claim_child_bounty`]."]
                claim_child_bounty {
                    #[codec(compact)]
                    parent_bounty_id: ::core::primitive::u32,
                    #[codec(compact)]
                    child_bounty_id: ::core::primitive::u32,
                },
                #[codec(index = 6)]
                #[doc = "See [`Pallet::close_child_bounty`]."]
                close_child_bounty {
                    #[codec(compact)]
                    parent_bounty_id: ::core::primitive::u32,
                    #[codec(compact)]
                    child_bounty_id: ::core::primitive::u32,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "The parent bounty is not in active state."]
                ParentBountyNotActive,
                #[codec(index = 1)]
                #[doc = "The bounty balance is not enough to add new child-bounty."]
                InsufficientBountyBalance,
                #[codec(index = 2)]
                #[doc = "Number of child bounties exceeds limit `MaxActiveChildBountyCount`."]
                TooManyChildBounties,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A child-bounty is added."]
                Added {
                    index: ::core::primitive::u32,
                    child_index: ::core::primitive::u32,
                },
                #[codec(index = 1)]
                #[doc = "A child-bounty is awarded to a beneficiary."]
                Awarded {
                    index: ::core::primitive::u32,
                    child_index: ::core::primitive::u32,
                    beneficiary: ::subxt::utils::AccountId32,
                },
                #[codec(index = 2)]
                #[doc = "A child-bounty is claimed by beneficiary."]
                Claimed {
                    index: ::core::primitive::u32,
                    child_index: ::core::primitive::u32,
                    payout: ::core::primitive::u128,
                    beneficiary: ::subxt::utils::AccountId32,
                },
                #[codec(index = 3)]
                #[doc = "A child-bounty is cancelled."]
                Canceled {
                    index: ::core::primitive::u32,
                    child_index: ::core::primitive::u32,
                },
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct ChildBounty<_0, _1, _2> {
            pub parent_bounty: ::core::primitive::u32,
            pub value: _1,
            pub fee: _1,
            pub curator_deposit: _1,
            pub status: runtime_types::pallet_child_bounties::ChildBountyStatus<_0, _2>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum ChildBountyStatus<_0, _1> {
            #[codec(index = 0)]
            Added,
            #[codec(index = 1)]
            CuratorProposed { curator: _0 },
            #[codec(index = 2)]
            Active { curator: _0 },
            #[codec(index = 3)]
            PendingPayout {
                curator: _0,
                beneficiary: _0,
                unlock_at: _1,
            },
        }
    }
    pub mod pallet_collective {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::set_members`]."]
                set_members {
                    new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    prime: ::core::option::Option<::subxt::utils::AccountId32>,
                    old_count: ::core::primitive::u32,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::execute`]."]
                execute {
                    proposal: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                    #[codec(compact)]
                    length_bound: ::core::primitive::u32,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::propose`]."]
                propose {
                    #[codec(compact)]
                    threshold: ::core::primitive::u32,
                    proposal: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                    #[codec(compact)]
                    length_bound: ::core::primitive::u32,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::vote`]."]
                vote {
                    proposal: ::subxt::utils::H256,
                    #[codec(compact)]
                    index: ::core::primitive::u32,
                    approve: ::core::primitive::bool,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::disapprove_proposal`]."]
                disapprove_proposal { proposal_hash: ::subxt::utils::H256 },
                #[codec(index = 6)]
                #[doc = "See [`Pallet::close`]."]
                close {
                    proposal_hash: ::subxt::utils::H256,
                    #[codec(compact)]
                    index: ::core::primitive::u32,
                    proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
                    #[codec(compact)]
                    length_bound: ::core::primitive::u32,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call2 {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::set_members`]."]
                set_members {
                    new_members: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    prime: ::core::option::Option<::subxt::utils::AccountId32>,
                    old_count: ::core::primitive::u32,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::execute`]."]
                execute {
                    proposal: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                    #[codec(compact)]
                    length_bound: ::core::primitive::u32,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::propose`]."]
                propose {
                    #[codec(compact)]
                    threshold: ::core::primitive::u32,
                    proposal: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                    #[codec(compact)]
                    length_bound: ::core::primitive::u32,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::vote`]."]
                vote {
                    proposal: ::subxt::utils::H256,
                    #[codec(compact)]
                    index: ::core::primitive::u32,
                    approve: ::core::primitive::bool,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::disapprove_proposal`]."]
                disapprove_proposal { proposal_hash: ::subxt::utils::H256 },
                #[codec(index = 6)]
                #[doc = "See [`Pallet::close`]."]
                close {
                    proposal_hash: ::subxt::utils::H256,
                    #[codec(compact)]
                    index: ::core::primitive::u32,
                    proposal_weight_bound: runtime_types::sp_weights::weight_v2::Weight,
                    #[codec(compact)]
                    length_bound: ::core::primitive::u32,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Account is not a member"]
                NotMember,
                #[codec(index = 1)]
                #[doc = "Duplicate proposals not allowed"]
                DuplicateProposal,
                #[codec(index = 2)]
                #[doc = "Proposal must exist"]
                ProposalMissing,
                #[codec(index = 3)]
                #[doc = "Mismatched index"]
                WrongIndex,
                #[codec(index = 4)]
                #[doc = "Duplicate vote ignored"]
                DuplicateVote,
                #[codec(index = 5)]
                #[doc = "Members are already initialized!"]
                AlreadyInitialized,
                #[codec(index = 6)]
                #[doc = "The close call was made too early, before the end of the voting."]
                TooEarly,
                #[codec(index = 7)]
                #[doc = "There can only be a maximum of `MaxProposals` active proposals."]
                TooManyProposals,
                #[codec(index = 8)]
                #[doc = "The given weight bound for the proposal was too low."]
                WrongProposalWeight,
                #[codec(index = 9)]
                #[doc = "The given length bound for the proposal was too low."]
                WrongProposalLength,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error2 {
                #[codec(index = 0)]
                #[doc = "Account is not a member"]
                NotMember,
                #[codec(index = 1)]
                #[doc = "Duplicate proposals not allowed"]
                DuplicateProposal,
                #[codec(index = 2)]
                #[doc = "Proposal must exist"]
                ProposalMissing,
                #[codec(index = 3)]
                #[doc = "Mismatched index"]
                WrongIndex,
                #[codec(index = 4)]
                #[doc = "Duplicate vote ignored"]
                DuplicateVote,
                #[codec(index = 5)]
                #[doc = "Members are already initialized!"]
                AlreadyInitialized,
                #[codec(index = 6)]
                #[doc = "The close call was made too early, before the end of the voting."]
                TooEarly,
                #[codec(index = 7)]
                #[doc = "There can only be a maximum of `MaxProposals` active proposals."]
                TooManyProposals,
                #[codec(index = 8)]
                #[doc = "The given weight bound for the proposal was too low."]
                WrongProposalWeight,
                #[codec(index = 9)]
                #[doc = "The given length bound for the proposal was too low."]
                WrongProposalLength,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
                #[doc = "`MemberCount`)."]
                Proposed {
                    account: ::subxt::utils::AccountId32,
                    proposal_index: ::core::primitive::u32,
                    proposal_hash: ::subxt::utils::H256,
                    threshold: ::core::primitive::u32,
                },
                #[codec(index = 1)]
                #[doc = "A motion (given hash) has been voted on by given account, leaving"]
                #[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
                Voted {
                    account: ::subxt::utils::AccountId32,
                    proposal_hash: ::subxt::utils::H256,
                    voted: ::core::primitive::bool,
                    yes: ::core::primitive::u32,
                    no: ::core::primitive::u32,
                },
                #[codec(index = 2)]
                #[doc = "A motion was approved by the required threshold."]
                Approved { proposal_hash: ::subxt::utils::H256 },
                #[codec(index = 3)]
                #[doc = "A motion was not approved by the required threshold."]
                Disapproved { proposal_hash: ::subxt::utils::H256 },
                #[codec(index = 4)]
                #[doc = "A motion was executed; result will be `Ok` if it returned without error."]
                Executed {
                    proposal_hash: ::subxt::utils::H256,
                    result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                },
                #[codec(index = 5)]
                #[doc = "A single member did some action; result will be `Ok` if it returned without error."]
                MemberExecuted {
                    proposal_hash: ::subxt::utils::H256,
                    result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                },
                #[codec(index = 6)]
                #[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
                Closed {
                    proposal_hash: ::subxt::utils::H256,
                    yes: ::core::primitive::u32,
                    no: ::core::primitive::u32,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event2 {
                #[codec(index = 0)]
                #[doc = "A motion (given hash) has been proposed (by given account) with a threshold (given"]
                #[doc = "`MemberCount`)."]
                Proposed {
                    account: ::subxt::utils::AccountId32,
                    proposal_index: ::core::primitive::u32,
                    proposal_hash: ::subxt::utils::H256,
                    threshold: ::core::primitive::u32,
                },
                #[codec(index = 1)]
                #[doc = "A motion (given hash) has been voted on by given account, leaving"]
                #[doc = "a tally (yes votes and no votes given respectively as `MemberCount`)."]
                Voted {
                    account: ::subxt::utils::AccountId32,
                    proposal_hash: ::subxt::utils::H256,
                    voted: ::core::primitive::bool,
                    yes: ::core::primitive::u32,
                    no: ::core::primitive::u32,
                },
                #[codec(index = 2)]
                #[doc = "A motion was approved by the required threshold."]
                Approved { proposal_hash: ::subxt::utils::H256 },
                #[codec(index = 3)]
                #[doc = "A motion was not approved by the required threshold."]
                Disapproved { proposal_hash: ::subxt::utils::H256 },
                #[codec(index = 4)]
                #[doc = "A motion was executed; result will be `Ok` if it returned without error."]
                Executed {
                    proposal_hash: ::subxt::utils::H256,
                    result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                },
                #[codec(index = 5)]
                #[doc = "A single member did some action; result will be `Ok` if it returned without error."]
                MemberExecuted {
                    proposal_hash: ::subxt::utils::H256,
                    result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                },
                #[codec(index = 6)]
                #[doc = "A proposal was closed because its threshold was reached or after its duration was up."]
                Closed {
                    proposal_hash: ::subxt::utils::H256,
                    yes: ::core::primitive::u32,
                    no: ::core::primitive::u32,
                },
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum RawOrigin<_0> {
            #[codec(index = 0)]
            Members(::core::primitive::u32, ::core::primitive::u32),
            #[codec(index = 1)]
            Member(_0),
            #[codec(index = 2)]
            _Phantom,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Votes<_0, _1> {
            pub index: ::core::primitive::u32,
            pub threshold: ::core::primitive::u32,
            pub ayes: ::std::vec::Vec<_0>,
            pub nays: ::std::vec::Vec<_0>,
            pub end: _1,
        }
    }
    pub mod pallet_conviction_voting {
        use super::runtime_types;
        pub mod conviction {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Conviction {
                #[codec(index = 0)]
                None,
                #[codec(index = 1)]
                Locked1x,
                #[codec(index = 2)]
                Locked2x,
                #[codec(index = 3)]
                Locked3x,
                #[codec(index = 4)]
                Locked4x,
                #[codec(index = 5)]
                Locked5x,
                #[codec(index = 6)]
                Locked6x,
            }
        }
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::vote`]."]
                vote {
                    #[codec(compact)]
                    poll_index: ::core::primitive::u32,
                    vote: runtime_types::pallet_conviction_voting::vote::AccountVote<
                        ::core::primitive::u128,
                    >,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::delegate`]."]
                delegate {
                    class: ::core::primitive::u16,
                    to: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    conviction: runtime_types::pallet_conviction_voting::conviction::Conviction,
                    balance: ::core::primitive::u128,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::undelegate`]."]
                undelegate { class: ::core::primitive::u16 },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::unlock`]."]
                unlock {
                    class: ::core::primitive::u16,
                    target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::remove_vote`]."]
                remove_vote {
                    class: ::core::option::Option<::core::primitive::u16>,
                    index: ::core::primitive::u32,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::remove_other_vote`]."]
                remove_other_vote {
                    target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    class: ::core::primitive::u16,
                    index: ::core::primitive::u32,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Poll is not ongoing."]
                NotOngoing,
                #[codec(index = 1)]
                #[doc = "The given account did not vote on the poll."]
                NotVoter,
                #[codec(index = 2)]
                #[doc = "The actor has no permission to conduct the action."]
                NoPermission,
                #[codec(index = 3)]
                #[doc = "The actor has no permission to conduct the action right now but will do in the future."]
                NoPermissionYet,
                #[codec(index = 4)]
                #[doc = "The account is already delegating."]
                AlreadyDelegating,
                #[codec(index = 5)]
                #[doc = "The account currently has votes attached to it and the operation cannot succeed until"]
                #[doc = "these are removed, either through `unvote` or `reap_vote`."]
                AlreadyVoting,
                #[codec(index = 6)]
                #[doc = "Too high a balance was provided that the account cannot afford."]
                InsufficientFunds,
                #[codec(index = 7)]
                #[doc = "The account is not currently delegating."]
                NotDelegating,
                #[codec(index = 8)]
                #[doc = "Delegation to oneself makes no sense."]
                Nonsense,
                #[codec(index = 9)]
                #[doc = "Maximum number of votes reached."]
                MaxVotesReached,
                #[codec(index = 10)]
                #[doc = "The class must be supplied since it is not easily determinable from the state."]
                ClassNeeded,
                #[codec(index = 11)]
                #[doc = "The class ID supplied is invalid."]
                BadClass,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "An account has delegated their vote to another account. \\[who, target\\]"]
                Delegated(::subxt::utils::AccountId32, ::subxt::utils::AccountId32),
                #[codec(index = 1)]
                #[doc = "An \\[account\\] has cancelled a previous delegation operation."]
                Undelegated(::subxt::utils::AccountId32),
            }
        }
        pub mod types {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Delegations<_0> {
                pub votes: _0,
                pub capital: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Tally<_0> {
                pub ayes: _0,
                pub nays: _0,
                pub support: _0,
            }
        }
        pub mod vote {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum AccountVote<_0> {
                #[codec(index = 0)]
                Standard {
                    vote: runtime_types::pallet_conviction_voting::vote::Vote,
                    balance: _0,
                },
                #[codec(index = 1)]
                Split { aye: _0, nay: _0 },
                #[codec(index = 2)]
                SplitAbstain { aye: _0, nay: _0, abstain: _0 },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Casting<_0, _1, _2> {
                pub votes: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                    _1,
                    runtime_types::pallet_conviction_voting::vote::AccountVote<_0>,
                )>,
                pub delegations: runtime_types::pallet_conviction_voting::types::Delegations<_0>,
                pub prior: runtime_types::pallet_conviction_voting::vote::PriorLock<_1, _0>,
                #[codec(skip)]
                pub __subxt_unused_type_params: ::core::marker::PhantomData<_2>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Delegating<_0, _1, _2> {
                pub balance: _0,
                pub target: _1,
                pub conviction: runtime_types::pallet_conviction_voting::conviction::Conviction,
                pub delegations: runtime_types::pallet_conviction_voting::types::Delegations<_0>,
                pub prior: runtime_types::pallet_conviction_voting::vote::PriorLock<_2, _0>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PriorLock<_0, _1>(pub _0, pub _1);
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Vote(pub ::core::primitive::u8);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Voting<_0, _1, _2, _3> {
                #[codec(index = 0)]
                Casting(runtime_types::pallet_conviction_voting::vote::Casting<_0, _2, _2>),
                #[codec(index = 1)]
                Delegating(runtime_types::pallet_conviction_voting::vote::Delegating<_0, _1, _2>),
                __Ignore(::core::marker::PhantomData<_3>),
            }
        }
    }
    pub mod pallet_democracy {
        use super::runtime_types;
        pub mod conviction {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Conviction {
                #[codec(index = 0)]
                None,
                #[codec(index = 1)]
                Locked1x,
                #[codec(index = 2)]
                Locked2x,
                #[codec(index = 3)]
                Locked3x,
                #[codec(index = 4)]
                Locked4x,
                #[codec(index = 5)]
                Locked5x,
                #[codec(index = 6)]
                Locked6x,
            }
        }
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::propose`]."]
                propose {
                    proposal: runtime_types::frame_support::traits::preimages::Bounded<
                        runtime_types::polkadot_runtime::RuntimeCall,
                    >,
                    #[codec(compact)]
                    value: ::core::primitive::u128,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::second`]."]
                second {
                    #[codec(compact)]
                    proposal: ::core::primitive::u32,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::vote`]."]
                vote {
                    #[codec(compact)]
                    ref_index: ::core::primitive::u32,
                    vote:
                        runtime_types::pallet_democracy::vote::AccountVote<::core::primitive::u128>,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::emergency_cancel`]."]
                emergency_cancel { ref_index: ::core::primitive::u32 },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::external_propose`]."]
                external_propose {
                    proposal: runtime_types::frame_support::traits::preimages::Bounded<
                        runtime_types::polkadot_runtime::RuntimeCall,
                    >,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::external_propose_majority`]."]
                external_propose_majority {
                    proposal: runtime_types::frame_support::traits::preimages::Bounded<
                        runtime_types::polkadot_runtime::RuntimeCall,
                    >,
                },
                #[codec(index = 6)]
                #[doc = "See [`Pallet::external_propose_default`]."]
                external_propose_default {
                    proposal: runtime_types::frame_support::traits::preimages::Bounded<
                        runtime_types::polkadot_runtime::RuntimeCall,
                    >,
                },
                #[codec(index = 7)]
                #[doc = "See [`Pallet::fast_track`]."]
                fast_track {
                    proposal_hash: ::subxt::utils::H256,
                    voting_period: ::core::primitive::u32,
                    delay: ::core::primitive::u32,
                },
                #[codec(index = 8)]
                #[doc = "See [`Pallet::veto_external`]."]
                veto_external { proposal_hash: ::subxt::utils::H256 },
                #[codec(index = 9)]
                #[doc = "See [`Pallet::cancel_referendum`]."]
                cancel_referendum {
                    #[codec(compact)]
                    ref_index: ::core::primitive::u32,
                },
                #[codec(index = 10)]
                #[doc = "See [`Pallet::delegate`]."]
                delegate {
                    to: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    conviction: runtime_types::pallet_democracy::conviction::Conviction,
                    balance: ::core::primitive::u128,
                },
                #[codec(index = 11)]
                #[doc = "See [`Pallet::undelegate`]."]
                undelegate,
                #[codec(index = 12)]
                #[doc = "See [`Pallet::clear_public_proposals`]."]
                clear_public_proposals,
                #[codec(index = 13)]
                #[doc = "See [`Pallet::unlock`]."]
                unlock {
                    target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 14)]
                #[doc = "See [`Pallet::remove_vote`]."]
                remove_vote { index: ::core::primitive::u32 },
                #[codec(index = 15)]
                #[doc = "See [`Pallet::remove_other_vote`]."]
                remove_other_vote {
                    target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    index: ::core::primitive::u32,
                },
                #[codec(index = 16)]
                #[doc = "See [`Pallet::blacklist`]."]
                blacklist {
                    proposal_hash: ::subxt::utils::H256,
                    maybe_ref_index: ::core::option::Option<::core::primitive::u32>,
                },
                #[codec(index = 17)]
                #[doc = "See [`Pallet::cancel_proposal`]."]
                cancel_proposal {
                    #[codec(compact)]
                    prop_index: ::core::primitive::u32,
                },
                #[codec(index = 18)]
                #[doc = "See [`Pallet::set_metadata`]."]
                set_metadata {
                    owner: runtime_types::pallet_democracy::types::MetadataOwner,
                    maybe_hash: ::core::option::Option<::subxt::utils::H256>,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Value too low"]
                ValueLow,
                #[codec(index = 1)]
                #[doc = "Proposal does not exist"]
                ProposalMissing,
                #[codec(index = 2)]
                #[doc = "Cannot cancel the same proposal twice"]
                AlreadyCanceled,
                #[codec(index = 3)]
                #[doc = "Proposal already made"]
                DuplicateProposal,
                #[codec(index = 4)]
                #[doc = "Proposal still blacklisted"]
                ProposalBlacklisted,
                #[codec(index = 5)]
                #[doc = "Next external proposal not simple majority"]
                NotSimpleMajority,
                #[codec(index = 6)]
                #[doc = "Invalid hash"]
                InvalidHash,
                #[codec(index = 7)]
                #[doc = "No external proposal"]
                NoProposal,
                #[codec(index = 8)]
                #[doc = "Identity may not veto a proposal twice"]
                AlreadyVetoed,
                #[codec(index = 9)]
                #[doc = "Vote given for invalid referendum"]
                ReferendumInvalid,
                #[codec(index = 10)]
                #[doc = "No proposals waiting"]
                NoneWaiting,
                #[codec(index = 11)]
                #[doc = "The given account did not vote on the referendum."]
                NotVoter,
                #[codec(index = 12)]
                #[doc = "The actor has no permission to conduct the action."]
                NoPermission,
                #[codec(index = 13)]
                #[doc = "The account is already delegating."]
                AlreadyDelegating,
                #[codec(index = 14)]
                #[doc = "Too high a balance was provided that the account cannot afford."]
                InsufficientFunds,
                #[codec(index = 15)]
                #[doc = "The account is not currently delegating."]
                NotDelegating,
                #[codec(index = 16)]
                #[doc = "The account currently has votes attached to it and the operation cannot succeed until"]
                #[doc = "these are removed, either through `unvote` or `reap_vote`."]
                VotesExist,
                #[codec(index = 17)]
                #[doc = "The instant referendum origin is currently disallowed."]
                InstantNotAllowed,
                #[codec(index = 18)]
                #[doc = "Delegation to oneself makes no sense."]
                Nonsense,
                #[codec(index = 19)]
                #[doc = "Invalid upper bound."]
                WrongUpperBound,
                #[codec(index = 20)]
                #[doc = "Maximum number of votes reached."]
                MaxVotesReached,
                #[codec(index = 21)]
                #[doc = "Maximum number of items reached."]
                TooMany,
                #[codec(index = 22)]
                #[doc = "Voting period too low"]
                VotingPeriodLow,
                #[codec(index = 23)]
                #[doc = "The preimage does not exist."]
                PreimageNotExist,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A motion has been proposed by a public account."]
                Proposed {
                    proposal_index: ::core::primitive::u32,
                    deposit: ::core::primitive::u128,
                },
                #[codec(index = 1)]
                #[doc = "A public proposal has been tabled for referendum vote."]
                Tabled {
                    proposal_index: ::core::primitive::u32,
                    deposit: ::core::primitive::u128,
                },
                #[codec(index = 2)]
                #[doc = "An external proposal has been tabled."]
                ExternalTabled,
                #[codec(index = 3)]
                #[doc = "A referendum has begun."]
                Started {
                    ref_index: ::core::primitive::u32,
                    threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
                },
                #[codec(index = 4)]
                #[doc = "A proposal has been approved by referendum."]
                Passed { ref_index: ::core::primitive::u32 },
                #[codec(index = 5)]
                #[doc = "A proposal has been rejected by referendum."]
                NotPassed { ref_index: ::core::primitive::u32 },
                #[codec(index = 6)]
                #[doc = "A referendum has been cancelled."]
                Cancelled { ref_index: ::core::primitive::u32 },
                #[codec(index = 7)]
                #[doc = "An account has delegated their vote to another account."]
                Delegated {
                    who: ::subxt::utils::AccountId32,
                    target: ::subxt::utils::AccountId32,
                },
                #[codec(index = 8)]
                #[doc = "An account has cancelled a previous delegation operation."]
                Undelegated {
                    account: ::subxt::utils::AccountId32,
                },
                #[codec(index = 9)]
                #[doc = "An external proposal has been vetoed."]
                Vetoed {
                    who: ::subxt::utils::AccountId32,
                    proposal_hash: ::subxt::utils::H256,
                    until: ::core::primitive::u32,
                },
                #[codec(index = 10)]
                #[doc = "A proposal_hash has been blacklisted permanently."]
                Blacklisted { proposal_hash: ::subxt::utils::H256 },
                #[codec(index = 11)]
                #[doc = "An account has voted in a referendum"]
                Voted {
                    voter: ::subxt::utils::AccountId32,
                    ref_index: ::core::primitive::u32,
                    vote:
                        runtime_types::pallet_democracy::vote::AccountVote<::core::primitive::u128>,
                },
                #[codec(index = 12)]
                #[doc = "An account has secconded a proposal"]
                Seconded {
                    seconder: ::subxt::utils::AccountId32,
                    prop_index: ::core::primitive::u32,
                },
                #[codec(index = 13)]
                #[doc = "A proposal got canceled."]
                ProposalCanceled { prop_index: ::core::primitive::u32 },
                #[codec(index = 14)]
                #[doc = "Metadata for a proposal or a referendum has been set."]
                MetadataSet {
                    owner: runtime_types::pallet_democracy::types::MetadataOwner,
                    hash: ::subxt::utils::H256,
                },
                #[codec(index = 15)]
                #[doc = "Metadata for a proposal or a referendum has been cleared."]
                MetadataCleared {
                    owner: runtime_types::pallet_democracy::types::MetadataOwner,
                    hash: ::subxt::utils::H256,
                },
                #[codec(index = 16)]
                #[doc = "Metadata has been transferred to new owner."]
                MetadataTransferred {
                    prev_owner: runtime_types::pallet_democracy::types::MetadataOwner,
                    owner: runtime_types::pallet_democracy::types::MetadataOwner,
                    hash: ::subxt::utils::H256,
                },
            }
        }
        pub mod types {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Delegations<_0> {
                pub votes: _0,
                pub capital: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum MetadataOwner {
                #[codec(index = 0)]
                External,
                #[codec(index = 1)]
                Proposal(::core::primitive::u32),
                #[codec(index = 2)]
                Referendum(::core::primitive::u32),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum ReferendumInfo<_0, _1, _2> {
                #[codec(index = 0)]
                Ongoing(runtime_types::pallet_democracy::types::ReferendumStatus<_0, _1, _2>),
                #[codec(index = 1)]
                Finished {
                    approved: ::core::primitive::bool,
                    end: _0,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ReferendumStatus<_0, _1, _2> {
                pub end: _0,
                pub proposal: _1,
                pub threshold: runtime_types::pallet_democracy::vote_threshold::VoteThreshold,
                pub delay: _0,
                pub tally: runtime_types::pallet_democracy::types::Tally<_2>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Tally<_0> {
                pub ayes: _0,
                pub nays: _0,
                pub turnout: _0,
            }
        }
        pub mod vote {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum AccountVote<_0> {
                #[codec(index = 0)]
                Standard {
                    vote: runtime_types::pallet_democracy::vote::Vote,
                    balance: _0,
                },
                #[codec(index = 1)]
                Split { aye: _0, nay: _0 },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PriorLock<_0, _1>(pub _0, pub _1);
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Vote(pub ::core::primitive::u8);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Voting<_0, _1, _2> {
                #[codec(index = 0)]
                Direct {
                    votes: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                        _2,
                        runtime_types::pallet_democracy::vote::AccountVote<_0>,
                    )>,
                    delegations: runtime_types::pallet_democracy::types::Delegations<_0>,
                    prior: runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
                },
                #[codec(index = 1)]
                Delegating {
                    balance: _0,
                    target: _1,
                    conviction: runtime_types::pallet_democracy::conviction::Conviction,
                    delegations: runtime_types::pallet_democracy::types::Delegations<_0>,
                    prior: runtime_types::pallet_democracy::vote::PriorLock<_2, _0>,
                },
            }
        }
        pub mod vote_threshold {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum VoteThreshold {
                #[codec(index = 0)]
                SuperMajorityApprove,
                #[codec(index = 1)]
                SuperMajorityAgainst,
                #[codec(index = 2)]
                SimpleMajority,
            }
        }
    }
    pub mod pallet_election_provider_multi_phase {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::submit_unsigned`]."]
                submit_unsigned {
                    raw_solution: ::std::boxed::Box<
                        runtime_types::pallet_election_provider_multi_phase::RawSolution<
                            runtime_types::polkadot_runtime::NposCompactSolution16,
                        >,
                    >,
                    witness:
                        runtime_types::pallet_election_provider_multi_phase::SolutionOrSnapshotSize,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::set_minimum_untrusted_score`]."]
                set_minimum_untrusted_score {
                    maybe_next_score:
                        ::core::option::Option<runtime_types::sp_npos_elections::ElectionScore>,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::set_emergency_election_result`]."]
                set_emergency_election_result {
                    supports: ::std::vec::Vec<(
                        ::subxt::utils::AccountId32,
                        runtime_types::sp_npos_elections::Support<::subxt::utils::AccountId32>,
                    )>,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::submit`]."]
                submit {
                    raw_solution: ::std::boxed::Box<
                        runtime_types::pallet_election_provider_multi_phase::RawSolution<
                            runtime_types::polkadot_runtime::NposCompactSolution16,
                        >,
                    >,
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::governance_fallback`]."]
                governance_fallback {
                    maybe_max_voters: ::core::option::Option<::core::primitive::u32>,
                    maybe_max_targets: ::core::option::Option<::core::primitive::u32>,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Error of the pallet that can be returned in response to dispatches."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Submission was too early."]
                PreDispatchEarlySubmission,
                #[codec(index = 1)]
                #[doc = "Wrong number of winners presented."]
                PreDispatchWrongWinnerCount,
                #[codec(index = 2)]
                #[doc = "Submission was too weak, score-wise."]
                PreDispatchWeakSubmission,
                #[codec(index = 3)]
                #[doc = "The queue was full, and the solution was not better than any of the existing ones."]
                SignedQueueFull,
                #[codec(index = 4)]
                #[doc = "The origin failed to pay the deposit."]
                SignedCannotPayDeposit,
                #[codec(index = 5)]
                #[doc = "Witness data to dispatchable is invalid."]
                SignedInvalidWitness,
                #[codec(index = 6)]
                #[doc = "The signed submission consumes too much weight"]
                SignedTooMuchWeight,
                #[codec(index = 7)]
                #[doc = "OCW submitted solution for wrong round"]
                OcwCallWrongEra,
                #[codec(index = 8)]
                #[doc = "Snapshot metadata should exist but didn't."]
                MissingSnapshotMetadata,
                #[codec(index = 9)]
                #[doc = "`Self::insert_submission` returned an invalid index."]
                InvalidSubmissionIndex,
                #[codec(index = 10)]
                #[doc = "The call is not allowed at this point."]
                CallNotAllowed,
                #[codec(index = 11)]
                #[doc = "The fallback failed"]
                FallbackFailed,
                #[codec(index = 12)]
                #[doc = "Some bound not met"]
                BoundNotMet,
                #[codec(index = 13)]
                #[doc = "Submitted solution has too many winners"]
                TooManyWinners,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A solution was stored with the given compute."]
                #[doc = ""]
                #[doc = "The `origin` indicates the origin of the solution. If `origin` is `Some(AccountId)`,"]
                #[doc = "the stored solution was submited in the signed phase by a miner with the `AccountId`."]
                #[doc = "Otherwise, the solution was stored either during the unsigned phase or by"]
                #[doc = "`T::ForceOrigin`. The `bool` is `true` when a previous solution was ejected to make"]
                #[doc = "room for this one."]
                SolutionStored {
                    compute: runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                    origin: ::core::option::Option<::subxt::utils::AccountId32>,
                    prev_ejected: ::core::primitive::bool,
                },
                #[codec(index = 1)]
                #[doc = "The election has been finalized, with the given computation and score."]
                ElectionFinalized {
                    compute: runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
                    score: runtime_types::sp_npos_elections::ElectionScore,
                },
                #[codec(index = 2)]
                #[doc = "An election failed."]
                #[doc = ""]
                #[doc = "Not much can be said about which computes failed in the process."]
                ElectionFailed,
                #[codec(index = 3)]
                #[doc = "An account has been rewarded for their signed submission being finalized."]
                Rewarded {
                    account: ::subxt::utils::AccountId32,
                    value: ::core::primitive::u128,
                },
                #[codec(index = 4)]
                #[doc = "An account has been slashed for submitting an invalid signed submission."]
                Slashed {
                    account: ::subxt::utils::AccountId32,
                    value: ::core::primitive::u128,
                },
                #[codec(index = 5)]
                #[doc = "There was a phase transition in a given round."]
                PhaseTransitioned {
                    from: runtime_types::pallet_election_provider_multi_phase::Phase<
                        ::core::primitive::u32,
                    >,
                    to: runtime_types::pallet_election_provider_multi_phase::Phase<
                        ::core::primitive::u32,
                    >,
                    round: ::core::primitive::u32,
                },
            }
        }
        pub mod signed {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct SignedSubmission<_0, _1, _2> {
                pub who: _0,
                pub deposit: _1,
                pub raw_solution:
                    runtime_types::pallet_election_provider_multi_phase::RawSolution<_2>,
                pub call_fee: _1,
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum ElectionCompute {
            #[codec(index = 0)]
            OnChain,
            #[codec(index = 1)]
            Signed,
            #[codec(index = 2)]
            Unsigned,
            #[codec(index = 3)]
            Fallback,
            #[codec(index = 4)]
            Emergency,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum Phase<_0> {
            #[codec(index = 0)]
            Off,
            #[codec(index = 1)]
            Signed,
            #[codec(index = 2)]
            Unsigned((::core::primitive::bool, _0)),
            #[codec(index = 3)]
            Emergency,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct RawSolution<_0> {
            pub solution: _0,
            pub score: runtime_types::sp_npos_elections::ElectionScore,
            pub round: ::core::primitive::u32,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct ReadySolution {
            pub supports: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                ::subxt::utils::AccountId32,
                runtime_types::sp_npos_elections::Support<::subxt::utils::AccountId32>,
            )>,
            pub score: runtime_types::sp_npos_elections::ElectionScore,
            pub compute: runtime_types::pallet_election_provider_multi_phase::ElectionCompute,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct RoundSnapshot<_0, _1> {
            pub voters: ::std::vec::Vec<_1>,
            pub targets: ::std::vec::Vec<_0>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct SolutionOrSnapshotSize {
            #[codec(compact)]
            pub voters: ::core::primitive::u32,
            #[codec(compact)]
            pub targets: ::core::primitive::u32,
        }
    }
    pub mod pallet_elections_phragmen {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::vote`]."]
                vote {
                    votes: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    #[codec(compact)]
                    value: ::core::primitive::u128,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::remove_voter`]."]
                remove_voter,
                #[codec(index = 2)]
                #[doc = "See [`Pallet::submit_candidacy`]."]
                submit_candidacy {
                    #[codec(compact)]
                    candidate_count: ::core::primitive::u32,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::renounce_candidacy`]."]
                renounce_candidacy {
                    renouncing: runtime_types::pallet_elections_phragmen::Renouncing,
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::remove_member`]."]
                remove_member {
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    slash_bond: ::core::primitive::bool,
                    rerun_election: ::core::primitive::bool,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::clean_defunct_voters`]."]
                clean_defunct_voters {
                    num_voters: ::core::primitive::u32,
                    num_defunct: ::core::primitive::u32,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Cannot vote when no candidates or members exist."]
                UnableToVote,
                #[codec(index = 1)]
                #[doc = "Must vote for at least one candidate."]
                NoVotes,
                #[codec(index = 2)]
                #[doc = "Cannot vote more than candidates."]
                TooManyVotes,
                #[codec(index = 3)]
                #[doc = "Cannot vote more than maximum allowed."]
                MaximumVotesExceeded,
                #[codec(index = 4)]
                #[doc = "Cannot vote with stake less than minimum balance."]
                LowBalance,
                #[codec(index = 5)]
                #[doc = "Voter can not pay voting bond."]
                UnableToPayBond,
                #[codec(index = 6)]
                #[doc = "Must be a voter."]
                MustBeVoter,
                #[codec(index = 7)]
                #[doc = "Duplicated candidate submission."]
                DuplicatedCandidate,
                #[codec(index = 8)]
                #[doc = "Too many candidates have been created."]
                TooManyCandidates,
                #[codec(index = 9)]
                #[doc = "Member cannot re-submit candidacy."]
                MemberSubmit,
                #[codec(index = 10)]
                #[doc = "Runner cannot re-submit candidacy."]
                RunnerUpSubmit,
                #[codec(index = 11)]
                #[doc = "Candidate does not have enough funds."]
                InsufficientCandidateFunds,
                #[codec(index = 12)]
                #[doc = "Not a member."]
                NotMember,
                #[codec(index = 13)]
                #[doc = "The provided count of number of candidates is incorrect."]
                InvalidWitnessData,
                #[codec(index = 14)]
                #[doc = "The provided count of number of votes is incorrect."]
                InvalidVoteCount,
                #[codec(index = 15)]
                #[doc = "The renouncing origin presented a wrong `Renouncing` parameter."]
                InvalidRenouncing,
                #[codec(index = 16)]
                #[doc = "Prediction regarding replacement after member removal is wrong."]
                InvalidReplacement,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A new term with new_members. This indicates that enough candidates existed to run"]
                #[doc = "the election, not that enough have has been elected. The inner value must be examined"]
                #[doc = "for this purpose. A `NewTerm(\\[\\])` indicates that some candidates got their bond"]
                #[doc = "slashed and none were elected, whilst `EmptyTerm` means that no candidates existed to"]
                #[doc = "begin with."]
                NewTerm {
                    new_members:
                        ::std::vec::Vec<(::subxt::utils::AccountId32, ::core::primitive::u128)>,
                },
                #[codec(index = 1)]
                #[doc = "No (or not enough) candidates existed for this round. This is different from"]
                #[doc = "`NewTerm(\\[\\])`. See the description of `NewTerm`."]
                EmptyTerm,
                #[codec(index = 2)]
                #[doc = "Internal error happened while trying to perform election."]
                ElectionError,
                #[codec(index = 3)]
                #[doc = "A member has been removed. This should always be followed by either `NewTerm` or"]
                #[doc = "`EmptyTerm`."]
                MemberKicked { member: ::subxt::utils::AccountId32 },
                #[codec(index = 4)]
                #[doc = "Someone has renounced their candidacy."]
                Renounced {
                    candidate: ::subxt::utils::AccountId32,
                },
                #[codec(index = 5)]
                #[doc = "A candidate was slashed by amount due to failing to obtain a seat as member or"]
                #[doc = "runner-up."]
                #[doc = ""]
                #[doc = "Note that old members and runners-up are also candidates."]
                CandidateSlashed {
                    candidate: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 6)]
                #[doc = "A seat holder was slashed by amount by being forcefully removed from the set."]
                SeatHolderSlashed {
                    seat_holder: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum Renouncing {
            #[codec(index = 0)]
            Member,
            #[codec(index = 1)]
            RunnerUp,
            #[codec(index = 2)]
            Candidate(#[codec(compact)] ::core::primitive::u32),
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct SeatHolder<_0, _1> {
            pub who: _0,
            pub stake: _1,
            pub deposit: _1,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Voter<_0, _1> {
            pub votes: ::std::vec::Vec<_0>,
            pub stake: _1,
            pub deposit: _1,
        }
    }
    pub mod pallet_fast_unstake {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::register_fast_unstake`]."]
                register_fast_unstake,
                #[codec(index = 1)]
                #[doc = "See [`Pallet::deregister`]."]
                deregister,
                #[codec(index = 2)]
                #[doc = "See [`Pallet::control`]."]
                control {
                    eras_to_check: ::core::primitive::u32,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "The provided Controller account was not found."]
                #[doc = ""]
                #[doc = "This means that the given account is not bonded."]
                NotController,
                #[codec(index = 1)]
                #[doc = "The bonded account has already been queued."]
                AlreadyQueued,
                #[codec(index = 2)]
                #[doc = "The bonded account has active unlocking chunks."]
                NotFullyBonded,
                #[codec(index = 3)]
                #[doc = "The provided un-staker is not in the `Queue`."]
                NotQueued,
                #[codec(index = 4)]
                #[doc = "The provided un-staker is already in Head, and cannot deregister."]
                AlreadyHead,
                #[codec(index = 5)]
                #[doc = "The call is not allowed at this point because the pallet is not active."]
                CallNotAllowed,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A staker was unstaked."]
                Unstaked {
                    stash: ::subxt::utils::AccountId32,
                    result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                },
                #[codec(index = 1)]
                #[doc = "A staker was slashed for requesting fast-unstake whilst being exposed."]
                Slashed {
                    stash: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 2)]
                #[doc = "A batch was partially checked for the given eras, but the process did not finish."]
                BatchChecked {
                    eras: ::std::vec::Vec<::core::primitive::u32>,
                },
                #[codec(index = 3)]
                #[doc = "A batch of a given size was terminated."]
                #[doc = ""]
                #[doc = "This is always follows by a number of `Unstaked` or `Slashed` events, marking the end"]
                #[doc = "of the batch. A new batch will be created upon next block."]
                BatchFinished { size: ::core::primitive::u32 },
                #[codec(index = 4)]
                #[doc = "An internal error happened. Operations will be paused now."]
                InternalError,
            }
        }
        pub mod types {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct UnstakeRequest {
                pub stashes: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                    ::subxt::utils::AccountId32,
                    ::core::primitive::u128,
                )>,
                pub checked: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    ::core::primitive::u32,
                >,
            }
        }
    }
    pub mod pallet_grandpa {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::report_equivocation`]."]
                report_equivocation {
                    equivocation_proof: ::std::boxed::Box<
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt::utils::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::report_equivocation_unsigned`]."]
                report_equivocation_unsigned {
                    equivocation_proof: ::std::boxed::Box<
                        runtime_types::sp_consensus_grandpa::EquivocationProof<
                            ::subxt::utils::H256,
                            ::core::primitive::u32,
                        >,
                    >,
                    key_owner_proof: runtime_types::sp_session::MembershipProof,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::note_stalled`]."]
                note_stalled {
                    delay: ::core::primitive::u32,
                    best_finalized_block_number: ::core::primitive::u32,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Attempt to signal GRANDPA pause when the authority set isn't live"]
                #[doc = "(either paused or already pending pause)."]
                PauseFailed,
                #[codec(index = 1)]
                #[doc = "Attempt to signal GRANDPA resume when the authority set isn't paused"]
                #[doc = "(either live or already pending resume)."]
                ResumeFailed,
                #[codec(index = 2)]
                #[doc = "Attempt to signal GRANDPA change with one already pending."]
                ChangePending,
                #[codec(index = 3)]
                #[doc = "Cannot signal forced change so soon after last."]
                TooSoon,
                #[codec(index = 4)]
                #[doc = "A key ownership proof provided as part of an equivocation report is invalid."]
                InvalidKeyOwnershipProof,
                #[codec(index = 5)]
                #[doc = "An equivocation proof provided as part of an equivocation report is invalid."]
                InvalidEquivocationProof,
                #[codec(index = 6)]
                #[doc = "A given equivocation report is valid but already previously reported."]
                DuplicateOffenceReport,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "New authority set has been applied."]
                NewAuthorities {
                    authority_set: ::std::vec::Vec<(
                        runtime_types::sp_consensus_grandpa::app::Public,
                        ::core::primitive::u64,
                    )>,
                },
                #[codec(index = 1)]
                #[doc = "Current authority set has been paused."]
                Paused,
                #[codec(index = 2)]
                #[doc = "Current authority set has been resumed."]
                Resumed,
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct StoredPendingChange<_0> {
            pub scheduled_at: _0,
            pub delay: _0,
            pub next_authorities:
                runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<(
                    runtime_types::sp_consensus_grandpa::app::Public,
                    ::core::primitive::u64,
                )>,
            pub forced: ::core::option::Option<_0>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum StoredState<_0> {
            #[codec(index = 0)]
            Live,
            #[codec(index = 1)]
            PendingPause { scheduled_at: _0, delay: _0 },
            #[codec(index = 2)]
            Paused,
            #[codec(index = 3)]
            PendingResume { scheduled_at: _0, delay: _0 },
        }
    }
    pub mod pallet_identity {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Identity pallet declaration."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::add_registrar`]."]
                add_registrar {
                    account: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::set_identity`]."]
                set_identity {
                    info: ::std::boxed::Box<runtime_types::pallet_identity::types::IdentityInfo>,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::set_subs`]."]
                set_subs {
                    subs: ::std::vec::Vec<(
                        ::subxt::utils::AccountId32,
                        runtime_types::pallet_identity::types::Data,
                    )>,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::clear_identity`]."]
                clear_identity,
                #[codec(index = 4)]
                #[doc = "See [`Pallet::request_judgement`]."]
                request_judgement {
                    #[codec(compact)]
                    reg_index: ::core::primitive::u32,
                    #[codec(compact)]
                    max_fee: ::core::primitive::u128,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::cancel_request`]."]
                cancel_request { reg_index: ::core::primitive::u32 },
                #[codec(index = 6)]
                #[doc = "See [`Pallet::set_fee`]."]
                set_fee {
                    #[codec(compact)]
                    index: ::core::primitive::u32,
                    #[codec(compact)]
                    fee: ::core::primitive::u128,
                },
                #[codec(index = 7)]
                #[doc = "See [`Pallet::set_account_id`]."]
                set_account_id {
                    #[codec(compact)]
                    index: ::core::primitive::u32,
                    new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 8)]
                #[doc = "See [`Pallet::set_fields`]."]
                set_fields {
                    #[codec(compact)]
                    index: ::core::primitive::u32,
                    fields: runtime_types::pallet_identity::types::BitFlags<
                        runtime_types::pallet_identity::types::IdentityField,
                    >,
                },
                #[codec(index = 9)]
                #[doc = "See [`Pallet::provide_judgement`]."]
                provide_judgement {
                    #[codec(compact)]
                    reg_index: ::core::primitive::u32,
                    target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    judgement:
                        runtime_types::pallet_identity::types::Judgement<::core::primitive::u128>,
                    identity: ::subxt::utils::H256,
                },
                #[codec(index = 10)]
                #[doc = "See [`Pallet::kill_identity`]."]
                kill_identity {
                    target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 11)]
                #[doc = "See [`Pallet::add_sub`]."]
                add_sub {
                    sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    data: runtime_types::pallet_identity::types::Data,
                },
                #[codec(index = 12)]
                #[doc = "See [`Pallet::rename_sub`]."]
                rename_sub {
                    sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    data: runtime_types::pallet_identity::types::Data,
                },
                #[codec(index = 13)]
                #[doc = "See [`Pallet::remove_sub`]."]
                remove_sub {
                    sub: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 14)]
                #[doc = "See [`Pallet::quit_sub`]."]
                quit_sub,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Too many subs-accounts."]
                TooManySubAccounts,
                #[codec(index = 1)]
                #[doc = "Account isn't found."]
                NotFound,
                #[codec(index = 2)]
                #[doc = "Account isn't named."]
                NotNamed,
                #[codec(index = 3)]
                #[doc = "Empty index."]
                EmptyIndex,
                #[codec(index = 4)]
                #[doc = "Fee is changed."]
                FeeChanged,
                #[codec(index = 5)]
                #[doc = "No identity found."]
                NoIdentity,
                #[codec(index = 6)]
                #[doc = "Sticky judgement."]
                StickyJudgement,
                #[codec(index = 7)]
                #[doc = "Judgement given."]
                JudgementGiven,
                #[codec(index = 8)]
                #[doc = "Invalid judgement."]
                InvalidJudgement,
                #[codec(index = 9)]
                #[doc = "The index is invalid."]
                InvalidIndex,
                #[codec(index = 10)]
                #[doc = "The target is invalid."]
                InvalidTarget,
                #[codec(index = 11)]
                #[doc = "Too many additional fields."]
                TooManyFields,
                #[codec(index = 12)]
                #[doc = "Maximum amount of registrars reached. Cannot add any more."]
                TooManyRegistrars,
                #[codec(index = 13)]
                #[doc = "Account ID is already named."]
                AlreadyClaimed,
                #[codec(index = 14)]
                #[doc = "Sender is not a sub-account."]
                NotSub,
                #[codec(index = 15)]
                #[doc = "Sub-account isn't owned by sender."]
                NotOwned,
                #[codec(index = 16)]
                #[doc = "The provided judgement was for a different identity."]
                JudgementForDifferentIdentity,
                #[codec(index = 17)]
                #[doc = "Error that occurs when there is an issue paying for judgement."]
                JudgementPaymentFailed,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A name was set or reset (which will remove all judgements)."]
                IdentitySet { who: ::subxt::utils::AccountId32 },
                #[codec(index = 1)]
                #[doc = "A name was cleared, and the given balance returned."]
                IdentityCleared {
                    who: ::subxt::utils::AccountId32,
                    deposit: ::core::primitive::u128,
                },
                #[codec(index = 2)]
                #[doc = "A name was removed and the given balance slashed."]
                IdentityKilled {
                    who: ::subxt::utils::AccountId32,
                    deposit: ::core::primitive::u128,
                },
                #[codec(index = 3)]
                #[doc = "A judgement was asked from a registrar."]
                JudgementRequested {
                    who: ::subxt::utils::AccountId32,
                    registrar_index: ::core::primitive::u32,
                },
                #[codec(index = 4)]
                #[doc = "A judgement request was retracted."]
                JudgementUnrequested {
                    who: ::subxt::utils::AccountId32,
                    registrar_index: ::core::primitive::u32,
                },
                #[codec(index = 5)]
                #[doc = "A judgement was given by a registrar."]
                JudgementGiven {
                    target: ::subxt::utils::AccountId32,
                    registrar_index: ::core::primitive::u32,
                },
                #[codec(index = 6)]
                #[doc = "A registrar was added."]
                RegistrarAdded {
                    registrar_index: ::core::primitive::u32,
                },
                #[codec(index = 7)]
                #[doc = "A sub-identity was added to an identity and the deposit paid."]
                SubIdentityAdded {
                    sub: ::subxt::utils::AccountId32,
                    main: ::subxt::utils::AccountId32,
                    deposit: ::core::primitive::u128,
                },
                #[codec(index = 8)]
                #[doc = "A sub-identity was removed from an identity and the deposit freed."]
                SubIdentityRemoved {
                    sub: ::subxt::utils::AccountId32,
                    main: ::subxt::utils::AccountId32,
                    deposit: ::core::primitive::u128,
                },
                #[codec(index = 9)]
                #[doc = "A sub-identity was cleared, and the given deposit repatriated from the"]
                #[doc = "main identity account to the sub-identity account."]
                SubIdentityRevoked {
                    sub: ::subxt::utils::AccountId32,
                    main: ::subxt::utils::AccountId32,
                    deposit: ::core::primitive::u128,
                },
            }
        }
        pub mod types {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct BitFlags<_0>(
                pub ::core::primitive::u64,
                #[codec(skip)] pub ::core::marker::PhantomData<_0>,
            );
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Data {
                #[codec(index = 0)]
                None,
                #[codec(index = 1)]
                Raw0([::core::primitive::u8; 0usize]),
                #[codec(index = 2)]
                Raw1([::core::primitive::u8; 1usize]),
                #[codec(index = 3)]
                Raw2([::core::primitive::u8; 2usize]),
                #[codec(index = 4)]
                Raw3([::core::primitive::u8; 3usize]),
                #[codec(index = 5)]
                Raw4([::core::primitive::u8; 4usize]),
                #[codec(index = 6)]
                Raw5([::core::primitive::u8; 5usize]),
                #[codec(index = 7)]
                Raw6([::core::primitive::u8; 6usize]),
                #[codec(index = 8)]
                Raw7([::core::primitive::u8; 7usize]),
                #[codec(index = 9)]
                Raw8([::core::primitive::u8; 8usize]),
                #[codec(index = 10)]
                Raw9([::core::primitive::u8; 9usize]),
                #[codec(index = 11)]
                Raw10([::core::primitive::u8; 10usize]),
                #[codec(index = 12)]
                Raw11([::core::primitive::u8; 11usize]),
                #[codec(index = 13)]
                Raw12([::core::primitive::u8; 12usize]),
                #[codec(index = 14)]
                Raw13([::core::primitive::u8; 13usize]),
                #[codec(index = 15)]
                Raw14([::core::primitive::u8; 14usize]),
                #[codec(index = 16)]
                Raw15([::core::primitive::u8; 15usize]),
                #[codec(index = 17)]
                Raw16([::core::primitive::u8; 16usize]),
                #[codec(index = 18)]
                Raw17([::core::primitive::u8; 17usize]),
                #[codec(index = 19)]
                Raw18([::core::primitive::u8; 18usize]),
                #[codec(index = 20)]
                Raw19([::core::primitive::u8; 19usize]),
                #[codec(index = 21)]
                Raw20([::core::primitive::u8; 20usize]),
                #[codec(index = 22)]
                Raw21([::core::primitive::u8; 21usize]),
                #[codec(index = 23)]
                Raw22([::core::primitive::u8; 22usize]),
                #[codec(index = 24)]
                Raw23([::core::primitive::u8; 23usize]),
                #[codec(index = 25)]
                Raw24([::core::primitive::u8; 24usize]),
                #[codec(index = 26)]
                Raw25([::core::primitive::u8; 25usize]),
                #[codec(index = 27)]
                Raw26([::core::primitive::u8; 26usize]),
                #[codec(index = 28)]
                Raw27([::core::primitive::u8; 27usize]),
                #[codec(index = 29)]
                Raw28([::core::primitive::u8; 28usize]),
                #[codec(index = 30)]
                Raw29([::core::primitive::u8; 29usize]),
                #[codec(index = 31)]
                Raw30([::core::primitive::u8; 30usize]),
                #[codec(index = 32)]
                Raw31([::core::primitive::u8; 31usize]),
                #[codec(index = 33)]
                Raw32([::core::primitive::u8; 32usize]),
                #[codec(index = 34)]
                BlakeTwo256([::core::primitive::u8; 32usize]),
                #[codec(index = 35)]
                Sha256([::core::primitive::u8; 32usize]),
                #[codec(index = 36)]
                Keccak256([::core::primitive::u8; 32usize]),
                #[codec(index = 37)]
                ShaThree256([::core::primitive::u8; 32usize]),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum IdentityField {
                #[codec(index = 1)]
                Display,
                #[codec(index = 2)]
                Legal,
                #[codec(index = 4)]
                Web,
                #[codec(index = 8)]
                Riot,
                #[codec(index = 16)]
                Email,
                #[codec(index = 32)]
                PgpFingerprint,
                #[codec(index = 64)]
                Image,
                #[codec(index = 128)]
                Twitter,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct IdentityInfo {
                pub additional: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                    runtime_types::pallet_identity::types::Data,
                    runtime_types::pallet_identity::types::Data,
                )>,
                pub display: runtime_types::pallet_identity::types::Data,
                pub legal: runtime_types::pallet_identity::types::Data,
                pub web: runtime_types::pallet_identity::types::Data,
                pub riot: runtime_types::pallet_identity::types::Data,
                pub email: runtime_types::pallet_identity::types::Data,
                pub pgp_fingerprint: ::core::option::Option<[::core::primitive::u8; 20usize]>,
                pub image: runtime_types::pallet_identity::types::Data,
                pub twitter: runtime_types::pallet_identity::types::Data,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Judgement<_0> {
                #[codec(index = 0)]
                Unknown,
                #[codec(index = 1)]
                FeePaid(_0),
                #[codec(index = 2)]
                Reasonable,
                #[codec(index = 3)]
                KnownGood,
                #[codec(index = 4)]
                OutOfDate,
                #[codec(index = 5)]
                LowQuality,
                #[codec(index = 6)]
                Erroneous,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RegistrarInfo<_0, _1> {
                pub account: _1,
                pub fee: _0,
                pub fields: runtime_types::pallet_identity::types::BitFlags<
                    runtime_types::pallet_identity::types::IdentityField,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Registration<_0> {
                pub judgements: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                    ::core::primitive::u32,
                    runtime_types::pallet_identity::types::Judgement<_0>,
                )>,
                pub deposit: _0,
                pub info: runtime_types::pallet_identity::types::IdentityInfo,
            }
        }
    }
    pub mod pallet_im_online {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::heartbeat`]."]
                heartbeat {
                    heartbeat: runtime_types::pallet_im_online::Heartbeat<::core::primitive::u32>,
                    signature: runtime_types::pallet_im_online::sr25519::app_sr25519::Signature,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Non existent public key."]
                InvalidKey,
                #[codec(index = 1)]
                #[doc = "Duplicated heartbeat."]
                DuplicatedHeartbeat,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A new heartbeat was received from `AuthorityId`."]
                HeartbeatReceived {
                    authority_id: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
                },
                #[codec(index = 1)]
                #[doc = "At the end of the session, no offence was committed."]
                AllGood,
                #[codec(index = 2)]
                #[doc = "At the end of the session, at least one validator was found to be offline."]
                SomeOffline {
                    offline: ::std::vec::Vec<(
                        ::subxt::utils::AccountId32,
                        runtime_types::pallet_staking::Exposure<
                            ::subxt::utils::AccountId32,
                            ::core::primitive::u128,
                        >,
                    )>,
                },
            }
        }
        pub mod sr25519 {
            use super::runtime_types;
            pub mod app_sr25519 {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Heartbeat<_0> {
            pub block_number: _0,
            pub session_index: ::core::primitive::u32,
            pub authority_index: ::core::primitive::u32,
            pub validators_len: ::core::primitive::u32,
        }
    }
    pub mod pallet_indices {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::claim`]."]
                claim { index: ::core::primitive::u32 },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::transfer`]."]
                transfer {
                    new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    index: ::core::primitive::u32,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::free`]."]
                free { index: ::core::primitive::u32 },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::force_transfer`]."]
                force_transfer {
                    new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    index: ::core::primitive::u32,
                    freeze: ::core::primitive::bool,
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::freeze`]."]
                freeze { index: ::core::primitive::u32 },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "The index was not already assigned."]
                NotAssigned,
                #[codec(index = 1)]
                #[doc = "The index is assigned to another account."]
                NotOwner,
                #[codec(index = 2)]
                #[doc = "The index was not available."]
                InUse,
                #[codec(index = 3)]
                #[doc = "The source and destination accounts are identical."]
                NotTransfer,
                #[codec(index = 4)]
                #[doc = "The index is permanent and may not be freed/changed."]
                Permanent,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A account index was assigned."]
                IndexAssigned {
                    who: ::subxt::utils::AccountId32,
                    index: ::core::primitive::u32,
                },
                #[codec(index = 1)]
                #[doc = "A account index has been freed up (unassigned)."]
                IndexFreed { index: ::core::primitive::u32 },
                #[codec(index = 2)]
                #[doc = "A account index has been frozen to its current account ID."]
                IndexFrozen {
                    index: ::core::primitive::u32,
                    who: ::subxt::utils::AccountId32,
                },
            }
        }
    }
    pub mod pallet_membership {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::add_member`]."]
                add_member {
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::remove_member`]."]
                remove_member {
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::swap_member`]."]
                swap_member {
                    remove: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    add: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::reset_members`]."]
                reset_members {
                    members: ::std::vec::Vec<::subxt::utils::AccountId32>,
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::change_key`]."]
                change_key {
                    new: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::set_prime`]."]
                set_prime {
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 6)]
                #[doc = "See [`Pallet::clear_prime`]."]
                clear_prime,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Already a member."]
                AlreadyMember,
                #[codec(index = 1)]
                #[doc = "Not a member."]
                NotMember,
                #[codec(index = 2)]
                #[doc = "Too many members."]
                TooManyMembers,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "The given member was added; see the transaction for who."]
                MemberAdded,
                #[codec(index = 1)]
                #[doc = "The given member was removed; see the transaction for who."]
                MemberRemoved,
                #[codec(index = 2)]
                #[doc = "Two members were swapped; see the transaction for who."]
                MembersSwapped,
                #[codec(index = 3)]
                #[doc = "The membership was reset; see the transaction for who the new set is."]
                MembersReset,
                #[codec(index = 4)]
                #[doc = "One of the members' keys changed."]
                KeyChanged,
                #[codec(index = 5)]
                #[doc = "Phantom member, never used."]
                Dummy,
            }
        }
    }
    pub mod pallet_message_queue {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                # [codec (index = 0)] # [doc = "See [`Pallet::reap_page`]."] reap_page { message_origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , page_index : :: core :: primitive :: u32 , } , # [codec (index = 1)] # [doc = "See [`Pallet::execute_overweight`]."] execute_overweight { message_origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , page : :: core :: primitive :: u32 , index : :: core :: primitive :: u32 , weight_limit : runtime_types :: sp_weights :: weight_v2 :: Weight , } , }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Page is not reapable because it has items remaining to be processed and is not old"]
                #[doc = "enough."]
                NotReapable,
                #[codec(index = 1)]
                #[doc = "Page to be reaped does not exist."]
                NoPage,
                #[codec(index = 2)]
                #[doc = "The referenced message could not be found."]
                NoMessage,
                #[codec(index = 3)]
                #[doc = "The message was already processed and cannot be processed again."]
                AlreadyProcessed,
                #[codec(index = 4)]
                #[doc = "The message is queued for future execution."]
                Queued,
                #[codec(index = 5)]
                #[doc = "There is temporarily not enough weight to continue servicing messages."]
                InsufficientWeight,
                #[codec(index = 6)]
                #[doc = "This message is temporarily unprocessable."]
                #[doc = ""]
                #[doc = "Such errors are expected, but not guaranteed, to resolve themselves eventually through"]
                #[doc = "retrying."]
                TemporarilyUnprocessable,
                #[codec(index = 7)]
                #[doc = "The queue is paused and no message can be executed from it."]
                #[doc = ""]
                #[doc = "This can change at any time and may resolve in the future by re-trying."]
                QueuePaused,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                # [codec (index = 0)] # [doc = "Message discarded due to an error in the `MessageProcessor` (usually a format error)."] ProcessingFailed { id : [:: core :: primitive :: u8 ; 32usize] , origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , error : runtime_types :: frame_support :: traits :: messages :: ProcessMessageError , } , # [codec (index = 1)] # [doc = "Message is processed."] Processed { id : [:: core :: primitive :: u8 ; 32usize] , origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , weight_used : runtime_types :: sp_weights :: weight_v2 :: Weight , success : :: core :: primitive :: bool , } , # [codec (index = 2)] # [doc = "Message placed in overweight queue."] OverweightEnqueued { id : [:: core :: primitive :: u8 ; 32usize] , origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , page_index : :: core :: primitive :: u32 , message_index : :: core :: primitive :: u32 , } , # [codec (index = 3)] # [doc = "This page was reaped."] PageReaped { origin : runtime_types :: polkadot_runtime_parachains :: inclusion :: AggregateMessageOrigin , index : :: core :: primitive :: u32 , } , }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct BookState<_0> {
            pub begin: ::core::primitive::u32,
            pub end: ::core::primitive::u32,
            pub count: ::core::primitive::u32,
            pub ready_neighbours:
                ::core::option::Option<runtime_types::pallet_message_queue::Neighbours<_0>>,
            pub message_count: ::core::primitive::u64,
            pub size: ::core::primitive::u64,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Neighbours<_0> {
            pub prev: _0,
            pub next: _0,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Page<_0> {
            pub remaining: _0,
            pub remaining_size: _0,
            pub first_index: _0,
            pub first: _0,
            pub last: _0,
            pub heap:
                runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u8>,
        }
    }
    pub mod pallet_multisig {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::as_multi_threshold_1`]."]
                as_multi_threshold_1 {
                    other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::as_multi`]."]
                as_multi {
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                    max_weight: runtime_types::sp_weights::weight_v2::Weight,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::approve_as_multi`]."]
                approve_as_multi {
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    maybe_timepoint: ::core::option::Option<
                        runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    >,
                    call_hash: [::core::primitive::u8; 32usize],
                    max_weight: runtime_types::sp_weights::weight_v2::Weight,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::cancel_as_multi`]."]
                cancel_as_multi {
                    threshold: ::core::primitive::u16,
                    other_signatories: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    call_hash: [::core::primitive::u8; 32usize],
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Threshold must be 2 or greater."]
                MinimumThreshold,
                #[codec(index = 1)]
                #[doc = "Call is already approved by this signatory."]
                AlreadyApproved,
                #[codec(index = 2)]
                #[doc = "Call doesn't need any (more) approvals."]
                NoApprovalsNeeded,
                #[codec(index = 3)]
                #[doc = "There are too few signatories in the list."]
                TooFewSignatories,
                #[codec(index = 4)]
                #[doc = "There are too many signatories in the list."]
                TooManySignatories,
                #[codec(index = 5)]
                #[doc = "The signatories were provided out of order; they should be ordered."]
                SignatoriesOutOfOrder,
                #[codec(index = 6)]
                #[doc = "The sender was contained in the other signatories; it shouldn't be."]
                SenderInSignatories,
                #[codec(index = 7)]
                #[doc = "Multisig operation not found when attempting to cancel."]
                NotFound,
                #[codec(index = 8)]
                #[doc = "Only the account that originally created the multisig is able to cancel it."]
                NotOwner,
                #[codec(index = 9)]
                #[doc = "No timepoint was given, yet the multisig operation is already underway."]
                NoTimepoint,
                #[codec(index = 10)]
                #[doc = "A different timepoint was given to the multisig operation that is underway."]
                WrongTimepoint,
                #[codec(index = 11)]
                #[doc = "A timepoint was given, yet no multisig operation is underway."]
                UnexpectedTimepoint,
                #[codec(index = 12)]
                #[doc = "The maximum weight information provided was too low."]
                MaxWeightTooLow,
                #[codec(index = 13)]
                #[doc = "The data to be stored is already stored."]
                AlreadyStored,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A new multisig operation has begun."]
                NewMultisig {
                    approving: ::subxt::utils::AccountId32,
                    multisig: ::subxt::utils::AccountId32,
                    call_hash: [::core::primitive::u8; 32usize],
                },
                #[codec(index = 1)]
                #[doc = "A multisig operation has been approved by someone."]
                MultisigApproval {
                    approving: ::subxt::utils::AccountId32,
                    timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    multisig: ::subxt::utils::AccountId32,
                    call_hash: [::core::primitive::u8; 32usize],
                },
                #[codec(index = 2)]
                #[doc = "A multisig operation has been executed."]
                MultisigExecuted {
                    approving: ::subxt::utils::AccountId32,
                    timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    multisig: ::subxt::utils::AccountId32,
                    call_hash: [::core::primitive::u8; 32usize],
                    result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                },
                #[codec(index = 3)]
                #[doc = "A multisig operation has been cancelled."]
                MultisigCancelled {
                    cancelling: ::subxt::utils::AccountId32,
                    timepoint: runtime_types::pallet_multisig::Timepoint<::core::primitive::u32>,
                    multisig: ::subxt::utils::AccountId32,
                    call_hash: [::core::primitive::u8; 32usize],
                },
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Multisig<_0, _1, _2> {
            pub when: runtime_types::pallet_multisig::Timepoint<_0>,
            pub deposit: _1,
            pub depositor: _2,
            pub approvals: runtime_types::bounded_collections::bounded_vec::BoundedVec<_2>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Timepoint<_0> {
            pub height: _0,
            pub index: ::core::primitive::u32,
        }
    }
    pub mod pallet_nomination_pools {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::join`]."]
                join {
                    #[codec(compact)]
                    amount: ::core::primitive::u128,
                    pool_id: ::core::primitive::u32,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::bond_extra`]."]
                bond_extra {
                    extra:
                        runtime_types::pallet_nomination_pools::BondExtra<::core::primitive::u128>,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::claim_payout`]."]
                claim_payout,
                #[codec(index = 3)]
                #[doc = "See [`Pallet::unbond`]."]
                unbond {
                    member_account: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    unbonding_points: ::core::primitive::u128,
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::pool_withdraw_unbonded`]."]
                pool_withdraw_unbonded {
                    pool_id: ::core::primitive::u32,
                    num_slashing_spans: ::core::primitive::u32,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::withdraw_unbonded`]."]
                withdraw_unbonded {
                    member_account: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    num_slashing_spans: ::core::primitive::u32,
                },
                #[codec(index = 6)]
                #[doc = "See [`Pallet::create`]."]
                create {
                    #[codec(compact)]
                    amount: ::core::primitive::u128,
                    root: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    nominator: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    bouncer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 7)]
                #[doc = "See [`Pallet::create_with_pool_id`]."]
                create_with_pool_id {
                    #[codec(compact)]
                    amount: ::core::primitive::u128,
                    root: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    nominator: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    bouncer: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    pool_id: ::core::primitive::u32,
                },
                #[codec(index = 8)]
                #[doc = "See [`Pallet::nominate`]."]
                nominate {
                    pool_id: ::core::primitive::u32,
                    validators: ::std::vec::Vec<::subxt::utils::AccountId32>,
                },
                #[codec(index = 9)]
                #[doc = "See [`Pallet::set_state`]."]
                set_state {
                    pool_id: ::core::primitive::u32,
                    state: runtime_types::pallet_nomination_pools::PoolState,
                },
                #[codec(index = 10)]
                #[doc = "See [`Pallet::set_metadata`]."]
                set_metadata {
                    pool_id: ::core::primitive::u32,
                    metadata: ::std::vec::Vec<::core::primitive::u8>,
                },
                #[codec(index = 11)]
                #[doc = "See [`Pallet::set_configs`]."]
                set_configs {
                    min_join_bond:
                        runtime_types::pallet_nomination_pools::ConfigOp<::core::primitive::u128>,
                    min_create_bond:
                        runtime_types::pallet_nomination_pools::ConfigOp<::core::primitive::u128>,
                    max_pools:
                        runtime_types::pallet_nomination_pools::ConfigOp<::core::primitive::u32>,
                    max_members:
                        runtime_types::pallet_nomination_pools::ConfigOp<::core::primitive::u32>,
                    max_members_per_pool:
                        runtime_types::pallet_nomination_pools::ConfigOp<::core::primitive::u32>,
                    global_max_commission: runtime_types::pallet_nomination_pools::ConfigOp<
                        runtime_types::sp_arithmetic::per_things::Perbill,
                    >,
                },
                #[codec(index = 12)]
                #[doc = "See [`Pallet::update_roles`]."]
                update_roles {
                    pool_id: ::core::primitive::u32,
                    new_root: runtime_types::pallet_nomination_pools::ConfigOp<
                        ::subxt::utils::AccountId32,
                    >,
                    new_nominator: runtime_types::pallet_nomination_pools::ConfigOp<
                        ::subxt::utils::AccountId32,
                    >,
                    new_bouncer: runtime_types::pallet_nomination_pools::ConfigOp<
                        ::subxt::utils::AccountId32,
                    >,
                },
                #[codec(index = 13)]
                #[doc = "See [`Pallet::chill`]."]
                chill { pool_id: ::core::primitive::u32 },
                #[codec(index = 14)]
                #[doc = "See [`Pallet::bond_extra_other`]."]
                bond_extra_other {
                    member: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    extra:
                        runtime_types::pallet_nomination_pools::BondExtra<::core::primitive::u128>,
                },
                #[codec(index = 15)]
                #[doc = "See [`Pallet::set_claim_permission`]."]
                set_claim_permission {
                    permission: runtime_types::pallet_nomination_pools::ClaimPermission,
                },
                #[codec(index = 16)]
                #[doc = "See [`Pallet::claim_payout_other`]."]
                claim_payout_other { other: ::subxt::utils::AccountId32 },
                #[codec(index = 17)]
                #[doc = "See [`Pallet::set_commission`]."]
                set_commission {
                    pool_id: ::core::primitive::u32,
                    new_commission: ::core::option::Option<(
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        ::subxt::utils::AccountId32,
                    )>,
                },
                #[codec(index = 18)]
                #[doc = "See [`Pallet::set_commission_max`]."]
                set_commission_max {
                    pool_id: ::core::primitive::u32,
                    max_commission: runtime_types::sp_arithmetic::per_things::Perbill,
                },
                #[codec(index = 19)]
                #[doc = "See [`Pallet::set_commission_change_rate`]."]
                set_commission_change_rate {
                    pool_id: ::core::primitive::u32,
                    change_rate: runtime_types::pallet_nomination_pools::CommissionChangeRate<
                        ::core::primitive::u32,
                    >,
                },
                #[codec(index = 20)]
                #[doc = "See [`Pallet::claim_commission`]."]
                claim_commission { pool_id: ::core::primitive::u32 },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum DefensiveError {
                #[codec(index = 0)]
                NotEnoughSpaceInUnbondPool,
                #[codec(index = 1)]
                PoolNotFound,
                #[codec(index = 2)]
                RewardPoolNotFound,
                #[codec(index = 3)]
                SubPoolsNotFound,
                #[codec(index = 4)]
                BondedStashKilledPrematurely,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "A (bonded) pool id does not exist."]
                PoolNotFound,
                #[codec(index = 1)]
                #[doc = "An account is not a member."]
                PoolMemberNotFound,
                #[codec(index = 2)]
                #[doc = "A reward pool does not exist. In all cases this is a system logic error."]
                RewardPoolNotFound,
                #[codec(index = 3)]
                #[doc = "A sub pool does not exist."]
                SubPoolsNotFound,
                #[codec(index = 4)]
                #[doc = "An account is already delegating in another pool. An account may only belong to one"]
                #[doc = "pool at a time."]
                AccountBelongsToOtherPool,
                #[codec(index = 5)]
                #[doc = "The member is fully unbonded (and thus cannot access the bonded and reward pool"]
                #[doc = "anymore to, for example, collect rewards)."]
                FullyUnbonding,
                #[codec(index = 6)]
                #[doc = "The member cannot unbond further chunks due to reaching the limit."]
                MaxUnbondingLimit,
                #[codec(index = 7)]
                #[doc = "None of the funds can be withdrawn yet because the bonding duration has not passed."]
                CannotWithdrawAny,
                #[codec(index = 8)]
                #[doc = "The amount does not meet the minimum bond to either join or create a pool."]
                #[doc = ""]
                #[doc = "The depositor can never unbond to a value less than"]
                #[doc = "`Pallet::depositor_min_bond`. The caller does not have nominating"]
                #[doc = "permissions for the pool. Members can never unbond to a value below `MinJoinBond`."]
                MinimumBondNotMet,
                #[codec(index = 9)]
                #[doc = "The transaction could not be executed due to overflow risk for the pool."]
                OverflowRisk,
                #[codec(index = 10)]
                #[doc = "A pool must be in [`PoolState::Destroying`] in order for the depositor to unbond or for"]
                #[doc = "other members to be permissionlessly unbonded."]
                NotDestroying,
                #[codec(index = 11)]
                #[doc = "The caller does not have nominating permissions for the pool."]
                NotNominator,
                #[codec(index = 12)]
                #[doc = "Either a) the caller cannot make a valid kick or b) the pool is not destroying."]
                NotKickerOrDestroying,
                #[codec(index = 13)]
                #[doc = "The pool is not open to join"]
                NotOpen,
                #[codec(index = 14)]
                #[doc = "The system is maxed out on pools."]
                MaxPools,
                #[codec(index = 15)]
                #[doc = "Too many members in the pool or system."]
                MaxPoolMembers,
                #[codec(index = 16)]
                #[doc = "The pools state cannot be changed."]
                CanNotChangeState,
                #[codec(index = 17)]
                #[doc = "The caller does not have adequate permissions."]
                DoesNotHavePermission,
                #[codec(index = 18)]
                #[doc = "Metadata exceeds [`Config::MaxMetadataLen`]"]
                MetadataExceedsMaxLen,
                #[codec(index = 19)]
                #[doc = "Some error occurred that should never happen. This should be reported to the"]
                #[doc = "maintainers."]
                Defensive(runtime_types::pallet_nomination_pools::pallet::DefensiveError),
                #[codec(index = 20)]
                #[doc = "Partial unbonding now allowed permissionlessly."]
                PartialUnbondNotAllowedPermissionlessly,
                #[codec(index = 21)]
                #[doc = "The pool's max commission cannot be set higher than the existing value."]
                MaxCommissionRestricted,
                #[codec(index = 22)]
                #[doc = "The supplied commission exceeds the max allowed commission."]
                CommissionExceedsMaximum,
                #[codec(index = 23)]
                #[doc = "Not enough blocks have surpassed since the last commission update."]
                CommissionChangeThrottled,
                #[codec(index = 24)]
                #[doc = "The submitted changes to commission change rate are not allowed."]
                CommissionChangeRateNotAllowed,
                #[codec(index = 25)]
                #[doc = "There is no pending commission to claim."]
                NoPendingCommission,
                #[codec(index = 26)]
                #[doc = "No commission current has been set."]
                NoCommissionCurrentSet,
                #[codec(index = 27)]
                #[doc = "Pool id currently in use."]
                PoolIdInUse,
                #[codec(index = 28)]
                #[doc = "Pool id provided is not correct/usable."]
                InvalidPoolId,
                #[codec(index = 29)]
                #[doc = "Bonding extra is restricted to the exact pending reward amount."]
                BondExtraRestricted,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Events of this pallet."]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A pool has been created."]
                Created {
                    depositor: ::subxt::utils::AccountId32,
                    pool_id: ::core::primitive::u32,
                },
                #[codec(index = 1)]
                #[doc = "A member has became bonded in a pool."]
                Bonded {
                    member: ::subxt::utils::AccountId32,
                    pool_id: ::core::primitive::u32,
                    bonded: ::core::primitive::u128,
                    joined: ::core::primitive::bool,
                },
                #[codec(index = 2)]
                #[doc = "A payout has been made to a member."]
                PaidOut {
                    member: ::subxt::utils::AccountId32,
                    pool_id: ::core::primitive::u32,
                    payout: ::core::primitive::u128,
                },
                #[codec(index = 3)]
                #[doc = "A member has unbonded from their pool."]
                #[doc = ""]
                #[doc = "- `balance` is the corresponding balance of the number of points that has been"]
                #[doc = "  requested to be unbonded (the argument of the `unbond` transaction) from the bonded"]
                #[doc = "  pool."]
                #[doc = "- `points` is the number of points that are issued as a result of `balance` being"]
                #[doc = "dissolved into the corresponding unbonding pool."]
                #[doc = "- `era` is the era in which the balance will be unbonded."]
                #[doc = "In the absence of slashing, these values will match. In the presence of slashing, the"]
                #[doc = "number of points that are issued in the unbonding pool will be less than the amount"]
                #[doc = "requested to be unbonded."]
                Unbonded {
                    member: ::subxt::utils::AccountId32,
                    pool_id: ::core::primitive::u32,
                    balance: ::core::primitive::u128,
                    points: ::core::primitive::u128,
                    era: ::core::primitive::u32,
                },
                #[codec(index = 4)]
                #[doc = "A member has withdrawn from their pool."]
                #[doc = ""]
                #[doc = "The given number of `points` have been dissolved in return of `balance`."]
                #[doc = ""]
                #[doc = "Similar to `Unbonded` event, in the absence of slashing, the ratio of point to balance"]
                #[doc = "will be 1."]
                Withdrawn {
                    member: ::subxt::utils::AccountId32,
                    pool_id: ::core::primitive::u32,
                    balance: ::core::primitive::u128,
                    points: ::core::primitive::u128,
                },
                #[codec(index = 5)]
                #[doc = "A pool has been destroyed."]
                Destroyed { pool_id: ::core::primitive::u32 },
                #[codec(index = 6)]
                #[doc = "The state of a pool has changed"]
                StateChanged {
                    pool_id: ::core::primitive::u32,
                    new_state: runtime_types::pallet_nomination_pools::PoolState,
                },
                #[codec(index = 7)]
                #[doc = "A member has been removed from a pool."]
                #[doc = ""]
                #[doc = "The removal can be voluntary (withdrawn all unbonded funds) or involuntary (kicked)."]
                MemberRemoved {
                    pool_id: ::core::primitive::u32,
                    member: ::subxt::utils::AccountId32,
                },
                #[codec(index = 8)]
                #[doc = "The roles of a pool have been updated to the given new roles. Note that the depositor"]
                #[doc = "can never change."]
                RolesUpdated {
                    root: ::core::option::Option<::subxt::utils::AccountId32>,
                    bouncer: ::core::option::Option<::subxt::utils::AccountId32>,
                    nominator: ::core::option::Option<::subxt::utils::AccountId32>,
                },
                #[codec(index = 9)]
                #[doc = "The active balance of pool `pool_id` has been slashed to `balance`."]
                PoolSlashed {
                    pool_id: ::core::primitive::u32,
                    balance: ::core::primitive::u128,
                },
                #[codec(index = 10)]
                #[doc = "The unbond pool at `era` of pool `pool_id` has been slashed to `balance`."]
                UnbondingPoolSlashed {
                    pool_id: ::core::primitive::u32,
                    era: ::core::primitive::u32,
                    balance: ::core::primitive::u128,
                },
                #[codec(index = 11)]
                #[doc = "A pool's commission setting has been changed."]
                PoolCommissionUpdated {
                    pool_id: ::core::primitive::u32,
                    current: ::core::option::Option<(
                        runtime_types::sp_arithmetic::per_things::Perbill,
                        ::subxt::utils::AccountId32,
                    )>,
                },
                #[codec(index = 12)]
                #[doc = "A pool's maximum commission setting has been changed."]
                PoolMaxCommissionUpdated {
                    pool_id: ::core::primitive::u32,
                    max_commission: runtime_types::sp_arithmetic::per_things::Perbill,
                },
                #[codec(index = 13)]
                #[doc = "A pool's commission `change_rate` has been changed."]
                PoolCommissionChangeRateUpdated {
                    pool_id: ::core::primitive::u32,
                    change_rate: runtime_types::pallet_nomination_pools::CommissionChangeRate<
                        ::core::primitive::u32,
                    >,
                },
                #[codec(index = 14)]
                #[doc = "Pool commission has been claimed."]
                PoolCommissionClaimed {
                    pool_id: ::core::primitive::u32,
                    commission: ::core::primitive::u128,
                },
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum BondExtra<_0> {
            #[codec(index = 0)]
            FreeBalance(_0),
            #[codec(index = 1)]
            Rewards,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct BondedPoolInner {
            pub commission: runtime_types::pallet_nomination_pools::Commission,
            pub member_counter: ::core::primitive::u32,
            pub points: ::core::primitive::u128,
            pub roles:
                runtime_types::pallet_nomination_pools::PoolRoles<::subxt::utils::AccountId32>,
            pub state: runtime_types::pallet_nomination_pools::PoolState,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum ClaimPermission {
            #[codec(index = 0)]
            Permissioned,
            #[codec(index = 1)]
            PermissionlessCompound,
            #[codec(index = 2)]
            PermissionlessWithdraw,
            #[codec(index = 3)]
            PermissionlessAll,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Commission {
            pub current: ::core::option::Option<(
                runtime_types::sp_arithmetic::per_things::Perbill,
                ::subxt::utils::AccountId32,
            )>,
            pub max: ::core::option::Option<runtime_types::sp_arithmetic::per_things::Perbill>,
            pub change_rate: ::core::option::Option<
                runtime_types::pallet_nomination_pools::CommissionChangeRate<
                    ::core::primitive::u32,
                >,
            >,
            pub throttle_from: ::core::option::Option<::core::primitive::u32>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct CommissionChangeRate<_0> {
            pub max_increase: runtime_types::sp_arithmetic::per_things::Perbill,
            pub min_delay: _0,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum ConfigOp<_0> {
            #[codec(index = 0)]
            Noop,
            #[codec(index = 1)]
            Set(_0),
            #[codec(index = 2)]
            Remove,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct PoolMember {
            pub pool_id: ::core::primitive::u32,
            pub points: ::core::primitive::u128,
            pub last_recorded_reward_counter: runtime_types::sp_arithmetic::fixed_point::FixedU128,
            pub unbonding_eras:
                runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
                    ::core::primitive::u32,
                    ::core::primitive::u128,
                >,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct PoolRoles<_0> {
            pub depositor: _0,
            pub root: ::core::option::Option<_0>,
            pub nominator: ::core::option::Option<_0>,
            pub bouncer: ::core::option::Option<_0>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum PoolState {
            #[codec(index = 0)]
            Open,
            #[codec(index = 1)]
            Blocked,
            #[codec(index = 2)]
            Destroying,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct RewardPool {
            pub last_recorded_reward_counter: runtime_types::sp_arithmetic::fixed_point::FixedU128,
            pub last_recorded_total_payouts: ::core::primitive::u128,
            pub total_rewards_claimed: ::core::primitive::u128,
            pub total_commission_pending: ::core::primitive::u128,
            pub total_commission_claimed: ::core::primitive::u128,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct SubPools {
            pub no_era: runtime_types::pallet_nomination_pools::UnbondPool,
            pub with_era: runtime_types::bounded_collections::bounded_btree_map::BoundedBTreeMap<
                ::core::primitive::u32,
                runtime_types::pallet_nomination_pools::UnbondPool,
            >,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct UnbondPool {
            pub points: ::core::primitive::u128,
            pub balance: ::core::primitive::u128,
        }
    }
    pub mod pallet_offences {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Events type."]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "There is an offence reported of the given `kind` happened at the `session_index` and"]
                #[doc = "(kind-specific) time slot. This event is not deposited for duplicate slashes."]
                #[doc = "\\[kind, timeslot\\]."]
                Offence {
                    kind: [::core::primitive::u8; 16usize],
                    timeslot: ::std::vec::Vec<::core::primitive::u8>,
                },
            }
        }
    }
    pub mod pallet_preimage {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::note_preimage`]."]
                note_preimage {
                    bytes: ::std::vec::Vec<::core::primitive::u8>,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::unnote_preimage`]."]
                unnote_preimage { hash: ::subxt::utils::H256 },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::request_preimage`]."]
                request_preimage { hash: ::subxt::utils::H256 },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::unrequest_preimage`]."]
                unrequest_preimage { hash: ::subxt::utils::H256 },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Preimage is too large to store on-chain."]
                TooBig,
                #[codec(index = 1)]
                #[doc = "Preimage has already been noted on-chain."]
                AlreadyNoted,
                #[codec(index = 2)]
                #[doc = "The user is not authorized to perform this action."]
                NotAuthorized,
                #[codec(index = 3)]
                #[doc = "The preimage cannot be removed since it has not yet been noted."]
                NotNoted,
                #[codec(index = 4)]
                #[doc = "A preimage may not be removed when there are outstanding requests."]
                Requested,
                #[codec(index = 5)]
                #[doc = "The preimage request cannot be removed since no outstanding requests exist."]
                NotRequested,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A preimage has been noted."]
                Noted { hash: ::subxt::utils::H256 },
                #[codec(index = 1)]
                #[doc = "A preimage has been requested."]
                Requested { hash: ::subxt::utils::H256 },
                #[codec(index = 2)]
                #[doc = "A preimage has ben cleared."]
                Cleared { hash: ::subxt::utils::H256 },
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum RequestStatus<_0, _1> {
            #[codec(index = 0)]
            Unrequested {
                deposit: (_0, _1),
                len: ::core::primitive::u32,
            },
            #[codec(index = 1)]
            Requested {
                deposit: ::core::option::Option<(_0, _1)>,
                count: ::core::primitive::u32,
                len: ::core::option::Option<::core::primitive::u32>,
            },
        }
    }
    pub mod pallet_proxy {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::proxy`]."]
                proxy {
                    real: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    force_proxy_type:
                        ::core::option::Option<runtime_types::polkadot_runtime::ProxyType>,
                    call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::add_proxy`]."]
                add_proxy {
                    delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    proxy_type: runtime_types::polkadot_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::remove_proxy`]."]
                remove_proxy {
                    delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    proxy_type: runtime_types::polkadot_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::remove_proxies`]."]
                remove_proxies,
                #[codec(index = 4)]
                #[doc = "See [`Pallet::create_pure`]."]
                create_pure {
                    proxy_type: runtime_types::polkadot_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                    index: ::core::primitive::u16,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::kill_pure`]."]
                kill_pure {
                    spawner: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    proxy_type: runtime_types::polkadot_runtime::ProxyType,
                    index: ::core::primitive::u16,
                    #[codec(compact)]
                    height: ::core::primitive::u32,
                    #[codec(compact)]
                    ext_index: ::core::primitive::u32,
                },
                #[codec(index = 6)]
                #[doc = "See [`Pallet::announce`]."]
                announce {
                    real: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    call_hash: ::subxt::utils::H256,
                },
                #[codec(index = 7)]
                #[doc = "See [`Pallet::remove_announcement`]."]
                remove_announcement {
                    real: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    call_hash: ::subxt::utils::H256,
                },
                #[codec(index = 8)]
                #[doc = "See [`Pallet::reject_announcement`]."]
                reject_announcement {
                    delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    call_hash: ::subxt::utils::H256,
                },
                #[codec(index = 9)]
                #[doc = "See [`Pallet::proxy_announced`]."]
                proxy_announced {
                    delegate: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    real: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    force_proxy_type:
                        ::core::option::Option<runtime_types::polkadot_runtime::ProxyType>,
                    call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "There are too many proxies registered or too many announcements pending."]
                TooMany,
                #[codec(index = 1)]
                #[doc = "Proxy registration not found."]
                NotFound,
                #[codec(index = 2)]
                #[doc = "Sender is not a proxy of the account to be proxied."]
                NotProxy,
                #[codec(index = 3)]
                #[doc = "A call which is incompatible with the proxy type's filter was attempted."]
                Unproxyable,
                #[codec(index = 4)]
                #[doc = "Account is already a proxy."]
                Duplicate,
                #[codec(index = 5)]
                #[doc = "Call may not be made by proxy because it may escalate its privileges."]
                NoPermission,
                #[codec(index = 6)]
                #[doc = "Announcement, if made at all, was made too recently."]
                Unannounced,
                #[codec(index = 7)]
                #[doc = "Cannot add self as proxy."]
                NoSelfProxy,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A proxy was executed correctly, with the given."]
                ProxyExecuted {
                    result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                },
                #[codec(index = 1)]
                #[doc = "A pure account has been created by new proxy with given"]
                #[doc = "disambiguation index and proxy type."]
                PureCreated {
                    pure: ::subxt::utils::AccountId32,
                    who: ::subxt::utils::AccountId32,
                    proxy_type: runtime_types::polkadot_runtime::ProxyType,
                    disambiguation_index: ::core::primitive::u16,
                },
                #[codec(index = 2)]
                #[doc = "An announcement was placed to make a call in the future."]
                Announced {
                    real: ::subxt::utils::AccountId32,
                    proxy: ::subxt::utils::AccountId32,
                    call_hash: ::subxt::utils::H256,
                },
                #[codec(index = 3)]
                #[doc = "A proxy was added."]
                ProxyAdded {
                    delegator: ::subxt::utils::AccountId32,
                    delegatee: ::subxt::utils::AccountId32,
                    proxy_type: runtime_types::polkadot_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                },
                #[codec(index = 4)]
                #[doc = "A proxy was removed."]
                ProxyRemoved {
                    delegator: ::subxt::utils::AccountId32,
                    delegatee: ::subxt::utils::AccountId32,
                    proxy_type: runtime_types::polkadot_runtime::ProxyType,
                    delay: ::core::primitive::u32,
                },
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Announcement<_0, _1, _2> {
            pub real: _0,
            pub call_hash: _1,
            pub height: _2,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct ProxyDefinition<_0, _1, _2> {
            pub delegate: _0,
            pub proxy_type: _1,
            pub delay: _2,
        }
    }
    pub mod pallet_referenda {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::submit`]."]
                submit {
                    proposal_origin:
                        ::std::boxed::Box<runtime_types::polkadot_runtime::OriginCaller>,
                    proposal: runtime_types::frame_support::traits::preimages::Bounded<
                        runtime_types::polkadot_runtime::RuntimeCall,
                    >,
                    enactment_moment: runtime_types::frame_support::traits::schedule::DispatchTime<
                        ::core::primitive::u32,
                    >,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::place_decision_deposit`]."]
                place_decision_deposit { index: ::core::primitive::u32 },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::refund_decision_deposit`]."]
                refund_decision_deposit { index: ::core::primitive::u32 },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::cancel`]."]
                cancel { index: ::core::primitive::u32 },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::kill`]."]
                kill { index: ::core::primitive::u32 },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::nudge_referendum`]."]
                nudge_referendum { index: ::core::primitive::u32 },
                #[codec(index = 6)]
                #[doc = "See [`Pallet::one_fewer_deciding`]."]
                one_fewer_deciding { track: ::core::primitive::u16 },
                #[codec(index = 7)]
                #[doc = "See [`Pallet::refund_submission_deposit`]."]
                refund_submission_deposit { index: ::core::primitive::u32 },
                #[codec(index = 8)]
                #[doc = "See [`Pallet::set_metadata`]."]
                set_metadata {
                    index: ::core::primitive::u32,
                    maybe_hash: ::core::option::Option<::subxt::utils::H256>,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Referendum is not ongoing."]
                NotOngoing,
                #[codec(index = 1)]
                #[doc = "Referendum's decision deposit is already paid."]
                HasDeposit,
                #[codec(index = 2)]
                #[doc = "The track identifier given was invalid."]
                BadTrack,
                #[codec(index = 3)]
                #[doc = "There are already a full complement of referenda in progress for this track."]
                Full,
                #[codec(index = 4)]
                #[doc = "The queue of the track is empty."]
                QueueEmpty,
                #[codec(index = 5)]
                #[doc = "The referendum index provided is invalid in this context."]
                BadReferendum,
                #[codec(index = 6)]
                #[doc = "There was nothing to do in the advancement."]
                NothingToDo,
                #[codec(index = 7)]
                #[doc = "No track exists for the proposal origin."]
                NoTrack,
                #[codec(index = 8)]
                #[doc = "Any deposit cannot be refunded until after the decision is over."]
                Unfinished,
                #[codec(index = 9)]
                #[doc = "The deposit refunder is not the depositor."]
                NoPermission,
                #[codec(index = 10)]
                #[doc = "The deposit cannot be refunded since none was made."]
                NoDeposit,
                #[codec(index = 11)]
                #[doc = "The referendum status is invalid for this operation."]
                BadStatus,
                #[codec(index = 12)]
                #[doc = "The preimage does not exist."]
                PreimageNotExist,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A referendum has been submitted."]
                Submitted {
                    index: ::core::primitive::u32,
                    track: ::core::primitive::u16,
                    proposal: runtime_types::frame_support::traits::preimages::Bounded<
                        runtime_types::polkadot_runtime::RuntimeCall,
                    >,
                },
                #[codec(index = 1)]
                #[doc = "The decision deposit has been placed."]
                DecisionDepositPlaced {
                    index: ::core::primitive::u32,
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 2)]
                #[doc = "The decision deposit has been refunded."]
                DecisionDepositRefunded {
                    index: ::core::primitive::u32,
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 3)]
                #[doc = "A deposit has been slashaed."]
                DepositSlashed {
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 4)]
                #[doc = "A referendum has moved into the deciding phase."]
                DecisionStarted {
                    index: ::core::primitive::u32,
                    track: ::core::primitive::u16,
                    proposal: runtime_types::frame_support::traits::preimages::Bounded<
                        runtime_types::polkadot_runtime::RuntimeCall,
                    >,
                    tally: runtime_types::pallet_conviction_voting::types::Tally<
                        ::core::primitive::u128,
                    >,
                },
                #[codec(index = 5)]
                ConfirmStarted { index: ::core::primitive::u32 },
                #[codec(index = 6)]
                ConfirmAborted { index: ::core::primitive::u32 },
                #[codec(index = 7)]
                #[doc = "A referendum has ended its confirmation phase and is ready for approval."]
                Confirmed {
                    index: ::core::primitive::u32,
                    tally: runtime_types::pallet_conviction_voting::types::Tally<
                        ::core::primitive::u128,
                    >,
                },
                #[codec(index = 8)]
                #[doc = "A referendum has been approved and its proposal has been scheduled."]
                Approved { index: ::core::primitive::u32 },
                #[codec(index = 9)]
                #[doc = "A proposal has been rejected by referendum."]
                Rejected {
                    index: ::core::primitive::u32,
                    tally: runtime_types::pallet_conviction_voting::types::Tally<
                        ::core::primitive::u128,
                    >,
                },
                #[codec(index = 10)]
                #[doc = "A referendum has been timed out without being decided."]
                TimedOut {
                    index: ::core::primitive::u32,
                    tally: runtime_types::pallet_conviction_voting::types::Tally<
                        ::core::primitive::u128,
                    >,
                },
                #[codec(index = 11)]
                #[doc = "A referendum has been cancelled."]
                Cancelled {
                    index: ::core::primitive::u32,
                    tally: runtime_types::pallet_conviction_voting::types::Tally<
                        ::core::primitive::u128,
                    >,
                },
                #[codec(index = 12)]
                #[doc = "A referendum has been killed."]
                Killed {
                    index: ::core::primitive::u32,
                    tally: runtime_types::pallet_conviction_voting::types::Tally<
                        ::core::primitive::u128,
                    >,
                },
                #[codec(index = 13)]
                #[doc = "The submission deposit has been refunded."]
                SubmissionDepositRefunded {
                    index: ::core::primitive::u32,
                    who: ::subxt::utils::AccountId32,
                    amount: ::core::primitive::u128,
                },
                #[codec(index = 14)]
                #[doc = "Metadata for a referendum has been set."]
                MetadataSet {
                    index: ::core::primitive::u32,
                    hash: ::subxt::utils::H256,
                },
                #[codec(index = 15)]
                #[doc = "Metadata for a referendum has been cleared."]
                MetadataCleared {
                    index: ::core::primitive::u32,
                    hash: ::subxt::utils::H256,
                },
            }
        }
        pub mod types {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Curve {
                #[codec(index = 0)]
                LinearDecreasing {
                    length: runtime_types::sp_arithmetic::per_things::Perbill,
                    floor: runtime_types::sp_arithmetic::per_things::Perbill,
                    ceil: runtime_types::sp_arithmetic::per_things::Perbill,
                },
                #[codec(index = 1)]
                SteppedDecreasing {
                    begin: runtime_types::sp_arithmetic::per_things::Perbill,
                    end: runtime_types::sp_arithmetic::per_things::Perbill,
                    step: runtime_types::sp_arithmetic::per_things::Perbill,
                    period: runtime_types::sp_arithmetic::per_things::Perbill,
                },
                #[codec(index = 2)]
                Reciprocal {
                    factor: runtime_types::sp_arithmetic::fixed_point::FixedI64,
                    x_offset: runtime_types::sp_arithmetic::fixed_point::FixedI64,
                    y_offset: runtime_types::sp_arithmetic::fixed_point::FixedI64,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct DecidingStatus<_0> {
                pub since: _0,
                pub confirming: ::core::option::Option<_0>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Deposit<_0, _1> {
                pub who: _0,
                pub amount: _1,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum ReferendumInfo<_0, _1, _2, _3, _4, _5, _6, _7> {
                #[codec(index = 0)]
                Ongoing(
                    runtime_types::pallet_referenda::types::ReferendumStatus<
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
                #[codec(index = 1)]
                Approved(
                    _2,
                    ::core::option::Option<runtime_types::pallet_referenda::types::Deposit<_6, _4>>,
                    ::core::option::Option<runtime_types::pallet_referenda::types::Deposit<_6, _4>>,
                ),
                #[codec(index = 2)]
                Rejected(
                    _2,
                    ::core::option::Option<runtime_types::pallet_referenda::types::Deposit<_6, _4>>,
                    ::core::option::Option<runtime_types::pallet_referenda::types::Deposit<_6, _4>>,
                ),
                #[codec(index = 3)]
                Cancelled(
                    _2,
                    ::core::option::Option<runtime_types::pallet_referenda::types::Deposit<_6, _4>>,
                    ::core::option::Option<runtime_types::pallet_referenda::types::Deposit<_6, _4>>,
                ),
                #[codec(index = 4)]
                TimedOut(
                    _2,
                    ::core::option::Option<runtime_types::pallet_referenda::types::Deposit<_6, _4>>,
                    ::core::option::Option<runtime_types::pallet_referenda::types::Deposit<_6, _4>>,
                ),
                #[codec(index = 5)]
                Killed(_2),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ReferendumStatus<_0, _1, _2, _3, _4, _5, _6, _7> {
                pub track: _0,
                pub origin: _1,
                pub proposal: _3,
                pub enactment: runtime_types::frame_support::traits::schedule::DispatchTime<_2>,
                pub submitted: _2,
                pub submission_deposit: runtime_types::pallet_referenda::types::Deposit<_6, _4>,
                pub decision_deposit:
                    ::core::option::Option<runtime_types::pallet_referenda::types::Deposit<_6, _4>>,
                pub deciding: ::core::option::Option<
                    runtime_types::pallet_referenda::types::DecidingStatus<_2>,
                >,
                pub tally: _5,
                pub in_queue: ::core::primitive::bool,
                pub alarm: ::core::option::Option<(_2, _7)>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct TrackInfo<_0, _1> {
                pub name: ::std::string::String,
                pub max_deciding: ::core::primitive::u32,
                pub decision_deposit: _0,
                pub prepare_period: _1,
                pub decision_period: _1,
                pub confirm_period: _1,
                pub min_enactment_period: _1,
                pub min_approval: runtime_types::pallet_referenda::types::Curve,
                pub min_support: runtime_types::pallet_referenda::types::Curve,
            }
        }
    }
    pub mod pallet_scheduler {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::schedule`]."]
                schedule {
                    when: ::core::primitive::u32,
                    maybe_periodic:
                        ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    priority: ::core::primitive::u8,
                    call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::cancel`]."]
                cancel {
                    when: ::core::primitive::u32,
                    index: ::core::primitive::u32,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::schedule_named`]."]
                schedule_named {
                    id: [::core::primitive::u8; 32usize],
                    when: ::core::primitive::u32,
                    maybe_periodic:
                        ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    priority: ::core::primitive::u8,
                    call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::cancel_named`]."]
                cancel_named {
                    id: [::core::primitive::u8; 32usize],
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::schedule_after`]."]
                schedule_after {
                    after: ::core::primitive::u32,
                    maybe_periodic:
                        ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    priority: ::core::primitive::u8,
                    call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::schedule_named_after`]."]
                schedule_named_after {
                    id: [::core::primitive::u8; 32usize],
                    after: ::core::primitive::u32,
                    maybe_periodic:
                        ::core::option::Option<(::core::primitive::u32, ::core::primitive::u32)>,
                    priority: ::core::primitive::u8,
                    call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Failed to schedule a call"]
                FailedToSchedule,
                #[codec(index = 1)]
                #[doc = "Cannot find the scheduled call."]
                NotFound,
                #[codec(index = 2)]
                #[doc = "Given target block number is in the past."]
                TargetBlockNumberInPast,
                #[codec(index = 3)]
                #[doc = "Reschedule failed because it does not change scheduled time."]
                RescheduleNoChange,
                #[codec(index = 4)]
                #[doc = "Attempt to use a non-named function on a named task."]
                Named,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Events type."]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "Scheduled some task."]
                Scheduled {
                    when: ::core::primitive::u32,
                    index: ::core::primitive::u32,
                },
                #[codec(index = 1)]
                #[doc = "Canceled some task."]
                Canceled {
                    when: ::core::primitive::u32,
                    index: ::core::primitive::u32,
                },
                #[codec(index = 2)]
                #[doc = "Dispatched some task."]
                Dispatched {
                    task: (::core::primitive::u32, ::core::primitive::u32),
                    id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                    result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                },
                #[codec(index = 3)]
                #[doc = "The call for the provided hash was not found so the task has been aborted."]
                CallUnavailable {
                    task: (::core::primitive::u32, ::core::primitive::u32),
                    id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                },
                #[codec(index = 4)]
                #[doc = "The given task was unable to be renewed since the agenda is full at that block."]
                PeriodicFailed {
                    task: (::core::primitive::u32, ::core::primitive::u32),
                    id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                },
                #[codec(index = 5)]
                #[doc = "The given task can never be executed since it is overweight."]
                PermanentlyOverweight {
                    task: (::core::primitive::u32, ::core::primitive::u32),
                    id: ::core::option::Option<[::core::primitive::u8; 32usize]>,
                },
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Scheduled<_0, _1, _2, _3, _4> {
            pub maybe_id: ::core::option::Option<_0>,
            pub priority: ::core::primitive::u8,
            pub call: _1,
            pub maybe_periodic: ::core::option::Option<(_2, _2)>,
            pub origin: _3,
            #[codec(skip)]
            pub __subxt_unused_type_params: ::core::marker::PhantomData<_4>,
        }
    }
    pub mod pallet_session {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::set_keys`]."]
                set_keys {
                    keys: runtime_types::polkadot_runtime::SessionKeys,
                    proof: ::std::vec::Vec<::core::primitive::u8>,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::purge_keys`]."]
                purge_keys,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Error for the session pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Invalid ownership proof."]
                InvalidProof,
                #[codec(index = 1)]
                #[doc = "No associated validator ID for account."]
                NoAssociatedValidatorId,
                #[codec(index = 2)]
                #[doc = "Registered duplicate key."]
                DuplicatedKey,
                #[codec(index = 3)]
                #[doc = "No keys are associated with this account."]
                NoKeys,
                #[codec(index = 4)]
                #[doc = "Key setting account is not live, so it's impossible to associate keys."]
                NoAccount,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "New session has happened. Note that the argument is the session index, not the"]
                #[doc = "block number as the type might suggest."]
                NewSession {
                    session_index: ::core::primitive::u32,
                },
            }
        }
    }
    pub mod pallet_staking {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::bond`]."]
                    bond {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        payee: runtime_types::pallet_staking::RewardDestination<
                            ::subxt::utils::AccountId32,
                        >,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::bond_extra`]."]
                    bond_extra {
                        #[codec(compact)]
                        max_additional: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::unbond`]."]
                    unbond {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "See [`Pallet::withdraw_unbonded`]."]
                    withdraw_unbonded {
                        num_slashing_spans: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    #[doc = "See [`Pallet::validate`]."]
                    validate {
                        prefs: runtime_types::pallet_staking::ValidatorPrefs,
                    },
                    #[codec(index = 5)]
                    #[doc = "See [`Pallet::nominate`]."]
                    nominate {
                        targets: ::std::vec::Vec<
                            ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        >,
                    },
                    #[codec(index = 6)]
                    #[doc = "See [`Pallet::chill`]."]
                    chill,
                    #[codec(index = 7)]
                    #[doc = "See [`Pallet::set_payee`]."]
                    set_payee {
                        payee: runtime_types::pallet_staking::RewardDestination<
                            ::subxt::utils::AccountId32,
                        >,
                    },
                    #[codec(index = 8)]
                    #[doc = "See [`Pallet::set_controller`]."]
                    set_controller,
                    #[codec(index = 9)]
                    #[doc = "See [`Pallet::set_validator_count`]."]
                    set_validator_count {
                        #[codec(compact)]
                        new: ::core::primitive::u32,
                    },
                    #[codec(index = 10)]
                    #[doc = "See [`Pallet::increase_validator_count`]."]
                    increase_validator_count {
                        #[codec(compact)]
                        additional: ::core::primitive::u32,
                    },
                    #[codec(index = 11)]
                    #[doc = "See [`Pallet::scale_validator_count`]."]
                    scale_validator_count {
                        factor: runtime_types::sp_arithmetic::per_things::Percent,
                    },
                    #[codec(index = 12)]
                    #[doc = "See [`Pallet::force_no_eras`]."]
                    force_no_eras,
                    #[codec(index = 13)]
                    #[doc = "See [`Pallet::force_new_era`]."]
                    force_new_era,
                    #[codec(index = 14)]
                    #[doc = "See [`Pallet::set_invulnerables`]."]
                    set_invulnerables {
                        invulnerables: ::std::vec::Vec<::subxt::utils::AccountId32>,
                    },
                    #[codec(index = 15)]
                    #[doc = "See [`Pallet::force_unstake`]."]
                    force_unstake {
                        stash: ::subxt::utils::AccountId32,
                        num_slashing_spans: ::core::primitive::u32,
                    },
                    #[codec(index = 16)]
                    #[doc = "See [`Pallet::force_new_era_always`]."]
                    force_new_era_always,
                    #[codec(index = 17)]
                    #[doc = "See [`Pallet::cancel_deferred_slash`]."]
                    cancel_deferred_slash {
                        era: ::core::primitive::u32,
                        slash_indices: ::std::vec::Vec<::core::primitive::u32>,
                    },
                    #[codec(index = 18)]
                    #[doc = "See [`Pallet::payout_stakers`]."]
                    payout_stakers {
                        validator_stash: ::subxt::utils::AccountId32,
                        era: ::core::primitive::u32,
                    },
                    #[codec(index = 19)]
                    #[doc = "See [`Pallet::rebond`]."]
                    rebond {
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                    },
                    #[codec(index = 20)]
                    #[doc = "See [`Pallet::reap_stash`]."]
                    reap_stash {
                        stash: ::subxt::utils::AccountId32,
                        num_slashing_spans: ::core::primitive::u32,
                    },
                    #[codec(index = 21)]
                    #[doc = "See [`Pallet::kick`]."]
                    kick {
                        who: ::std::vec::Vec<
                            ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                        >,
                    },
                    #[codec(index = 22)]
                    #[doc = "See [`Pallet::set_staking_configs`]."]
                    set_staking_configs {
                        min_nominator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                            ::core::primitive::u128,
                        >,
                        min_validator_bond: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                            ::core::primitive::u128,
                        >,
                        max_nominator_count:
                            runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                                ::core::primitive::u32,
                            >,
                        max_validator_count:
                            runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                                ::core::primitive::u32,
                            >,
                        chill_threshold: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                            runtime_types::sp_arithmetic::per_things::Percent,
                        >,
                        min_commission: runtime_types::pallet_staking::pallet::pallet::ConfigOp<
                            runtime_types::sp_arithmetic::per_things::Perbill,
                        >,
                    },
                    #[codec(index = 23)]
                    #[doc = "See [`Pallet::chill_other`]."]
                    chill_other {
                        controller: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 24)]
                    #[doc = "See [`Pallet::force_apply_min_commission`]."]
                    force_apply_min_commission {
                        validator_stash: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 25)]
                    #[doc = "See [`Pallet::set_min_commission`]."]
                    set_min_commission {
                        new: runtime_types::sp_arithmetic::per_things::Perbill,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum ConfigOp<_0> {
                    #[codec(index = 0)]
                    Noop,
                    #[codec(index = 1)]
                    Set(_0),
                    #[codec(index = 2)]
                    Remove,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Not a controller account."]
                    NotController,
                    #[codec(index = 1)]
                    #[doc = "Not a stash account."]
                    NotStash,
                    #[codec(index = 2)]
                    #[doc = "Stash is already bonded."]
                    AlreadyBonded,
                    #[codec(index = 3)]
                    #[doc = "Controller is already paired."]
                    AlreadyPaired,
                    #[codec(index = 4)]
                    #[doc = "Targets cannot be empty."]
                    EmptyTargets,
                    #[codec(index = 5)]
                    #[doc = "Duplicate index."]
                    DuplicateIndex,
                    #[codec(index = 6)]
                    #[doc = "Slash record index out of bounds."]
                    InvalidSlashIndex,
                    #[codec(index = 7)]
                    #[doc = "Cannot have a validator or nominator role, with value less than the minimum defined by"]
                    #[doc = "governance (see `MinValidatorBond` and `MinNominatorBond`). If unbonding is the"]
                    #[doc = "intention, `chill` first to remove one's role as validator/nominator."]
                    InsufficientBond,
                    #[codec(index = 8)]
                    #[doc = "Can not schedule more unlock chunks."]
                    NoMoreChunks,
                    #[codec(index = 9)]
                    #[doc = "Can not rebond without unlocking chunks."]
                    NoUnlockChunk,
                    #[codec(index = 10)]
                    #[doc = "Attempting to target a stash that still has funds."]
                    FundedTarget,
                    #[codec(index = 11)]
                    #[doc = "Invalid era to reward."]
                    InvalidEraToReward,
                    #[codec(index = 12)]
                    #[doc = "Invalid number of nominations."]
                    InvalidNumberOfNominations,
                    #[codec(index = 13)]
                    #[doc = "Items are not sorted and unique."]
                    NotSortedAndUnique,
                    #[codec(index = 14)]
                    #[doc = "Rewards for this era have already been claimed for this validator."]
                    AlreadyClaimed,
                    #[codec(index = 15)]
                    #[doc = "Incorrect previous history depth input provided."]
                    IncorrectHistoryDepth,
                    #[codec(index = 16)]
                    #[doc = "Incorrect number of slashing spans provided."]
                    IncorrectSlashingSpans,
                    #[codec(index = 17)]
                    #[doc = "Internal state has become somehow corrupted and the operation cannot continue."]
                    BadState,
                    #[codec(index = 18)]
                    #[doc = "Too many nomination targets supplied."]
                    TooManyTargets,
                    #[codec(index = 19)]
                    #[doc = "A nomination target was supplied that was blocked or otherwise not a validator."]
                    BadTarget,
                    #[codec(index = 20)]
                    #[doc = "The user has enough bond and thus cannot be chilled forcefully by an external person."]
                    CannotChillOther,
                    #[codec(index = 21)]
                    #[doc = "There are too many nominators in the system. Governance needs to adjust the staking"]
                    #[doc = "settings to keep things safe for the runtime."]
                    TooManyNominators,
                    #[codec(index = 22)]
                    #[doc = "There are too many validator candidates in the system. Governance needs to adjust the"]
                    #[doc = "staking settings to keep things safe for the runtime."]
                    TooManyValidators,
                    #[codec(index = 23)]
                    #[doc = "Commission is too low. Must be at least `MinCommission`."]
                    CommissionTooLow,
                    #[codec(index = 24)]
                    #[doc = "Some bound is not met."]
                    BoundNotMet,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "The era payout has been set; the first balance is the validator-payout; the second is"]
                    #[doc = "the remainder from the maximum amount of reward."]
                    EraPaid {
                        era_index: ::core::primitive::u32,
                        validator_payout: ::core::primitive::u128,
                        remainder: ::core::primitive::u128,
                    },
                    #[codec(index = 1)]
                    #[doc = "The nominator has been rewarded by this amount."]
                    Rewarded {
                        stash: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "A staker (validator or nominator) has been slashed by the given amount."]
                    Slashed {
                        staker: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "A slash for the given validator, for the given percentage of their stake, at the given"]
                    #[doc = "era as been reported."]
                    SlashReported {
                        validator: ::subxt::utils::AccountId32,
                        fraction: runtime_types::sp_arithmetic::per_things::Perbill,
                        slash_era: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    #[doc = "An old slashing report from a prior era was discarded because it could"]
                    #[doc = "not be processed."]
                    OldSlashingReportDiscarded {
                        session_index: ::core::primitive::u32,
                    },
                    #[codec(index = 5)]
                    #[doc = "A new set of stakers was elected."]
                    StakersElected,
                    #[codec(index = 6)]
                    #[doc = "An account has bonded this amount. \\[stash, amount\\]"]
                    #[doc = ""]
                    #[doc = "NOTE: This event is only emitted when funds are bonded via a dispatchable. Notably,"]
                    #[doc = "it will not be emitted for staking rewards when they are added to stake."]
                    Bonded {
                        stash: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 7)]
                    #[doc = "An account has unbonded this amount."]
                    Unbonded {
                        stash: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 8)]
                    #[doc = "An account has called `withdraw_unbonded` and removed unbonding chunks worth `Balance`"]
                    #[doc = "from the unlocking queue."]
                    Withdrawn {
                        stash: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 9)]
                    #[doc = "A nominator has been kicked from a validator."]
                    Kicked {
                        nominator: ::subxt::utils::AccountId32,
                        stash: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 10)]
                    #[doc = "The election failed. No new era is planned."]
                    StakingElectionFailed,
                    #[codec(index = 11)]
                    #[doc = "An account has stopped participating as either a validator or nominator."]
                    Chilled { stash: ::subxt::utils::AccountId32 },
                    #[codec(index = 12)]
                    #[doc = "The stakers' rewards are getting paid."]
                    PayoutStarted {
                        era_index: ::core::primitive::u32,
                        validator_stash: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 13)]
                    #[doc = "A validator has set their preferences."]
                    ValidatorPrefsSet {
                        stash: ::subxt::utils::AccountId32,
                        prefs: runtime_types::pallet_staking::ValidatorPrefs,
                    },
                    #[codec(index = 14)]
                    #[doc = "A new force era mode was set."]
                    ForceEra {
                        mode: runtime_types::pallet_staking::Forcing,
                    },
                }
            }
        }
        pub mod slashing {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct SlashingSpans {
                pub span_index: ::core::primitive::u32,
                pub last_start: ::core::primitive::u32,
                pub last_nonzero_slash: ::core::primitive::u32,
                pub prior: ::std::vec::Vec<::core::primitive::u32>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct SpanRecord<_0> {
                pub slashed: _0,
                pub paid_out: _0,
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct ActiveEraInfo {
            pub index: ::core::primitive::u32,
            pub start: ::core::option::Option<::core::primitive::u64>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct EraRewardPoints<_0> {
            pub total: ::core::primitive::u32,
            pub individual: ::subxt::utils::KeyedVec<_0, ::core::primitive::u32>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Exposure<_0, _1> {
            #[codec(compact)]
            pub total: _1,
            #[codec(compact)]
            pub own: _1,
            pub others: ::std::vec::Vec<runtime_types::pallet_staking::IndividualExposure<_0, _1>>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum Forcing {
            #[codec(index = 0)]
            NotForcing,
            #[codec(index = 1)]
            ForceNew,
            #[codec(index = 2)]
            ForceNone,
            #[codec(index = 3)]
            ForceAlways,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct IndividualExposure<_0, _1> {
            pub who: _0,
            #[codec(compact)]
            pub value: _1,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Nominations {
            pub targets: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                ::subxt::utils::AccountId32,
            >,
            pub submitted_in: ::core::primitive::u32,
            pub suppressed: ::core::primitive::bool,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum RewardDestination<_0> {
            #[codec(index = 0)]
            Staked,
            #[codec(index = 1)]
            Stash,
            #[codec(index = 2)]
            Controller,
            #[codec(index = 3)]
            Account(_0),
            #[codec(index = 4)]
            None,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct StakingLedger {
            pub stash: ::subxt::utils::AccountId32,
            #[codec(compact)]
            pub total: ::core::primitive::u128,
            #[codec(compact)]
            pub active: ::core::primitive::u128,
            pub unlocking: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                runtime_types::pallet_staking::UnlockChunk<::core::primitive::u128>,
            >,
            pub claimed_rewards:
                runtime_types::bounded_collections::bounded_vec::BoundedVec<::core::primitive::u32>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct UnappliedSlash<_0, _1> {
            pub validator: _0,
            pub own: _1,
            pub others: ::std::vec::Vec<(_0, _1)>,
            pub reporters: ::std::vec::Vec<_0>,
            pub payout: _1,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct UnlockChunk<_0> {
            #[codec(compact)]
            pub value: _0,
            #[codec(compact)]
            pub era: ::core::primitive::u32,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct ValidatorPrefs {
            #[codec(compact)]
            pub commission: runtime_types::sp_arithmetic::per_things::Perbill,
            pub blocked: ::core::primitive::bool,
        }
    }
    pub mod pallet_timestamp {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::set`]."]
                set {
                    #[codec(compact)]
                    now: ::core::primitive::u64,
                },
            }
        }
    }
    pub mod pallet_tips {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::report_awesome`]."]
                report_awesome {
                    reason: ::std::vec::Vec<::core::primitive::u8>,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::retract_tip`]."]
                retract_tip { hash: ::subxt::utils::H256 },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::tip_new`]."]
                tip_new {
                    reason: ::std::vec::Vec<::core::primitive::u8>,
                    who: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    #[codec(compact)]
                    tip_value: ::core::primitive::u128,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::tip`]."]
                tip {
                    hash: ::subxt::utils::H256,
                    #[codec(compact)]
                    tip_value: ::core::primitive::u128,
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::close_tip`]."]
                close_tip { hash: ::subxt::utils::H256 },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::slash_tip`]."]
                slash_tip { hash: ::subxt::utils::H256 },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "The reason given is just too big."]
                ReasonTooBig,
                #[codec(index = 1)]
                #[doc = "The tip was already found/started."]
                AlreadyKnown,
                #[codec(index = 2)]
                #[doc = "The tip hash is unknown."]
                UnknownTip,
                #[codec(index = 3)]
                #[doc = "The account attempting to retract the tip is not the finder of the tip."]
                NotFinder,
                #[codec(index = 4)]
                #[doc = "The tip cannot be claimed/closed because there are not enough tippers yet."]
                StillOpen,
                #[codec(index = 5)]
                #[doc = "The tip cannot be claimed/closed because it's still in the countdown period."]
                Premature,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A new tip suggestion has been opened."]
                NewTip { tip_hash: ::subxt::utils::H256 },
                #[codec(index = 1)]
                #[doc = "A tip suggestion has reached threshold and is closing."]
                TipClosing { tip_hash: ::subxt::utils::H256 },
                #[codec(index = 2)]
                #[doc = "A tip suggestion has been closed."]
                TipClosed {
                    tip_hash: ::subxt::utils::H256,
                    who: ::subxt::utils::AccountId32,
                    payout: ::core::primitive::u128,
                },
                #[codec(index = 3)]
                #[doc = "A tip suggestion has been retracted."]
                TipRetracted { tip_hash: ::subxt::utils::H256 },
                #[codec(index = 4)]
                #[doc = "A tip suggestion has been slashed."]
                TipSlashed {
                    tip_hash: ::subxt::utils::H256,
                    finder: ::subxt::utils::AccountId32,
                    deposit: ::core::primitive::u128,
                },
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "A transaction fee `actual_fee`, of which `tip` was added to the minimum inclusion fee,"]
                #[doc = "has been paid by `who`."]
                TransactionFeePaid {
                    who: ::subxt::utils::AccountId32,
                    actual_fee: ::core::primitive::u128,
                    tip: ::core::primitive::u128,
                },
            }
        }
        pub mod types {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct FeeDetails<_0> {
                pub inclusion_fee: ::core::option::Option<
                    runtime_types::pallet_transaction_payment::types::InclusionFee<_0>,
                >,
                pub tip: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct InclusionFee<_0> {
                pub base_fee: _0,
                pub len_fee: _0,
                pub adjusted_weight_fee: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RuntimeDispatchInfo<_0, _1> {
                pub weight: _1,
                pub class: runtime_types::frame_support::dispatch::DispatchClass,
                pub partial_fee: _0,
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct ChargeTransactionPayment(#[codec(compact)] pub ::core::primitive::u128);
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum Releases {
            #[codec(index = 0)]
            V1Ancient,
            #[codec(index = 1)]
            V2,
        }
    }
    pub mod pallet_treasury {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::propose_spend`]."]
                propose_spend {
                    #[codec(compact)]
                    value: ::core::primitive::u128,
                    beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::reject_proposal`]."]
                reject_proposal {
                    #[codec(compact)]
                    proposal_id: ::core::primitive::u32,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::approve_proposal`]."]
                approve_proposal {
                    #[codec(compact)]
                    proposal_id: ::core::primitive::u32,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::spend`]."]
                spend {
                    #[codec(compact)]
                    amount: ::core::primitive::u128,
                    beneficiary: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::remove_approval`]."]
                remove_approval {
                    #[codec(compact)]
                    proposal_id: ::core::primitive::u32,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Error for the treasury pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Proposer's balance is too low."]
                InsufficientProposersBalance,
                #[codec(index = 1)]
                #[doc = "No proposal or bounty at that index."]
                InvalidIndex,
                #[codec(index = 2)]
                #[doc = "Too many approvals in the queue."]
                TooManyApprovals,
                #[codec(index = 3)]
                #[doc = "The spend origin is valid but the amount it is allowed to spend is lower than the"]
                #[doc = "amount to be spent."]
                InsufficientPermission,
                #[codec(index = 4)]
                #[doc = "Proposal has not been approved."]
                ProposalNotApproved,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "New proposal."]
                Proposed {
                    proposal_index: ::core::primitive::u32,
                },
                #[codec(index = 1)]
                #[doc = "We have ended a spend period and will now allocate funds."]
                Spending {
                    budget_remaining: ::core::primitive::u128,
                },
                #[codec(index = 2)]
                #[doc = "Some funds have been allocated."]
                Awarded {
                    proposal_index: ::core::primitive::u32,
                    award: ::core::primitive::u128,
                    account: ::subxt::utils::AccountId32,
                },
                #[codec(index = 3)]
                #[doc = "A proposal was rejected; funds were slashed."]
                Rejected {
                    proposal_index: ::core::primitive::u32,
                    slashed: ::core::primitive::u128,
                },
                #[codec(index = 4)]
                #[doc = "Some of our funds have been burnt."]
                Burnt {
                    burnt_funds: ::core::primitive::u128,
                },
                #[codec(index = 5)]
                #[doc = "Spending has finished; this is the amount that rolls over until next spend."]
                Rollover {
                    rollover_balance: ::core::primitive::u128,
                },
                #[codec(index = 6)]
                #[doc = "Some funds have been deposited."]
                Deposit { value: ::core::primitive::u128 },
                #[codec(index = 7)]
                #[doc = "A new spend proposal has been approved."]
                SpendApproved {
                    proposal_index: ::core::primitive::u32,
                    amount: ::core::primitive::u128,
                    beneficiary: ::subxt::utils::AccountId32,
                },
                #[codec(index = 8)]
                #[doc = "The inactive funds of the pallet have been updated."]
                UpdatedInactive {
                    reactivated: ::core::primitive::u128,
                    deactivated: ::core::primitive::u128,
                },
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Proposal<_0, _1> {
            pub proposer: _0,
            pub value: _1,
            pub beneficiary: _0,
            pub bond: _1,
        }
    }
    pub mod pallet_utility {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::batch`]."]
                batch {
                    calls: ::std::vec::Vec<runtime_types::polkadot_runtime::RuntimeCall>,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::as_derivative`]."]
                as_derivative {
                    index: ::core::primitive::u16,
                    call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::batch_all`]."]
                batch_all {
                    calls: ::std::vec::Vec<runtime_types::polkadot_runtime::RuntimeCall>,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::dispatch_as`]."]
                dispatch_as {
                    as_origin: ::std::boxed::Box<runtime_types::polkadot_runtime::OriginCaller>,
                    call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::force_batch`]."]
                force_batch {
                    calls: ::std::vec::Vec<runtime_types::polkadot_runtime::RuntimeCall>,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::with_weight`]."]
                with_weight {
                    call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                    weight: runtime_types::sp_weights::weight_v2::Weight,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "Too many calls batched."]
                TooManyCalls,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "Batch of dispatches did not complete fully. Index of first failing dispatch given, as"]
                #[doc = "well as the error."]
                BatchInterrupted {
                    index: ::core::primitive::u32,
                    error: runtime_types::sp_runtime::DispatchError,
                },
                #[codec(index = 1)]
                #[doc = "Batch of dispatches completed fully with no error."]
                BatchCompleted,
                #[codec(index = 2)]
                #[doc = "Batch of dispatches completed but has errors."]
                BatchCompletedWithErrors,
                #[codec(index = 3)]
                #[doc = "A single item within a Batch of dispatches has completed with no error."]
                ItemCompleted,
                #[codec(index = 4)]
                #[doc = "A single item within a Batch of dispatches has completed with error."]
                ItemFailed {
                    error: runtime_types::sp_runtime::DispatchError,
                },
                #[codec(index = 5)]
                #[doc = "A call was dispatched."]
                DispatchedAs {
                    result: ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                },
            }
        }
    }
    pub mod pallet_vesting {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::vest`]."]
                vest,
                #[codec(index = 1)]
                #[doc = "See [`Pallet::vest_other`]."]
                vest_other {
                    target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::vested_transfer`]."]
                vested_transfer {
                    target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::force_vested_transfer`]."]
                force_vested_transfer {
                    source: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    target: ::subxt::utils::MultiAddress<::subxt::utils::AccountId32, ()>,
                    schedule: runtime_types::pallet_vesting::vesting_info::VestingInfo<
                        ::core::primitive::u128,
                        ::core::primitive::u32,
                    >,
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::merge_schedules`]."]
                merge_schedules {
                    schedule1_index: ::core::primitive::u32,
                    schedule2_index: ::core::primitive::u32,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Error for the vesting pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "The account given is not vesting."]
                NotVesting,
                #[codec(index = 1)]
                #[doc = "The account already has `MaxVestingSchedules` count of schedules and thus"]
                #[doc = "cannot add another one. Consider merging existing schedules in order to add another."]
                AtMaxVestingSchedules,
                #[codec(index = 2)]
                #[doc = "Amount being transferred is too low to create a vesting schedule."]
                AmountLow,
                #[codec(index = 3)]
                #[doc = "An index was out of bounds of the vesting schedules."]
                ScheduleIndexOutOfBounds,
                #[codec(index = 4)]
                #[doc = "Failed to create a new schedule because some parameter was invalid."]
                InvalidScheduleParams,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "The amount vested has been updated. This could indicate a change in funds available."]
                #[doc = "The balance given is the amount which is left unvested (and thus locked)."]
                VestingUpdated {
                    account: ::subxt::utils::AccountId32,
                    unvested: ::core::primitive::u128,
                },
                #[codec(index = 1)]
                #[doc = "An \\[account\\] has become fully vested."]
                VestingCompleted {
                    account: ::subxt::utils::AccountId32,
                },
            }
        }
        pub mod vesting_info {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct VestingInfo<_0, _1> {
                pub locked: _0,
                pub per_block: _0,
                pub starting_block: _1,
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum Releases {
            #[codec(index = 0)]
            V0,
            #[codec(index = 1)]
            V1,
        }
    }
    pub mod pallet_whitelist {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::whitelist_call`]."]
                whitelist_call { call_hash: ::subxt::utils::H256 },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::remove_whitelisted_call`]."]
                remove_whitelisted_call { call_hash: ::subxt::utils::H256 },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::dispatch_whitelisted_call`]."]
                dispatch_whitelisted_call {
                    call_hash: ::subxt::utils::H256,
                    call_encoded_len: ::core::primitive::u32,
                    call_weight_witness: runtime_types::sp_weights::weight_v2::Weight,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::dispatch_whitelisted_call_with_preimage`]."]
                dispatch_whitelisted_call_with_preimage {
                    call: ::std::boxed::Box<runtime_types::polkadot_runtime::RuntimeCall>,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "The preimage of the call hash could not be loaded."]
                UnavailablePreImage,
                #[codec(index = 1)]
                #[doc = "The call could not be decoded."]
                UndecodableCall,
                #[codec(index = 2)]
                #[doc = "The weight of the decoded call was higher than the witness."]
                InvalidCallWeightWitness,
                #[codec(index = 3)]
                #[doc = "The call was not whitelisted."]
                CallIsNotWhitelisted,
                #[codec(index = 4)]
                #[doc = "The call was already whitelisted; No-Op."]
                CallAlreadyWhitelisted,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                CallWhitelisted { call_hash: ::subxt::utils::H256 },
                #[codec(index = 1)]
                WhitelistedCallRemoved { call_hash: ::subxt::utils::H256 },
                #[codec(index = 2)]
                WhitelistedCallDispatched {
                    call_hash: ::subxt::utils::H256,
                    result: ::core::result::Result<
                        runtime_types::frame_support::dispatch::PostDispatchInfo,
                        runtime_types::sp_runtime::DispatchErrorWithPostInfo<
                            runtime_types::frame_support::dispatch::PostDispatchInfo,
                        >,
                    >,
                },
            }
        }
    }
    pub mod pallet_xcm {
        use super::runtime_types;
        pub mod pallet {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
            pub enum Call {
                #[codec(index = 0)]
                #[doc = "See [`Pallet::send`]."]
                send {
                    dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm>,
                },
                #[codec(index = 1)]
                #[doc = "See [`Pallet::teleport_assets`]."]
                teleport_assets {
                    dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                    fee_asset_item: ::core::primitive::u32,
                },
                #[codec(index = 2)]
                #[doc = "See [`Pallet::reserve_transfer_assets`]."]
                reserve_transfer_assets {
                    dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                    fee_asset_item: ::core::primitive::u32,
                },
                #[codec(index = 3)]
                #[doc = "See [`Pallet::execute`]."]
                execute {
                    message: ::std::boxed::Box<runtime_types::xcm::VersionedXcm2>,
                    max_weight: runtime_types::sp_weights::weight_v2::Weight,
                },
                #[codec(index = 4)]
                #[doc = "See [`Pallet::force_xcm_version`]."]
                force_xcm_version {
                    location:
                        ::std::boxed::Box<runtime_types::xcm::v3::multilocation::MultiLocation>,
                    version: ::core::primitive::u32,
                },
                #[codec(index = 5)]
                #[doc = "See [`Pallet::force_default_xcm_version`]."]
                force_default_xcm_version {
                    maybe_xcm_version: ::core::option::Option<::core::primitive::u32>,
                },
                #[codec(index = 6)]
                #[doc = "See [`Pallet::force_subscribe_version_notify`]."]
                force_subscribe_version_notify {
                    location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                },
                #[codec(index = 7)]
                #[doc = "See [`Pallet::force_unsubscribe_version_notify`]."]
                force_unsubscribe_version_notify {
                    location: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                },
                #[codec(index = 8)]
                #[doc = "See [`Pallet::limited_reserve_transfer_assets`]."]
                limited_reserve_transfer_assets {
                    dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                    fee_asset_item: ::core::primitive::u32,
                    weight_limit: runtime_types::xcm::v3::WeightLimit,
                },
                #[codec(index = 9)]
                #[doc = "See [`Pallet::limited_teleport_assets`]."]
                limited_teleport_assets {
                    dest: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    beneficiary: ::std::boxed::Box<runtime_types::xcm::VersionedMultiLocation>,
                    assets: ::std::boxed::Box<runtime_types::xcm::VersionedMultiAssets>,
                    fee_asset_item: ::core::primitive::u32,
                    weight_limit: runtime_types::xcm::v3::WeightLimit,
                },
                #[codec(index = 10)]
                #[doc = "See [`Pallet::force_suspension`]."]
                force_suspension { suspended: ::core::primitive::bool },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Error` enum of this pallet."]
            pub enum Error {
                #[codec(index = 0)]
                #[doc = "The desired destination was unreachable, generally because there is a no way of routing"]
                #[doc = "to it."]
                Unreachable,
                #[codec(index = 1)]
                #[doc = "There was some other issue (i.e. not to do with routing) in sending the message. Perhaps"]
                #[doc = "a lack of space for buffering the message."]
                SendFailure,
                #[codec(index = 2)]
                #[doc = "The message execution fails the filter."]
                Filtered,
                #[codec(index = 3)]
                #[doc = "The message's weight could not be determined."]
                UnweighableMessage,
                #[codec(index = 4)]
                #[doc = "The destination `MultiLocation` provided cannot be inverted."]
                DestinationNotInvertible,
                #[codec(index = 5)]
                #[doc = "The assets to be sent are empty."]
                Empty,
                #[codec(index = 6)]
                #[doc = "Could not re-anchor the assets to declare the fees for the destination chain."]
                CannotReanchor,
                #[codec(index = 7)]
                #[doc = "Too many assets have been attempted for transfer."]
                TooManyAssets,
                #[codec(index = 8)]
                #[doc = "Origin is invalid for sending."]
                InvalidOrigin,
                #[codec(index = 9)]
                #[doc = "The version of the `Versioned` value used is not able to be interpreted."]
                BadVersion,
                #[codec(index = 10)]
                #[doc = "The given location could not be used (e.g. because it cannot be expressed in the"]
                #[doc = "desired version of XCM)."]
                BadLocation,
                #[codec(index = 11)]
                #[doc = "The referenced subscription could not be found."]
                NoSubscription,
                #[codec(index = 12)]
                #[doc = "The location is invalid since it already has a subscription from us."]
                AlreadySubscribed,
                #[codec(index = 13)]
                #[doc = "Invalid asset for the operation."]
                InvalidAsset,
                #[codec(index = 14)]
                #[doc = "The owner does not own (all) of the asset that they wish to do the operation on."]
                LowBalance,
                #[codec(index = 15)]
                #[doc = "The asset owner has too many locks on the asset."]
                TooManyLocks,
                #[codec(index = 16)]
                #[doc = "The given account is not an identifiable sovereign account for any location."]
                AccountNotSovereign,
                #[codec(index = 17)]
                #[doc = "The operation required fees to be paid which the initiator could not meet."]
                FeesNotMet,
                #[codec(index = 18)]
                #[doc = "A remote lock with the corresponding data could not be found."]
                LockNotFound,
                #[codec(index = 19)]
                #[doc = "The unlock operation cannot succeed because there are still consumers of the lock."]
                InUse,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            #[doc = "The `Event` enum of this pallet"]
            pub enum Event {
                #[codec(index = 0)]
                #[doc = "Execution of an XCM message was attempted."]
                Attempted {
                    outcome: runtime_types::xcm::v3::traits::Outcome,
                },
                #[codec(index = 1)]
                #[doc = "A XCM message was sent."]
                Sent {
                    origin: runtime_types::xcm::v3::multilocation::MultiLocation,
                    destination: runtime_types::xcm::v3::multilocation::MultiLocation,
                    message: runtime_types::xcm::v3::Xcm,
                    message_id: [::core::primitive::u8; 32usize],
                },
                #[codec(index = 2)]
                #[doc = "Query response received which does not match a registered query. This may be because a"]
                #[doc = "matching query was never registered, it may be because it is a duplicate response, or"]
                #[doc = "because the query timed out."]
                UnexpectedResponse {
                    origin: runtime_types::xcm::v3::multilocation::MultiLocation,
                    query_id: ::core::primitive::u64,
                },
                #[codec(index = 3)]
                #[doc = "Query response has been received and is ready for taking with `take_response`. There is"]
                #[doc = "no registered notification call."]
                ResponseReady {
                    query_id: ::core::primitive::u64,
                    response: runtime_types::xcm::v3::Response,
                },
                #[codec(index = 4)]
                #[doc = "Query response has been received and query is removed. The registered notification has"]
                #[doc = "been dispatched and executed successfully."]
                Notified {
                    query_id: ::core::primitive::u64,
                    pallet_index: ::core::primitive::u8,
                    call_index: ::core::primitive::u8,
                },
                #[codec(index = 5)]
                #[doc = "Query response has been received and query is removed. The registered notification could"]
                #[doc = "not be dispatched because the dispatch weight is greater than the maximum weight"]
                #[doc = "originally budgeted by this runtime for the query result."]
                NotifyOverweight {
                    query_id: ::core::primitive::u64,
                    pallet_index: ::core::primitive::u8,
                    call_index: ::core::primitive::u8,
                    actual_weight: runtime_types::sp_weights::weight_v2::Weight,
                    max_budgeted_weight: runtime_types::sp_weights::weight_v2::Weight,
                },
                #[codec(index = 6)]
                #[doc = "Query response has been received and query is removed. There was a general error with"]
                #[doc = "dispatching the notification call."]
                NotifyDispatchError {
                    query_id: ::core::primitive::u64,
                    pallet_index: ::core::primitive::u8,
                    call_index: ::core::primitive::u8,
                },
                #[codec(index = 7)]
                #[doc = "Query response has been received and query is removed. The dispatch was unable to be"]
                #[doc = "decoded into a `Call`; this might be due to dispatch function having a signature which"]
                #[doc = "is not `(origin, QueryId, Response)`."]
                NotifyDecodeFailed {
                    query_id: ::core::primitive::u64,
                    pallet_index: ::core::primitive::u8,
                    call_index: ::core::primitive::u8,
                },
                #[codec(index = 8)]
                #[doc = "Expected query response has been received but the origin location of the response does"]
                #[doc = "not match that expected. The query remains registered for a later, valid, response to"]
                #[doc = "be received and acted upon."]
                InvalidResponder {
                    origin: runtime_types::xcm::v3::multilocation::MultiLocation,
                    query_id: ::core::primitive::u64,
                    expected_location: ::core::option::Option<
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                    >,
                },
                #[codec(index = 9)]
                #[doc = "Expected query response has been received but the expected origin location placed in"]
                #[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
                #[doc = ""]
                #[doc = "This is unexpected (since a location placed in storage in a previously executing"]
                #[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
                #[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
                #[doc = "needed."]
                InvalidResponderVersion {
                    origin: runtime_types::xcm::v3::multilocation::MultiLocation,
                    query_id: ::core::primitive::u64,
                },
                #[codec(index = 10)]
                #[doc = "Received query response has been read and removed."]
                ResponseTaken { query_id: ::core::primitive::u64 },
                #[codec(index = 11)]
                #[doc = "Some assets have been placed in an asset trap."]
                AssetsTrapped {
                    hash: ::subxt::utils::H256,
                    origin: runtime_types::xcm::v3::multilocation::MultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                },
                #[codec(index = 12)]
                #[doc = "An XCM version change notification message has been attempted to be sent."]
                #[doc = ""]
                #[doc = "The cost of sending it (borne by the chain) is included."]
                VersionChangeNotified {
                    destination: runtime_types::xcm::v3::multilocation::MultiLocation,
                    result: ::core::primitive::u32,
                    cost: runtime_types::xcm::v3::multiasset::MultiAssets,
                    message_id: [::core::primitive::u8; 32usize],
                },
                #[codec(index = 13)]
                #[doc = "The supported version of a location has been changed. This might be through an"]
                #[doc = "automatic notification or a manual intervention."]
                SupportedVersionChanged {
                    location: runtime_types::xcm::v3::multilocation::MultiLocation,
                    version: ::core::primitive::u32,
                },
                #[codec(index = 14)]
                #[doc = "A given location which had a version change subscription was dropped owing to an error"]
                #[doc = "sending the notification to it."]
                NotifyTargetSendFail {
                    location: runtime_types::xcm::v3::multilocation::MultiLocation,
                    query_id: ::core::primitive::u64,
                    error: runtime_types::xcm::v3::traits::Error,
                },
                #[codec(index = 15)]
                #[doc = "A given location which had a version change subscription was dropped owing to an error"]
                #[doc = "migrating the location to our new XCM format."]
                NotifyTargetMigrationFail {
                    location: runtime_types::xcm::VersionedMultiLocation,
                    query_id: ::core::primitive::u64,
                },
                #[codec(index = 16)]
                #[doc = "Expected query response has been received but the expected querier location placed in"]
                #[doc = "storage by this runtime previously cannot be decoded. The query remains registered."]
                #[doc = ""]
                #[doc = "This is unexpected (since a location placed in storage in a previously executing"]
                #[doc = "runtime should be readable prior to query timeout) and dangerous since the possibly"]
                #[doc = "valid response will be dropped. Manual governance intervention is probably going to be"]
                #[doc = "needed."]
                InvalidQuerierVersion {
                    origin: runtime_types::xcm::v3::multilocation::MultiLocation,
                    query_id: ::core::primitive::u64,
                },
                #[codec(index = 17)]
                #[doc = "Expected query response has been received but the querier location of the response does"]
                #[doc = "not match the expected. The query remains registered for a later, valid, response to"]
                #[doc = "be received and acted upon."]
                InvalidQuerier {
                    origin: runtime_types::xcm::v3::multilocation::MultiLocation,
                    query_id: ::core::primitive::u64,
                    expected_querier: runtime_types::xcm::v3::multilocation::MultiLocation,
                    maybe_actual_querier: ::core::option::Option<
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                    >,
                },
                #[codec(index = 18)]
                #[doc = "A remote has requested XCM version change notification from us and we have honored it."]
                #[doc = "A version information message is sent to them and its cost is included."]
                VersionNotifyStarted {
                    destination: runtime_types::xcm::v3::multilocation::MultiLocation,
                    cost: runtime_types::xcm::v3::multiasset::MultiAssets,
                    message_id: [::core::primitive::u8; 32usize],
                },
                #[codec(index = 19)]
                #[doc = "We have requested that a remote chain send us XCM version change notifications."]
                VersionNotifyRequested {
                    destination: runtime_types::xcm::v3::multilocation::MultiLocation,
                    cost: runtime_types::xcm::v3::multiasset::MultiAssets,
                    message_id: [::core::primitive::u8; 32usize],
                },
                #[codec(index = 20)]
                #[doc = "We have requested that a remote chain stops sending us XCM version change notifications."]
                VersionNotifyUnrequested {
                    destination: runtime_types::xcm::v3::multilocation::MultiLocation,
                    cost: runtime_types::xcm::v3::multiasset::MultiAssets,
                    message_id: [::core::primitive::u8; 32usize],
                },
                #[codec(index = 21)]
                #[doc = "Fees were paid from a location for an operation (often for using `SendXcm`)."]
                FeesPaid {
                    paying: runtime_types::xcm::v3::multilocation::MultiLocation,
                    fees: runtime_types::xcm::v3::multiasset::MultiAssets,
                },
                #[codec(index = 22)]
                #[doc = "Some assets have been claimed from an asset trap"]
                AssetsClaimed {
                    hash: ::subxt::utils::H256,
                    origin: runtime_types::xcm::v3::multilocation::MultiLocation,
                    assets: runtime_types::xcm::VersionedMultiAssets,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Origin {
                #[codec(index = 0)]
                Xcm(runtime_types::xcm::v3::multilocation::MultiLocation),
                #[codec(index = 1)]
                Response(runtime_types::xcm::v3::multilocation::MultiLocation),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum QueryStatus<_0> {
                #[codec(index = 0)]
                Pending {
                    responder: runtime_types::xcm::VersionedMultiLocation,
                    maybe_match_querier:
                        ::core::option::Option<runtime_types::xcm::VersionedMultiLocation>,
                    maybe_notify:
                        ::core::option::Option<(::core::primitive::u8, ::core::primitive::u8)>,
                    timeout: _0,
                },
                #[codec(index = 1)]
                VersionNotifier {
                    origin: runtime_types::xcm::VersionedMultiLocation,
                    is_active: ::core::primitive::bool,
                },
                #[codec(index = 2)]
                Ready {
                    response: runtime_types::xcm::VersionedResponse,
                    at: _0,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct RemoteLockedFungibleRecord<_0> {
                pub amount: ::core::primitive::u128,
                pub owner: runtime_types::xcm::VersionedMultiLocation,
                pub locker: runtime_types::xcm::VersionedMultiLocation,
                pub consumers: runtime_types::bounded_collections::bounded_vec::BoundedVec<(
                    _0,
                    ::core::primitive::u128,
                )>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum VersionMigrationStage {
                #[codec(index = 0)]
                MigrateSupportedVersion,
                #[codec(index = 1)]
                MigrateVersionNotifiers,
                #[codec(index = 2)]
                NotifyCurrentTargets(
                    ::core::option::Option<::std::vec::Vec<::core::primitive::u8>>,
                ),
                #[codec(index = 3)]
                MigrateAndNotifyOldTargets,
            }
        }
    }
    pub mod polkadot_core_primitives {
        use super::runtime_types;
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct CandidateHash(pub ::subxt::utils::H256);
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct InboundDownwardMessage<_0> {
            pub sent_at: _0,
            pub msg: ::std::vec::Vec<::core::primitive::u8>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct InboundHrmpMessage<_0> {
            pub sent_at: _0,
            pub data: ::std::vec::Vec<::core::primitive::u8>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct OutboundHrmpMessage<_0> {
            pub recipient: _0,
            pub data: ::std::vec::Vec<::core::primitive::u8>,
        }
    }
    pub mod polkadot_parachain {
        use super::runtime_types;
        pub mod primitives {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct HeadData(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct HrmpChannelId {
                pub sender: runtime_types::polkadot_parachain::primitives::Id,
                pub recipient: runtime_types::polkadot_parachain::primitives::Id,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Id(pub ::core::primitive::u32);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ValidationCode(pub ::std::vec::Vec<::core::primitive::u8>);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ValidationCodeHash(pub ::subxt::utils::H256);
        }
    }
    pub mod polkadot_primitives {
        use super::runtime_types;
        pub mod v5 {
            use super::runtime_types;
            pub mod assignment_app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
            }
            pub mod collator_app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
            }
            pub mod executor_params {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum ExecutorParam {
                    #[codec(index = 1)]
                    MaxMemoryPages(::core::primitive::u32),
                    #[codec(index = 2)]
                    StackLogicalMax(::core::primitive::u32),
                    #[codec(index = 3)]
                    StackNativeMax(::core::primitive::u32),
                    #[codec(index = 4)]
                    PrecheckingMaxMemory(::core::primitive::u64),
                    #[codec(index = 5)]
                    PvfPrepTimeout(
                        runtime_types::polkadot_primitives::v5::PvfPrepTimeoutKind,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 6)]
                    PvfExecTimeout(
                        runtime_types::polkadot_primitives::v5::PvfExecTimeoutKind,
                        ::core::primitive::u64,
                    ),
                    #[codec(index = 7)]
                    WasmExtBulkMemory,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct ExecutorParams(
                    pub  ::std::vec::Vec<
                        runtime_types::polkadot_primitives::v5::executor_params::ExecutorParam,
                    >,
                );
            }
            pub mod signed {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UncheckedSigned<_0, _1> {
                    pub payload: _0,
                    pub validator_index: runtime_types::polkadot_primitives::v5::ValidatorIndex,
                    pub signature: runtime_types::polkadot_primitives::v5::validator_app::Signature,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                }
            }
            pub mod slashing {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DisputeProof {
                    pub time_slot:
                        runtime_types::polkadot_primitives::v5::slashing::DisputesTimeSlot,
                    pub kind: runtime_types::polkadot_primitives::v5::slashing::SlashingOffenceKind,
                    pub validator_index: runtime_types::polkadot_primitives::v5::ValidatorIndex,
                    pub validator_id: runtime_types::polkadot_primitives::v5::validator_app::Public,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct DisputesTimeSlot {
                    pub session_index: ::core::primitive::u32,
                    pub candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct OpaqueKeyOwnershipProof(pub ::std::vec::Vec<::core::primitive::u8>);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct PendingSlashes {
                    pub keys: ::subxt::utils::KeyedVec<
                        runtime_types::polkadot_primitives::v5::ValidatorIndex,
                        runtime_types::polkadot_primitives::v5::validator_app::Public,
                    >,
                    pub kind: runtime_types::polkadot_primitives::v5::slashing::SlashingOffenceKind,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum SlashingOffenceKind {
                    #[codec(index = 0)]
                    ForInvalid,
                    #[codec(index = 1)]
                    AgainstValid,
                }
            }
            pub mod validator_app {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Public(pub runtime_types::sp_core::sr25519::Public);
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Signature(pub runtime_types::sp_core::sr25519::Signature);
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AvailabilityBitfield(
                pub  ::subxt::utils::bits::DecodedBits<
                    ::core::primitive::u8,
                    ::subxt::utils::bits::Lsb0,
                >,
            );
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct BackedCandidate<_0> {
                pub candidate:
                    runtime_types::polkadot_primitives::v5::CommittedCandidateReceipt<_0>,
                pub validity_votes:
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v5::ValidityAttestation>,
                pub validator_indices: ::subxt::utils::bits::DecodedBits<
                    ::core::primitive::u8,
                    ::subxt::utils::bits::Lsb0,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CandidateCommitments<_0> {
                pub upward_messages: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    ::std::vec::Vec<::core::primitive::u8>,
                >,
                pub horizontal_messages:
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::polkadot_core_primitives::OutboundHrmpMessage<
                            runtime_types::polkadot_parachain::primitives::Id,
                        >,
                    >,
                pub new_validation_code: ::core::option::Option<
                    runtime_types::polkadot_parachain::primitives::ValidationCode,
                >,
                pub head_data: runtime_types::polkadot_parachain::primitives::HeadData,
                pub processed_downward_messages: ::core::primitive::u32,
                pub hrmp_watermark: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CandidateDescriptor<_0> {
                pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                pub relay_parent: _0,
                pub collator: runtime_types::polkadot_primitives::v5::collator_app::Public,
                pub persisted_validation_data_hash: ::subxt::utils::H256,
                pub pov_hash: ::subxt::utils::H256,
                pub erasure_root: ::subxt::utils::H256,
                pub signature: runtime_types::polkadot_primitives::v5::collator_app::Signature,
                pub para_head: ::subxt::utils::H256,
                pub validation_code_hash:
                    runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum CandidateEvent<_0> {
                #[codec(index = 0)]
                CandidateBacked(
                    runtime_types::polkadot_primitives::v5::CandidateReceipt<_0>,
                    runtime_types::polkadot_parachain::primitives::HeadData,
                    runtime_types::polkadot_primitives::v5::CoreIndex,
                    runtime_types::polkadot_primitives::v5::GroupIndex,
                ),
                #[codec(index = 1)]
                CandidateIncluded(
                    runtime_types::polkadot_primitives::v5::CandidateReceipt<_0>,
                    runtime_types::polkadot_parachain::primitives::HeadData,
                    runtime_types::polkadot_primitives::v5::CoreIndex,
                    runtime_types::polkadot_primitives::v5::GroupIndex,
                ),
                #[codec(index = 2)]
                CandidateTimedOut(
                    runtime_types::polkadot_primitives::v5::CandidateReceipt<_0>,
                    runtime_types::polkadot_parachain::primitives::HeadData,
                    runtime_types::polkadot_primitives::v5::CoreIndex,
                ),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CandidateReceipt<_0> {
                pub descriptor: runtime_types::polkadot_primitives::v5::CandidateDescriptor<_0>,
                pub commitments_hash: ::subxt::utils::H256,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CommittedCandidateReceipt<_0> {
                pub descriptor: runtime_types::polkadot_primitives::v5::CandidateDescriptor<_0>,
                pub commitments: runtime_types::polkadot_primitives::v5::CandidateCommitments<
                    ::core::primitive::u32,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CoreIndex(pub ::core::primitive::u32);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum CoreOccupied {
                #[codec(index = 0)]
                Parathread(runtime_types::polkadot_primitives::v5::ParathreadEntry),
                #[codec(index = 1)]
                Parachain,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum CoreState<_0, _1> {
                #[codec(index = 0)]
                Occupied(runtime_types::polkadot_primitives::v5::OccupiedCore<_0, _1>),
                #[codec(index = 1)]
                Scheduled(runtime_types::polkadot_primitives::v5::ScheduledCore),
                #[codec(index = 2)]
                Free,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct DisputeState<_0> {
                pub validators_for: ::subxt::utils::bits::DecodedBits<
                    ::core::primitive::u8,
                    ::subxt::utils::bits::Lsb0,
                >,
                pub validators_against: ::subxt::utils::bits::DecodedBits<
                    ::core::primitive::u8,
                    ::subxt::utils::bits::Lsb0,
                >,
                pub start: _0,
                pub concluded_at: ::core::option::Option<_0>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum DisputeStatement {
                #[codec(index = 0)]
                Valid(runtime_types::polkadot_primitives::v5::ValidDisputeStatementKind),
                #[codec(index = 1)]
                Invalid(runtime_types::polkadot_primitives::v5::InvalidDisputeStatementKind),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct DisputeStatementSet {
                pub candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash,
                pub session: ::core::primitive::u32,
                pub statements: ::std::vec::Vec<(
                    runtime_types::polkadot_primitives::v5::DisputeStatement,
                    runtime_types::polkadot_primitives::v5::ValidatorIndex,
                    runtime_types::polkadot_primitives::v5::validator_app::Signature,
                )>,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct GroupIndex(pub ::core::primitive::u32);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct GroupRotationInfo<_0> {
                pub session_start_block: _0,
                pub group_rotation_frequency: _0,
                pub now: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct IndexedVec<_0, _1>(
                pub ::std::vec::Vec<_1>,
                #[codec(skip)] pub ::core::marker::PhantomData<_0>,
            );
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct InherentData<_0> {
                pub bitfields: ::std::vec::Vec<
                    runtime_types::polkadot_primitives::v5::signed::UncheckedSigned<
                        runtime_types::polkadot_primitives::v5::AvailabilityBitfield,
                        runtime_types::polkadot_primitives::v5::AvailabilityBitfield,
                    >,
                >,
                pub backed_candidates: ::std::vec::Vec<
                    runtime_types::polkadot_primitives::v5::BackedCandidate<::subxt::utils::H256>,
                >,
                pub disputes:
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v5::DisputeStatementSet>,
                pub parent_header: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum InvalidDisputeStatementKind {
                #[codec(index = 0)]
                Explicit,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OccupiedCore<_0, _1> {
                pub next_up_on_available:
                    ::core::option::Option<runtime_types::polkadot_primitives::v5::ScheduledCore>,
                pub occupied_since: _1,
                pub time_out_at: _1,
                pub next_up_on_time_out:
                    ::core::option::Option<runtime_types::polkadot_primitives::v5::ScheduledCore>,
                pub availability: ::subxt::utils::bits::DecodedBits<
                    ::core::primitive::u8,
                    ::subxt::utils::bits::Lsb0,
                >,
                pub group_responsible: runtime_types::polkadot_primitives::v5::GroupIndex,
                pub candidate_hash: runtime_types::polkadot_core_primitives::CandidateHash,
                pub candidate_descriptor:
                    runtime_types::polkadot_primitives::v5::CandidateDescriptor<_0>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum OccupiedCoreAssumption {
                #[codec(index = 0)]
                Included,
                #[codec(index = 1)]
                TimedOut,
                #[codec(index = 2)]
                Free,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ParathreadClaim(
                pub runtime_types::polkadot_parachain::primitives::Id,
                pub runtime_types::polkadot_primitives::v5::collator_app::Public,
            );
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ParathreadEntry {
                pub claim: runtime_types::polkadot_primitives::v5::ParathreadClaim,
                pub retries: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PersistedValidationData<_0, _1> {
                pub parent_head: runtime_types::polkadot_parachain::primitives::HeadData,
                pub relay_parent_number: _1,
                pub relay_parent_storage_root: _0,
                pub max_pov_size: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PvfCheckStatement {
                pub accept: ::core::primitive::bool,
                pub subject: runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                pub session_index: ::core::primitive::u32,
                pub validator_index: runtime_types::polkadot_primitives::v5::ValidatorIndex,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum PvfExecTimeoutKind {
                #[codec(index = 0)]
                Backing,
                #[codec(index = 1)]
                Approval,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum PvfPrepTimeoutKind {
                #[codec(index = 0)]
                Precheck,
                #[codec(index = 1)]
                Lenient,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ScheduledCore {
                pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                pub collator: ::core::option::Option<
                    runtime_types::polkadot_primitives::v5::collator_app::Public,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ScrapedOnChainVotes<_0> {
                pub session: ::core::primitive::u32,
                pub backing_validators_per_candidate: ::std::vec::Vec<(
                    runtime_types::polkadot_primitives::v5::CandidateReceipt<_0>,
                    ::std::vec::Vec<(
                        runtime_types::polkadot_primitives::v5::ValidatorIndex,
                        runtime_types::polkadot_primitives::v5::ValidityAttestation,
                    )>,
                )>,
                pub disputes:
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v5::DisputeStatementSet>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct SessionInfo {
                pub active_validator_indices:
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v5::ValidatorIndex>,
                pub random_seed: [::core::primitive::u8; 32usize],
                pub dispute_period: ::core::primitive::u32,
                pub validators: runtime_types::polkadot_primitives::v5::IndexedVec<
                    runtime_types::polkadot_primitives::v5::ValidatorIndex,
                    runtime_types::polkadot_primitives::v5::validator_app::Public,
                >,
                pub discovery_keys:
                    ::std::vec::Vec<runtime_types::sp_authority_discovery::app::Public>,
                pub assignment_keys:
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v5::assignment_app::Public>,
                pub validator_groups: runtime_types::polkadot_primitives::v5::IndexedVec<
                    runtime_types::polkadot_primitives::v5::GroupIndex,
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v5::ValidatorIndex>,
                >,
                pub n_cores: ::core::primitive::u32,
                pub zeroth_delay_tranche_width: ::core::primitive::u32,
                pub relay_vrf_modulo_samples: ::core::primitive::u32,
                pub n_delay_tranches: ::core::primitive::u32,
                pub no_show_slots: ::core::primitive::u32,
                pub needed_approvals: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum UpgradeGoAhead {
                #[codec(index = 0)]
                Abort,
                #[codec(index = 1)]
                GoAhead,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum UpgradeRestriction {
                #[codec(index = 0)]
                Present,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum ValidDisputeStatementKind {
                #[codec(index = 0)]
                Explicit,
                #[codec(index = 1)]
                BackingSeconded(::subxt::utils::H256),
                #[codec(index = 2)]
                BackingValid(::subxt::utils::H256),
                #[codec(index = 3)]
                ApprovalChecking,
            }
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ValidatorIndex(pub ::core::primitive::u32);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum ValidityAttestation {
                #[codec(index = 1)]
                Implicit(runtime_types::polkadot_primitives::v5::validator_app::Signature),
                #[codec(index = 2)]
                Explicit(runtime_types::polkadot_primitives::v5::validator_app::Signature),
            }
        }
        pub mod vstaging {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AsyncBackingParams {
                pub max_candidate_depth: ::core::primitive::u32,
                pub allowed_ancestry_len: ::core::primitive::u32,
            }
        }
    }
    pub mod polkadot_runtime {
        use super::runtime_types;
        pub mod governance {
            use super::runtime_types;
            pub mod origins {
                use super::runtime_types;
                pub mod pallet_custom_origins {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    pub enum Origin {
                        #[codec(index = 0)]
                        StakingAdmin,
                        #[codec(index = 1)]
                        Treasurer,
                        #[codec(index = 2)]
                        FellowshipAdmin,
                        #[codec(index = 3)]
                        GeneralAdmin,
                        #[codec(index = 4)]
                        AuctionAdmin,
                        #[codec(index = 5)]
                        LeaseAdmin,
                        #[codec(index = 6)]
                        ReferendumCanceller,
                        #[codec(index = 7)]
                        ReferendumKiller,
                        #[codec(index = 8)]
                        SmallTipper,
                        #[codec(index = 9)]
                        BigTipper,
                        #[codec(index = 10)]
                        SmallSpender,
                        #[codec(index = 11)]
                        MediumSpender,
                        #[codec(index = 12)]
                        BigSpender,
                        #[codec(index = 13)]
                        WhitelistedCaller,
                    }
                }
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct NposCompactSolution16 {
            pub votes1: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
            pub votes2: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                (
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                    ::subxt::ext::codec::Compact<runtime_types::sp_arithmetic::per_things::PerU16>,
                ),
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
            pub votes3: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                [(
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                    ::subxt::ext::codec::Compact<runtime_types::sp_arithmetic::per_things::PerU16>,
                ); 2usize],
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
            pub votes4: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                [(
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                    ::subxt::ext::codec::Compact<runtime_types::sp_arithmetic::per_things::PerU16>,
                ); 3usize],
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
            pub votes5: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                [(
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                    ::subxt::ext::codec::Compact<runtime_types::sp_arithmetic::per_things::PerU16>,
                ); 4usize],
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
            pub votes6: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                [(
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                    ::subxt::ext::codec::Compact<runtime_types::sp_arithmetic::per_things::PerU16>,
                ); 5usize],
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
            pub votes7: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                [(
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                    ::subxt::ext::codec::Compact<runtime_types::sp_arithmetic::per_things::PerU16>,
                ); 6usize],
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
            pub votes8: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                [(
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                    ::subxt::ext::codec::Compact<runtime_types::sp_arithmetic::per_things::PerU16>,
                ); 7usize],
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
            pub votes9: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                [(
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                    ::subxt::ext::codec::Compact<runtime_types::sp_arithmetic::per_things::PerU16>,
                ); 8usize],
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
            pub votes10: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                [(
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                    ::subxt::ext::codec::Compact<runtime_types::sp_arithmetic::per_things::PerU16>,
                ); 9usize],
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
            pub votes11: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                [(
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                    ::subxt::ext::codec::Compact<runtime_types::sp_arithmetic::per_things::PerU16>,
                ); 10usize],
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
            pub votes12: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                [(
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                    ::subxt::ext::codec::Compact<runtime_types::sp_arithmetic::per_things::PerU16>,
                ); 11usize],
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
            pub votes13: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                [(
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                    ::subxt::ext::codec::Compact<runtime_types::sp_arithmetic::per_things::PerU16>,
                ); 12usize],
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
            pub votes14: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                [(
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                    ::subxt::ext::codec::Compact<runtime_types::sp_arithmetic::per_things::PerU16>,
                ); 13usize],
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
            pub votes15: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                [(
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                    ::subxt::ext::codec::Compact<runtime_types::sp_arithmetic::per_things::PerU16>,
                ); 14usize],
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
            pub votes16: ::std::vec::Vec<(
                ::subxt::ext::codec::Compact<::core::primitive::u32>,
                [(
                    ::subxt::ext::codec::Compact<::core::primitive::u16>,
                    ::subxt::ext::codec::Compact<runtime_types::sp_arithmetic::per_things::PerU16>,
                ); 15usize],
                ::subxt::ext::codec::Compact<::core::primitive::u16>,
            )>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum OriginCaller {
            #[codec(index = 0)]
            system(runtime_types::frame_support::dispatch::RawOrigin<::subxt::utils::AccountId32>),
            #[codec(index = 15)]
            Council(runtime_types::pallet_collective::RawOrigin<::subxt::utils::AccountId32>),
            #[codec(index = 16)]
            TechnicalCommittee(
                runtime_types::pallet_collective::RawOrigin<::subxt::utils::AccountId32>,
            ),
            #[codec(index = 22)]
            Origins(
                runtime_types::polkadot_runtime::governance::origins::pallet_custom_origins::Origin,
            ),
            #[codec(index = 50)]
            ParachainsOrigin(runtime_types::polkadot_runtime_parachains::origin::pallet::Origin),
            #[codec(index = 99)]
            XcmPallet(runtime_types::pallet_xcm::pallet::Origin),
            #[codec(index = 6)]
            Void(runtime_types::sp_core::Void),
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum ProxyType {
            #[codec(index = 0)]
            Any,
            #[codec(index = 1)]
            NonTransfer,
            #[codec(index = 2)]
            Governance,
            #[codec(index = 3)]
            Staking,
            #[codec(index = 5)]
            IdentityJudgement,
            #[codec(index = 6)]
            CancelProxy,
            #[codec(index = 7)]
            Auction,
            #[codec(index = 8)]
            NominationPools,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Runtime;
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum RuntimeCall {
            #[codec(index = 0)]
            System(runtime_types::frame_system::pallet::Call),
            #[codec(index = 1)]
            Scheduler(runtime_types::pallet_scheduler::pallet::Call),
            #[codec(index = 10)]
            Preimage(runtime_types::pallet_preimage::pallet::Call),
            #[codec(index = 2)]
            Babe(runtime_types::pallet_babe::pallet::Call),
            #[codec(index = 3)]
            Timestamp(runtime_types::pallet_timestamp::pallet::Call),
            #[codec(index = 4)]
            Indices(runtime_types::pallet_indices::pallet::Call),
            #[codec(index = 5)]
            Balances(runtime_types::pallet_balances::pallet::Call),
            #[codec(index = 7)]
            Staking(runtime_types::pallet_staking::pallet::pallet::Call),
            #[codec(index = 9)]
            Session(runtime_types::pallet_session::pallet::Call),
            #[codec(index = 11)]
            Grandpa(runtime_types::pallet_grandpa::pallet::Call),
            #[codec(index = 12)]
            ImOnline(runtime_types::pallet_im_online::pallet::Call),
            #[codec(index = 14)]
            Democracy(runtime_types::pallet_democracy::pallet::Call),
            #[codec(index = 15)]
            Council(runtime_types::pallet_collective::pallet::Call),
            #[codec(index = 16)]
            TechnicalCommittee(runtime_types::pallet_collective::pallet::Call2),
            #[codec(index = 17)]
            PhragmenElection(runtime_types::pallet_elections_phragmen::pallet::Call),
            #[codec(index = 18)]
            TechnicalMembership(runtime_types::pallet_membership::pallet::Call),
            #[codec(index = 19)]
            Treasury(runtime_types::pallet_treasury::pallet::Call),
            #[codec(index = 20)]
            ConvictionVoting(runtime_types::pallet_conviction_voting::pallet::Call),
            #[codec(index = 21)]
            Referenda(runtime_types::pallet_referenda::pallet::Call),
            #[codec(index = 23)]
            Whitelist(runtime_types::pallet_whitelist::pallet::Call),
            #[codec(index = 24)]
            Claims(runtime_types::polkadot_runtime_common::claims::pallet::Call),
            #[codec(index = 25)]
            Vesting(runtime_types::pallet_vesting::pallet::Call),
            #[codec(index = 26)]
            Utility(runtime_types::pallet_utility::pallet::Call),
            #[codec(index = 28)]
            Identity(runtime_types::pallet_identity::pallet::Call),
            #[codec(index = 29)]
            Proxy(runtime_types::pallet_proxy::pallet::Call),
            #[codec(index = 30)]
            Multisig(runtime_types::pallet_multisig::pallet::Call),
            #[codec(index = 34)]
            Bounties(runtime_types::pallet_bounties::pallet::Call),
            #[codec(index = 38)]
            ChildBounties(runtime_types::pallet_child_bounties::pallet::Call),
            #[codec(index = 35)]
            Tips(runtime_types::pallet_tips::pallet::Call),
            #[codec(index = 36)]
            ElectionProviderMultiPhase(
                runtime_types::pallet_election_provider_multi_phase::pallet::Call,
            ),
            #[codec(index = 37)]
            VoterList(runtime_types::pallet_bags_list::pallet::Call),
            #[codec(index = 39)]
            NominationPools(runtime_types::pallet_nomination_pools::pallet::Call),
            #[codec(index = 40)]
            FastUnstake(runtime_types::pallet_fast_unstake::pallet::Call),
            #[codec(index = 51)]
            Configuration(runtime_types::polkadot_runtime_parachains::configuration::pallet::Call),
            #[codec(index = 52)]
            ParasShared(runtime_types::polkadot_runtime_parachains::shared::pallet::Call),
            #[codec(index = 53)]
            ParaInclusion(runtime_types::polkadot_runtime_parachains::inclusion::pallet::Call),
            #[codec(index = 54)]
            ParaInherent(runtime_types::polkadot_runtime_parachains::paras_inherent::pallet::Call),
            #[codec(index = 56)]
            Paras(runtime_types::polkadot_runtime_parachains::paras::pallet::Call),
            #[codec(index = 57)]
            Initializer(runtime_types::polkadot_runtime_parachains::initializer::pallet::Call),
            #[codec(index = 60)]
            Hrmp(runtime_types::polkadot_runtime_parachains::hrmp::pallet::Call),
            #[codec(index = 62)]
            ParasDisputes(runtime_types::polkadot_runtime_parachains::disputes::pallet::Call),
            #[codec(index = 63)]
            ParasSlashing(
                runtime_types::polkadot_runtime_parachains::disputes::slashing::pallet::Call,
            ),
            #[codec(index = 70)]
            Registrar(runtime_types::polkadot_runtime_common::paras_registrar::pallet::Call),
            #[codec(index = 71)]
            Slots(runtime_types::polkadot_runtime_common::slots::pallet::Call),
            #[codec(index = 72)]
            Auctions(runtime_types::polkadot_runtime_common::auctions::pallet::Call),
            #[codec(index = 73)]
            Crowdloan(runtime_types::polkadot_runtime_common::crowdloan::pallet::Call),
            #[codec(index = 99)]
            XcmPallet(runtime_types::pallet_xcm::pallet::Call),
            #[codec(index = 100)]
            MessageQueue(runtime_types::pallet_message_queue::pallet::Call),
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum RuntimeError {
            #[codec(index = 0)]
            System(runtime_types::frame_system::pallet::Error),
            #[codec(index = 1)]
            Scheduler(runtime_types::pallet_scheduler::pallet::Error),
            #[codec(index = 10)]
            Preimage(runtime_types::pallet_preimage::pallet::Error),
            #[codec(index = 2)]
            Babe(runtime_types::pallet_babe::pallet::Error),
            #[codec(index = 4)]
            Indices(runtime_types::pallet_indices::pallet::Error),
            #[codec(index = 5)]
            Balances(runtime_types::pallet_balances::pallet::Error),
            #[codec(index = 7)]
            Staking(runtime_types::pallet_staking::pallet::pallet::Error),
            #[codec(index = 9)]
            Session(runtime_types::pallet_session::pallet::Error),
            #[codec(index = 11)]
            Grandpa(runtime_types::pallet_grandpa::pallet::Error),
            #[codec(index = 12)]
            ImOnline(runtime_types::pallet_im_online::pallet::Error),
            #[codec(index = 14)]
            Democracy(runtime_types::pallet_democracy::pallet::Error),
            #[codec(index = 15)]
            Council(runtime_types::pallet_collective::pallet::Error),
            #[codec(index = 16)]
            TechnicalCommittee(runtime_types::pallet_collective::pallet::Error2),
            #[codec(index = 17)]
            PhragmenElection(runtime_types::pallet_elections_phragmen::pallet::Error),
            #[codec(index = 18)]
            TechnicalMembership(runtime_types::pallet_membership::pallet::Error),
            #[codec(index = 19)]
            Treasury(runtime_types::pallet_treasury::pallet::Error),
            #[codec(index = 20)]
            ConvictionVoting(runtime_types::pallet_conviction_voting::pallet::Error),
            #[codec(index = 21)]
            Referenda(runtime_types::pallet_referenda::pallet::Error),
            #[codec(index = 23)]
            Whitelist(runtime_types::pallet_whitelist::pallet::Error),
            #[codec(index = 24)]
            Claims(runtime_types::polkadot_runtime_common::claims::pallet::Error),
            #[codec(index = 25)]
            Vesting(runtime_types::pallet_vesting::pallet::Error),
            #[codec(index = 26)]
            Utility(runtime_types::pallet_utility::pallet::Error),
            #[codec(index = 28)]
            Identity(runtime_types::pallet_identity::pallet::Error),
            #[codec(index = 29)]
            Proxy(runtime_types::pallet_proxy::pallet::Error),
            #[codec(index = 30)]
            Multisig(runtime_types::pallet_multisig::pallet::Error),
            #[codec(index = 34)]
            Bounties(runtime_types::pallet_bounties::pallet::Error),
            #[codec(index = 38)]
            ChildBounties(runtime_types::pallet_child_bounties::pallet::Error),
            #[codec(index = 35)]
            Tips(runtime_types::pallet_tips::pallet::Error),
            #[codec(index = 36)]
            ElectionProviderMultiPhase(
                runtime_types::pallet_election_provider_multi_phase::pallet::Error,
            ),
            #[codec(index = 37)]
            VoterList(runtime_types::pallet_bags_list::pallet::Error),
            #[codec(index = 39)]
            NominationPools(runtime_types::pallet_nomination_pools::pallet::Error),
            #[codec(index = 40)]
            FastUnstake(runtime_types::pallet_fast_unstake::pallet::Error),
            #[codec(index = 51)]
            Configuration(runtime_types::polkadot_runtime_parachains::configuration::pallet::Error),
            #[codec(index = 53)]
            ParaInclusion(runtime_types::polkadot_runtime_parachains::inclusion::pallet::Error),
            #[codec(index = 54)]
            ParaInherent(runtime_types::polkadot_runtime_parachains::paras_inherent::pallet::Error),
            #[codec(index = 56)]
            Paras(runtime_types::polkadot_runtime_parachains::paras::pallet::Error),
            #[codec(index = 60)]
            Hrmp(runtime_types::polkadot_runtime_parachains::hrmp::pallet::Error),
            #[codec(index = 62)]
            ParasDisputes(runtime_types::polkadot_runtime_parachains::disputes::pallet::Error),
            #[codec(index = 63)]
            ParasSlashing(
                runtime_types::polkadot_runtime_parachains::disputes::slashing::pallet::Error,
            ),
            #[codec(index = 70)]
            Registrar(runtime_types::polkadot_runtime_common::paras_registrar::pallet::Error),
            #[codec(index = 71)]
            Slots(runtime_types::polkadot_runtime_common::slots::pallet::Error),
            #[codec(index = 72)]
            Auctions(runtime_types::polkadot_runtime_common::auctions::pallet::Error),
            #[codec(index = 73)]
            Crowdloan(runtime_types::polkadot_runtime_common::crowdloan::pallet::Error),
            #[codec(index = 99)]
            XcmPallet(runtime_types::pallet_xcm::pallet::Error),
            #[codec(index = 100)]
            MessageQueue(runtime_types::pallet_message_queue::pallet::Error),
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum RuntimeEvent {
            #[codec(index = 0)]
            System(runtime_types::frame_system::pallet::Event),
            #[codec(index = 1)]
            Scheduler(runtime_types::pallet_scheduler::pallet::Event),
            #[codec(index = 10)]
            Preimage(runtime_types::pallet_preimage::pallet::Event),
            #[codec(index = 4)]
            Indices(runtime_types::pallet_indices::pallet::Event),
            #[codec(index = 5)]
            Balances(runtime_types::pallet_balances::pallet::Event),
            #[codec(index = 32)]
            TransactionPayment(runtime_types::pallet_transaction_payment::pallet::Event),
            #[codec(index = 7)]
            Staking(runtime_types::pallet_staking::pallet::pallet::Event),
            #[codec(index = 8)]
            Offences(runtime_types::pallet_offences::pallet::Event),
            #[codec(index = 9)]
            Session(runtime_types::pallet_session::pallet::Event),
            #[codec(index = 11)]
            Grandpa(runtime_types::pallet_grandpa::pallet::Event),
            #[codec(index = 12)]
            ImOnline(runtime_types::pallet_im_online::pallet::Event),
            #[codec(index = 14)]
            Democracy(runtime_types::pallet_democracy::pallet::Event),
            #[codec(index = 15)]
            Council(runtime_types::pallet_collective::pallet::Event),
            #[codec(index = 16)]
            TechnicalCommittee(runtime_types::pallet_collective::pallet::Event2),
            #[codec(index = 17)]
            PhragmenElection(runtime_types::pallet_elections_phragmen::pallet::Event),
            #[codec(index = 18)]
            TechnicalMembership(runtime_types::pallet_membership::pallet::Event),
            #[codec(index = 19)]
            Treasury(runtime_types::pallet_treasury::pallet::Event),
            #[codec(index = 20)]
            ConvictionVoting(runtime_types::pallet_conviction_voting::pallet::Event),
            #[codec(index = 21)]
            Referenda(runtime_types::pallet_referenda::pallet::Event),
            #[codec(index = 23)]
            Whitelist(runtime_types::pallet_whitelist::pallet::Event),
            #[codec(index = 24)]
            Claims(runtime_types::polkadot_runtime_common::claims::pallet::Event),
            #[codec(index = 25)]
            Vesting(runtime_types::pallet_vesting::pallet::Event),
            #[codec(index = 26)]
            Utility(runtime_types::pallet_utility::pallet::Event),
            #[codec(index = 28)]
            Identity(runtime_types::pallet_identity::pallet::Event),
            #[codec(index = 29)]
            Proxy(runtime_types::pallet_proxy::pallet::Event),
            #[codec(index = 30)]
            Multisig(runtime_types::pallet_multisig::pallet::Event),
            #[codec(index = 34)]
            Bounties(runtime_types::pallet_bounties::pallet::Event),
            #[codec(index = 38)]
            ChildBounties(runtime_types::pallet_child_bounties::pallet::Event),
            #[codec(index = 35)]
            Tips(runtime_types::pallet_tips::pallet::Event),
            #[codec(index = 36)]
            ElectionProviderMultiPhase(
                runtime_types::pallet_election_provider_multi_phase::pallet::Event,
            ),
            #[codec(index = 37)]
            VoterList(runtime_types::pallet_bags_list::pallet::Event),
            #[codec(index = 39)]
            NominationPools(runtime_types::pallet_nomination_pools::pallet::Event),
            #[codec(index = 40)]
            FastUnstake(runtime_types::pallet_fast_unstake::pallet::Event),
            #[codec(index = 53)]
            ParaInclusion(runtime_types::polkadot_runtime_parachains::inclusion::pallet::Event),
            #[codec(index = 56)]
            Paras(runtime_types::polkadot_runtime_parachains::paras::pallet::Event),
            #[codec(index = 60)]
            Hrmp(runtime_types::polkadot_runtime_parachains::hrmp::pallet::Event),
            #[codec(index = 62)]
            ParasDisputes(runtime_types::polkadot_runtime_parachains::disputes::pallet::Event),
            #[codec(index = 70)]
            Registrar(runtime_types::polkadot_runtime_common::paras_registrar::pallet::Event),
            #[codec(index = 71)]
            Slots(runtime_types::polkadot_runtime_common::slots::pallet::Event),
            #[codec(index = 72)]
            Auctions(runtime_types::polkadot_runtime_common::auctions::pallet::Event),
            #[codec(index = 73)]
            Crowdloan(runtime_types::polkadot_runtime_common::crowdloan::pallet::Event),
            #[codec(index = 99)]
            XcmPallet(runtime_types::pallet_xcm::pallet::Event),
            #[codec(index = 100)]
            MessageQueue(runtime_types::pallet_message_queue::pallet::Event),
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum RuntimeHoldReason {}
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct SessionKeys {
            pub grandpa: runtime_types::sp_consensus_grandpa::app::Public,
            pub babe: runtime_types::sp_consensus_babe::app::Public,
            pub im_online: runtime_types::pallet_im_online::sr25519::app_sr25519::Public,
            pub para_validator: runtime_types::polkadot_primitives::v5::validator_app::Public,
            pub para_assignment: runtime_types::polkadot_primitives::v5::assignment_app::Public,
            pub authority_discovery: runtime_types::sp_authority_discovery::app::Public,
        }
    }
    pub mod polkadot_runtime_common {
        use super::runtime_types;
        pub mod auctions {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::new_auction`]."]
                    new_auction {
                        #[codec(compact)]
                        duration: ::core::primitive::u32,
                        #[codec(compact)]
                        lease_period_index: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::bid`]."]
                    bid {
                        #[codec(compact)]
                        para: runtime_types::polkadot_parachain::primitives::Id,
                        #[codec(compact)]
                        auction_index: ::core::primitive::u32,
                        #[codec(compact)]
                        first_slot: ::core::primitive::u32,
                        #[codec(compact)]
                        last_slot: ::core::primitive::u32,
                        #[codec(compact)]
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::cancel_auction`]."]
                    cancel_auction,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "This auction is already in progress."]
                    AuctionInProgress,
                    #[codec(index = 1)]
                    #[doc = "The lease period is in the past."]
                    LeasePeriodInPast,
                    #[codec(index = 2)]
                    #[doc = "Para is not registered"]
                    ParaNotRegistered,
                    #[codec(index = 3)]
                    #[doc = "Not a current auction."]
                    NotCurrentAuction,
                    #[codec(index = 4)]
                    #[doc = "Not an auction."]
                    NotAuction,
                    #[codec(index = 5)]
                    #[doc = "Auction has already ended."]
                    AuctionEnded,
                    #[codec(index = 6)]
                    #[doc = "The para is already leased out for part of this range."]
                    AlreadyLeasedOut,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "An auction started. Provides its index and the block number where it will begin to"]
                    #[doc = "close and the first lease period of the quadruplet that is auctioned."]
                    AuctionStarted {
                        auction_index: ::core::primitive::u32,
                        lease_period: ::core::primitive::u32,
                        ending: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "An auction ended. All funds become unreserved."]
                    AuctionClosed {
                        auction_index: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    #[doc = "Funds were reserved for a winning bid. First balance is the extra amount reserved."]
                    #[doc = "Second is the total."]
                    Reserved {
                        bidder: ::subxt::utils::AccountId32,
                        extra_reserved: ::core::primitive::u128,
                        total_amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "Funds were unreserved since bidder is no longer active. `[bidder, amount]`"]
                    Unreserved {
                        bidder: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 4)]
                    #[doc = "Someone attempted to lease the same slot twice for a parachain. The amount is held in reserve"]
                    #[doc = "but no parachain slot has been leased."]
                    ReserveConfiscated {
                        para_id: runtime_types::polkadot_parachain::primitives::Id,
                        leaser: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 5)]
                    #[doc = "A new bid has been accepted as the current winner."]
                    BidAccepted {
                        bidder: ::subxt::utils::AccountId32,
                        para_id: runtime_types::polkadot_parachain::primitives::Id,
                        amount: ::core::primitive::u128,
                        first_slot: ::core::primitive::u32,
                        last_slot: ::core::primitive::u32,
                    },
                    #[codec(index = 6)]
                    #[doc = "The winning offset was chosen for an auction. This will map into the `Winning` storage map."]
                    WinningOffset {
                        auction_index: ::core::primitive::u32,
                        block_number: ::core::primitive::u32,
                    },
                }
            }
        }
        pub mod claims {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::claim`]."]
                    claim {
                        dest: ::subxt::utils::AccountId32,
                        ethereum_signature:
                            runtime_types::polkadot_runtime_common::claims::EcdsaSignature,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::mint_claim`]."]
                    mint_claim {
                        who: runtime_types::polkadot_runtime_common::claims::EthereumAddress,
                        value: ::core::primitive::u128,
                        vesting_schedule: ::core::option::Option<(
                            ::core::primitive::u128,
                            ::core::primitive::u128,
                            ::core::primitive::u32,
                        )>,
                        statement: ::core::option::Option<
                            runtime_types::polkadot_runtime_common::claims::StatementKind,
                        >,
                    },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::claim_attest`]."]
                    claim_attest {
                        dest: ::subxt::utils::AccountId32,
                        ethereum_signature:
                            runtime_types::polkadot_runtime_common::claims::EcdsaSignature,
                        statement: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 3)]
                    #[doc = "See [`Pallet::attest`]."]
                    attest {
                        statement: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 4)]
                    #[doc = "See [`Pallet::move_claim`]."]
                    move_claim {
                        old: runtime_types::polkadot_runtime_common::claims::EthereumAddress,
                        new: runtime_types::polkadot_runtime_common::claims::EthereumAddress,
                        maybe_preclaim: ::core::option::Option<::subxt::utils::AccountId32>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Invalid Ethereum signature."]
                    InvalidEthereumSignature,
                    #[codec(index = 1)]
                    #[doc = "Ethereum address has no claim."]
                    SignerHasNoClaim,
                    #[codec(index = 2)]
                    #[doc = "Account ID sending transaction has no claim."]
                    SenderHasNoClaim,
                    #[codec(index = 3)]
                    #[doc = "There's not enough in the pot to pay out some unvested amount. Generally implies a logic"]
                    #[doc = "error."]
                    PotUnderflow,
                    #[codec(index = 4)]
                    #[doc = "A needed statement was not included."]
                    InvalidStatement,
                    #[codec(index = 5)]
                    #[doc = "The account already has a vested balance."]
                    VestedBalanceExists,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Someone claimed some DOTs."]
                    Claimed {
                        who: ::subxt::utils::AccountId32,
                        ethereum_address:
                            runtime_types::polkadot_runtime_common::claims::EthereumAddress,
                        amount: ::core::primitive::u128,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EcdsaSignature(pub [::core::primitive::u8; 65usize]);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct EthereumAddress(pub [::core::primitive::u8; 20usize]);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PrevalidateAttests;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum StatementKind {
                #[codec(index = 0)]
                Regular,
                #[codec(index = 1)]
                Saft,
            }
        }
        pub mod crowdloan {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::create`]."]
                    create {
                        #[codec(compact)]
                        index: runtime_types::polkadot_parachain::primitives::Id,
                        #[codec(compact)]
                        cap: ::core::primitive::u128,
                        #[codec(compact)]
                        first_period: ::core::primitive::u32,
                        #[codec(compact)]
                        last_period: ::core::primitive::u32,
                        #[codec(compact)]
                        end: ::core::primitive::u32,
                        verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::contribute`]."]
                    contribute {
                        #[codec(compact)]
                        index: runtime_types::polkadot_parachain::primitives::Id,
                        #[codec(compact)]
                        value: ::core::primitive::u128,
                        signature:
                            ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
                    },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::withdraw`]."]
                    withdraw {
                        who: ::subxt::utils::AccountId32,
                        #[codec(compact)]
                        index: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 3)]
                    #[doc = "See [`Pallet::refund`]."]
                    refund {
                        #[codec(compact)]
                        index: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 4)]
                    #[doc = "See [`Pallet::dissolve`]."]
                    dissolve {
                        #[codec(compact)]
                        index: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 5)]
                    #[doc = "See [`Pallet::edit`]."]
                    edit {
                        #[codec(compact)]
                        index: runtime_types::polkadot_parachain::primitives::Id,
                        #[codec(compact)]
                        cap: ::core::primitive::u128,
                        #[codec(compact)]
                        first_period: ::core::primitive::u32,
                        #[codec(compact)]
                        last_period: ::core::primitive::u32,
                        #[codec(compact)]
                        end: ::core::primitive::u32,
                        verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
                    },
                    #[codec(index = 6)]
                    #[doc = "See [`Pallet::add_memo`]."]
                    add_memo {
                        index: runtime_types::polkadot_parachain::primitives::Id,
                        memo: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 7)]
                    #[doc = "See [`Pallet::poke`]."]
                    poke {
                        index: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 8)]
                    #[doc = "See [`Pallet::contribute_all`]."]
                    contribute_all {
                        #[codec(compact)]
                        index: runtime_types::polkadot_parachain::primitives::Id,
                        signature:
                            ::core::option::Option<runtime_types::sp_runtime::MultiSignature>,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The current lease period is more than the first lease period."]
                    FirstPeriodInPast,
                    #[codec(index = 1)]
                    #[doc = "The first lease period needs to at least be less than 3 `max_value`."]
                    FirstPeriodTooFarInFuture,
                    #[codec(index = 2)]
                    #[doc = "Last lease period must be greater than first lease period."]
                    LastPeriodBeforeFirstPeriod,
                    #[codec(index = 3)]
                    #[doc = "The last lease period cannot be more than 3 periods after the first period."]
                    LastPeriodTooFarInFuture,
                    #[codec(index = 4)]
                    #[doc = "The campaign ends before the current block number. The end must be in the future."]
                    CannotEndInPast,
                    #[codec(index = 5)]
                    #[doc = "The end date for this crowdloan is not sensible."]
                    EndTooFarInFuture,
                    #[codec(index = 6)]
                    #[doc = "There was an overflow."]
                    Overflow,
                    #[codec(index = 7)]
                    #[doc = "The contribution was below the minimum, `MinContribution`."]
                    ContributionTooSmall,
                    #[codec(index = 8)]
                    #[doc = "Invalid fund index."]
                    InvalidParaId,
                    #[codec(index = 9)]
                    #[doc = "Contributions exceed maximum amount."]
                    CapExceeded,
                    #[codec(index = 10)]
                    #[doc = "The contribution period has already ended."]
                    ContributionPeriodOver,
                    #[codec(index = 11)]
                    #[doc = "The origin of this call is invalid."]
                    InvalidOrigin,
                    #[codec(index = 12)]
                    #[doc = "This crowdloan does not correspond to a parachain."]
                    NotParachain,
                    #[codec(index = 13)]
                    #[doc = "This parachain lease is still active and retirement cannot yet begin."]
                    LeaseActive,
                    #[codec(index = 14)]
                    #[doc = "This parachain's bid or lease is still active and withdraw cannot yet begin."]
                    BidOrLeaseActive,
                    #[codec(index = 15)]
                    #[doc = "The crowdloan has not yet ended."]
                    FundNotEnded,
                    #[codec(index = 16)]
                    #[doc = "There are no contributions stored in this crowdloan."]
                    NoContributions,
                    #[codec(index = 17)]
                    #[doc = "The crowdloan is not ready to dissolve. Potentially still has a slot or in retirement period."]
                    NotReadyToDissolve,
                    #[codec(index = 18)]
                    #[doc = "Invalid signature."]
                    InvalidSignature,
                    #[codec(index = 19)]
                    #[doc = "The provided memo is too large."]
                    MemoTooLarge,
                    #[codec(index = 20)]
                    #[doc = "The fund is already in `NewRaise`"]
                    AlreadyInNewRaise,
                    #[codec(index = 21)]
                    #[doc = "No contributions allowed during the VRF delay"]
                    VrfDelayInProgress,
                    #[codec(index = 22)]
                    #[doc = "A lease period has not started yet, due to an offset in the starting block."]
                    NoLeasePeriod,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Create a new crowdloaning campaign."]
                    Created {
                        para_id: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 1)]
                    #[doc = "Contributed to a crowd sale."]
                    Contributed {
                        who: ::subxt::utils::AccountId32,
                        fund_index: runtime_types::polkadot_parachain::primitives::Id,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 2)]
                    #[doc = "Withdrew full balance of a contributor."]
                    Withdrew {
                        who: ::subxt::utils::AccountId32,
                        fund_index: runtime_types::polkadot_parachain::primitives::Id,
                        amount: ::core::primitive::u128,
                    },
                    #[codec(index = 3)]
                    #[doc = "The loans in a fund have been partially dissolved, i.e. there are some left"]
                    #[doc = "over child keys that still need to be killed."]
                    PartiallyRefunded {
                        para_id: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 4)]
                    #[doc = "All loans in a fund have been refunded."]
                    AllRefunded {
                        para_id: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 5)]
                    #[doc = "Fund is dissolved."]
                    Dissolved {
                        para_id: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 6)]
                    #[doc = "The result of trying to submit a new bid to the Slots pallet."]
                    HandleBidResult {
                        para_id: runtime_types::polkadot_parachain::primitives::Id,
                        result:
                            ::core::result::Result<(), runtime_types::sp_runtime::DispatchError>,
                    },
                    #[codec(index = 7)]
                    #[doc = "The configuration to a crowdloan has been edited."]
                    Edited {
                        para_id: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 8)]
                    #[doc = "A memo has been updated."]
                    MemoUpdated {
                        who: ::subxt::utils::AccountId32,
                        para_id: runtime_types::polkadot_parachain::primitives::Id,
                        memo: ::std::vec::Vec<::core::primitive::u8>,
                    },
                    #[codec(index = 9)]
                    #[doc = "A parachain has been moved to `NewRaise`"]
                    AddedToNewRaise {
                        para_id: runtime_types::polkadot_parachain::primitives::Id,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct FundInfo<_0, _1, _2, _3> {
                pub depositor: _0,
                pub verifier: ::core::option::Option<runtime_types::sp_runtime::MultiSigner>,
                pub deposit: _1,
                pub raised: _1,
                pub end: _2,
                pub cap: _1,
                pub last_contribution:
                    runtime_types::polkadot_runtime_common::crowdloan::LastContribution<_2>,
                pub first_period: _3,
                pub last_period: _3,
                pub fund_index: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum LastContribution<_0> {
                #[codec(index = 0)]
                Never,
                #[codec(index = 1)]
                PreEnding(::core::primitive::u32),
                #[codec(index = 2)]
                Ending(_0),
            }
        }
        pub mod paras_registrar {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::register`]."]
                    register {
                        id: runtime_types::polkadot_parachain::primitives::Id,
                        genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                        validation_code:
                            runtime_types::polkadot_parachain::primitives::ValidationCode,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::force_register`]."]
                    force_register {
                        who: ::subxt::utils::AccountId32,
                        deposit: ::core::primitive::u128,
                        id: runtime_types::polkadot_parachain::primitives::Id,
                        genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                        validation_code:
                            runtime_types::polkadot_parachain::primitives::ValidationCode,
                    },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::deregister`]."]
                    deregister {
                        id: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 3)]
                    #[doc = "See [`Pallet::swap`]."]
                    swap {
                        id: runtime_types::polkadot_parachain::primitives::Id,
                        other: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 4)]
                    #[doc = "See [`Pallet::remove_lock`]."]
                    remove_lock {
                        para: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 5)]
                    #[doc = "See [`Pallet::reserve`]."]
                    reserve,
                    #[codec(index = 6)]
                    #[doc = "See [`Pallet::add_lock`]."]
                    add_lock {
                        para: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 7)]
                    #[doc = "See [`Pallet::schedule_code_upgrade`]."]
                    schedule_code_upgrade {
                        para: runtime_types::polkadot_parachain::primitives::Id,
                        new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                    },
                    #[codec(index = 8)]
                    #[doc = "See [`Pallet::set_current_head`]."]
                    set_current_head {
                        para: runtime_types::polkadot_parachain::primitives::Id,
                        new_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The ID is not registered."]
                    NotRegistered,
                    #[codec(index = 1)]
                    #[doc = "The ID is already registered."]
                    AlreadyRegistered,
                    #[codec(index = 2)]
                    #[doc = "The caller is not the owner of this Id."]
                    NotOwner,
                    #[codec(index = 3)]
                    #[doc = "Invalid para code size."]
                    CodeTooLarge,
                    #[codec(index = 4)]
                    #[doc = "Invalid para head data size."]
                    HeadDataTooLarge,
                    #[codec(index = 5)]
                    #[doc = "Para is not a Parachain."]
                    NotParachain,
                    #[codec(index = 6)]
                    #[doc = "Para is not a Parathread."]
                    NotParathread,
                    #[codec(index = 7)]
                    #[doc = "Cannot deregister para"]
                    CannotDeregister,
                    #[codec(index = 8)]
                    #[doc = "Cannot schedule downgrade of parachain to parathread"]
                    CannotDowngrade,
                    #[codec(index = 9)]
                    #[doc = "Cannot schedule upgrade of parathread to parachain"]
                    CannotUpgrade,
                    #[codec(index = 10)]
                    #[doc = "Para is locked from manipulation by the manager. Must use parachain or relay chain governance."]
                    ParaLocked,
                    #[codec(index = 11)]
                    #[doc = "The ID given for registration has not been reserved."]
                    NotReserved,
                    #[codec(index = 12)]
                    #[doc = "Registering parachain with empty code is not allowed."]
                    EmptyCode,
                    #[codec(index = 13)]
                    #[doc = "Cannot perform a parachain slot / lifecycle swap. Check that the state of both paras are"]
                    #[doc = "correct for the swap to work."]
                    CannotSwap,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    Registered {
                        para_id: runtime_types::polkadot_parachain::primitives::Id,
                        manager: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 1)]
                    Deregistered {
                        para_id: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 2)]
                    Reserved {
                        para_id: runtime_types::polkadot_parachain::primitives::Id,
                        who: ::subxt::utils::AccountId32,
                    },
                    #[codec(index = 3)]
                    Swapped {
                        para_id: runtime_types::polkadot_parachain::primitives::Id,
                        other_id: runtime_types::polkadot_parachain::primitives::Id,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ParaInfo<_0, _1> {
                pub manager: _0,
                pub deposit: _1,
                pub locked: ::core::primitive::bool,
            }
        }
        pub mod slots {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::force_lease`]."]
                    force_lease {
                        para: runtime_types::polkadot_parachain::primitives::Id,
                        leaser: ::subxt::utils::AccountId32,
                        amount: ::core::primitive::u128,
                        period_begin: ::core::primitive::u32,
                        period_count: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::clear_all_leases`]."]
                    clear_all_leases {
                        para: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::trigger_onboard`]."]
                    trigger_onboard {
                        para: runtime_types::polkadot_parachain::primitives::Id,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The parachain ID is not onboarding."]
                    ParaNotOnboarding,
                    #[codec(index = 1)]
                    #[doc = "There was an error with the lease."]
                    LeaseError,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A new `[lease_period]` is beginning."]
                    NewLeasePeriod {
                        lease_period: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "A para has won the right to a continuous set of lease periods as a parachain."]
                    #[doc = "First balance is any extra amount reserved on top of the para's existing deposit."]
                    #[doc = "Second balance is the total amount reserved."]
                    Leased {
                        para_id: runtime_types::polkadot_parachain::primitives::Id,
                        leaser: ::subxt::utils::AccountId32,
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
        use super::runtime_types;
        pub mod configuration {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::set_validation_upgrade_cooldown`]."]
                    set_validation_upgrade_cooldown { new: ::core::primitive::u32 },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::set_validation_upgrade_delay`]."]
                    set_validation_upgrade_delay { new: ::core::primitive::u32 },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::set_code_retention_period`]."]
                    set_code_retention_period { new: ::core::primitive::u32 },
                    #[codec(index = 3)]
                    #[doc = "See [`Pallet::set_max_code_size`]."]
                    set_max_code_size { new: ::core::primitive::u32 },
                    #[codec(index = 4)]
                    #[doc = "See [`Pallet::set_max_pov_size`]."]
                    set_max_pov_size { new: ::core::primitive::u32 },
                    #[codec(index = 5)]
                    #[doc = "See [`Pallet::set_max_head_data_size`]."]
                    set_max_head_data_size { new: ::core::primitive::u32 },
                    #[codec(index = 6)]
                    #[doc = "See [`Pallet::set_parathread_cores`]."]
                    set_parathread_cores { new: ::core::primitive::u32 },
                    #[codec(index = 7)]
                    #[doc = "See [`Pallet::set_parathread_retries`]."]
                    set_parathread_retries { new: ::core::primitive::u32 },
                    #[codec(index = 8)]
                    #[doc = "See [`Pallet::set_group_rotation_frequency`]."]
                    set_group_rotation_frequency { new: ::core::primitive::u32 },
                    #[codec(index = 9)]
                    #[doc = "See [`Pallet::set_chain_availability_period`]."]
                    set_chain_availability_period { new: ::core::primitive::u32 },
                    #[codec(index = 10)]
                    #[doc = "See [`Pallet::set_thread_availability_period`]."]
                    set_thread_availability_period { new: ::core::primitive::u32 },
                    #[codec(index = 11)]
                    #[doc = "See [`Pallet::set_scheduling_lookahead`]."]
                    set_scheduling_lookahead { new: ::core::primitive::u32 },
                    #[codec(index = 12)]
                    #[doc = "See [`Pallet::set_max_validators_per_core`]."]
                    set_max_validators_per_core {
                        new: ::core::option::Option<::core::primitive::u32>,
                    },
                    #[codec(index = 13)]
                    #[doc = "See [`Pallet::set_max_validators`]."]
                    set_max_validators {
                        new: ::core::option::Option<::core::primitive::u32>,
                    },
                    #[codec(index = 14)]
                    #[doc = "See [`Pallet::set_dispute_period`]."]
                    set_dispute_period { new: ::core::primitive::u32 },
                    #[codec(index = 15)]
                    #[doc = "See [`Pallet::set_dispute_post_conclusion_acceptance_period`]."]
                    set_dispute_post_conclusion_acceptance_period { new: ::core::primitive::u32 },
                    #[codec(index = 18)]
                    #[doc = "See [`Pallet::set_no_show_slots`]."]
                    set_no_show_slots { new: ::core::primitive::u32 },
                    #[codec(index = 19)]
                    #[doc = "See [`Pallet::set_n_delay_tranches`]."]
                    set_n_delay_tranches { new: ::core::primitive::u32 },
                    #[codec(index = 20)]
                    #[doc = "See [`Pallet::set_zeroth_delay_tranche_width`]."]
                    set_zeroth_delay_tranche_width { new: ::core::primitive::u32 },
                    #[codec(index = 21)]
                    #[doc = "See [`Pallet::set_needed_approvals`]."]
                    set_needed_approvals { new: ::core::primitive::u32 },
                    #[codec(index = 22)]
                    #[doc = "See [`Pallet::set_relay_vrf_modulo_samples`]."]
                    set_relay_vrf_modulo_samples { new: ::core::primitive::u32 },
                    #[codec(index = 23)]
                    #[doc = "See [`Pallet::set_max_upward_queue_count`]."]
                    set_max_upward_queue_count { new: ::core::primitive::u32 },
                    #[codec(index = 24)]
                    #[doc = "See [`Pallet::set_max_upward_queue_size`]."]
                    set_max_upward_queue_size { new: ::core::primitive::u32 },
                    #[codec(index = 25)]
                    #[doc = "See [`Pallet::set_max_downward_message_size`]."]
                    set_max_downward_message_size { new: ::core::primitive::u32 },
                    #[codec(index = 27)]
                    #[doc = "See [`Pallet::set_max_upward_message_size`]."]
                    set_max_upward_message_size { new: ::core::primitive::u32 },
                    #[codec(index = 28)]
                    #[doc = "See [`Pallet::set_max_upward_message_num_per_candidate`]."]
                    set_max_upward_message_num_per_candidate { new: ::core::primitive::u32 },
                    #[codec(index = 29)]
                    #[doc = "See [`Pallet::set_hrmp_open_request_ttl`]."]
                    set_hrmp_open_request_ttl { new: ::core::primitive::u32 },
                    #[codec(index = 30)]
                    #[doc = "See [`Pallet::set_hrmp_sender_deposit`]."]
                    set_hrmp_sender_deposit { new: ::core::primitive::u128 },
                    #[codec(index = 31)]
                    #[doc = "See [`Pallet::set_hrmp_recipient_deposit`]."]
                    set_hrmp_recipient_deposit { new: ::core::primitive::u128 },
                    #[codec(index = 32)]
                    #[doc = "See [`Pallet::set_hrmp_channel_max_capacity`]."]
                    set_hrmp_channel_max_capacity { new: ::core::primitive::u32 },
                    #[codec(index = 33)]
                    #[doc = "See [`Pallet::set_hrmp_channel_max_total_size`]."]
                    set_hrmp_channel_max_total_size { new: ::core::primitive::u32 },
                    #[codec(index = 34)]
                    #[doc = "See [`Pallet::set_hrmp_max_parachain_inbound_channels`]."]
                    set_hrmp_max_parachain_inbound_channels { new: ::core::primitive::u32 },
                    #[codec(index = 35)]
                    #[doc = "See [`Pallet::set_hrmp_max_parathread_inbound_channels`]."]
                    set_hrmp_max_parathread_inbound_channels { new: ::core::primitive::u32 },
                    #[codec(index = 36)]
                    #[doc = "See [`Pallet::set_hrmp_channel_max_message_size`]."]
                    set_hrmp_channel_max_message_size { new: ::core::primitive::u32 },
                    #[codec(index = 37)]
                    #[doc = "See [`Pallet::set_hrmp_max_parachain_outbound_channels`]."]
                    set_hrmp_max_parachain_outbound_channels { new: ::core::primitive::u32 },
                    #[codec(index = 38)]
                    #[doc = "See [`Pallet::set_hrmp_max_parathread_outbound_channels`]."]
                    set_hrmp_max_parathread_outbound_channels { new: ::core::primitive::u32 },
                    #[codec(index = 39)]
                    #[doc = "See [`Pallet::set_hrmp_max_message_num_per_candidate`]."]
                    set_hrmp_max_message_num_per_candidate { new: ::core::primitive::u32 },
                    #[codec(index = 41)]
                    #[doc = "See [`Pallet::set_pvf_checking_enabled`]."]
                    set_pvf_checking_enabled { new: ::core::primitive::bool },
                    #[codec(index = 42)]
                    #[doc = "See [`Pallet::set_pvf_voting_ttl`]."]
                    set_pvf_voting_ttl { new: ::core::primitive::u32 },
                    #[codec(index = 43)]
                    #[doc = "See [`Pallet::set_minimum_validation_upgrade_delay`]."]
                    set_minimum_validation_upgrade_delay { new: ::core::primitive::u32 },
                    #[codec(index = 44)]
                    #[doc = "See [`Pallet::set_bypass_consistency_check`]."]
                    set_bypass_consistency_check { new: ::core::primitive::bool },
                    #[codec(index = 45)]
                    #[doc = "See [`Pallet::set_async_backing_params`]."]
                    set_async_backing_params {
                        new: runtime_types::polkadot_primitives::vstaging::AsyncBackingParams,
                    },
                    #[codec(index = 46)]
                    #[doc = "See [`Pallet::set_executor_params`]."]
                    set_executor_params {
                        new:
                            runtime_types::polkadot_primitives::v5::executor_params::ExecutorParams,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The new value for a configuration parameter is invalid."]
                    InvalidNewValue,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
                pub async_backing_params:
                    runtime_types::polkadot_primitives::vstaging::AsyncBackingParams,
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
                pub executor_params:
                    runtime_types::polkadot_primitives::v5::executor_params::ExecutorParams,
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
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::force_unfreeze`]."]
                    force_unfreeze,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Duplicate dispute statement sets provided."]
                    DuplicateDisputeStatementSets,
                    #[codec(index = 1)]
                    #[doc = "Ancient dispute statement provided."]
                    AncientDisputeStatement,
                    #[codec(index = 2)]
                    #[doc = "Validator index on statement is out of bounds for session."]
                    ValidatorIndexOutOfBounds,
                    #[codec(index = 3)]
                    #[doc = "Invalid signature on statement."]
                    InvalidSignature,
                    #[codec(index = 4)]
                    #[doc = "Validator vote submitted more than once to dispute."]
                    DuplicateStatement,
                    #[codec(index = 5)]
                    #[doc = "A dispute where there are only votes on one side."]
                    SingleSidedDispute,
                    #[codec(index = 6)]
                    #[doc = "A dispute vote from a malicious backer."]
                    MaliciousBacker,
                    #[codec(index = 7)]
                    #[doc = "No backing votes were provides along dispute statements."]
                    MissingBackingVotes,
                    #[codec(index = 8)]
                    #[doc = "Unconfirmed dispute statement sets provided."]
                    UnconfirmedDispute,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A dispute has been initiated. \\[candidate hash, dispute location\\]"]
                    DisputeInitiated(
                        runtime_types::polkadot_core_primitives::CandidateHash,
                        runtime_types::polkadot_runtime_parachains::disputes::DisputeLocation,
                    ),
                    #[codec(index = 1)]
                    #[doc = "A dispute has concluded for or against a candidate."]
                    #[doc = "`\\[para id, candidate hash, dispute result\\]`"]
                    DisputeConcluded(
                        runtime_types::polkadot_core_primitives::CandidateHash,
                        runtime_types::polkadot_runtime_parachains::disputes::DisputeResult,
                    ),
                    #[codec(index = 2)]
                    #[doc = "A dispute has concluded with supermajority against a candidate."]
                    #[doc = "Block authors should no longer build on top of this head and should"]
                    #[doc = "instead revert the block at the given height. This should be the"]
                    #[doc = "number of the child of the last known valid block in the chain."]
                    Revert(::core::primitive::u32),
                }
            }
            pub mod slashing {
                use super::runtime_types;
                pub mod pallet {
                    use super::runtime_types;
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                    pub enum Call {
                        #[codec(index = 0)]
                        #[doc = "See [`Pallet::report_dispute_lost_unsigned`]."]
                        report_dispute_lost_unsigned {
                            dispute_proof: ::std::boxed::Box<
                                runtime_types::polkadot_primitives::v5::slashing::DisputeProof,
                            >,
                            key_owner_proof: runtime_types::sp_session::MembershipProof,
                        },
                    }
                    #[derive(
                        :: subxt :: ext :: codec :: Decode,
                        :: subxt :: ext :: codec :: Encode,
                        :: subxt :: ext :: scale_decode :: DecodeAsType,
                        :: subxt :: ext :: scale_encode :: EncodeAsType,
                        Debug,
                    )]
                    # [codec (crate = :: subxt :: ext :: codec)]
                    #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                    #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                    #[doc = "The `Error` enum of this pallet."]
                    pub enum Error {
                        #[codec(index = 0)]
                        #[doc = "The key ownership proof is invalid."]
                        InvalidKeyOwnershipProof,
                        #[codec(index = 1)]
                        #[doc = "The session index is too old or invalid."]
                        InvalidSessionIndex,
                        #[codec(index = 2)]
                        #[doc = "The candidate hash is invalid."]
                        InvalidCandidateHash,
                        #[codec(index = 3)]
                        #[doc = "There is no pending slash for the given validator index and time"]
                        #[doc = "slot."]
                        InvalidValidatorIndex,
                        #[codec(index = 4)]
                        #[doc = "The validator index does not match the validator id."]
                        ValidatorIndexIdMismatch,
                        #[codec(index = 5)]
                        #[doc = "The given slashing report is valid but already previously reported."]
                        DuplicateSlashingReport,
                    }
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum DisputeLocation {
                #[codec(index = 0)]
                Local,
                #[codec(index = 1)]
                Remote,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum DisputeResult {
                #[codec(index = 0)]
                Valid,
                #[codec(index = 1)]
                Invalid,
            }
        }
        pub mod hrmp {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::hrmp_init_open_channel`]."]
                    hrmp_init_open_channel {
                        recipient: runtime_types::polkadot_parachain::primitives::Id,
                        proposed_max_capacity: ::core::primitive::u32,
                        proposed_max_message_size: ::core::primitive::u32,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::hrmp_accept_open_channel`]."]
                    hrmp_accept_open_channel {
                        sender: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::hrmp_close_channel`]."]
                    hrmp_close_channel {
                        channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                    },
                    #[codec(index = 3)]
                    #[doc = "See [`Pallet::force_clean_hrmp`]."]
                    force_clean_hrmp {
                        para: runtime_types::polkadot_parachain::primitives::Id,
                        inbound: ::core::primitive::u32,
                        outbound: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    #[doc = "See [`Pallet::force_process_hrmp_open`]."]
                    force_process_hrmp_open { channels: ::core::primitive::u32 },
                    #[codec(index = 5)]
                    #[doc = "See [`Pallet::force_process_hrmp_close`]."]
                    force_process_hrmp_close { channels: ::core::primitive::u32 },
                    #[codec(index = 6)]
                    #[doc = "See [`Pallet::hrmp_cancel_open_request`]."]
                    hrmp_cancel_open_request {
                        channel_id: runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                        open_requests: ::core::primitive::u32,
                    },
                    #[codec(index = 7)]
                    #[doc = "See [`Pallet::force_open_hrmp_channel`]."]
                    force_open_hrmp_channel {
                        sender: runtime_types::polkadot_parachain::primitives::Id,
                        recipient: runtime_types::polkadot_parachain::primitives::Id,
                        max_capacity: ::core::primitive::u32,
                        max_message_size: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "The sender tried to open a channel to themselves."]
                    OpenHrmpChannelToSelf,
                    #[codec(index = 1)]
                    #[doc = "The recipient is not a valid para."]
                    OpenHrmpChannelInvalidRecipient,
                    #[codec(index = 2)]
                    #[doc = "The requested capacity is zero."]
                    OpenHrmpChannelZeroCapacity,
                    #[codec(index = 3)]
                    #[doc = "The requested capacity exceeds the global limit."]
                    OpenHrmpChannelCapacityExceedsLimit,
                    #[codec(index = 4)]
                    #[doc = "The requested maximum message size is 0."]
                    OpenHrmpChannelZeroMessageSize,
                    #[codec(index = 5)]
                    #[doc = "The open request requested the message size that exceeds the global limit."]
                    OpenHrmpChannelMessageSizeExceedsLimit,
                    #[codec(index = 6)]
                    #[doc = "The channel already exists"]
                    OpenHrmpChannelAlreadyExists,
                    #[codec(index = 7)]
                    #[doc = "There is already a request to open the same channel."]
                    OpenHrmpChannelAlreadyRequested,
                    #[codec(index = 8)]
                    #[doc = "The sender already has the maximum number of allowed outbound channels."]
                    OpenHrmpChannelLimitExceeded,
                    #[codec(index = 9)]
                    #[doc = "The channel from the sender to the origin doesn't exist."]
                    AcceptHrmpChannelDoesntExist,
                    #[codec(index = 10)]
                    #[doc = "The channel is already confirmed."]
                    AcceptHrmpChannelAlreadyConfirmed,
                    #[codec(index = 11)]
                    #[doc = "The recipient already has the maximum number of allowed inbound channels."]
                    AcceptHrmpChannelLimitExceeded,
                    #[codec(index = 12)]
                    #[doc = "The origin tries to close a channel where it is neither the sender nor the recipient."]
                    CloseHrmpChannelUnauthorized,
                    #[codec(index = 13)]
                    #[doc = "The channel to be closed doesn't exist."]
                    CloseHrmpChannelDoesntExist,
                    #[codec(index = 14)]
                    #[doc = "The channel close request is already requested."]
                    CloseHrmpChannelAlreadyUnderway,
                    #[codec(index = 15)]
                    #[doc = "Canceling is requested by neither the sender nor recipient of the open channel request."]
                    CancelHrmpOpenChannelUnauthorized,
                    #[codec(index = 16)]
                    #[doc = "The open request doesn't exist."]
                    OpenHrmpChannelDoesntExist,
                    #[codec(index = 17)]
                    #[doc = "Cannot cancel an HRMP open channel request because it is already confirmed."]
                    OpenHrmpChannelAlreadyConfirmed,
                    #[codec(index = 18)]
                    #[doc = "The provided witness data is wrong."]
                    WrongWitness,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Open HRMP channel requested."]
                    #[doc = "`[sender, recipient, proposed_max_capacity, proposed_max_message_size]`"]
                    OpenChannelRequested(
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 1)]
                    #[doc = "An HRMP channel request sent by the receiver was canceled by either party."]
                    #[doc = "`[by_parachain, channel_id]`"]
                    OpenChannelCanceled(
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                    ),
                    #[codec(index = 2)]
                    #[doc = "Open HRMP channel accepted. `[sender, recipient]`"]
                    OpenChannelAccepted(
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::polkadot_parachain::primitives::Id,
                    ),
                    #[codec(index = 3)]
                    #[doc = "HRMP channel closed. `[by_parachain, channel_id]`"]
                    ChannelClosed(
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::polkadot_parachain::primitives::HrmpChannelId,
                    ),
                    #[codec(index = 4)]
                    #[doc = "An HRMP channel was opened via Root origin."]
                    #[doc = "`[sender, recipient, proposed_max_capacity, proposed_max_message_size]`"]
                    HrmpChannelForceOpened(
                        runtime_types::polkadot_parachain::primitives::Id,
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                        ::core::primitive::u32,
                    ),
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct HrmpChannel {
                pub max_capacity: ::core::primitive::u32,
                pub max_total_size: ::core::primitive::u32,
                pub max_message_size: ::core::primitive::u32,
                pub msg_count: ::core::primitive::u32,
                pub total_size: ::core::primitive::u32,
                pub mqc_head: ::core::option::Option<::subxt::utils::H256>,
                pub sender_deposit: ::core::primitive::u128,
                pub recipient_deposit: ::core::primitive::u128,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
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
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {}
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Validator indices are out of order or contains duplicates."]
                    UnsortedOrDuplicateValidatorIndices,
                    #[codec(index = 1)]
                    #[doc = "Dispute statement sets are out of order or contain duplicates."]
                    UnsortedOrDuplicateDisputeStatementSet,
                    #[codec(index = 2)]
                    #[doc = "Backed candidates are out of order (core index) or contain duplicates."]
                    UnsortedOrDuplicateBackedCandidates,
                    #[codec(index = 3)]
                    #[doc = "A different relay parent was provided compared to the on-chain stored one."]
                    UnexpectedRelayParent,
                    #[codec(index = 4)]
                    #[doc = "Availability bitfield has unexpected size."]
                    WrongBitfieldSize,
                    #[codec(index = 5)]
                    #[doc = "Bitfield consists of zeros only."]
                    BitfieldAllZeros,
                    #[codec(index = 6)]
                    #[doc = "Multiple bitfields submitted by same validator or validators out of order by index."]
                    BitfieldDuplicateOrUnordered,
                    #[codec(index = 7)]
                    #[doc = "Validator index out of bounds."]
                    ValidatorIndexOutOfBounds,
                    #[codec(index = 8)]
                    #[doc = "Invalid signature"]
                    InvalidBitfieldSignature,
                    #[codec(index = 9)]
                    #[doc = "Candidate submitted but para not scheduled."]
                    UnscheduledCandidate,
                    #[codec(index = 10)]
                    #[doc = "Candidate scheduled despite pending candidate already existing for the para."]
                    CandidateScheduledBeforeParaFree,
                    #[codec(index = 11)]
                    #[doc = "Candidate included with the wrong collator."]
                    WrongCollator,
                    #[codec(index = 12)]
                    #[doc = "Scheduled cores out of order."]
                    ScheduledOutOfOrder,
                    #[codec(index = 13)]
                    #[doc = "Head data exceeds the configured maximum."]
                    HeadDataTooLarge,
                    #[codec(index = 14)]
                    #[doc = "Code upgrade prematurely."]
                    PrematureCodeUpgrade,
                    #[codec(index = 15)]
                    #[doc = "Output code is too large"]
                    NewCodeTooLarge,
                    #[codec(index = 16)]
                    #[doc = "Candidate not in parent context."]
                    CandidateNotInParentContext,
                    #[codec(index = 17)]
                    #[doc = "Invalid group index in core assignment."]
                    InvalidGroupIndex,
                    #[codec(index = 18)]
                    #[doc = "Insufficient (non-majority) backing."]
                    InsufficientBacking,
                    #[codec(index = 19)]
                    #[doc = "Invalid (bad signature, unknown validator, etc.) backing."]
                    InvalidBacking,
                    #[codec(index = 20)]
                    #[doc = "Collator did not sign PoV."]
                    NotCollatorSigned,
                    #[codec(index = 21)]
                    #[doc = "The validation data hash does not match expected."]
                    ValidationDataHashMismatch,
                    #[codec(index = 22)]
                    #[doc = "The downward message queue is not processed correctly."]
                    IncorrectDownwardMessageHandling,
                    #[codec(index = 23)]
                    #[doc = "At least one upward message sent does not pass the acceptance criteria."]
                    InvalidUpwardMessages,
                    #[codec(index = 24)]
                    #[doc = "The candidate didn't follow the rules of HRMP watermark advancement."]
                    HrmpWatermarkMishandling,
                    #[codec(index = 25)]
                    #[doc = "The HRMP messages sent by the candidate is not valid."]
                    InvalidOutboundHrmp,
                    #[codec(index = 26)]
                    #[doc = "The validation code hash of the candidate is not valid."]
                    InvalidValidationCodeHash,
                    #[codec(index = 27)]
                    #[doc = "The `para_head` hash in the candidate descriptor doesn't match the hash of the actual para head in the"]
                    #[doc = "commitments."]
                    ParaHeadMismatch,
                    #[codec(index = 28)]
                    #[doc = "A bitfield that references a freed core,"]
                    #[doc = "either intentionally or as part of a concluded"]
                    #[doc = "invalid dispute."]
                    BitfieldReferencesFreedCore,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "A candidate was backed. `[candidate, head_data]`"]
                    CandidateBacked(
                        runtime_types::polkadot_primitives::v5::CandidateReceipt<
                            ::subxt::utils::H256,
                        >,
                        runtime_types::polkadot_parachain::primitives::HeadData,
                        runtime_types::polkadot_primitives::v5::CoreIndex,
                        runtime_types::polkadot_primitives::v5::GroupIndex,
                    ),
                    #[codec(index = 1)]
                    #[doc = "A candidate was included. `[candidate, head_data]`"]
                    CandidateIncluded(
                        runtime_types::polkadot_primitives::v5::CandidateReceipt<
                            ::subxt::utils::H256,
                        >,
                        runtime_types::polkadot_parachain::primitives::HeadData,
                        runtime_types::polkadot_primitives::v5::CoreIndex,
                        runtime_types::polkadot_primitives::v5::GroupIndex,
                    ),
                    #[codec(index = 2)]
                    #[doc = "A candidate timed out. `[candidate, head_data]`"]
                    CandidateTimedOut(
                        runtime_types::polkadot_primitives::v5::CandidateReceipt<
                            ::subxt::utils::H256,
                        >,
                        runtime_types::polkadot_parachain::primitives::HeadData,
                        runtime_types::polkadot_primitives::v5::CoreIndex,
                    ),
                    #[codec(index = 3)]
                    #[doc = "Some upward messages have been received and will be processed."]
                    UpwardMessagesReceived {
                        from: runtime_types::polkadot_parachain::primitives::Id,
                        count: ::core::primitive::u32,
                    },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum AggregateMessageOrigin {
                #[codec(index = 0)]
                Ump(runtime_types::polkadot_runtime_parachains::inclusion::UmpQueueId),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct AvailabilityBitfieldRecord<_0> {
                pub bitfield: runtime_types::polkadot_primitives::v5::AvailabilityBitfield,
                pub submitted_at: _0,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CandidatePendingAvailability<_0, _1> {
                pub core: runtime_types::polkadot_primitives::v5::CoreIndex,
                pub hash: runtime_types::polkadot_core_primitives::CandidateHash,
                pub descriptor: runtime_types::polkadot_primitives::v5::CandidateDescriptor<_0>,
                pub availability_votes: ::subxt::utils::bits::DecodedBits<
                    ::core::primitive::u8,
                    ::subxt::utils::bits::Lsb0,
                >,
                pub backers: ::subxt::utils::bits::DecodedBits<
                    ::core::primitive::u8,
                    ::subxt::utils::bits::Lsb0,
                >,
                pub relay_parent_number: _1,
                pub backed_in_number: _1,
                pub backing_group: runtime_types::polkadot_primitives::v5::GroupIndex,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum UmpQueueId {
                #[codec(index = 0)]
                Para(runtime_types::polkadot_parachain::primitives::Id),
            }
        }
        pub mod initializer {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::force_approve`]."]
                    force_approve { up_to: ::core::primitive::u32 },
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct BufferedSessionChange {
                pub validators:
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v5::validator_app::Public>,
                pub queued:
                    ::std::vec::Vec<runtime_types::polkadot_primitives::v5::validator_app::Public>,
                pub session_index: ::core::primitive::u32,
            }
        }
        pub mod origin {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Origin {
                    #[codec(index = 0)]
                    Parachain(runtime_types::polkadot_parachain::primitives::Id),
                }
            }
        }
        pub mod paras {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::force_set_current_code`]."]
                    force_set_current_code {
                        para: runtime_types::polkadot_parachain::primitives::Id,
                        new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                    },
                    #[codec(index = 1)]
                    #[doc = "See [`Pallet::force_set_current_head`]."]
                    force_set_current_head {
                        para: runtime_types::polkadot_parachain::primitives::Id,
                        new_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    },
                    #[codec(index = 2)]
                    #[doc = "See [`Pallet::force_schedule_code_upgrade`]."]
                    force_schedule_code_upgrade {
                        para: runtime_types::polkadot_parachain::primitives::Id,
                        new_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                        relay_parent_number: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    #[doc = "See [`Pallet::force_note_new_head`]."]
                    force_note_new_head {
                        para: runtime_types::polkadot_parachain::primitives::Id,
                        new_head: runtime_types::polkadot_parachain::primitives::HeadData,
                    },
                    #[codec(index = 4)]
                    #[doc = "See [`Pallet::force_queue_action`]."]
                    force_queue_action {
                        para: runtime_types::polkadot_parachain::primitives::Id,
                    },
                    #[codec(index = 5)]
                    #[doc = "See [`Pallet::add_trusted_validation_code`]."]
                    add_trusted_validation_code {
                        validation_code:
                            runtime_types::polkadot_parachain::primitives::ValidationCode,
                    },
                    #[codec(index = 6)]
                    #[doc = "See [`Pallet::poke_unused_validation_code`]."]
                    poke_unused_validation_code {
                        validation_code_hash:
                            runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                    },
                    #[codec(index = 7)]
                    #[doc = "See [`Pallet::include_pvf_check_statement`]."]
                    include_pvf_check_statement {
                        stmt: runtime_types::polkadot_primitives::v5::PvfCheckStatement,
                        signature: runtime_types::polkadot_primitives::v5::validator_app::Signature,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Para is not registered in our system."]
                    NotRegistered,
                    #[codec(index = 1)]
                    #[doc = "Para cannot be onboarded because it is already tracked by our system."]
                    CannotOnboard,
                    #[codec(index = 2)]
                    #[doc = "Para cannot be offboarded at this time."]
                    CannotOffboard,
                    #[codec(index = 3)]
                    #[doc = "Para cannot be upgraded to a parachain."]
                    CannotUpgrade,
                    #[codec(index = 4)]
                    #[doc = "Para cannot be downgraded to a parathread."]
                    CannotDowngrade,
                    #[codec(index = 5)]
                    #[doc = "The statement for PVF pre-checking is stale."]
                    PvfCheckStatementStale,
                    #[codec(index = 6)]
                    #[doc = "The statement for PVF pre-checking is for a future session."]
                    PvfCheckStatementFuture,
                    #[codec(index = 7)]
                    #[doc = "Claimed validator index is out of bounds."]
                    PvfCheckValidatorIndexOutOfBounds,
                    #[codec(index = 8)]
                    #[doc = "The signature for the PVF pre-checking is invalid."]
                    PvfCheckInvalidSignature,
                    #[codec(index = 9)]
                    #[doc = "The given validator already has cast a vote."]
                    PvfCheckDoubleVote,
                    #[codec(index = 10)]
                    #[doc = "The given PVF does not exist at the moment of process a vote."]
                    PvfCheckSubjectInvalid,
                    #[codec(index = 11)]
                    #[doc = "Parachain cannot currently schedule a code upgrade."]
                    CannotUpgradeCode,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Event` enum of this pallet"]
                pub enum Event {
                    #[codec(index = 0)]
                    #[doc = "Current code has been updated for a Para. `para_id`"]
                    CurrentCodeUpdated(runtime_types::polkadot_parachain::primitives::Id),
                    #[codec(index = 1)]
                    #[doc = "Current head has been updated for a Para. `para_id`"]
                    CurrentHeadUpdated(runtime_types::polkadot_parachain::primitives::Id),
                    #[codec(index = 2)]
                    #[doc = "A code upgrade has been scheduled for a Para. `para_id`"]
                    CodeUpgradeScheduled(runtime_types::polkadot_parachain::primitives::Id),
                    #[codec(index = 3)]
                    #[doc = "A new head has been noted for a Para. `para_id`"]
                    NewHeadNoted(runtime_types::polkadot_parachain::primitives::Id),
                    #[codec(index = 4)]
                    #[doc = "A para has been queued to execute pending actions. `para_id`"]
                    ActionQueued(
                        runtime_types::polkadot_parachain::primitives::Id,
                        ::core::primitive::u32,
                    ),
                    #[codec(index = 5)]
                    #[doc = "The given para either initiated or subscribed to a PVF check for the given validation"]
                    #[doc = "code. `code_hash` `para_id`"]
                    PvfCheckStarted(
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                        runtime_types::polkadot_parachain::primitives::Id,
                    ),
                    #[codec(index = 6)]
                    #[doc = "The given validation code was accepted by the PVF pre-checking vote."]
                    #[doc = "`code_hash` `para_id`"]
                    PvfCheckAccepted(
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                        runtime_types::polkadot_parachain::primitives::Id,
                    ),
                    #[codec(index = 7)]
                    #[doc = "The given validation code was rejected by the PVF pre-checking vote."]
                    #[doc = "`code_hash` `para_id`"]
                    PvfCheckRejected(
                        runtime_types::polkadot_parachain::primitives::ValidationCodeHash,
                        runtime_types::polkadot_parachain::primitives::Id,
                    ),
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ParaGenesisArgs {
                pub genesis_head: runtime_types::polkadot_parachain::primitives::HeadData,
                pub validation_code: runtime_types::polkadot_parachain::primitives::ValidationCode,
                pub para_kind: ::core::primitive::bool,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum ParaLifecycle {
                #[codec(index = 0)]
                Onboarding,
                #[codec(index = 1)]
                Parathread,
                #[codec(index = 2)]
                Parachain,
                #[codec(index = 3)]
                UpgradingParathread,
                #[codec(index = 4)]
                DowngradingParachain,
                #[codec(index = 5)]
                OffboardingParathread,
                #[codec(index = 6)]
                OffboardingParachain,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ParaPastCodeMeta<_0> {
                pub upgrade_times: ::std::vec::Vec<
                    runtime_types::polkadot_runtime_parachains::paras::ReplacementTimes<_0>,
                >,
                pub last_pruned: ::core::option::Option<_0>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PvfCheckActiveVoteState<_0> {
                pub votes_accept: ::subxt::utils::bits::DecodedBits<
                    ::core::primitive::u8,
                    ::subxt::utils::bits::Lsb0,
                >,
                pub votes_reject: ::subxt::utils::bits::DecodedBits<
                    ::core::primitive::u8,
                    ::subxt::utils::bits::Lsb0,
                >,
                pub age: ::core::primitive::u32,
                pub created_at: _0,
                pub causes: ::std::vec::Vec<
                    runtime_types::polkadot_runtime_parachains::paras::PvfCheckCause<_0>,
                >,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum PvfCheckCause<_0> {
                #[codec(index = 0)]
                Onboarding(runtime_types::polkadot_parachain::primitives::Id),
                #[codec(index = 1)]
                Upgrade {
                    id: runtime_types::polkadot_parachain::primitives::Id,
                    relay_parent_number: _0,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ReplacementTimes<_0> {
                pub expected_at: _0,
                pub activated_at: _0,
            }
        }
        pub mod paras_inherent {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {
                    #[codec(index = 0)]
                    #[doc = "See [`Pallet::enter`]."]
                    enter {
                        data: runtime_types::polkadot_primitives::v5::InherentData<
                            runtime_types::sp_runtime::generic::header::Header<
                                ::core::primitive::u32,
                                runtime_types::sp_runtime::traits::BlakeTwo256,
                            >,
                        >,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "The `Error` enum of this pallet."]
                pub enum Error {
                    #[codec(index = 0)]
                    #[doc = "Inclusion inherent called more than once per block."]
                    TooManyInclusionInherents,
                    #[codec(index = 1)]
                    #[doc = "The hash of the submitted parent header doesn't correspond to the saved block hash of"]
                    #[doc = "the parent."]
                    InvalidParentHeader,
                    #[codec(index = 2)]
                    #[doc = "Disputed candidate that was concluded invalid."]
                    CandidateConcludedInvalid,
                    #[codec(index = 3)]
                    #[doc = "The data given to the inherent will result in an overweight block."]
                    InherentOverweight,
                    #[codec(index = 4)]
                    #[doc = "The ordering of dispute statements was invalid."]
                    DisputeStatementsUnsortedOrDuplicates,
                    #[codec(index = 5)]
                    #[doc = "A dispute statement was invalid."]
                    DisputeInvalid,
                }
            }
        }
        pub mod scheduler {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum AssignmentKind {
                #[codec(index = 0)]
                Parachain,
                #[codec(index = 1)]
                Parathread(
                    runtime_types::polkadot_primitives::v5::collator_app::Public,
                    ::core::primitive::u32,
                ),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct CoreAssignment {
                pub core: runtime_types::polkadot_primitives::v5::CoreIndex,
                pub para_id: runtime_types::polkadot_parachain::primitives::Id,
                pub kind: runtime_types::polkadot_runtime_parachains::scheduler::AssignmentKind,
                pub group_idx: runtime_types::polkadot_primitives::v5::GroupIndex,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ParathreadClaimQueue {
                pub queue: ::std::vec::Vec<
                    runtime_types::polkadot_runtime_parachains::scheduler::QueuedParathread,
                >,
                pub next_core_offset: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct QueuedParathread {
                pub claim: runtime_types::polkadot_primitives::v5::ParathreadEntry,
                pub core_offset: ::core::primitive::u32,
            }
        }
        pub mod shared {
            use super::runtime_types;
            pub mod pallet {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                #[doc = "Contains a variant per dispatchable extrinsic that this pallet has."]
                pub enum Call {}
            }
        }
    }
    pub mod sp_arithmetic {
        use super::runtime_types;
        pub mod fixed_point {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct FixedI64(pub ::core::primitive::i64);
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct FixedU128(pub ::core::primitive::u128);
        }
        pub mod per_things {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PerU16(pub ::core::primitive::u16);
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Perbill(pub ::core::primitive::u32);
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Percent(pub ::core::primitive::u8);
            #[derive(
                :: subxt :: ext :: codec :: CompactAs,
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Permill(pub ::core::primitive::u32);
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum ArithmeticError {
            #[codec(index = 0)]
            Underflow,
            #[codec(index = 1)]
            Overflow,
            #[codec(index = 2)]
            DivisionByZero,
        }
    }
    pub mod sp_authority_discovery {
        use super::runtime_types;
        pub mod app {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Public(pub runtime_types::sp_core::sr25519::Public);
        }
    }
    pub mod sp_consensus_babe {
        use super::runtime_types;
        pub mod app {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Public(pub runtime_types::sp_core::sr25519::Public);
        }
        pub mod digests {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum NextConfigDescriptor {
                #[codec(index = 1)]
                V1 {
                    c: (::core::primitive::u64, ::core::primitive::u64),
                    allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum PreDigest {
                #[codec(index = 1)]
                Primary(runtime_types::sp_consensus_babe::digests::PrimaryPreDigest),
                #[codec(index = 2)]
                SecondaryPlain(runtime_types::sp_consensus_babe::digests::SecondaryPlainPreDigest),
                #[codec(index = 3)]
                SecondaryVRF(runtime_types::sp_consensus_babe::digests::SecondaryVRFPreDigest),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PrimaryPreDigest {
                pub authority_index: ::core::primitive::u32,
                pub slot: runtime_types::sp_consensus_slots::Slot,
                pub vrf_signature: runtime_types::sp_core::sr25519::vrf::VrfSignature,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct SecondaryPlainPreDigest {
                pub authority_index: ::core::primitive::u32,
                pub slot: runtime_types::sp_consensus_slots::Slot,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct SecondaryVRFPreDigest {
                pub authority_index: ::core::primitive::u32,
                pub slot: runtime_types::sp_consensus_slots::Slot,
                pub vrf_signature: runtime_types::sp_core::sr25519::vrf::VrfSignature,
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum AllowedSlots {
            #[codec(index = 0)]
            PrimarySlots,
            #[codec(index = 1)]
            PrimaryAndSecondaryPlainSlots,
            #[codec(index = 2)]
            PrimaryAndSecondaryVRFSlots,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct BabeConfiguration {
            pub slot_duration: ::core::primitive::u64,
            pub epoch_length: ::core::primitive::u64,
            pub c: (::core::primitive::u64, ::core::primitive::u64),
            pub authorities: ::std::vec::Vec<(
                runtime_types::sp_consensus_babe::app::Public,
                ::core::primitive::u64,
            )>,
            pub randomness: [::core::primitive::u8; 32usize],
            pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct BabeEpochConfiguration {
            pub c: (::core::primitive::u64, ::core::primitive::u64),
            pub allowed_slots: runtime_types::sp_consensus_babe::AllowedSlots,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Epoch {
            pub epoch_index: ::core::primitive::u64,
            pub start_slot: runtime_types::sp_consensus_slots::Slot,
            pub duration: ::core::primitive::u64,
            pub authorities: ::std::vec::Vec<(
                runtime_types::sp_consensus_babe::app::Public,
                ::core::primitive::u64,
            )>,
            pub randomness: [::core::primitive::u8; 32usize],
            pub config: runtime_types::sp_consensus_babe::BabeEpochConfiguration,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct OpaqueKeyOwnershipProof(pub ::std::vec::Vec<::core::primitive::u8>);
    }
    pub mod sp_consensus_beefy {
        use super::runtime_types;
        pub mod commitment {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Commitment<_0> {
                pub payload: runtime_types::sp_consensus_beefy::payload::Payload,
                pub block_number: _0,
                pub validator_set_id: ::core::primitive::u64,
            }
        }
        pub mod crypto {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Public(pub runtime_types::sp_core::ecdsa::Public);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Signature(pub runtime_types::sp_core::ecdsa::Signature);
        }
        pub mod payload {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Payload(
                pub  ::std::vec::Vec<(
                    [::core::primitive::u8; 2usize],
                    ::std::vec::Vec<::core::primitive::u8>,
                )>,
            );
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct EquivocationProof<_0, _1, _2> {
            pub first: runtime_types::sp_consensus_beefy::VoteMessage<_0, _1, _2>,
            pub second: runtime_types::sp_consensus_beefy::VoteMessage<_0, _1, _2>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct OpaqueKeyOwnershipProof(pub ::std::vec::Vec<::core::primitive::u8>);
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct ValidatorSet<_0> {
            pub validators: ::std::vec::Vec<_0>,
            pub id: ::core::primitive::u64,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct VoteMessage<_0, _1, _2> {
            pub commitment: runtime_types::sp_consensus_beefy::commitment::Commitment<_0>,
            pub id: _1,
            pub signature: _2,
        }
    }
    pub mod sp_consensus_grandpa {
        use super::runtime_types;
        pub mod app {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Public(pub runtime_types::sp_core::ed25519::Public);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Signature(pub runtime_types::sp_core::ed25519::Signature);
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum Equivocation<_0, _1> {
            #[codec(index = 0)]
            Prevote(
                runtime_types::finality_grandpa::Equivocation<
                    runtime_types::sp_consensus_grandpa::app::Public,
                    runtime_types::finality_grandpa::Prevote<_0, _1>,
                    runtime_types::sp_consensus_grandpa::app::Signature,
                >,
            ),
            #[codec(index = 1)]
            Precommit(
                runtime_types::finality_grandpa::Equivocation<
                    runtime_types::sp_consensus_grandpa::app::Public,
                    runtime_types::finality_grandpa::Precommit<_0, _1>,
                    runtime_types::sp_consensus_grandpa::app::Signature,
                >,
            ),
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct EquivocationProof<_0, _1> {
            pub set_id: ::core::primitive::u64,
            pub equivocation: runtime_types::sp_consensus_grandpa::Equivocation<_0, _1>,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct OpaqueKeyOwnershipProof(pub ::std::vec::Vec<::core::primitive::u8>);
    }
    pub mod sp_consensus_slots {
        use super::runtime_types;
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct EquivocationProof<_0, _1> {
            pub offender: _1,
            pub slot: runtime_types::sp_consensus_slots::Slot,
            pub first_header: _0,
            pub second_header: _0,
        }
        #[derive(
            :: subxt :: ext :: codec :: CompactAs,
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Slot(pub ::core::primitive::u64);
    }
    pub mod sp_core {
        use super::runtime_types;
        pub mod crypto {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct KeyTypeId(pub [::core::primitive::u8; 4usize]);
        }
        pub mod ecdsa {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Public(pub [::core::primitive::u8; 33usize]);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Signature(pub [::core::primitive::u8; 65usize]);
        }
        pub mod ed25519 {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Public(pub [::core::primitive::u8; 32usize]);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Signature(pub [::core::primitive::u8; 64usize]);
        }
        pub mod sr25519 {
            use super::runtime_types;
            pub mod vrf {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct VrfSignature {
                    pub output: [::core::primitive::u8; 32usize],
                    pub proof: [::core::primitive::u8; 64usize],
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Public(pub [::core::primitive::u8; 32usize]);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Signature(pub [::core::primitive::u8; 64usize]);
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct OpaqueMetadata(pub ::std::vec::Vec<::core::primitive::u8>);
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum Void {}
    }
    pub mod sp_inherents {
        use super::runtime_types;
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct CheckInherentsResult {
            pub okay: ::core::primitive::bool,
            pub fatal_error: ::core::primitive::bool,
            pub errors: runtime_types::sp_inherents::InherentData,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct InherentData {
            pub data: ::subxt::utils::KeyedVec<
                [::core::primitive::u8; 8usize],
                ::std::vec::Vec<::core::primitive::u8>,
            >,
        }
    }
    pub mod sp_mmr_primitives {
        use super::runtime_types;
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct EncodableOpaqueLeaf(pub ::std::vec::Vec<::core::primitive::u8>);
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum Error {
            #[codec(index = 0)]
            InvalidNumericOp,
            #[codec(index = 1)]
            Push,
            #[codec(index = 2)]
            GetRoot,
            #[codec(index = 3)]
            Commit,
            #[codec(index = 4)]
            GenerateProof,
            #[codec(index = 5)]
            Verify,
            #[codec(index = 6)]
            LeafNotFound,
            #[codec(index = 7)]
            PalletNotIncluded,
            #[codec(index = 8)]
            InvalidLeafIndex,
            #[codec(index = 9)]
            InvalidBestKnownBlock,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Proof<_0> {
            pub leaf_indices: ::std::vec::Vec<::core::primitive::u64>,
            pub leaf_count: ::core::primitive::u64,
            pub items: ::std::vec::Vec<_0>,
        }
    }
    pub mod sp_npos_elections {
        use super::runtime_types;
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct ElectionScore {
            pub minimal_stake: ::core::primitive::u128,
            pub sum_stake: ::core::primitive::u128,
            pub sum_stake_squared: ::core::primitive::u128,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct Support<_0> {
            pub total: ::core::primitive::u128,
            pub voters: ::std::vec::Vec<(_0, ::core::primitive::u128)>,
        }
    }
    pub mod sp_runtime {
        use super::runtime_types;
        pub mod generic {
            use super::runtime_types;
            pub mod block {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Block<_0, _1> {
                    pub header: _0,
                    pub extrinsics: ::std::vec::Vec<_1>,
                }
            }
            pub mod digest {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Digest {
                    pub logs:
                        ::std::vec::Vec<runtime_types::sp_runtime::generic::digest::DigestItem>,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum DigestItem {
                    #[codec(index = 6)]
                    PreRuntime(
                        [::core::primitive::u8; 4usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 4)]
                    Consensus(
                        [::core::primitive::u8; 4usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 5)]
                    Seal(
                        [::core::primitive::u8; 4usize],
                        ::std::vec::Vec<::core::primitive::u8>,
                    ),
                    #[codec(index = 0)]
                    Other(::std::vec::Vec<::core::primitive::u8>),
                    #[codec(index = 8)]
                    RuntimeEnvironmentUpdated,
                }
            }
            pub mod era {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Era {
                    #[codec(index = 0)]
                    Immortal,
                    #[codec(index = 1)]
                    Mortal1(::core::primitive::u8),
                    #[codec(index = 2)]
                    Mortal2(::core::primitive::u8),
                    #[codec(index = 3)]
                    Mortal3(::core::primitive::u8),
                    #[codec(index = 4)]
                    Mortal4(::core::primitive::u8),
                    #[codec(index = 5)]
                    Mortal5(::core::primitive::u8),
                    #[codec(index = 6)]
                    Mortal6(::core::primitive::u8),
                    #[codec(index = 7)]
                    Mortal7(::core::primitive::u8),
                    #[codec(index = 8)]
                    Mortal8(::core::primitive::u8),
                    #[codec(index = 9)]
                    Mortal9(::core::primitive::u8),
                    #[codec(index = 10)]
                    Mortal10(::core::primitive::u8),
                    #[codec(index = 11)]
                    Mortal11(::core::primitive::u8),
                    #[codec(index = 12)]
                    Mortal12(::core::primitive::u8),
                    #[codec(index = 13)]
                    Mortal13(::core::primitive::u8),
                    #[codec(index = 14)]
                    Mortal14(::core::primitive::u8),
                    #[codec(index = 15)]
                    Mortal15(::core::primitive::u8),
                    #[codec(index = 16)]
                    Mortal16(::core::primitive::u8),
                    #[codec(index = 17)]
                    Mortal17(::core::primitive::u8),
                    #[codec(index = 18)]
                    Mortal18(::core::primitive::u8),
                    #[codec(index = 19)]
                    Mortal19(::core::primitive::u8),
                    #[codec(index = 20)]
                    Mortal20(::core::primitive::u8),
                    #[codec(index = 21)]
                    Mortal21(::core::primitive::u8),
                    #[codec(index = 22)]
                    Mortal22(::core::primitive::u8),
                    #[codec(index = 23)]
                    Mortal23(::core::primitive::u8),
                    #[codec(index = 24)]
                    Mortal24(::core::primitive::u8),
                    #[codec(index = 25)]
                    Mortal25(::core::primitive::u8),
                    #[codec(index = 26)]
                    Mortal26(::core::primitive::u8),
                    #[codec(index = 27)]
                    Mortal27(::core::primitive::u8),
                    #[codec(index = 28)]
                    Mortal28(::core::primitive::u8),
                    #[codec(index = 29)]
                    Mortal29(::core::primitive::u8),
                    #[codec(index = 30)]
                    Mortal30(::core::primitive::u8),
                    #[codec(index = 31)]
                    Mortal31(::core::primitive::u8),
                    #[codec(index = 32)]
                    Mortal32(::core::primitive::u8),
                    #[codec(index = 33)]
                    Mortal33(::core::primitive::u8),
                    #[codec(index = 34)]
                    Mortal34(::core::primitive::u8),
                    #[codec(index = 35)]
                    Mortal35(::core::primitive::u8),
                    #[codec(index = 36)]
                    Mortal36(::core::primitive::u8),
                    #[codec(index = 37)]
                    Mortal37(::core::primitive::u8),
                    #[codec(index = 38)]
                    Mortal38(::core::primitive::u8),
                    #[codec(index = 39)]
                    Mortal39(::core::primitive::u8),
                    #[codec(index = 40)]
                    Mortal40(::core::primitive::u8),
                    #[codec(index = 41)]
                    Mortal41(::core::primitive::u8),
                    #[codec(index = 42)]
                    Mortal42(::core::primitive::u8),
                    #[codec(index = 43)]
                    Mortal43(::core::primitive::u8),
                    #[codec(index = 44)]
                    Mortal44(::core::primitive::u8),
                    #[codec(index = 45)]
                    Mortal45(::core::primitive::u8),
                    #[codec(index = 46)]
                    Mortal46(::core::primitive::u8),
                    #[codec(index = 47)]
                    Mortal47(::core::primitive::u8),
                    #[codec(index = 48)]
                    Mortal48(::core::primitive::u8),
                    #[codec(index = 49)]
                    Mortal49(::core::primitive::u8),
                    #[codec(index = 50)]
                    Mortal50(::core::primitive::u8),
                    #[codec(index = 51)]
                    Mortal51(::core::primitive::u8),
                    #[codec(index = 52)]
                    Mortal52(::core::primitive::u8),
                    #[codec(index = 53)]
                    Mortal53(::core::primitive::u8),
                    #[codec(index = 54)]
                    Mortal54(::core::primitive::u8),
                    #[codec(index = 55)]
                    Mortal55(::core::primitive::u8),
                    #[codec(index = 56)]
                    Mortal56(::core::primitive::u8),
                    #[codec(index = 57)]
                    Mortal57(::core::primitive::u8),
                    #[codec(index = 58)]
                    Mortal58(::core::primitive::u8),
                    #[codec(index = 59)]
                    Mortal59(::core::primitive::u8),
                    #[codec(index = 60)]
                    Mortal60(::core::primitive::u8),
                    #[codec(index = 61)]
                    Mortal61(::core::primitive::u8),
                    #[codec(index = 62)]
                    Mortal62(::core::primitive::u8),
                    #[codec(index = 63)]
                    Mortal63(::core::primitive::u8),
                    #[codec(index = 64)]
                    Mortal64(::core::primitive::u8),
                    #[codec(index = 65)]
                    Mortal65(::core::primitive::u8),
                    #[codec(index = 66)]
                    Mortal66(::core::primitive::u8),
                    #[codec(index = 67)]
                    Mortal67(::core::primitive::u8),
                    #[codec(index = 68)]
                    Mortal68(::core::primitive::u8),
                    #[codec(index = 69)]
                    Mortal69(::core::primitive::u8),
                    #[codec(index = 70)]
                    Mortal70(::core::primitive::u8),
                    #[codec(index = 71)]
                    Mortal71(::core::primitive::u8),
                    #[codec(index = 72)]
                    Mortal72(::core::primitive::u8),
                    #[codec(index = 73)]
                    Mortal73(::core::primitive::u8),
                    #[codec(index = 74)]
                    Mortal74(::core::primitive::u8),
                    #[codec(index = 75)]
                    Mortal75(::core::primitive::u8),
                    #[codec(index = 76)]
                    Mortal76(::core::primitive::u8),
                    #[codec(index = 77)]
                    Mortal77(::core::primitive::u8),
                    #[codec(index = 78)]
                    Mortal78(::core::primitive::u8),
                    #[codec(index = 79)]
                    Mortal79(::core::primitive::u8),
                    #[codec(index = 80)]
                    Mortal80(::core::primitive::u8),
                    #[codec(index = 81)]
                    Mortal81(::core::primitive::u8),
                    #[codec(index = 82)]
                    Mortal82(::core::primitive::u8),
                    #[codec(index = 83)]
                    Mortal83(::core::primitive::u8),
                    #[codec(index = 84)]
                    Mortal84(::core::primitive::u8),
                    #[codec(index = 85)]
                    Mortal85(::core::primitive::u8),
                    #[codec(index = 86)]
                    Mortal86(::core::primitive::u8),
                    #[codec(index = 87)]
                    Mortal87(::core::primitive::u8),
                    #[codec(index = 88)]
                    Mortal88(::core::primitive::u8),
                    #[codec(index = 89)]
                    Mortal89(::core::primitive::u8),
                    #[codec(index = 90)]
                    Mortal90(::core::primitive::u8),
                    #[codec(index = 91)]
                    Mortal91(::core::primitive::u8),
                    #[codec(index = 92)]
                    Mortal92(::core::primitive::u8),
                    #[codec(index = 93)]
                    Mortal93(::core::primitive::u8),
                    #[codec(index = 94)]
                    Mortal94(::core::primitive::u8),
                    #[codec(index = 95)]
                    Mortal95(::core::primitive::u8),
                    #[codec(index = 96)]
                    Mortal96(::core::primitive::u8),
                    #[codec(index = 97)]
                    Mortal97(::core::primitive::u8),
                    #[codec(index = 98)]
                    Mortal98(::core::primitive::u8),
                    #[codec(index = 99)]
                    Mortal99(::core::primitive::u8),
                    #[codec(index = 100)]
                    Mortal100(::core::primitive::u8),
                    #[codec(index = 101)]
                    Mortal101(::core::primitive::u8),
                    #[codec(index = 102)]
                    Mortal102(::core::primitive::u8),
                    #[codec(index = 103)]
                    Mortal103(::core::primitive::u8),
                    #[codec(index = 104)]
                    Mortal104(::core::primitive::u8),
                    #[codec(index = 105)]
                    Mortal105(::core::primitive::u8),
                    #[codec(index = 106)]
                    Mortal106(::core::primitive::u8),
                    #[codec(index = 107)]
                    Mortal107(::core::primitive::u8),
                    #[codec(index = 108)]
                    Mortal108(::core::primitive::u8),
                    #[codec(index = 109)]
                    Mortal109(::core::primitive::u8),
                    #[codec(index = 110)]
                    Mortal110(::core::primitive::u8),
                    #[codec(index = 111)]
                    Mortal111(::core::primitive::u8),
                    #[codec(index = 112)]
                    Mortal112(::core::primitive::u8),
                    #[codec(index = 113)]
                    Mortal113(::core::primitive::u8),
                    #[codec(index = 114)]
                    Mortal114(::core::primitive::u8),
                    #[codec(index = 115)]
                    Mortal115(::core::primitive::u8),
                    #[codec(index = 116)]
                    Mortal116(::core::primitive::u8),
                    #[codec(index = 117)]
                    Mortal117(::core::primitive::u8),
                    #[codec(index = 118)]
                    Mortal118(::core::primitive::u8),
                    #[codec(index = 119)]
                    Mortal119(::core::primitive::u8),
                    #[codec(index = 120)]
                    Mortal120(::core::primitive::u8),
                    #[codec(index = 121)]
                    Mortal121(::core::primitive::u8),
                    #[codec(index = 122)]
                    Mortal122(::core::primitive::u8),
                    #[codec(index = 123)]
                    Mortal123(::core::primitive::u8),
                    #[codec(index = 124)]
                    Mortal124(::core::primitive::u8),
                    #[codec(index = 125)]
                    Mortal125(::core::primitive::u8),
                    #[codec(index = 126)]
                    Mortal126(::core::primitive::u8),
                    #[codec(index = 127)]
                    Mortal127(::core::primitive::u8),
                    #[codec(index = 128)]
                    Mortal128(::core::primitive::u8),
                    #[codec(index = 129)]
                    Mortal129(::core::primitive::u8),
                    #[codec(index = 130)]
                    Mortal130(::core::primitive::u8),
                    #[codec(index = 131)]
                    Mortal131(::core::primitive::u8),
                    #[codec(index = 132)]
                    Mortal132(::core::primitive::u8),
                    #[codec(index = 133)]
                    Mortal133(::core::primitive::u8),
                    #[codec(index = 134)]
                    Mortal134(::core::primitive::u8),
                    #[codec(index = 135)]
                    Mortal135(::core::primitive::u8),
                    #[codec(index = 136)]
                    Mortal136(::core::primitive::u8),
                    #[codec(index = 137)]
                    Mortal137(::core::primitive::u8),
                    #[codec(index = 138)]
                    Mortal138(::core::primitive::u8),
                    #[codec(index = 139)]
                    Mortal139(::core::primitive::u8),
                    #[codec(index = 140)]
                    Mortal140(::core::primitive::u8),
                    #[codec(index = 141)]
                    Mortal141(::core::primitive::u8),
                    #[codec(index = 142)]
                    Mortal142(::core::primitive::u8),
                    #[codec(index = 143)]
                    Mortal143(::core::primitive::u8),
                    #[codec(index = 144)]
                    Mortal144(::core::primitive::u8),
                    #[codec(index = 145)]
                    Mortal145(::core::primitive::u8),
                    #[codec(index = 146)]
                    Mortal146(::core::primitive::u8),
                    #[codec(index = 147)]
                    Mortal147(::core::primitive::u8),
                    #[codec(index = 148)]
                    Mortal148(::core::primitive::u8),
                    #[codec(index = 149)]
                    Mortal149(::core::primitive::u8),
                    #[codec(index = 150)]
                    Mortal150(::core::primitive::u8),
                    #[codec(index = 151)]
                    Mortal151(::core::primitive::u8),
                    #[codec(index = 152)]
                    Mortal152(::core::primitive::u8),
                    #[codec(index = 153)]
                    Mortal153(::core::primitive::u8),
                    #[codec(index = 154)]
                    Mortal154(::core::primitive::u8),
                    #[codec(index = 155)]
                    Mortal155(::core::primitive::u8),
                    #[codec(index = 156)]
                    Mortal156(::core::primitive::u8),
                    #[codec(index = 157)]
                    Mortal157(::core::primitive::u8),
                    #[codec(index = 158)]
                    Mortal158(::core::primitive::u8),
                    #[codec(index = 159)]
                    Mortal159(::core::primitive::u8),
                    #[codec(index = 160)]
                    Mortal160(::core::primitive::u8),
                    #[codec(index = 161)]
                    Mortal161(::core::primitive::u8),
                    #[codec(index = 162)]
                    Mortal162(::core::primitive::u8),
                    #[codec(index = 163)]
                    Mortal163(::core::primitive::u8),
                    #[codec(index = 164)]
                    Mortal164(::core::primitive::u8),
                    #[codec(index = 165)]
                    Mortal165(::core::primitive::u8),
                    #[codec(index = 166)]
                    Mortal166(::core::primitive::u8),
                    #[codec(index = 167)]
                    Mortal167(::core::primitive::u8),
                    #[codec(index = 168)]
                    Mortal168(::core::primitive::u8),
                    #[codec(index = 169)]
                    Mortal169(::core::primitive::u8),
                    #[codec(index = 170)]
                    Mortal170(::core::primitive::u8),
                    #[codec(index = 171)]
                    Mortal171(::core::primitive::u8),
                    #[codec(index = 172)]
                    Mortal172(::core::primitive::u8),
                    #[codec(index = 173)]
                    Mortal173(::core::primitive::u8),
                    #[codec(index = 174)]
                    Mortal174(::core::primitive::u8),
                    #[codec(index = 175)]
                    Mortal175(::core::primitive::u8),
                    #[codec(index = 176)]
                    Mortal176(::core::primitive::u8),
                    #[codec(index = 177)]
                    Mortal177(::core::primitive::u8),
                    #[codec(index = 178)]
                    Mortal178(::core::primitive::u8),
                    #[codec(index = 179)]
                    Mortal179(::core::primitive::u8),
                    #[codec(index = 180)]
                    Mortal180(::core::primitive::u8),
                    #[codec(index = 181)]
                    Mortal181(::core::primitive::u8),
                    #[codec(index = 182)]
                    Mortal182(::core::primitive::u8),
                    #[codec(index = 183)]
                    Mortal183(::core::primitive::u8),
                    #[codec(index = 184)]
                    Mortal184(::core::primitive::u8),
                    #[codec(index = 185)]
                    Mortal185(::core::primitive::u8),
                    #[codec(index = 186)]
                    Mortal186(::core::primitive::u8),
                    #[codec(index = 187)]
                    Mortal187(::core::primitive::u8),
                    #[codec(index = 188)]
                    Mortal188(::core::primitive::u8),
                    #[codec(index = 189)]
                    Mortal189(::core::primitive::u8),
                    #[codec(index = 190)]
                    Mortal190(::core::primitive::u8),
                    #[codec(index = 191)]
                    Mortal191(::core::primitive::u8),
                    #[codec(index = 192)]
                    Mortal192(::core::primitive::u8),
                    #[codec(index = 193)]
                    Mortal193(::core::primitive::u8),
                    #[codec(index = 194)]
                    Mortal194(::core::primitive::u8),
                    #[codec(index = 195)]
                    Mortal195(::core::primitive::u8),
                    #[codec(index = 196)]
                    Mortal196(::core::primitive::u8),
                    #[codec(index = 197)]
                    Mortal197(::core::primitive::u8),
                    #[codec(index = 198)]
                    Mortal198(::core::primitive::u8),
                    #[codec(index = 199)]
                    Mortal199(::core::primitive::u8),
                    #[codec(index = 200)]
                    Mortal200(::core::primitive::u8),
                    #[codec(index = 201)]
                    Mortal201(::core::primitive::u8),
                    #[codec(index = 202)]
                    Mortal202(::core::primitive::u8),
                    #[codec(index = 203)]
                    Mortal203(::core::primitive::u8),
                    #[codec(index = 204)]
                    Mortal204(::core::primitive::u8),
                    #[codec(index = 205)]
                    Mortal205(::core::primitive::u8),
                    #[codec(index = 206)]
                    Mortal206(::core::primitive::u8),
                    #[codec(index = 207)]
                    Mortal207(::core::primitive::u8),
                    #[codec(index = 208)]
                    Mortal208(::core::primitive::u8),
                    #[codec(index = 209)]
                    Mortal209(::core::primitive::u8),
                    #[codec(index = 210)]
                    Mortal210(::core::primitive::u8),
                    #[codec(index = 211)]
                    Mortal211(::core::primitive::u8),
                    #[codec(index = 212)]
                    Mortal212(::core::primitive::u8),
                    #[codec(index = 213)]
                    Mortal213(::core::primitive::u8),
                    #[codec(index = 214)]
                    Mortal214(::core::primitive::u8),
                    #[codec(index = 215)]
                    Mortal215(::core::primitive::u8),
                    #[codec(index = 216)]
                    Mortal216(::core::primitive::u8),
                    #[codec(index = 217)]
                    Mortal217(::core::primitive::u8),
                    #[codec(index = 218)]
                    Mortal218(::core::primitive::u8),
                    #[codec(index = 219)]
                    Mortal219(::core::primitive::u8),
                    #[codec(index = 220)]
                    Mortal220(::core::primitive::u8),
                    #[codec(index = 221)]
                    Mortal221(::core::primitive::u8),
                    #[codec(index = 222)]
                    Mortal222(::core::primitive::u8),
                    #[codec(index = 223)]
                    Mortal223(::core::primitive::u8),
                    #[codec(index = 224)]
                    Mortal224(::core::primitive::u8),
                    #[codec(index = 225)]
                    Mortal225(::core::primitive::u8),
                    #[codec(index = 226)]
                    Mortal226(::core::primitive::u8),
                    #[codec(index = 227)]
                    Mortal227(::core::primitive::u8),
                    #[codec(index = 228)]
                    Mortal228(::core::primitive::u8),
                    #[codec(index = 229)]
                    Mortal229(::core::primitive::u8),
                    #[codec(index = 230)]
                    Mortal230(::core::primitive::u8),
                    #[codec(index = 231)]
                    Mortal231(::core::primitive::u8),
                    #[codec(index = 232)]
                    Mortal232(::core::primitive::u8),
                    #[codec(index = 233)]
                    Mortal233(::core::primitive::u8),
                    #[codec(index = 234)]
                    Mortal234(::core::primitive::u8),
                    #[codec(index = 235)]
                    Mortal235(::core::primitive::u8),
                    #[codec(index = 236)]
                    Mortal236(::core::primitive::u8),
                    #[codec(index = 237)]
                    Mortal237(::core::primitive::u8),
                    #[codec(index = 238)]
                    Mortal238(::core::primitive::u8),
                    #[codec(index = 239)]
                    Mortal239(::core::primitive::u8),
                    #[codec(index = 240)]
                    Mortal240(::core::primitive::u8),
                    #[codec(index = 241)]
                    Mortal241(::core::primitive::u8),
                    #[codec(index = 242)]
                    Mortal242(::core::primitive::u8),
                    #[codec(index = 243)]
                    Mortal243(::core::primitive::u8),
                    #[codec(index = 244)]
                    Mortal244(::core::primitive::u8),
                    #[codec(index = 245)]
                    Mortal245(::core::primitive::u8),
                    #[codec(index = 246)]
                    Mortal246(::core::primitive::u8),
                    #[codec(index = 247)]
                    Mortal247(::core::primitive::u8),
                    #[codec(index = 248)]
                    Mortal248(::core::primitive::u8),
                    #[codec(index = 249)]
                    Mortal249(::core::primitive::u8),
                    #[codec(index = 250)]
                    Mortal250(::core::primitive::u8),
                    #[codec(index = 251)]
                    Mortal251(::core::primitive::u8),
                    #[codec(index = 252)]
                    Mortal252(::core::primitive::u8),
                    #[codec(index = 253)]
                    Mortal253(::core::primitive::u8),
                    #[codec(index = 254)]
                    Mortal254(::core::primitive::u8),
                    #[codec(index = 255)]
                    Mortal255(::core::primitive::u8),
                }
            }
            pub mod header {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct Header<_0, _1> {
                    pub parent_hash: ::subxt::utils::H256,
                    #[codec(compact)]
                    pub number: _0,
                    pub state_root: ::subxt::utils::H256,
                    pub extrinsics_root: ::subxt::utils::H256,
                    pub digest: runtime_types::sp_runtime::generic::digest::Digest,
                    #[codec(skip)]
                    pub __subxt_unused_type_params: ::core::marker::PhantomData<_1>,
                }
            }
            pub mod unchecked_extrinsic {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct UncheckedExtrinsic<_0, _1, _2, _3>(
                    pub ::std::vec::Vec<::core::primitive::u8>,
                    #[codec(skip)] pub ::core::marker::PhantomData<(_1, _0, _2, _3)>,
                );
            }
        }
        pub mod traits {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct BlakeTwo256;
        }
        pub mod transaction_validity {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum InvalidTransaction {
                #[codec(index = 0)]
                Call,
                #[codec(index = 1)]
                Payment,
                #[codec(index = 2)]
                Future,
                #[codec(index = 3)]
                Stale,
                #[codec(index = 4)]
                BadProof,
                #[codec(index = 5)]
                AncientBirthBlock,
                #[codec(index = 6)]
                ExhaustsResources,
                #[codec(index = 7)]
                Custom(::core::primitive::u8),
                #[codec(index = 8)]
                BadMandatory,
                #[codec(index = 9)]
                MandatoryValidation,
                #[codec(index = 10)]
                BadSigner,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TransactionSource {
                #[codec(index = 0)]
                InBlock,
                #[codec(index = 1)]
                Local,
                #[codec(index = 2)]
                External,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum TransactionValidityError {
                #[codec(index = 0)]
                Invalid(runtime_types::sp_runtime::transaction_validity::InvalidTransaction),
                #[codec(index = 1)]
                Unknown(runtime_types::sp_runtime::transaction_validity::UnknownTransaction),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum UnknownTransaction {
                #[codec(index = 0)]
                CannotLookup,
                #[codec(index = 1)]
                NoUnsignedValidator,
                #[codec(index = 2)]
                Custom(::core::primitive::u8),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct ValidTransaction {
                pub priority: ::core::primitive::u64,
                pub requires: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                pub provides: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
                pub longevity: ::core::primitive::u64,
                pub propagate: ::core::primitive::bool,
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum DispatchError {
            #[codec(index = 0)]
            Other,
            #[codec(index = 1)]
            CannotLookup,
            #[codec(index = 2)]
            BadOrigin,
            #[codec(index = 3)]
            Module(runtime_types::sp_runtime::ModuleError),
            #[codec(index = 4)]
            ConsumerRemaining,
            #[codec(index = 5)]
            NoProviders,
            #[codec(index = 6)]
            TooManyConsumers,
            #[codec(index = 7)]
            Token(runtime_types::sp_runtime::TokenError),
            #[codec(index = 8)]
            Arithmetic(runtime_types::sp_arithmetic::ArithmeticError),
            #[codec(index = 9)]
            Transactional(runtime_types::sp_runtime::TransactionalError),
            #[codec(index = 10)]
            Exhausted,
            #[codec(index = 11)]
            Corruption,
            #[codec(index = 12)]
            Unavailable,
            #[codec(index = 13)]
            RootNotAllowed,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct DispatchErrorWithPostInfo<_0> {
            pub post_info: _0,
            pub error: runtime_types::sp_runtime::DispatchError,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct ModuleError {
            pub index: ::core::primitive::u8,
            pub error: [::core::primitive::u8; 4usize],
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum MultiSignature {
            #[codec(index = 0)]
            Ed25519(runtime_types::sp_core::ed25519::Signature),
            #[codec(index = 1)]
            Sr25519(runtime_types::sp_core::sr25519::Signature),
            #[codec(index = 2)]
            Ecdsa(runtime_types::sp_core::ecdsa::Signature),
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum MultiSigner {
            #[codec(index = 0)]
            Ed25519(runtime_types::sp_core::ed25519::Public),
            #[codec(index = 1)]
            Sr25519(runtime_types::sp_core::sr25519::Public),
            #[codec(index = 2)]
            Ecdsa(runtime_types::sp_core::ecdsa::Public),
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum TokenError {
            #[codec(index = 0)]
            FundsUnavailable,
            #[codec(index = 1)]
            OnlyProvider,
            #[codec(index = 2)]
            BelowMinimum,
            #[codec(index = 3)]
            CannotCreate,
            #[codec(index = 4)]
            UnknownAsset,
            #[codec(index = 5)]
            Frozen,
            #[codec(index = 6)]
            Unsupported,
            #[codec(index = 7)]
            CannotCreateHold,
            #[codec(index = 8)]
            NotExpendable,
            #[codec(index = 9)]
            Blocked,
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum TransactionalError {
            #[codec(index = 0)]
            LimitReached,
            #[codec(index = 1)]
            NoLayer,
        }
    }
    pub mod sp_session {
        use super::runtime_types;
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct MembershipProof {
            pub session: ::core::primitive::u32,
            pub trie_nodes: ::std::vec::Vec<::std::vec::Vec<::core::primitive::u8>>,
            pub validator_count: ::core::primitive::u32,
        }
    }
    pub mod sp_staking {
        use super::runtime_types;
        pub mod offence {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct OffenceDetails<_0, _1> {
                pub offender: _1,
                pub reporters: ::std::vec::Vec<_0>,
            }
        }
    }
    pub mod sp_version {
        use super::runtime_types;
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct RuntimeVersion {
            pub spec_name: ::std::string::String,
            pub impl_name: ::std::string::String,
            pub authoring_version: ::core::primitive::u32,
            pub spec_version: ::core::primitive::u32,
            pub impl_version: ::core::primitive::u32,
            pub apis: ::std::vec::Vec<([::core::primitive::u8; 8usize], ::core::primitive::u32)>,
            pub transaction_version: ::core::primitive::u32,
            pub state_version: ::core::primitive::u8,
        }
    }
    pub mod sp_weights {
        use super::runtime_types;
        pub mod weight_v2 {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Weight {
                #[codec(compact)]
                pub ref_time: ::core::primitive::u64,
                #[codec(compact)]
                pub proof_size: ::core::primitive::u64,
            }
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub struct RuntimeDbWeight {
            pub read: ::core::primitive::u64,
            pub write: ::core::primitive::u64,
        }
    }
    pub mod xcm {
        use super::runtime_types;
        pub mod double_encoded {
            use super::runtime_types;
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct DoubleEncoded {
                pub encoded: ::std::vec::Vec<::core::primitive::u8>,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct DoubleEncoded2 {
                pub encoded: ::std::vec::Vec<::core::primitive::u8>,
            }
        }
        pub mod v2 {
            use super::runtime_types;
            pub mod junction {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Junction {
                    #[codec(index = 0)]
                    Parachain(#[codec(compact)] ::core::primitive::u32),
                    #[codec(index = 1)]
                    AccountId32 {
                        network: runtime_types::xcm::v2::NetworkId,
                        id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    AccountIndex64 {
                        network: runtime_types::xcm::v2::NetworkId,
                        #[codec(compact)]
                        index: ::core::primitive::u64,
                    },
                    #[codec(index = 3)]
                    AccountKey20 {
                        network: runtime_types::xcm::v2::NetworkId,
                        key: [::core::primitive::u8; 20usize],
                    },
                    #[codec(index = 4)]
                    PalletInstance(::core::primitive::u8),
                    #[codec(index = 5)]
                    GeneralIndex(#[codec(compact)] ::core::primitive::u128),
                    #[codec(index = 6)]
                    GeneralKey(
                        runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                            ::core::primitive::u8,
                        >,
                    ),
                    #[codec(index = 7)]
                    OnlyChild,
                    #[codec(index = 8)]
                    Plurality {
                        id: runtime_types::xcm::v2::BodyId,
                        part: runtime_types::xcm::v2::BodyPart,
                    },
                }
            }
            pub mod multiasset {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum AssetId {
                    #[codec(index = 0)]
                    Concrete(runtime_types::xcm::v2::multilocation::MultiLocation),
                    #[codec(index = 1)]
                    Abstract(::std::vec::Vec<::core::primitive::u8>),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum AssetInstance {
                    #[codec(index = 0)]
                    Undefined,
                    #[codec(index = 1)]
                    Index(#[codec(compact)] ::core::primitive::u128),
                    #[codec(index = 2)]
                    Array4([::core::primitive::u8; 4usize]),
                    #[codec(index = 3)]
                    Array8([::core::primitive::u8; 8usize]),
                    #[codec(index = 4)]
                    Array16([::core::primitive::u8; 16usize]),
                    #[codec(index = 5)]
                    Array32([::core::primitive::u8; 32usize]),
                    #[codec(index = 6)]
                    Blob(::std::vec::Vec<::core::primitive::u8>),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Fungibility {
                    #[codec(index = 0)]
                    Fungible(#[codec(compact)] ::core::primitive::u128),
                    #[codec(index = 1)]
                    NonFungible(runtime_types::xcm::v2::multiasset::AssetInstance),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MultiAsset {
                    pub id: runtime_types::xcm::v2::multiasset::AssetId,
                    pub fun: runtime_types::xcm::v2::multiasset::Fungibility,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum MultiAssetFilter {
                    #[codec(index = 0)]
                    Definite(runtime_types::xcm::v2::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    Wild(runtime_types::xcm::v2::multiasset::WildMultiAsset),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MultiAssets(
                    pub ::std::vec::Vec<runtime_types::xcm::v2::multiasset::MultiAsset>,
                );
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum WildFungibility {
                    #[codec(index = 0)]
                    Fungible,
                    #[codec(index = 1)]
                    NonFungible,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum WildMultiAsset {
                    #[codec(index = 0)]
                    All,
                    #[codec(index = 1)]
                    AllOf {
                        id: runtime_types::xcm::v2::multiasset::AssetId,
                        fun: runtime_types::xcm::v2::multiasset::WildFungibility,
                    },
                }
            }
            pub mod multilocation {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Junctions {
                    #[codec(index = 0)]
                    Here,
                    #[codec(index = 1)]
                    X1(runtime_types::xcm::v2::junction::Junction),
                    #[codec(index = 2)]
                    X2(
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                    ),
                    #[codec(index = 3)]
                    X3(
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                    ),
                    #[codec(index = 4)]
                    X4(
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                    ),
                    #[codec(index = 5)]
                    X5(
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                    ),
                    #[codec(index = 6)]
                    X6(
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                    ),
                    #[codec(index = 7)]
                    X7(
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                    ),
                    #[codec(index = 8)]
                    X8(
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                        runtime_types::xcm::v2::junction::Junction,
                    ),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MultiLocation {
                    pub parents: ::core::primitive::u8,
                    pub interior: runtime_types::xcm::v2::multilocation::Junctions,
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    Overflow,
                    #[codec(index = 1)]
                    Unimplemented,
                    #[codec(index = 2)]
                    UntrustedReserveLocation,
                    #[codec(index = 3)]
                    UntrustedTeleportLocation,
                    #[codec(index = 4)]
                    MultiLocationFull,
                    #[codec(index = 5)]
                    MultiLocationNotInvertible,
                    #[codec(index = 6)]
                    BadOrigin,
                    #[codec(index = 7)]
                    InvalidLocation,
                    #[codec(index = 8)]
                    AssetNotFound,
                    #[codec(index = 9)]
                    FailedToTransactAsset,
                    #[codec(index = 10)]
                    NotWithdrawable,
                    #[codec(index = 11)]
                    LocationCannotHold,
                    #[codec(index = 12)]
                    ExceedsMaxMessageSize,
                    #[codec(index = 13)]
                    DestinationUnsupported,
                    #[codec(index = 14)]
                    Transport,
                    #[codec(index = 15)]
                    Unroutable,
                    #[codec(index = 16)]
                    UnknownClaim,
                    #[codec(index = 17)]
                    FailedToDecode,
                    #[codec(index = 18)]
                    MaxWeightInvalid,
                    #[codec(index = 19)]
                    NotHoldingFees,
                    #[codec(index = 20)]
                    TooExpensive,
                    #[codec(index = 21)]
                    Trap(::core::primitive::u64),
                    #[codec(index = 22)]
                    UnhandledXcmVersion,
                    #[codec(index = 23)]
                    WeightLimitReached(::core::primitive::u64),
                    #[codec(index = 24)]
                    Barrier,
                    #[codec(index = 25)]
                    WeightNotComputable,
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum BodyId {
                #[codec(index = 0)]
                Unit,
                #[codec(index = 1)]
                Named(
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        ::core::primitive::u8,
                    >,
                ),
                #[codec(index = 2)]
                Index(#[codec(compact)] ::core::primitive::u32),
                #[codec(index = 3)]
                Executive,
                #[codec(index = 4)]
                Technical,
                #[codec(index = 5)]
                Legislative,
                #[codec(index = 6)]
                Judicial,
                #[codec(index = 7)]
                Defense,
                #[codec(index = 8)]
                Administration,
                #[codec(index = 9)]
                Treasury,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum BodyPart {
                #[codec(index = 0)]
                Voice,
                #[codec(index = 1)]
                Members {
                    #[codec(compact)]
                    count: ::core::primitive::u32,
                },
                #[codec(index = 2)]
                Fraction {
                    #[codec(compact)]
                    nom: ::core::primitive::u32,
                    #[codec(compact)]
                    denom: ::core::primitive::u32,
                },
                #[codec(index = 3)]
                AtLeastProportion {
                    #[codec(compact)]
                    nom: ::core::primitive::u32,
                    #[codec(compact)]
                    denom: ::core::primitive::u32,
                },
                #[codec(index = 4)]
                MoreThanProportion {
                    #[codec(compact)]
                    nom: ::core::primitive::u32,
                    #[codec(compact)]
                    denom: ::core::primitive::u32,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Instruction {
                #[codec(index = 0)]
                WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
                #[codec(index = 1)]
                ReserveAssetDeposited(runtime_types::xcm::v2::multiasset::MultiAssets),
                #[codec(index = 2)]
                ReceiveTeleportedAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
                #[codec(index = 3)]
                QueryResponse {
                    #[codec(compact)]
                    query_id: ::core::primitive::u64,
                    response: runtime_types::xcm::v2::Response,
                    #[codec(compact)]
                    max_weight: ::core::primitive::u64,
                },
                #[codec(index = 4)]
                TransferAsset {
                    assets: runtime_types::xcm::v2::multiasset::MultiAssets,
                    beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
                },
                #[codec(index = 5)]
                TransferReserveAsset {
                    assets: runtime_types::xcm::v2::multiasset::MultiAssets,
                    dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v2::Xcm,
                },
                #[codec(index = 6)]
                Transact {
                    origin_type: runtime_types::xcm::v2::OriginKind,
                    #[codec(compact)]
                    require_weight_at_most: ::core::primitive::u64,
                    call: runtime_types::xcm::double_encoded::DoubleEncoded,
                },
                #[codec(index = 7)]
                HrmpNewChannelOpenRequest {
                    #[codec(compact)]
                    sender: ::core::primitive::u32,
                    #[codec(compact)]
                    max_message_size: ::core::primitive::u32,
                    #[codec(compact)]
                    max_capacity: ::core::primitive::u32,
                },
                #[codec(index = 8)]
                HrmpChannelAccepted {
                    #[codec(compact)]
                    recipient: ::core::primitive::u32,
                },
                #[codec(index = 9)]
                HrmpChannelClosing {
                    #[codec(compact)]
                    initiator: ::core::primitive::u32,
                    #[codec(compact)]
                    sender: ::core::primitive::u32,
                    #[codec(compact)]
                    recipient: ::core::primitive::u32,
                },
                #[codec(index = 10)]
                ClearOrigin,
                #[codec(index = 11)]
                DescendOrigin(runtime_types::xcm::v2::multilocation::Junctions),
                #[codec(index = 12)]
                ReportError {
                    #[codec(compact)]
                    query_id: ::core::primitive::u64,
                    dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                    #[codec(compact)]
                    max_response_weight: ::core::primitive::u64,
                },
                #[codec(index = 13)]
                DepositAsset {
                    assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                    #[codec(compact)]
                    max_assets: ::core::primitive::u32,
                    beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
                },
                #[codec(index = 14)]
                DepositReserveAsset {
                    assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                    #[codec(compact)]
                    max_assets: ::core::primitive::u32,
                    dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v2::Xcm,
                },
                #[codec(index = 15)]
                ExchangeAsset {
                    give: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                    receive: runtime_types::xcm::v2::multiasset::MultiAssets,
                },
                #[codec(index = 16)]
                InitiateReserveWithdraw {
                    assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                    reserve: runtime_types::xcm::v2::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v2::Xcm,
                },
                #[codec(index = 17)]
                InitiateTeleport {
                    assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                    dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v2::Xcm,
                },
                #[codec(index = 18)]
                QueryHolding {
                    #[codec(compact)]
                    query_id: ::core::primitive::u64,
                    dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                    assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                    #[codec(compact)]
                    max_response_weight: ::core::primitive::u64,
                },
                #[codec(index = 19)]
                BuyExecution {
                    fees: runtime_types::xcm::v2::multiasset::MultiAsset,
                    weight_limit: runtime_types::xcm::v2::WeightLimit,
                },
                #[codec(index = 20)]
                RefundSurplus,
                #[codec(index = 21)]
                SetErrorHandler(runtime_types::xcm::v2::Xcm),
                #[codec(index = 22)]
                SetAppendix(runtime_types::xcm::v2::Xcm),
                #[codec(index = 23)]
                ClearError,
                #[codec(index = 24)]
                ClaimAsset {
                    assets: runtime_types::xcm::v2::multiasset::MultiAssets,
                    ticket: runtime_types::xcm::v2::multilocation::MultiLocation,
                },
                #[codec(index = 25)]
                Trap(#[codec(compact)] ::core::primitive::u64),
                #[codec(index = 26)]
                SubscribeVersion {
                    #[codec(compact)]
                    query_id: ::core::primitive::u64,
                    #[codec(compact)]
                    max_response_weight: ::core::primitive::u64,
                },
                #[codec(index = 27)]
                UnsubscribeVersion,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Instruction2 {
                #[codec(index = 0)]
                WithdrawAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
                #[codec(index = 1)]
                ReserveAssetDeposited(runtime_types::xcm::v2::multiasset::MultiAssets),
                #[codec(index = 2)]
                ReceiveTeleportedAsset(runtime_types::xcm::v2::multiasset::MultiAssets),
                #[codec(index = 3)]
                QueryResponse {
                    #[codec(compact)]
                    query_id: ::core::primitive::u64,
                    response: runtime_types::xcm::v2::Response,
                    #[codec(compact)]
                    max_weight: ::core::primitive::u64,
                },
                #[codec(index = 4)]
                TransferAsset {
                    assets: runtime_types::xcm::v2::multiasset::MultiAssets,
                    beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
                },
                #[codec(index = 5)]
                TransferReserveAsset {
                    assets: runtime_types::xcm::v2::multiasset::MultiAssets,
                    dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v2::Xcm,
                },
                #[codec(index = 6)]
                Transact {
                    origin_type: runtime_types::xcm::v2::OriginKind,
                    #[codec(compact)]
                    require_weight_at_most: ::core::primitive::u64,
                    call: runtime_types::xcm::double_encoded::DoubleEncoded2,
                },
                #[codec(index = 7)]
                HrmpNewChannelOpenRequest {
                    #[codec(compact)]
                    sender: ::core::primitive::u32,
                    #[codec(compact)]
                    max_message_size: ::core::primitive::u32,
                    #[codec(compact)]
                    max_capacity: ::core::primitive::u32,
                },
                #[codec(index = 8)]
                HrmpChannelAccepted {
                    #[codec(compact)]
                    recipient: ::core::primitive::u32,
                },
                #[codec(index = 9)]
                HrmpChannelClosing {
                    #[codec(compact)]
                    initiator: ::core::primitive::u32,
                    #[codec(compact)]
                    sender: ::core::primitive::u32,
                    #[codec(compact)]
                    recipient: ::core::primitive::u32,
                },
                #[codec(index = 10)]
                ClearOrigin,
                #[codec(index = 11)]
                DescendOrigin(runtime_types::xcm::v2::multilocation::Junctions),
                #[codec(index = 12)]
                ReportError {
                    #[codec(compact)]
                    query_id: ::core::primitive::u64,
                    dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                    #[codec(compact)]
                    max_response_weight: ::core::primitive::u64,
                },
                #[codec(index = 13)]
                DepositAsset {
                    assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                    #[codec(compact)]
                    max_assets: ::core::primitive::u32,
                    beneficiary: runtime_types::xcm::v2::multilocation::MultiLocation,
                },
                #[codec(index = 14)]
                DepositReserveAsset {
                    assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                    #[codec(compact)]
                    max_assets: ::core::primitive::u32,
                    dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v2::Xcm,
                },
                #[codec(index = 15)]
                ExchangeAsset {
                    give: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                    receive: runtime_types::xcm::v2::multiasset::MultiAssets,
                },
                #[codec(index = 16)]
                InitiateReserveWithdraw {
                    assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                    reserve: runtime_types::xcm::v2::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v2::Xcm,
                },
                #[codec(index = 17)]
                InitiateTeleport {
                    assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                    dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v2::Xcm,
                },
                #[codec(index = 18)]
                QueryHolding {
                    #[codec(compact)]
                    query_id: ::core::primitive::u64,
                    dest: runtime_types::xcm::v2::multilocation::MultiLocation,
                    assets: runtime_types::xcm::v2::multiasset::MultiAssetFilter,
                    #[codec(compact)]
                    max_response_weight: ::core::primitive::u64,
                },
                #[codec(index = 19)]
                BuyExecution {
                    fees: runtime_types::xcm::v2::multiasset::MultiAsset,
                    weight_limit: runtime_types::xcm::v2::WeightLimit,
                },
                #[codec(index = 20)]
                RefundSurplus,
                #[codec(index = 21)]
                SetErrorHandler(runtime_types::xcm::v2::Xcm2),
                #[codec(index = 22)]
                SetAppendix(runtime_types::xcm::v2::Xcm2),
                #[codec(index = 23)]
                ClearError,
                #[codec(index = 24)]
                ClaimAsset {
                    assets: runtime_types::xcm::v2::multiasset::MultiAssets,
                    ticket: runtime_types::xcm::v2::multilocation::MultiLocation,
                },
                #[codec(index = 25)]
                Trap(#[codec(compact)] ::core::primitive::u64),
                #[codec(index = 26)]
                SubscribeVersion {
                    #[codec(compact)]
                    query_id: ::core::primitive::u64,
                    #[codec(compact)]
                    max_response_weight: ::core::primitive::u64,
                },
                #[codec(index = 27)]
                UnsubscribeVersion,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum NetworkId {
                #[codec(index = 0)]
                Any,
                #[codec(index = 1)]
                Named(
                    runtime_types::bounded_collections::weak_bounded_vec::WeakBoundedVec<
                        ::core::primitive::u8,
                    >,
                ),
                #[codec(index = 2)]
                Polkadot,
                #[codec(index = 3)]
                Kusama,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum OriginKind {
                #[codec(index = 0)]
                Native,
                #[codec(index = 1)]
                SovereignAccount,
                #[codec(index = 2)]
                Superuser,
                #[codec(index = 3)]
                Xcm,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Response {
                #[codec(index = 0)]
                Null,
                #[codec(index = 1)]
                Assets(runtime_types::xcm::v2::multiasset::MultiAssets),
                #[codec(index = 2)]
                ExecutionResult(
                    ::core::option::Option<(
                        ::core::primitive::u32,
                        runtime_types::xcm::v2::traits::Error,
                    )>,
                ),
                #[codec(index = 3)]
                Version(::core::primitive::u32),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum WeightLimit {
                #[codec(index = 0)]
                Unlimited,
                #[codec(index = 1)]
                Limited(#[codec(compact)] ::core::primitive::u64),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction>);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Xcm2(pub ::std::vec::Vec<runtime_types::xcm::v2::Instruction2>);
        }
        pub mod v3 {
            use super::runtime_types;
            pub mod junction {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum BodyId {
                    #[codec(index = 0)]
                    Unit,
                    #[codec(index = 1)]
                    Moniker([::core::primitive::u8; 4usize]),
                    #[codec(index = 2)]
                    Index(#[codec(compact)] ::core::primitive::u32),
                    #[codec(index = 3)]
                    Executive,
                    #[codec(index = 4)]
                    Technical,
                    #[codec(index = 5)]
                    Legislative,
                    #[codec(index = 6)]
                    Judicial,
                    #[codec(index = 7)]
                    Defense,
                    #[codec(index = 8)]
                    Administration,
                    #[codec(index = 9)]
                    Treasury,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum BodyPart {
                    #[codec(index = 0)]
                    Voice,
                    #[codec(index = 1)]
                    Members {
                        #[codec(compact)]
                        count: ::core::primitive::u32,
                    },
                    #[codec(index = 2)]
                    Fraction {
                        #[codec(compact)]
                        nom: ::core::primitive::u32,
                        #[codec(compact)]
                        denom: ::core::primitive::u32,
                    },
                    #[codec(index = 3)]
                    AtLeastProportion {
                        #[codec(compact)]
                        nom: ::core::primitive::u32,
                        #[codec(compact)]
                        denom: ::core::primitive::u32,
                    },
                    #[codec(index = 4)]
                    MoreThanProportion {
                        #[codec(compact)]
                        nom: ::core::primitive::u32,
                        #[codec(compact)]
                        denom: ::core::primitive::u32,
                    },
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Junction {
                    #[codec(index = 0)]
                    Parachain(#[codec(compact)] ::core::primitive::u32),
                    #[codec(index = 1)]
                    AccountId32 {
                        network:
                            ::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
                        id: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    AccountIndex64 {
                        network:
                            ::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
                        #[codec(compact)]
                        index: ::core::primitive::u64,
                    },
                    #[codec(index = 3)]
                    AccountKey20 {
                        network:
                            ::core::option::Option<runtime_types::xcm::v3::junction::NetworkId>,
                        key: [::core::primitive::u8; 20usize],
                    },
                    #[codec(index = 4)]
                    PalletInstance(::core::primitive::u8),
                    #[codec(index = 5)]
                    GeneralIndex(#[codec(compact)] ::core::primitive::u128),
                    #[codec(index = 6)]
                    GeneralKey {
                        length: ::core::primitive::u8,
                        data: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 7)]
                    OnlyChild,
                    #[codec(index = 8)]
                    Plurality {
                        id: runtime_types::xcm::v3::junction::BodyId,
                        part: runtime_types::xcm::v3::junction::BodyPart,
                    },
                    #[codec(index = 9)]
                    GlobalConsensus(runtime_types::xcm::v3::junction::NetworkId),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum NetworkId {
                    #[codec(index = 0)]
                    ByGenesis([::core::primitive::u8; 32usize]),
                    #[codec(index = 1)]
                    ByFork {
                        block_number: ::core::primitive::u64,
                        block_hash: [::core::primitive::u8; 32usize],
                    },
                    #[codec(index = 2)]
                    Polkadot,
                    #[codec(index = 3)]
                    Kusama,
                    #[codec(index = 4)]
                    Westend,
                    #[codec(index = 5)]
                    Rococo,
                    #[codec(index = 6)]
                    Wococo,
                    #[codec(index = 7)]
                    Ethereum {
                        #[codec(compact)]
                        chain_id: ::core::primitive::u64,
                    },
                    #[codec(index = 8)]
                    BitcoinCore,
                    #[codec(index = 9)]
                    BitcoinCash,
                }
            }
            pub mod junctions {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Junctions {
                    #[codec(index = 0)]
                    Here,
                    #[codec(index = 1)]
                    X1(runtime_types::xcm::v3::junction::Junction),
                    #[codec(index = 2)]
                    X2(
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                    ),
                    #[codec(index = 3)]
                    X3(
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                    ),
                    #[codec(index = 4)]
                    X4(
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                    ),
                    #[codec(index = 5)]
                    X5(
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                    ),
                    #[codec(index = 6)]
                    X6(
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                    ),
                    #[codec(index = 7)]
                    X7(
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                    ),
                    #[codec(index = 8)]
                    X8(
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                        runtime_types::xcm::v3::junction::Junction,
                    ),
                }
            }
            pub mod multiasset {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum AssetId {
                    #[codec(index = 0)]
                    Concrete(runtime_types::xcm::v3::multilocation::MultiLocation),
                    #[codec(index = 1)]
                    Abstract([::core::primitive::u8; 32usize]),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum AssetInstance {
                    #[codec(index = 0)]
                    Undefined,
                    #[codec(index = 1)]
                    Index(#[codec(compact)] ::core::primitive::u128),
                    #[codec(index = 2)]
                    Array4([::core::primitive::u8; 4usize]),
                    #[codec(index = 3)]
                    Array8([::core::primitive::u8; 8usize]),
                    #[codec(index = 4)]
                    Array16([::core::primitive::u8; 16usize]),
                    #[codec(index = 5)]
                    Array32([::core::primitive::u8; 32usize]),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Fungibility {
                    #[codec(index = 0)]
                    Fungible(#[codec(compact)] ::core::primitive::u128),
                    #[codec(index = 1)]
                    NonFungible(runtime_types::xcm::v3::multiasset::AssetInstance),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MultiAsset {
                    pub id: runtime_types::xcm::v3::multiasset::AssetId,
                    pub fun: runtime_types::xcm::v3::multiasset::Fungibility,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum MultiAssetFilter {
                    #[codec(index = 0)]
                    Definite(runtime_types::xcm::v3::multiasset::MultiAssets),
                    #[codec(index = 1)]
                    Wild(runtime_types::xcm::v3::multiasset::WildMultiAsset),
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MultiAssets(
                    pub ::std::vec::Vec<runtime_types::xcm::v3::multiasset::MultiAsset>,
                );
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum WildFungibility {
                    #[codec(index = 0)]
                    Fungible,
                    #[codec(index = 1)]
                    NonFungible,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum WildMultiAsset {
                    #[codec(index = 0)]
                    All,
                    #[codec(index = 1)]
                    AllOf {
                        id: runtime_types::xcm::v3::multiasset::AssetId,
                        fun: runtime_types::xcm::v3::multiasset::WildFungibility,
                    },
                    #[codec(index = 2)]
                    AllCounted(#[codec(compact)] ::core::primitive::u32),
                    #[codec(index = 3)]
                    AllOfCounted {
                        id: runtime_types::xcm::v3::multiasset::AssetId,
                        fun: runtime_types::xcm::v3::multiasset::WildFungibility,
                        #[codec(compact)]
                        count: ::core::primitive::u32,
                    },
                }
            }
            pub mod multilocation {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub struct MultiLocation {
                    pub parents: ::core::primitive::u8,
                    pub interior: runtime_types::xcm::v3::junctions::Junctions,
                }
            }
            pub mod traits {
                use super::runtime_types;
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Error {
                    #[codec(index = 0)]
                    Overflow,
                    #[codec(index = 1)]
                    Unimplemented,
                    #[codec(index = 2)]
                    UntrustedReserveLocation,
                    #[codec(index = 3)]
                    UntrustedTeleportLocation,
                    #[codec(index = 4)]
                    LocationFull,
                    #[codec(index = 5)]
                    LocationNotInvertible,
                    #[codec(index = 6)]
                    BadOrigin,
                    #[codec(index = 7)]
                    InvalidLocation,
                    #[codec(index = 8)]
                    AssetNotFound,
                    #[codec(index = 9)]
                    FailedToTransactAsset,
                    #[codec(index = 10)]
                    NotWithdrawable,
                    #[codec(index = 11)]
                    LocationCannotHold,
                    #[codec(index = 12)]
                    ExceedsMaxMessageSize,
                    #[codec(index = 13)]
                    DestinationUnsupported,
                    #[codec(index = 14)]
                    Transport,
                    #[codec(index = 15)]
                    Unroutable,
                    #[codec(index = 16)]
                    UnknownClaim,
                    #[codec(index = 17)]
                    FailedToDecode,
                    #[codec(index = 18)]
                    MaxWeightInvalid,
                    #[codec(index = 19)]
                    NotHoldingFees,
                    #[codec(index = 20)]
                    TooExpensive,
                    #[codec(index = 21)]
                    Trap(::core::primitive::u64),
                    #[codec(index = 22)]
                    ExpectationFalse,
                    #[codec(index = 23)]
                    PalletNotFound,
                    #[codec(index = 24)]
                    NameMismatch,
                    #[codec(index = 25)]
                    VersionIncompatible,
                    #[codec(index = 26)]
                    HoldingWouldOverflow,
                    #[codec(index = 27)]
                    ExportError,
                    #[codec(index = 28)]
                    ReanchorFailed,
                    #[codec(index = 29)]
                    NoDeal,
                    #[codec(index = 30)]
                    FeesNotMet,
                    #[codec(index = 31)]
                    LockError,
                    #[codec(index = 32)]
                    NoPermission,
                    #[codec(index = 33)]
                    Unanchored,
                    #[codec(index = 34)]
                    NotDepositable,
                    #[codec(index = 35)]
                    UnhandledXcmVersion,
                    #[codec(index = 36)]
                    WeightLimitReached(runtime_types::sp_weights::weight_v2::Weight),
                    #[codec(index = 37)]
                    Barrier,
                    #[codec(index = 38)]
                    WeightNotComputable,
                    #[codec(index = 39)]
                    ExceedsStackLimit,
                }
                #[derive(
                    :: subxt :: ext :: codec :: Decode,
                    :: subxt :: ext :: codec :: Encode,
                    :: subxt :: ext :: scale_decode :: DecodeAsType,
                    :: subxt :: ext :: scale_encode :: EncodeAsType,
                    Debug,
                )]
                # [codec (crate = :: subxt :: ext :: codec)]
                #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
                #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
                pub enum Outcome {
                    #[codec(index = 0)]
                    Complete(runtime_types::sp_weights::weight_v2::Weight),
                    #[codec(index = 1)]
                    Incomplete(
                        runtime_types::sp_weights::weight_v2::Weight,
                        runtime_types::xcm::v3::traits::Error,
                    ),
                    #[codec(index = 2)]
                    Error(runtime_types::xcm::v3::traits::Error),
                }
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Instruction {
                #[codec(index = 0)]
                WithdrawAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                #[codec(index = 1)]
                ReserveAssetDeposited(runtime_types::xcm::v3::multiasset::MultiAssets),
                #[codec(index = 2)]
                ReceiveTeleportedAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                #[codec(index = 3)]
                QueryResponse {
                    #[codec(compact)]
                    query_id: ::core::primitive::u64,
                    response: runtime_types::xcm::v3::Response,
                    max_weight: runtime_types::sp_weights::weight_v2::Weight,
                    querier: ::core::option::Option<
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                    >,
                },
                #[codec(index = 4)]
                TransferAsset {
                    assets: runtime_types::xcm::v3::multiasset::MultiAssets,
                    beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
                },
                #[codec(index = 5)]
                TransferReserveAsset {
                    assets: runtime_types::xcm::v3::multiasset::MultiAssets,
                    dest: runtime_types::xcm::v3::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v3::Xcm,
                },
                #[codec(index = 6)]
                Transact {
                    origin_kind: runtime_types::xcm::v2::OriginKind,
                    require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
                    call: runtime_types::xcm::double_encoded::DoubleEncoded,
                },
                #[codec(index = 7)]
                HrmpNewChannelOpenRequest {
                    #[codec(compact)]
                    sender: ::core::primitive::u32,
                    #[codec(compact)]
                    max_message_size: ::core::primitive::u32,
                    #[codec(compact)]
                    max_capacity: ::core::primitive::u32,
                },
                #[codec(index = 8)]
                HrmpChannelAccepted {
                    #[codec(compact)]
                    recipient: ::core::primitive::u32,
                },
                #[codec(index = 9)]
                HrmpChannelClosing {
                    #[codec(compact)]
                    initiator: ::core::primitive::u32,
                    #[codec(compact)]
                    sender: ::core::primitive::u32,
                    #[codec(compact)]
                    recipient: ::core::primitive::u32,
                },
                #[codec(index = 10)]
                ClearOrigin,
                #[codec(index = 11)]
                DescendOrigin(runtime_types::xcm::v3::junctions::Junctions),
                #[codec(index = 12)]
                ReportError(runtime_types::xcm::v3::QueryResponseInfo),
                #[codec(index = 13)]
                DepositAsset {
                    assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                    beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
                },
                #[codec(index = 14)]
                DepositReserveAsset {
                    assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                    dest: runtime_types::xcm::v3::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v3::Xcm,
                },
                #[codec(index = 15)]
                ExchangeAsset {
                    give: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                    want: runtime_types::xcm::v3::multiasset::MultiAssets,
                    maximal: ::core::primitive::bool,
                },
                #[codec(index = 16)]
                InitiateReserveWithdraw {
                    assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                    reserve: runtime_types::xcm::v3::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v3::Xcm,
                },
                #[codec(index = 17)]
                InitiateTeleport {
                    assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                    dest: runtime_types::xcm::v3::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v3::Xcm,
                },
                #[codec(index = 18)]
                ReportHolding {
                    response_info: runtime_types::xcm::v3::QueryResponseInfo,
                    assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                },
                #[codec(index = 19)]
                BuyExecution {
                    fees: runtime_types::xcm::v3::multiasset::MultiAsset,
                    weight_limit: runtime_types::xcm::v3::WeightLimit,
                },
                #[codec(index = 20)]
                RefundSurplus,
                #[codec(index = 21)]
                SetErrorHandler(runtime_types::xcm::v3::Xcm),
                #[codec(index = 22)]
                SetAppendix(runtime_types::xcm::v3::Xcm),
                #[codec(index = 23)]
                ClearError,
                #[codec(index = 24)]
                ClaimAsset {
                    assets: runtime_types::xcm::v3::multiasset::MultiAssets,
                    ticket: runtime_types::xcm::v3::multilocation::MultiLocation,
                },
                #[codec(index = 25)]
                Trap(#[codec(compact)] ::core::primitive::u64),
                #[codec(index = 26)]
                SubscribeVersion {
                    #[codec(compact)]
                    query_id: ::core::primitive::u64,
                    max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
                },
                #[codec(index = 27)]
                UnsubscribeVersion,
                #[codec(index = 28)]
                BurnAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                #[codec(index = 29)]
                ExpectAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                #[codec(index = 30)]
                ExpectOrigin(
                    ::core::option::Option<runtime_types::xcm::v3::multilocation::MultiLocation>,
                ),
                #[codec(index = 31)]
                ExpectError(
                    ::core::option::Option<(
                        ::core::primitive::u32,
                        runtime_types::xcm::v3::traits::Error,
                    )>,
                ),
                #[codec(index = 32)]
                ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
                #[codec(index = 33)]
                QueryPallet {
                    module_name: ::std::vec::Vec<::core::primitive::u8>,
                    response_info: runtime_types::xcm::v3::QueryResponseInfo,
                },
                #[codec(index = 34)]
                ExpectPallet {
                    #[codec(compact)]
                    index: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    module_name: ::std::vec::Vec<::core::primitive::u8>,
                    #[codec(compact)]
                    crate_major: ::core::primitive::u32,
                    #[codec(compact)]
                    min_crate_minor: ::core::primitive::u32,
                },
                #[codec(index = 35)]
                ReportTransactStatus(runtime_types::xcm::v3::QueryResponseInfo),
                #[codec(index = 36)]
                ClearTransactStatus,
                #[codec(index = 37)]
                UniversalOrigin(runtime_types::xcm::v3::junction::Junction),
                #[codec(index = 38)]
                ExportMessage {
                    network: runtime_types::xcm::v3::junction::NetworkId,
                    destination: runtime_types::xcm::v3::junctions::Junctions,
                    xcm: runtime_types::xcm::v3::Xcm,
                },
                #[codec(index = 39)]
                LockAsset {
                    asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                    unlocker: runtime_types::xcm::v3::multilocation::MultiLocation,
                },
                #[codec(index = 40)]
                UnlockAsset {
                    asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                    target: runtime_types::xcm::v3::multilocation::MultiLocation,
                },
                #[codec(index = 41)]
                NoteUnlockable {
                    asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                    owner: runtime_types::xcm::v3::multilocation::MultiLocation,
                },
                #[codec(index = 42)]
                RequestUnlock {
                    asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                    locker: runtime_types::xcm::v3::multilocation::MultiLocation,
                },
                #[codec(index = 43)]
                SetFeesMode {
                    jit_withdraw: ::core::primitive::bool,
                },
                #[codec(index = 44)]
                SetTopic([::core::primitive::u8; 32usize]),
                #[codec(index = 45)]
                ClearTopic,
                #[codec(index = 46)]
                AliasOrigin(runtime_types::xcm::v3::multilocation::MultiLocation),
                #[codec(index = 47)]
                UnpaidExecution {
                    weight_limit: runtime_types::xcm::v3::WeightLimit,
                    check_origin: ::core::option::Option<
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                    >,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Instruction2 {
                #[codec(index = 0)]
                WithdrawAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                #[codec(index = 1)]
                ReserveAssetDeposited(runtime_types::xcm::v3::multiasset::MultiAssets),
                #[codec(index = 2)]
                ReceiveTeleportedAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                #[codec(index = 3)]
                QueryResponse {
                    #[codec(compact)]
                    query_id: ::core::primitive::u64,
                    response: runtime_types::xcm::v3::Response,
                    max_weight: runtime_types::sp_weights::weight_v2::Weight,
                    querier: ::core::option::Option<
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                    >,
                },
                #[codec(index = 4)]
                TransferAsset {
                    assets: runtime_types::xcm::v3::multiasset::MultiAssets,
                    beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
                },
                #[codec(index = 5)]
                TransferReserveAsset {
                    assets: runtime_types::xcm::v3::multiasset::MultiAssets,
                    dest: runtime_types::xcm::v3::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v3::Xcm,
                },
                #[codec(index = 6)]
                Transact {
                    origin_kind: runtime_types::xcm::v2::OriginKind,
                    require_weight_at_most: runtime_types::sp_weights::weight_v2::Weight,
                    call: runtime_types::xcm::double_encoded::DoubleEncoded2,
                },
                #[codec(index = 7)]
                HrmpNewChannelOpenRequest {
                    #[codec(compact)]
                    sender: ::core::primitive::u32,
                    #[codec(compact)]
                    max_message_size: ::core::primitive::u32,
                    #[codec(compact)]
                    max_capacity: ::core::primitive::u32,
                },
                #[codec(index = 8)]
                HrmpChannelAccepted {
                    #[codec(compact)]
                    recipient: ::core::primitive::u32,
                },
                #[codec(index = 9)]
                HrmpChannelClosing {
                    #[codec(compact)]
                    initiator: ::core::primitive::u32,
                    #[codec(compact)]
                    sender: ::core::primitive::u32,
                    #[codec(compact)]
                    recipient: ::core::primitive::u32,
                },
                #[codec(index = 10)]
                ClearOrigin,
                #[codec(index = 11)]
                DescendOrigin(runtime_types::xcm::v3::junctions::Junctions),
                #[codec(index = 12)]
                ReportError(runtime_types::xcm::v3::QueryResponseInfo),
                #[codec(index = 13)]
                DepositAsset {
                    assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                    beneficiary: runtime_types::xcm::v3::multilocation::MultiLocation,
                },
                #[codec(index = 14)]
                DepositReserveAsset {
                    assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                    dest: runtime_types::xcm::v3::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v3::Xcm,
                },
                #[codec(index = 15)]
                ExchangeAsset {
                    give: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                    want: runtime_types::xcm::v3::multiasset::MultiAssets,
                    maximal: ::core::primitive::bool,
                },
                #[codec(index = 16)]
                InitiateReserveWithdraw {
                    assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                    reserve: runtime_types::xcm::v3::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v3::Xcm,
                },
                #[codec(index = 17)]
                InitiateTeleport {
                    assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                    dest: runtime_types::xcm::v3::multilocation::MultiLocation,
                    xcm: runtime_types::xcm::v3::Xcm,
                },
                #[codec(index = 18)]
                ReportHolding {
                    response_info: runtime_types::xcm::v3::QueryResponseInfo,
                    assets: runtime_types::xcm::v3::multiasset::MultiAssetFilter,
                },
                #[codec(index = 19)]
                BuyExecution {
                    fees: runtime_types::xcm::v3::multiasset::MultiAsset,
                    weight_limit: runtime_types::xcm::v3::WeightLimit,
                },
                #[codec(index = 20)]
                RefundSurplus,
                #[codec(index = 21)]
                SetErrorHandler(runtime_types::xcm::v3::Xcm2),
                #[codec(index = 22)]
                SetAppendix(runtime_types::xcm::v3::Xcm2),
                #[codec(index = 23)]
                ClearError,
                #[codec(index = 24)]
                ClaimAsset {
                    assets: runtime_types::xcm::v3::multiasset::MultiAssets,
                    ticket: runtime_types::xcm::v3::multilocation::MultiLocation,
                },
                #[codec(index = 25)]
                Trap(#[codec(compact)] ::core::primitive::u64),
                #[codec(index = 26)]
                SubscribeVersion {
                    #[codec(compact)]
                    query_id: ::core::primitive::u64,
                    max_response_weight: runtime_types::sp_weights::weight_v2::Weight,
                },
                #[codec(index = 27)]
                UnsubscribeVersion,
                #[codec(index = 28)]
                BurnAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                #[codec(index = 29)]
                ExpectAsset(runtime_types::xcm::v3::multiasset::MultiAssets),
                #[codec(index = 30)]
                ExpectOrigin(
                    ::core::option::Option<runtime_types::xcm::v3::multilocation::MultiLocation>,
                ),
                #[codec(index = 31)]
                ExpectError(
                    ::core::option::Option<(
                        ::core::primitive::u32,
                        runtime_types::xcm::v3::traits::Error,
                    )>,
                ),
                #[codec(index = 32)]
                ExpectTransactStatus(runtime_types::xcm::v3::MaybeErrorCode),
                #[codec(index = 33)]
                QueryPallet {
                    module_name: ::std::vec::Vec<::core::primitive::u8>,
                    response_info: runtime_types::xcm::v3::QueryResponseInfo,
                },
                #[codec(index = 34)]
                ExpectPallet {
                    #[codec(compact)]
                    index: ::core::primitive::u32,
                    name: ::std::vec::Vec<::core::primitive::u8>,
                    module_name: ::std::vec::Vec<::core::primitive::u8>,
                    #[codec(compact)]
                    crate_major: ::core::primitive::u32,
                    #[codec(compact)]
                    min_crate_minor: ::core::primitive::u32,
                },
                #[codec(index = 35)]
                ReportTransactStatus(runtime_types::xcm::v3::QueryResponseInfo),
                #[codec(index = 36)]
                ClearTransactStatus,
                #[codec(index = 37)]
                UniversalOrigin(runtime_types::xcm::v3::junction::Junction),
                #[codec(index = 38)]
                ExportMessage {
                    network: runtime_types::xcm::v3::junction::NetworkId,
                    destination: runtime_types::xcm::v3::junctions::Junctions,
                    xcm: runtime_types::xcm::v3::Xcm,
                },
                #[codec(index = 39)]
                LockAsset {
                    asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                    unlocker: runtime_types::xcm::v3::multilocation::MultiLocation,
                },
                #[codec(index = 40)]
                UnlockAsset {
                    asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                    target: runtime_types::xcm::v3::multilocation::MultiLocation,
                },
                #[codec(index = 41)]
                NoteUnlockable {
                    asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                    owner: runtime_types::xcm::v3::multilocation::MultiLocation,
                },
                #[codec(index = 42)]
                RequestUnlock {
                    asset: runtime_types::xcm::v3::multiasset::MultiAsset,
                    locker: runtime_types::xcm::v3::multilocation::MultiLocation,
                },
                #[codec(index = 43)]
                SetFeesMode {
                    jit_withdraw: ::core::primitive::bool,
                },
                #[codec(index = 44)]
                SetTopic([::core::primitive::u8; 32usize]),
                #[codec(index = 45)]
                ClearTopic,
                #[codec(index = 46)]
                AliasOrigin(runtime_types::xcm::v3::multilocation::MultiLocation),
                #[codec(index = 47)]
                UnpaidExecution {
                    weight_limit: runtime_types::xcm::v3::WeightLimit,
                    check_origin: ::core::option::Option<
                        runtime_types::xcm::v3::multilocation::MultiLocation,
                    >,
                },
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum MaybeErrorCode {
                #[codec(index = 0)]
                Success,
                #[codec(index = 1)]
                Error(
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ),
                #[codec(index = 2)]
                TruncatedError(
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        ::core::primitive::u8,
                    >,
                ),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct PalletInfo {
                #[codec(compact)]
                pub index: ::core::primitive::u32,
                pub name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                pub module_name: runtime_types::bounded_collections::bounded_vec::BoundedVec<
                    ::core::primitive::u8,
                >,
                #[codec(compact)]
                pub major: ::core::primitive::u32,
                #[codec(compact)]
                pub minor: ::core::primitive::u32,
                #[codec(compact)]
                pub patch: ::core::primitive::u32,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct QueryResponseInfo {
                pub destination: runtime_types::xcm::v3::multilocation::MultiLocation,
                #[codec(compact)]
                pub query_id: ::core::primitive::u64,
                pub max_weight: runtime_types::sp_weights::weight_v2::Weight,
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum Response {
                #[codec(index = 0)]
                Null,
                #[codec(index = 1)]
                Assets(runtime_types::xcm::v3::multiasset::MultiAssets),
                #[codec(index = 2)]
                ExecutionResult(
                    ::core::option::Option<(
                        ::core::primitive::u32,
                        runtime_types::xcm::v3::traits::Error,
                    )>,
                ),
                #[codec(index = 3)]
                Version(::core::primitive::u32),
                #[codec(index = 4)]
                PalletsInfo(
                    runtime_types::bounded_collections::bounded_vec::BoundedVec<
                        runtime_types::xcm::v3::PalletInfo,
                    >,
                ),
                #[codec(index = 5)]
                DispatchResult(runtime_types::xcm::v3::MaybeErrorCode),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub enum WeightLimit {
                #[codec(index = 0)]
                Unlimited,
                #[codec(index = 1)]
                Limited(runtime_types::sp_weights::weight_v2::Weight),
            }
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Xcm(pub ::std::vec::Vec<runtime_types::xcm::v3::Instruction>);
            #[derive(
                :: subxt :: ext :: codec :: Decode,
                :: subxt :: ext :: codec :: Encode,
                :: subxt :: ext :: scale_decode :: DecodeAsType,
                :: subxt :: ext :: scale_encode :: EncodeAsType,
                Debug,
            )]
            # [codec (crate = :: subxt :: ext :: codec)]
            #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
            #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
            pub struct Xcm2(pub ::std::vec::Vec<runtime_types::xcm::v3::Instruction2>);
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum VersionedAssetId {
            #[codec(index = 3)]
            V3(runtime_types::xcm::v3::multiasset::AssetId),
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum VersionedMultiAssets {
            #[codec(index = 1)]
            V2(runtime_types::xcm::v2::multiasset::MultiAssets),
            #[codec(index = 3)]
            V3(runtime_types::xcm::v3::multiasset::MultiAssets),
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum VersionedMultiLocation {
            #[codec(index = 1)]
            V2(runtime_types::xcm::v2::multilocation::MultiLocation),
            #[codec(index = 3)]
            V3(runtime_types::xcm::v3::multilocation::MultiLocation),
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum VersionedResponse {
            #[codec(index = 2)]
            V2(runtime_types::xcm::v2::Response),
            #[codec(index = 3)]
            V3(runtime_types::xcm::v3::Response),
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum VersionedXcm {
            #[codec(index = 2)]
            V2(runtime_types::xcm::v2::Xcm),
            #[codec(index = 3)]
            V3(runtime_types::xcm::v3::Xcm),
        }
        #[derive(
            :: subxt :: ext :: codec :: Decode,
            :: subxt :: ext :: codec :: Encode,
            :: subxt :: ext :: scale_decode :: DecodeAsType,
            :: subxt :: ext :: scale_encode :: EncodeAsType,
            Debug,
        )]
        # [codec (crate = :: subxt :: ext :: codec)]
        #[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
        #[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
        pub enum VersionedXcm2 {
            #[codec(index = 2)]
            V2(runtime_types::xcm::v2::Xcm2),
            #[codec(index = 3)]
            V3(runtime_types::xcm::v3::Xcm2),
        }
    }
}
