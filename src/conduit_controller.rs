pub use conduitcontroller_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod conduitcontroller_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "ConduitController was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static CONDUITCONTROLLER_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"CallerIsNotNewPotentialOwner\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"CallerIsNotOwner\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ChannelOutOfRange\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ConduitAlreadyExists\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"InvalidCreator\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"InvalidInitialOwner\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newPotentialOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"NewPotentialOwnerAlreadySet\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"NewPotentialOwnerIsZeroAddress\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoConduit\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"NoPotentialOwnerCurrentlySet\",\n    \"type\": \"error\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"conduitKey\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"NewConduit\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"previousOwner\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnershipTransferred\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"newPotentialOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"PotentialOwnerUpdated\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"acceptOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"cancelOwnershipTransfer\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"conduitKey\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"initialOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"createConduit\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"channelIndex\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getChannel\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"channel\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"channel\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getChannelStatus\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"isOpen\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getChannels\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"channels\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"conduitKey\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"getConduit\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"exists\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getConduitCodeHashes\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"creationCodeHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"runtimeCodeHash\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getKey\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"conduitKey\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getPotentialOwner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"potentialOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getTotalChannels\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"totalChannels\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"ownerOf\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"newPotentialOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"transferOwnership\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"channel\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"isOpen\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"updateChannel\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]") . expect ("invalid abi")
        });
    pub struct ConduitController<M>(ethers::contract::Contract<M>);
    impl<M> Clone for ConduitController<M> {
        fn clone(&self) -> Self {
            ConduitController(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for ConduitController<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for ConduitController<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(ConduitController))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> ConduitController<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), CONDUITCONTROLLER_ABI.clone(), client)
                .into()
        }
        #[doc = "Calls the contract's `acceptOwnership` (0x51710e45) function"]
        pub fn accept_ownership(
            &self,
            conduit: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 113, 14, 69], conduit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancelOwnershipTransfer` (0x7b37e561) function"]
        pub fn cancel_ownership_transfer(
            &self,
            conduit: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 55, 229, 97], conduit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createConduit` (0x794593bc) function"]
        pub fn create_conduit(
            &self,
            conduit_key: [u8; 32],
            initial_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([121, 69, 147, 188], (conduit_key, initial_owner))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getChannel` (0x027cc764) function"]
        pub fn get_channel(
            &self,
            conduit: ethers::core::types::Address,
            channel_index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([2, 124, 199, 100], (conduit, channel_index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getChannelStatus` (0x33bc8572) function"]
        pub fn get_channel_status(
            &self,
            conduit: ethers::core::types::Address,
            channel: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([51, 188, 133, 114], (conduit, channel))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getChannels` (0x8b9e028b) function"]
        pub fn get_channels(
            &self,
            conduit: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([139, 158, 2, 139], conduit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getConduit` (0x6e9bfd9f) function"]
        pub fn get_conduit(
            &self,
            conduit_key: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, (ethers::core::types::Address, bool)>
        {
            self.0
                .method_hash([110, 155, 253, 159], conduit_key)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getConduitCodeHashes` (0x0a96ad39) function"]
        pub fn get_conduit_code_hashes(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ([u8; 32], [u8; 32])> {
            self.0
                .method_hash([10, 150, 173, 57], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getKey` (0x93790f44) function"]
        pub fn get_key(
            &self,
            conduit: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([147, 121, 15, 68], conduit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getPotentialOwner` (0x906c87cc) function"]
        pub fn get_potential_owner(
            &self,
            conduit: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([144, 108, 135, 204], conduit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTotalChannels` (0x4e3f9580) function"]
        pub fn get_total_channels(
            &self,
            conduit: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([78, 63, 149, 128], conduit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `ownerOf` (0x14afd79e) function"]
        pub fn owner_of(
            &self,
            conduit: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([20, 175, 215, 158], conduit)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0x6d435421) function"]
        pub fn transfer_ownership(
            &self,
            conduit: ethers::core::types::Address,
            new_potential_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 67, 84, 33], (conduit, new_potential_owner))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateChannel` (0x13ad9cab) function"]
        pub fn update_channel(
            &self,
            conduit: ethers::core::types::Address,
            channel: ethers::core::types::Address,
            is_open: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 173, 156, 171], (conduit, channel, is_open))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NewConduit` event"]
        pub fn new_conduit_filter(&self) -> ethers::contract::builders::Event<M, NewConduitFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `PotentialOwnerUpdated` event"]
        pub fn potential_owner_updated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, PotentialOwnerUpdatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, ConduitControllerEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for ConduitController<M>
    {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "NewConduit", abi = "NewConduit(address,bytes32)")]
    pub struct NewConduitFilter {
        pub conduit: ethers::core::types::Address,
        pub conduit_key: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub conduit: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "PotentialOwnerUpdated", abi = "PotentialOwnerUpdated(address)")]
    pub struct PotentialOwnerUpdatedFilter {
        #[ethevent(indexed)]
        pub new_potential_owner: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ConduitControllerEvents {
        NewConduitFilter(NewConduitFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PotentialOwnerUpdatedFilter(PotentialOwnerUpdatedFilter),
    }
    impl ethers::contract::EthLogDecode for ConduitControllerEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = NewConduitFilter::decode_log(log) {
                return Ok(ConduitControllerEvents::NewConduitFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(ConduitControllerEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = PotentialOwnerUpdatedFilter::decode_log(log) {
                return Ok(ConduitControllerEvents::PotentialOwnerUpdatedFilter(
                    decoded,
                ));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for ConduitControllerEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ConduitControllerEvents::NewConduitFilter(element) => element.fmt(f),
                ConduitControllerEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                ConduitControllerEvents::PotentialOwnerUpdatedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `acceptOwnership`function with signature `acceptOwnership(address)` and selector `[81, 113, 14, 69]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "acceptOwnership", abi = "acceptOwnership(address)")]
    pub struct AcceptOwnershipCall {
        pub conduit: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `cancelOwnershipTransfer`function with signature `cancelOwnershipTransfer(address)` and selector `[123, 55, 229, 97]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "cancelOwnershipTransfer",
        abi = "cancelOwnershipTransfer(address)"
    )]
    pub struct CancelOwnershipTransferCall {
        pub conduit: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `createConduit`function with signature `createConduit(bytes32,address)` and selector `[121, 69, 147, 188]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "createConduit", abi = "createConduit(bytes32,address)")]
    pub struct CreateConduitCall {
        pub conduit_key: [u8; 32],
        pub initial_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getChannel`function with signature `getChannel(address,uint256)` and selector `[2, 124, 199, 100]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getChannel", abi = "getChannel(address,uint256)")]
    pub struct GetChannelCall {
        pub conduit: ethers::core::types::Address,
        pub channel_index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getChannelStatus`function with signature `getChannelStatus(address,address)` and selector `[51, 188, 133, 114]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getChannelStatus", abi = "getChannelStatus(address,address)")]
    pub struct GetChannelStatusCall {
        pub conduit: ethers::core::types::Address,
        pub channel: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getChannels`function with signature `getChannels(address)` and selector `[139, 158, 2, 139]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getChannels", abi = "getChannels(address)")]
    pub struct GetChannelsCall {
        pub conduit: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getConduit`function with signature `getConduit(bytes32)` and selector `[110, 155, 253, 159]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getConduit", abi = "getConduit(bytes32)")]
    pub struct GetConduitCall {
        pub conduit_key: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getConduitCodeHashes`function with signature `getConduitCodeHashes()` and selector `[10, 150, 173, 57]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getConduitCodeHashes", abi = "getConduitCodeHashes()")]
    pub struct GetConduitCodeHashesCall;
    #[doc = "Container type for all input parameters for the `getKey`function with signature `getKey(address)` and selector `[147, 121, 15, 68]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getKey", abi = "getKey(address)")]
    pub struct GetKeyCall {
        pub conduit: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getPotentialOwner`function with signature `getPotentialOwner(address)` and selector `[144, 108, 135, 204]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getPotentialOwner", abi = "getPotentialOwner(address)")]
    pub struct GetPotentialOwnerCall {
        pub conduit: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getTotalChannels`function with signature `getTotalChannels(address)` and selector `[78, 63, 149, 128]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getTotalChannels", abi = "getTotalChannels(address)")]
    pub struct GetTotalChannelsCall {
        pub conduit: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `ownerOf`function with signature `ownerOf(address)` and selector `[20, 175, 215, 158]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "ownerOf", abi = "ownerOf(address)")]
    pub struct OwnerOfCall {
        pub conduit: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership`function with signature `transferOwnership(address,address)` and selector `[109, 67, 84, 33]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address,address)")]
    pub struct TransferOwnershipCall {
        pub conduit: ethers::core::types::Address,
        pub new_potential_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `updateChannel`function with signature `updateChannel(address,address,bool)` and selector `[19, 173, 156, 171]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "updateChannel", abi = "updateChannel(address,address,bool)")]
    pub struct UpdateChannelCall {
        pub conduit: ethers::core::types::Address,
        pub channel: ethers::core::types::Address,
        pub is_open: bool,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum ConduitControllerCalls {
        AcceptOwnership(AcceptOwnershipCall),
        CancelOwnershipTransfer(CancelOwnershipTransferCall),
        CreateConduit(CreateConduitCall),
        GetChannel(GetChannelCall),
        GetChannelStatus(GetChannelStatusCall),
        GetChannels(GetChannelsCall),
        GetConduit(GetConduitCall),
        GetConduitCodeHashes(GetConduitCodeHashesCall),
        GetKey(GetKeyCall),
        GetPotentialOwner(GetPotentialOwnerCall),
        GetTotalChannels(GetTotalChannelsCall),
        OwnerOf(OwnerOfCall),
        TransferOwnership(TransferOwnershipCall),
        UpdateChannel(UpdateChannelCall),
    }
    impl ethers::core::abi::AbiDecode for ConduitControllerCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <AcceptOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConduitControllerCalls::AcceptOwnership(decoded));
            }
            if let Ok(decoded) =
                <CancelOwnershipTransferCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConduitControllerCalls::CancelOwnershipTransfer(decoded));
            }
            if let Ok(decoded) =
                <CreateConduitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConduitControllerCalls::CreateConduit(decoded));
            }
            if let Ok(decoded) =
                <GetChannelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConduitControllerCalls::GetChannel(decoded));
            }
            if let Ok(decoded) =
                <GetChannelStatusCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConduitControllerCalls::GetChannelStatus(decoded));
            }
            if let Ok(decoded) =
                <GetChannelsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConduitControllerCalls::GetChannels(decoded));
            }
            if let Ok(decoded) =
                <GetConduitCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConduitControllerCalls::GetConduit(decoded));
            }
            if let Ok(decoded) =
                <GetConduitCodeHashesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConduitControllerCalls::GetConduitCodeHashes(decoded));
            }
            if let Ok(decoded) = <GetKeyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConduitControllerCalls::GetKey(decoded));
            }
            if let Ok(decoded) =
                <GetPotentialOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConduitControllerCalls::GetPotentialOwner(decoded));
            }
            if let Ok(decoded) =
                <GetTotalChannelsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConduitControllerCalls::GetTotalChannels(decoded));
            }
            if let Ok(decoded) =
                <OwnerOfCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConduitControllerCalls::OwnerOf(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConduitControllerCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <UpdateChannelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(ConduitControllerCalls::UpdateChannel(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for ConduitControllerCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                ConduitControllerCalls::AcceptOwnership(element) => element.encode(),
                ConduitControllerCalls::CancelOwnershipTransfer(element) => element.encode(),
                ConduitControllerCalls::CreateConduit(element) => element.encode(),
                ConduitControllerCalls::GetChannel(element) => element.encode(),
                ConduitControllerCalls::GetChannelStatus(element) => element.encode(),
                ConduitControllerCalls::GetChannels(element) => element.encode(),
                ConduitControllerCalls::GetConduit(element) => element.encode(),
                ConduitControllerCalls::GetConduitCodeHashes(element) => element.encode(),
                ConduitControllerCalls::GetKey(element) => element.encode(),
                ConduitControllerCalls::GetPotentialOwner(element) => element.encode(),
                ConduitControllerCalls::GetTotalChannels(element) => element.encode(),
                ConduitControllerCalls::OwnerOf(element) => element.encode(),
                ConduitControllerCalls::TransferOwnership(element) => element.encode(),
                ConduitControllerCalls::UpdateChannel(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for ConduitControllerCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                ConduitControllerCalls::AcceptOwnership(element) => element.fmt(f),
                ConduitControllerCalls::CancelOwnershipTransfer(element) => element.fmt(f),
                ConduitControllerCalls::CreateConduit(element) => element.fmt(f),
                ConduitControllerCalls::GetChannel(element) => element.fmt(f),
                ConduitControllerCalls::GetChannelStatus(element) => element.fmt(f),
                ConduitControllerCalls::GetChannels(element) => element.fmt(f),
                ConduitControllerCalls::GetConduit(element) => element.fmt(f),
                ConduitControllerCalls::GetConduitCodeHashes(element) => element.fmt(f),
                ConduitControllerCalls::GetKey(element) => element.fmt(f),
                ConduitControllerCalls::GetPotentialOwner(element) => element.fmt(f),
                ConduitControllerCalls::GetTotalChannels(element) => element.fmt(f),
                ConduitControllerCalls::OwnerOf(element) => element.fmt(f),
                ConduitControllerCalls::TransferOwnership(element) => element.fmt(f),
                ConduitControllerCalls::UpdateChannel(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<AcceptOwnershipCall> for ConduitControllerCalls {
        fn from(var: AcceptOwnershipCall) -> Self {
            ConduitControllerCalls::AcceptOwnership(var)
        }
    }
    impl ::std::convert::From<CancelOwnershipTransferCall> for ConduitControllerCalls {
        fn from(var: CancelOwnershipTransferCall) -> Self {
            ConduitControllerCalls::CancelOwnershipTransfer(var)
        }
    }
    impl ::std::convert::From<CreateConduitCall> for ConduitControllerCalls {
        fn from(var: CreateConduitCall) -> Self {
            ConduitControllerCalls::CreateConduit(var)
        }
    }
    impl ::std::convert::From<GetChannelCall> for ConduitControllerCalls {
        fn from(var: GetChannelCall) -> Self {
            ConduitControllerCalls::GetChannel(var)
        }
    }
    impl ::std::convert::From<GetChannelStatusCall> for ConduitControllerCalls {
        fn from(var: GetChannelStatusCall) -> Self {
            ConduitControllerCalls::GetChannelStatus(var)
        }
    }
    impl ::std::convert::From<GetChannelsCall> for ConduitControllerCalls {
        fn from(var: GetChannelsCall) -> Self {
            ConduitControllerCalls::GetChannels(var)
        }
    }
    impl ::std::convert::From<GetConduitCall> for ConduitControllerCalls {
        fn from(var: GetConduitCall) -> Self {
            ConduitControllerCalls::GetConduit(var)
        }
    }
    impl ::std::convert::From<GetConduitCodeHashesCall> for ConduitControllerCalls {
        fn from(var: GetConduitCodeHashesCall) -> Self {
            ConduitControllerCalls::GetConduitCodeHashes(var)
        }
    }
    impl ::std::convert::From<GetKeyCall> for ConduitControllerCalls {
        fn from(var: GetKeyCall) -> Self {
            ConduitControllerCalls::GetKey(var)
        }
    }
    impl ::std::convert::From<GetPotentialOwnerCall> for ConduitControllerCalls {
        fn from(var: GetPotentialOwnerCall) -> Self {
            ConduitControllerCalls::GetPotentialOwner(var)
        }
    }
    impl ::std::convert::From<GetTotalChannelsCall> for ConduitControllerCalls {
        fn from(var: GetTotalChannelsCall) -> Self {
            ConduitControllerCalls::GetTotalChannels(var)
        }
    }
    impl ::std::convert::From<OwnerOfCall> for ConduitControllerCalls {
        fn from(var: OwnerOfCall) -> Self {
            ConduitControllerCalls::OwnerOf(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for ConduitControllerCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            ConduitControllerCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<UpdateChannelCall> for ConduitControllerCalls {
        fn from(var: UpdateChannelCall) -> Self {
            ConduitControllerCalls::UpdateChannel(var)
        }
    }
}
