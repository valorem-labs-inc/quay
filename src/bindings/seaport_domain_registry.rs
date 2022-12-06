pub use seaportdomainregistry_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod seaportdomainregistry_mod {
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
    #[doc = "SeaportDomainRegistry was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static SEAPORTDOMAINREGISTRY_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n    {\n        \"inputs\": [\n            { \"internalType\": \"string\", \"name\": \"domain\", \"type\": \"string\" }\n        ],\n        \"name\": \"DomainAlreadyRegistered\",\n        \"type\": \"error\"\n    },\n    {\n        \"inputs\": [\n            { \"internalType\": \"bytes4\", \"name\": \"tag\", \"type\": \"bytes4\" },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"maxIndex\",\n                \"type\": \"uint256\"\n            },\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"suppliedIndex\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"DomainIndexOutOfRange\",\n        \"type\": \"error\"\n    },\n    {\n        \"anonymous\": false,\n        \"inputs\": [\n            {\n                \"indexed\": false,\n                \"internalType\": \"string\",\n                \"name\": \"domain\",\n                \"type\": \"string\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"bytes4\",\n                \"name\": \"tag\",\n                \"type\": \"bytes4\"\n            },\n            {\n                \"indexed\": false,\n                \"internalType\": \"uint256\",\n                \"name\": \"index\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"name\": \"DomainRegistered\",\n        \"type\": \"event\"\n    },\n    {\n        \"inputs\": [\n            { \"internalType\": \"bytes4\", \"name\": \"tag\", \"type\": \"bytes4\" },\n            { \"internalType\": \"uint256\", \"name\": \"index\", \"type\": \"uint256\" }\n        ],\n        \"name\": \"getDomain\",\n        \"outputs\": [\n            { \"internalType\": \"string\", \"name\": \"domain\", \"type\": \"string\" }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            { \"internalType\": \"bytes4\", \"name\": \"tag\", \"type\": \"bytes4\" }\n        ],\n        \"name\": \"getDomains\",\n        \"outputs\": [\n            {\n                \"internalType\": \"string[]\",\n                \"name\": \"domains\",\n                \"type\": \"string[]\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            { \"internalType\": \"bytes4\", \"name\": \"tag\", \"type\": \"bytes4\" }\n        ],\n        \"name\": \"getNumberOfDomains\",\n        \"outputs\": [\n            {\n                \"internalType\": \"uint256\",\n                \"name\": \"totalDomains\",\n                \"type\": \"uint256\"\n            }\n        ],\n        \"stateMutability\": \"view\",\n        \"type\": \"function\"\n    },\n    {\n        \"inputs\": [\n            { \"internalType\": \"string\", \"name\": \"domain\", \"type\": \"string\" }\n        ],\n        \"name\": \"setDomain\",\n        \"outputs\": [\n            { \"internalType\": \"bytes4\", \"name\": \"tag\", \"type\": \"bytes4\" }\n        ],\n        \"stateMutability\": \"nonpayable\",\n        \"type\": \"function\"\n    }\n]\n") . expect ("invalid abi")
        });
    pub struct SeaportDomainRegistry<M>(ethers::contract::Contract<M>);
    impl<M> Clone for SeaportDomainRegistry<M> {
        fn clone(&self) -> Self {
            SeaportDomainRegistry(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for SeaportDomainRegistry<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for SeaportDomainRegistry<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(SeaportDomainRegistry))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> SeaportDomainRegistry<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(
                address.into(),
                SEAPORTDOMAINREGISTRY_ABI.clone(),
                client,
            )
            .into()
        }
        #[doc = "Calls the contract's `getDomain` (0xeab5fc24) function"]
        pub fn get_domain(
            &self,
            tag: [u8; 4],
            index: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([234, 181, 252, 36], (tag, index))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getDomains` (0xd45619b6) function"]
        pub fn get_domains(
            &self,
            tag: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<String>> {
            self.0
                .method_hash([212, 86, 25, 182], tag)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNumberOfDomains` (0x432ba75c) function"]
        pub fn get_number_of_domains(
            &self,
            tag: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([67, 43, 167, 92], tag)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setDomain` (0xe5eab096) function"]
        pub fn set_domain(
            &self,
            domain: String,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([229, 234, 176, 150], domain)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `DomainRegistered` event"]
        pub fn domain_registered_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, DomainRegisteredFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, DomainRegisteredFilter> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>>
        for SeaportDomainRegistry<M>
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
    #[ethevent(
        name = "DomainRegistered",
        abi = "DomainRegistered(string,bytes4,uint256)"
    )]
    pub struct DomainRegisteredFilter {
        pub domain: String,
        pub tag: [u8; 4],
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getDomain` function with signature `getDomain(bytes4,uint256)` and selector `[234, 181, 252, 36]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getDomain", abi = "getDomain(bytes4,uint256)")]
    pub struct GetDomainCall {
        pub tag: [u8; 4],
        pub index: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getDomains` function with signature `getDomains(bytes4)` and selector `[212, 86, 25, 182]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getDomains", abi = "getDomains(bytes4)")]
    pub struct GetDomainsCall {
        pub tag: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `getNumberOfDomains` function with signature `getNumberOfDomains(bytes4)` and selector `[67, 43, 167, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getNumberOfDomains", abi = "getNumberOfDomains(bytes4)")]
    pub struct GetNumberOfDomainsCall {
        pub tag: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `setDomain` function with signature `setDomain(string)` and selector `[229, 234, 176, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "setDomain", abi = "setDomain(string)")]
    pub struct SetDomainCall {
        pub domain: String,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum SeaportDomainRegistryCalls {
        GetDomain(GetDomainCall),
        GetDomains(GetDomainsCall),
        GetNumberOfDomains(GetNumberOfDomainsCall),
        SetDomain(SetDomainCall),
    }
    impl ethers::core::abi::AbiDecode for SeaportDomainRegistryCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <GetDomainCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportDomainRegistryCalls::GetDomain(decoded));
            }
            if let Ok(decoded) =
                <GetDomainsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportDomainRegistryCalls::GetDomains(decoded));
            }
            if let Ok(decoded) =
                <GetNumberOfDomainsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportDomainRegistryCalls::GetNumberOfDomains(decoded));
            }
            if let Ok(decoded) =
                <SetDomainCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportDomainRegistryCalls::SetDomain(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for SeaportDomainRegistryCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                SeaportDomainRegistryCalls::GetDomain(element) => element.encode(),
                SeaportDomainRegistryCalls::GetDomains(element) => element.encode(),
                SeaportDomainRegistryCalls::GetNumberOfDomains(element) => element.encode(),
                SeaportDomainRegistryCalls::SetDomain(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for SeaportDomainRegistryCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                SeaportDomainRegistryCalls::GetDomain(element) => element.fmt(f),
                SeaportDomainRegistryCalls::GetDomains(element) => element.fmt(f),
                SeaportDomainRegistryCalls::GetNumberOfDomains(element) => element.fmt(f),
                SeaportDomainRegistryCalls::SetDomain(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<GetDomainCall> for SeaportDomainRegistryCalls {
        fn from(var: GetDomainCall) -> Self {
            SeaportDomainRegistryCalls::GetDomain(var)
        }
    }
    impl ::std::convert::From<GetDomainsCall> for SeaportDomainRegistryCalls {
        fn from(var: GetDomainsCall) -> Self {
            SeaportDomainRegistryCalls::GetDomains(var)
        }
    }
    impl ::std::convert::From<GetNumberOfDomainsCall> for SeaportDomainRegistryCalls {
        fn from(var: GetNumberOfDomainsCall) -> Self {
            SeaportDomainRegistryCalls::GetNumberOfDomains(var)
        }
    }
    impl ::std::convert::From<SetDomainCall> for SeaportDomainRegistryCalls {
        fn from(var: SetDomainCall) -> Self {
            SeaportDomainRegistryCalls::SetDomain(var)
        }
    }
    #[doc = "Container type for all return fields from the `getDomain` function with signature `getDomain(bytes4,uint256)` and selector `[234, 181, 252, 36]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetDomainReturn {
        pub domain: String,
    }
    #[doc = "Container type for all return fields from the `getDomains` function with signature `getDomains(bytes4)` and selector `[212, 86, 25, 182]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetDomainsReturn {
        pub domains: ::std::vec::Vec<String>,
    }
    #[doc = "Container type for all return fields from the `getNumberOfDomains` function with signature `getNumberOfDomains(bytes4)` and selector `[67, 43, 167, 92]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct GetNumberOfDomainsReturn {
        pub total_domains: ethers::core::types::U256,
    }
    #[doc = "Container type for all return fields from the `setDomain` function with signature `setDomain(string)` and selector `[229, 234, 176, 150]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SetDomainReturn {
        pub tag: [u8; 4],
    }
}
