pub use seaport_mod::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod seaport_mod {
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
    #[doc = "Seaport was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static SEAPORT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduitController\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"BadContractSignature\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"BadFraction\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"BadReturnValueFromERC20OnTransfer\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint8\",\n        \"name\": \"v\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"name\": \"BadSignatureV\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"ConsiderationCriteriaResolverOutOfRange\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"orderIndex\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"considerationIndex\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"shortfallAmount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"ConsiderationNotMet\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"CriteriaNotEnabledForItem\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"identifiers\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"amounts\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"name\": \"ERC1155BatchTransferGenericFailure\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"EtherTransferGenericFailure\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"InexactFraction\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"InsufficientEtherSupplied\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"Invalid1155BatchTransferEncoding\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"InvalidBasicOrderParameterEncoding\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"InvalidCallToConduit\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"InvalidCanceller\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"conduitKey\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduit\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"InvalidConduit\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"InvalidERC721TransferAmount\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"InvalidFulfillmentComponentData\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"InvalidMsgValue\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"InvalidNativeOfferItem\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"InvalidProof\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"orderHash\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"InvalidRestrictedOrder\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"InvalidSignature\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"InvalidSigner\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"InvalidTime\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"MismatchedFulfillmentOfferAndConsiderationComponents\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"enum Side\",\n        \"name\": \"side\",\n        \"type\": \"uint8\"\n      }\n    ],\n    \"name\": \"MissingFulfillmentComponentOnAggregation\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"MissingItemAmount\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"MissingOriginalConsiderationItems\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"NoContract\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoReentrantCalls\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"NoSpecifiedOrdersAvailable\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"OfferAndConsiderationRequiredOnFulfillment\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"OfferCriteriaResolverOutOfRange\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"orderHash\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"OrderAlreadyFilled\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"OrderCriteriaResolverOutOfRange\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"orderHash\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"OrderIsCancelled\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"orderHash\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"OrderPartiallyFilled\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"PartialFillsNotEnabledForOrder\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"token\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"from\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"identifier\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"amount\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"TokenTransferGenericFailure\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"UnresolvedConsiderationCriteria\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"UnresolvedOfferCriteria\",\n    \"type\": \"error\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"UnusedItemParameters\",\n    \"type\": \"error\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"newCounter\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"offerer\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"CounterIncremented\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"orderHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"offerer\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"zone\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OrderCancelled\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"orderHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"offerer\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"zone\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"enum ItemType\",\n            \"name\": \"itemType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"token\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"identifier\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"indexed\": false,\n        \"internalType\": \"struct SpentItem[]\",\n        \"name\": \"offer\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"enum ItemType\",\n            \"name\": \"itemType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"token\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"identifier\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"amount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address payable\",\n            \"name\": \"recipient\",\n            \"type\": \"address\"\n          }\n        ],\n        \"indexed\": false,\n        \"internalType\": \"struct ReceivedItem[]\",\n        \"name\": \"consideration\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"OrderFulfilled\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"orderHash\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"offerer\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"zone\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OrderValidated\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"offerer\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"zone\",\n            \"type\": \"address\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"enum ItemType\",\n                \"name\": \"itemType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"identifierOrCriteria\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"startAmount\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"endAmount\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct OfferItem[]\",\n            \"name\": \"offer\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"enum ItemType\",\n                \"name\": \"itemType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"identifierOrCriteria\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"startAmount\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"endAmount\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"address payable\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n              }\n            ],\n            \"internalType\": \"struct ConsiderationItem[]\",\n            \"name\": \"consideration\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"internalType\": \"enum OrderType\",\n            \"name\": \"orderType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"startTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"endTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"zoneHash\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"salt\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"conduitKey\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"counter\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct OrderComponents[]\",\n        \"name\": \"orders\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"cancel\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"cancelled\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"offerer\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"zone\",\n                \"type\": \"address\"\n              },\n              {\n                \"components\": [\n                  {\n                    \"internalType\": \"enum ItemType\",\n                    \"name\": \"itemType\",\n                    \"type\": \"uint8\"\n                  },\n                  {\n                    \"internalType\": \"address\",\n                    \"name\": \"token\",\n                    \"type\": \"address\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"identifierOrCriteria\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"startAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"endAmount\",\n                    \"type\": \"uint256\"\n                  }\n                ],\n                \"internalType\": \"struct OfferItem[]\",\n                \"name\": \"offer\",\n                \"type\": \"tuple[]\"\n              },\n              {\n                \"components\": [\n                  {\n                    \"internalType\": \"enum ItemType\",\n                    \"name\": \"itemType\",\n                    \"type\": \"uint8\"\n                  },\n                  {\n                    \"internalType\": \"address\",\n                    \"name\": \"token\",\n                    \"type\": \"address\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"identifierOrCriteria\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"startAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"endAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"address payable\",\n                    \"name\": \"recipient\",\n                    \"type\": \"address\"\n                  }\n                ],\n                \"internalType\": \"struct ConsiderationItem[]\",\n                \"name\": \"consideration\",\n                \"type\": \"tuple[]\"\n              },\n              {\n                \"internalType\": \"enum OrderType\",\n                \"name\": \"orderType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"startTime\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"endTime\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"zoneHash\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"salt\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"conduitKey\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"totalOriginalConsiderationItems\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct OrderParameters\",\n            \"name\": \"parameters\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"uint120\",\n            \"name\": \"numerator\",\n            \"type\": \"uint120\"\n          },\n          {\n            \"internalType\": \"uint120\",\n            \"name\": \"denominator\",\n            \"type\": \"uint120\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"signature\",\n            \"type\": \"bytes\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"extraData\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct AdvancedOrder\",\n        \"name\": \"advancedOrder\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"orderIndex\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum Side\",\n            \"name\": \"side\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"index\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"identifier\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bytes32[]\",\n            \"name\": \"criteriaProof\",\n            \"type\": \"bytes32[]\"\n          }\n        ],\n        \"internalType\": \"struct CriteriaResolver[]\",\n        \"name\": \"criteriaResolvers\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"fulfillerConduitKey\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"fulfillAdvancedOrder\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"fulfilled\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"offerer\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"zone\",\n                \"type\": \"address\"\n              },\n              {\n                \"components\": [\n                  {\n                    \"internalType\": \"enum ItemType\",\n                    \"name\": \"itemType\",\n                    \"type\": \"uint8\"\n                  },\n                  {\n                    \"internalType\": \"address\",\n                    \"name\": \"token\",\n                    \"type\": \"address\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"identifierOrCriteria\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"startAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"endAmount\",\n                    \"type\": \"uint256\"\n                  }\n                ],\n                \"internalType\": \"struct OfferItem[]\",\n                \"name\": \"offer\",\n                \"type\": \"tuple[]\"\n              },\n              {\n                \"components\": [\n                  {\n                    \"internalType\": \"enum ItemType\",\n                    \"name\": \"itemType\",\n                    \"type\": \"uint8\"\n                  },\n                  {\n                    \"internalType\": \"address\",\n                    \"name\": \"token\",\n                    \"type\": \"address\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"identifierOrCriteria\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"startAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"endAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"address payable\",\n                    \"name\": \"recipient\",\n                    \"type\": \"address\"\n                  }\n                ],\n                \"internalType\": \"struct ConsiderationItem[]\",\n                \"name\": \"consideration\",\n                \"type\": \"tuple[]\"\n              },\n              {\n                \"internalType\": \"enum OrderType\",\n                \"name\": \"orderType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"startTime\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"endTime\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"zoneHash\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"salt\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"conduitKey\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"totalOriginalConsiderationItems\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct OrderParameters\",\n            \"name\": \"parameters\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"uint120\",\n            \"name\": \"numerator\",\n            \"type\": \"uint120\"\n          },\n          {\n            \"internalType\": \"uint120\",\n            \"name\": \"denominator\",\n            \"type\": \"uint120\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"signature\",\n            \"type\": \"bytes\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"extraData\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct AdvancedOrder[]\",\n        \"name\": \"advancedOrders\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"orderIndex\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum Side\",\n            \"name\": \"side\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"index\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"identifier\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bytes32[]\",\n            \"name\": \"criteriaProof\",\n            \"type\": \"bytes32[]\"\n          }\n        ],\n        \"internalType\": \"struct CriteriaResolver[]\",\n        \"name\": \"criteriaResolvers\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"orderIndex\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"itemIndex\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct FulfillmentComponent[][]\",\n        \"name\": \"offerFulfillments\",\n        \"type\": \"tuple[][]\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"orderIndex\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"itemIndex\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct FulfillmentComponent[][]\",\n        \"name\": \"considerationFulfillments\",\n        \"type\": \"tuple[][]\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"fulfillerConduitKey\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"recipient\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"maximumFulfilled\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"fulfillAvailableAdvancedOrders\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool[]\",\n        \"name\": \"availableOrders\",\n        \"type\": \"bool[]\"\n      },\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"enum ItemType\",\n                \"name\": \"itemType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"identifier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"address payable\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n              }\n            ],\n            \"internalType\": \"struct ReceivedItem\",\n            \"name\": \"item\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"offerer\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"conduitKey\",\n            \"type\": \"bytes32\"\n          }\n        ],\n        \"internalType\": \"struct Execution[]\",\n        \"name\": \"executions\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"offerer\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"zone\",\n                \"type\": \"address\"\n              },\n              {\n                \"components\": [\n                  {\n                    \"internalType\": \"enum ItemType\",\n                    \"name\": \"itemType\",\n                    \"type\": \"uint8\"\n                  },\n                  {\n                    \"internalType\": \"address\",\n                    \"name\": \"token\",\n                    \"type\": \"address\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"identifierOrCriteria\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"startAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"endAmount\",\n                    \"type\": \"uint256\"\n                  }\n                ],\n                \"internalType\": \"struct OfferItem[]\",\n                \"name\": \"offer\",\n                \"type\": \"tuple[]\"\n              },\n              {\n                \"components\": [\n                  {\n                    \"internalType\": \"enum ItemType\",\n                    \"name\": \"itemType\",\n                    \"type\": \"uint8\"\n                  },\n                  {\n                    \"internalType\": \"address\",\n                    \"name\": \"token\",\n                    \"type\": \"address\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"identifierOrCriteria\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"startAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"endAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"address payable\",\n                    \"name\": \"recipient\",\n                    \"type\": \"address\"\n                  }\n                ],\n                \"internalType\": \"struct ConsiderationItem[]\",\n                \"name\": \"consideration\",\n                \"type\": \"tuple[]\"\n              },\n              {\n                \"internalType\": \"enum OrderType\",\n                \"name\": \"orderType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"startTime\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"endTime\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"zoneHash\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"salt\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"conduitKey\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"totalOriginalConsiderationItems\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct OrderParameters\",\n            \"name\": \"parameters\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"signature\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct Order[]\",\n        \"name\": \"orders\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"orderIndex\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"itemIndex\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct FulfillmentComponent[][]\",\n        \"name\": \"offerFulfillments\",\n        \"type\": \"tuple[][]\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"orderIndex\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"itemIndex\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct FulfillmentComponent[][]\",\n        \"name\": \"considerationFulfillments\",\n        \"type\": \"tuple[][]\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"fulfillerConduitKey\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"maximumFulfilled\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"fulfillAvailableOrders\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool[]\",\n        \"name\": \"availableOrders\",\n        \"type\": \"bool[]\"\n      },\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"enum ItemType\",\n                \"name\": \"itemType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"identifier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"address payable\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n              }\n            ],\n            \"internalType\": \"struct ReceivedItem\",\n            \"name\": \"item\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"offerer\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"conduitKey\",\n            \"type\": \"bytes32\"\n          }\n        ],\n        \"internalType\": \"struct Execution[]\",\n        \"name\": \"executions\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"considerationToken\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"considerationIdentifier\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"considerationAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"address payable\",\n            \"name\": \"offerer\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"zone\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"offerToken\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"offerIdentifier\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"offerAmount\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum BasicOrderType\",\n            \"name\": \"basicOrderType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"startTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"endTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"zoneHash\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"salt\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"offererConduitKey\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"fulfillerConduitKey\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"totalOriginalAdditionalRecipients\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"address payable\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n              }\n            ],\n            \"internalType\": \"struct AdditionalRecipient[]\",\n            \"name\": \"additionalRecipients\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"signature\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct BasicOrderParameters\",\n        \"name\": \"parameters\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"fulfillBasicOrder\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"fulfilled\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"offerer\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"zone\",\n                \"type\": \"address\"\n              },\n              {\n                \"components\": [\n                  {\n                    \"internalType\": \"enum ItemType\",\n                    \"name\": \"itemType\",\n                    \"type\": \"uint8\"\n                  },\n                  {\n                    \"internalType\": \"address\",\n                    \"name\": \"token\",\n                    \"type\": \"address\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"identifierOrCriteria\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"startAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"endAmount\",\n                    \"type\": \"uint256\"\n                  }\n                ],\n                \"internalType\": \"struct OfferItem[]\",\n                \"name\": \"offer\",\n                \"type\": \"tuple[]\"\n              },\n              {\n                \"components\": [\n                  {\n                    \"internalType\": \"enum ItemType\",\n                    \"name\": \"itemType\",\n                    \"type\": \"uint8\"\n                  },\n                  {\n                    \"internalType\": \"address\",\n                    \"name\": \"token\",\n                    \"type\": \"address\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"identifierOrCriteria\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"startAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"endAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"address payable\",\n                    \"name\": \"recipient\",\n                    \"type\": \"address\"\n                  }\n                ],\n                \"internalType\": \"struct ConsiderationItem[]\",\n                \"name\": \"consideration\",\n                \"type\": \"tuple[]\"\n              },\n              {\n                \"internalType\": \"enum OrderType\",\n                \"name\": \"orderType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"startTime\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"endTime\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"zoneHash\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"salt\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"conduitKey\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"totalOriginalConsiderationItems\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct OrderParameters\",\n            \"name\": \"parameters\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"signature\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct Order\",\n        \"name\": \"order\",\n        \"type\": \"tuple\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"fulfillerConduitKey\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"fulfillOrder\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"fulfilled\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"offerer\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"getCounter\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"counter\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"offerer\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"zone\",\n            \"type\": \"address\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"enum ItemType\",\n                \"name\": \"itemType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"identifierOrCriteria\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"startAmount\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"endAmount\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct OfferItem[]\",\n            \"name\": \"offer\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"enum ItemType\",\n                \"name\": \"itemType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"identifierOrCriteria\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"startAmount\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"endAmount\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"address payable\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n              }\n            ],\n            \"internalType\": \"struct ConsiderationItem[]\",\n            \"name\": \"consideration\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"internalType\": \"enum OrderType\",\n            \"name\": \"orderType\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"startTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"endTime\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"zoneHash\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"salt\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"conduitKey\",\n            \"type\": \"bytes32\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"counter\",\n            \"type\": \"uint256\"\n          }\n        ],\n        \"internalType\": \"struct OrderComponents\",\n        \"name\": \"order\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"name\": \"getOrderHash\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"orderHash\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"orderHash\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"getOrderStatus\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"isValidated\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"isCancelled\",\n        \"type\": \"bool\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"totalFilled\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"totalSize\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"incrementCounter\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"newCounter\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"information\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"version\",\n        \"type\": \"string\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"domainSeparator\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"conduitController\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"offerer\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"zone\",\n                \"type\": \"address\"\n              },\n              {\n                \"components\": [\n                  {\n                    \"internalType\": \"enum ItemType\",\n                    \"name\": \"itemType\",\n                    \"type\": \"uint8\"\n                  },\n                  {\n                    \"internalType\": \"address\",\n                    \"name\": \"token\",\n                    \"type\": \"address\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"identifierOrCriteria\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"startAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"endAmount\",\n                    \"type\": \"uint256\"\n                  }\n                ],\n                \"internalType\": \"struct OfferItem[]\",\n                \"name\": \"offer\",\n                \"type\": \"tuple[]\"\n              },\n              {\n                \"components\": [\n                  {\n                    \"internalType\": \"enum ItemType\",\n                    \"name\": \"itemType\",\n                    \"type\": \"uint8\"\n                  },\n                  {\n                    \"internalType\": \"address\",\n                    \"name\": \"token\",\n                    \"type\": \"address\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"identifierOrCriteria\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"startAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"endAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"address payable\",\n                    \"name\": \"recipient\",\n                    \"type\": \"address\"\n                  }\n                ],\n                \"internalType\": \"struct ConsiderationItem[]\",\n                \"name\": \"consideration\",\n                \"type\": \"tuple[]\"\n              },\n              {\n                \"internalType\": \"enum OrderType\",\n                \"name\": \"orderType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"startTime\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"endTime\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"zoneHash\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"salt\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"conduitKey\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"totalOriginalConsiderationItems\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct OrderParameters\",\n            \"name\": \"parameters\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"uint120\",\n            \"name\": \"numerator\",\n            \"type\": \"uint120\"\n          },\n          {\n            \"internalType\": \"uint120\",\n            \"name\": \"denominator\",\n            \"type\": \"uint120\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"signature\",\n            \"type\": \"bytes\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"extraData\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct AdvancedOrder[]\",\n        \"name\": \"advancedOrders\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"components\": [\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"orderIndex\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"enum Side\",\n            \"name\": \"side\",\n            \"type\": \"uint8\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"index\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"identifier\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bytes32[]\",\n            \"name\": \"criteriaProof\",\n            \"type\": \"bytes32[]\"\n          }\n        ],\n        \"internalType\": \"struct CriteriaResolver[]\",\n        \"name\": \"criteriaResolvers\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"orderIndex\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"itemIndex\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct FulfillmentComponent[]\",\n            \"name\": \"offerComponents\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"orderIndex\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"itemIndex\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct FulfillmentComponent[]\",\n            \"name\": \"considerationComponents\",\n            \"type\": \"tuple[]\"\n          }\n        ],\n        \"internalType\": \"struct Fulfillment[]\",\n        \"name\": \"fulfillments\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"matchAdvancedOrders\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"enum ItemType\",\n                \"name\": \"itemType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"identifier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"address payable\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n              }\n            ],\n            \"internalType\": \"struct ReceivedItem\",\n            \"name\": \"item\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"offerer\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"conduitKey\",\n            \"type\": \"bytes32\"\n          }\n        ],\n        \"internalType\": \"struct Execution[]\",\n        \"name\": \"executions\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"offerer\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"zone\",\n                \"type\": \"address\"\n              },\n              {\n                \"components\": [\n                  {\n                    \"internalType\": \"enum ItemType\",\n                    \"name\": \"itemType\",\n                    \"type\": \"uint8\"\n                  },\n                  {\n                    \"internalType\": \"address\",\n                    \"name\": \"token\",\n                    \"type\": \"address\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"identifierOrCriteria\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"startAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"endAmount\",\n                    \"type\": \"uint256\"\n                  }\n                ],\n                \"internalType\": \"struct OfferItem[]\",\n                \"name\": \"offer\",\n                \"type\": \"tuple[]\"\n              },\n              {\n                \"components\": [\n                  {\n                    \"internalType\": \"enum ItemType\",\n                    \"name\": \"itemType\",\n                    \"type\": \"uint8\"\n                  },\n                  {\n                    \"internalType\": \"address\",\n                    \"name\": \"token\",\n                    \"type\": \"address\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"identifierOrCriteria\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"startAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"endAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"address payable\",\n                    \"name\": \"recipient\",\n                    \"type\": \"address\"\n                  }\n                ],\n                \"internalType\": \"struct ConsiderationItem[]\",\n                \"name\": \"consideration\",\n                \"type\": \"tuple[]\"\n              },\n              {\n                \"internalType\": \"enum OrderType\",\n                \"name\": \"orderType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"startTime\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"endTime\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"zoneHash\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"salt\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"conduitKey\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"totalOriginalConsiderationItems\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct OrderParameters\",\n            \"name\": \"parameters\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"signature\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct Order[]\",\n        \"name\": \"orders\",\n        \"type\": \"tuple[]\"\n      },\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"orderIndex\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"itemIndex\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct FulfillmentComponent[]\",\n            \"name\": \"offerComponents\",\n            \"type\": \"tuple[]\"\n          },\n          {\n            \"components\": [\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"orderIndex\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"itemIndex\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct FulfillmentComponent[]\",\n            \"name\": \"considerationComponents\",\n            \"type\": \"tuple[]\"\n          }\n        ],\n        \"internalType\": \"struct Fulfillment[]\",\n        \"name\": \"fulfillments\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"matchOrders\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"enum ItemType\",\n                \"name\": \"itemType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"token\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"identifier\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"amount\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"address payable\",\n                \"name\": \"recipient\",\n                \"type\": \"address\"\n              }\n            ],\n            \"internalType\": \"struct ReceivedItem\",\n            \"name\": \"item\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"address\",\n            \"name\": \"offerer\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes32\",\n            \"name\": \"conduitKey\",\n            \"type\": \"bytes32\"\n          }\n        ],\n        \"internalType\": \"struct Execution[]\",\n        \"name\": \"executions\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"name\",\n    \"outputs\": [\n      {\n        \"internalType\": \"string\",\n        \"name\": \"contractName\",\n        \"type\": \"string\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"components\": [\n          {\n            \"components\": [\n              {\n                \"internalType\": \"address\",\n                \"name\": \"offerer\",\n                \"type\": \"address\"\n              },\n              {\n                \"internalType\": \"address\",\n                \"name\": \"zone\",\n                \"type\": \"address\"\n              },\n              {\n                \"components\": [\n                  {\n                    \"internalType\": \"enum ItemType\",\n                    \"name\": \"itemType\",\n                    \"type\": \"uint8\"\n                  },\n                  {\n                    \"internalType\": \"address\",\n                    \"name\": \"token\",\n                    \"type\": \"address\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"identifierOrCriteria\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"startAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"endAmount\",\n                    \"type\": \"uint256\"\n                  }\n                ],\n                \"internalType\": \"struct OfferItem[]\",\n                \"name\": \"offer\",\n                \"type\": \"tuple[]\"\n              },\n              {\n                \"components\": [\n                  {\n                    \"internalType\": \"enum ItemType\",\n                    \"name\": \"itemType\",\n                    \"type\": \"uint8\"\n                  },\n                  {\n                    \"internalType\": \"address\",\n                    \"name\": \"token\",\n                    \"type\": \"address\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"identifierOrCriteria\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"startAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"uint256\",\n                    \"name\": \"endAmount\",\n                    \"type\": \"uint256\"\n                  },\n                  {\n                    \"internalType\": \"address payable\",\n                    \"name\": \"recipient\",\n                    \"type\": \"address\"\n                  }\n                ],\n                \"internalType\": \"struct ConsiderationItem[]\",\n                \"name\": \"consideration\",\n                \"type\": \"tuple[]\"\n              },\n              {\n                \"internalType\": \"enum OrderType\",\n                \"name\": \"orderType\",\n                \"type\": \"uint8\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"startTime\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"endTime\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"zoneHash\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"salt\",\n                \"type\": \"uint256\"\n              },\n              {\n                \"internalType\": \"bytes32\",\n                \"name\": \"conduitKey\",\n                \"type\": \"bytes32\"\n              },\n              {\n                \"internalType\": \"uint256\",\n                \"name\": \"totalOriginalConsiderationItems\",\n                \"type\": \"uint256\"\n              }\n            ],\n            \"internalType\": \"struct OrderParameters\",\n            \"name\": \"parameters\",\n            \"type\": \"tuple\"\n          },\n          {\n            \"internalType\": \"bytes\",\n            \"name\": \"signature\",\n            \"type\": \"bytes\"\n          }\n        ],\n        \"internalType\": \"struct Order[]\",\n        \"name\": \"orders\",\n        \"type\": \"tuple[]\"\n      }\n    ],\n    \"name\": \"validate\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"validated\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]") . expect ("invalid abi")
        });
    pub struct Seaport<M>(ethers::contract::Contract<M>);
    impl<M> Clone for Seaport<M> {
        fn clone(&self) -> Self {
            Seaport(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for Seaport<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for Seaport<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(Seaport))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> Seaport<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), SEAPORT_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `cancel` (0xfd9f1e10) function"]
        pub fn cancel(
            &self,
            orders: ::std::vec::Vec<OrderComponents>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([253, 159, 30, 16], orders)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fulfillAdvancedOrder` (0xe7acab24) function"]
        pub fn fulfill_advanced_order(
            &self,
            advanced_order: AdvancedOrder,
            criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
            fulfiller_conduit_key: [u8; 32],
            recipient: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [231, 172, 171, 36],
                    (
                        advanced_order,
                        criteria_resolvers,
                        fulfiller_conduit_key,
                        recipient,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fulfillAvailableAdvancedOrders` (0x87201b41) function"]
        pub fn fulfill_available_advanced_orders(
            &self,
            advanced_orders: ::std::vec::Vec<AdvancedOrder>,
            criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
            offer_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
            consideration_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
            fulfiller_conduit_key: [u8; 32],
            recipient: ethers::core::types::Address,
            maximum_fulfilled: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<bool>, ::std::vec::Vec<Execution>),
        > {
            self.0
                .method_hash(
                    [135, 32, 27, 65],
                    (
                        advanced_orders,
                        criteria_resolvers,
                        offer_fulfillments,
                        consideration_fulfillments,
                        fulfiller_conduit_key,
                        recipient,
                        maximum_fulfilled,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fulfillAvailableOrders` (0xed98a574) function"]
        pub fn fulfill_available_orders(
            &self,
            orders: ::std::vec::Vec<Order>,
            offer_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
            consideration_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
            fulfiller_conduit_key: [u8; 32],
            maximum_fulfilled: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<bool>, ::std::vec::Vec<Execution>),
        > {
            self.0
                .method_hash(
                    [237, 152, 165, 116],
                    (
                        orders,
                        offer_fulfillments,
                        consideration_fulfillments,
                        fulfiller_conduit_key,
                        maximum_fulfilled,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fulfillBasicOrder` (0xfb0f3ee1) function"]
        pub fn fulfill_basic_order(
            &self,
            parameters: BasicOrderParameters,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([251, 15, 62, 225], (parameters,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `fulfillOrder` (0xb3a34c4c) function"]
        pub fn fulfill_order(
            &self,
            order: Order,
            fulfiller_conduit_key: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([179, 163, 76, 76], (order, fulfiller_conduit_key))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCounter` (0xf07ec373) function"]
        pub fn get_counter(
            &self,
            offerer: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([240, 126, 195, 115], offerer)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOrderHash` (0x79df72bd) function"]
        pub fn get_order_hash(
            &self,
            order: OrderComponents,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([121, 223, 114, 189], (order,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOrderStatus` (0x46423aa7) function"]
        pub fn get_order_status(
            &self,
            order_hash: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                bool,
                bool,
                ethers::core::types::U256,
                ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([70, 66, 58, 167], order_hash)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `incrementCounter` (0x5b34b966) function"]
        pub fn increment_counter(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([91, 52, 185, 102], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `information` (0xf47b7740) function"]
        pub fn information(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (String, [u8; 32], ethers::core::types::Address),
        > {
            self.0
                .method_hash([244, 123, 119, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `matchAdvancedOrders` (0x55944a42) function"]
        pub fn match_advanced_orders(
            &self,
            advanced_orders: ::std::vec::Vec<AdvancedOrder>,
            criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
            fulfillments: ::std::vec::Vec<Fulfillment>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Execution>> {
            self.0
                .method_hash(
                    [85, 148, 74, 66],
                    (advanced_orders, criteria_resolvers, fulfillments),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `matchOrders` (0xa8174404) function"]
        pub fn match_orders(
            &self,
            orders: ::std::vec::Vec<Order>,
            fulfillments: ::std::vec::Vec<Fulfillment>,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Execution>> {
            self.0
                .method_hash([168, 23, 68, 4], (orders, fulfillments))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `name` (0x06fdde03) function"]
        pub fn name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `validate` (0x88147732) function"]
        pub fn validate(
            &self,
            orders: ::std::vec::Vec<Order>,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([136, 20, 119, 50], orders)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `CounterIncremented` event"]
        pub fn counter_incremented_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CounterIncrementedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OrderCancelled` event"]
        pub fn order_cancelled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OrderCancelledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OrderFulfilled` event"]
        pub fn order_fulfilled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OrderFulfilledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OrderValidated` event"]
        pub fn order_validated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OrderValidatedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, SeaportEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for Seaport<M> {
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
        name = "CounterIncremented",
        abi = "CounterIncremented(uint256,address)"
    )]
    pub struct CounterIncrementedFilter {
        pub new_counter: ethers::core::types::U256,
        #[ethevent(indexed)]
        pub offerer: ethers::core::types::Address,
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
        name = "OrderCancelled",
        abi = "OrderCancelled(bytes32,address,address)"
    )]
    pub struct OrderCancelledFilter {
        pub order_hash: [u8; 32],
        #[ethevent(indexed)]
        pub offerer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub zone: ethers::core::types::Address,
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
        name = "OrderFulfilled",
        abi = "OrderFulfilled(bytes32,address,address,address,(uint8,address,uint256,uint256)[],(uint8,address,uint256,uint256,address)[])"
    )]
    pub struct OrderFulfilledFilter {
        pub order_hash: [u8; 32],
        #[ethevent(indexed)]
        pub offerer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub zone: ethers::core::types::Address,
        pub recipient: ethers::core::types::Address,
        pub offer: Vec<(
            u8,
            ethers::core::types::Address,
            ethers::core::types::U256,
            ethers::core::types::U256,
        )>,
        pub consideration: Vec<(
            u8,
            ethers::core::types::Address,
            ethers::core::types::U256,
            ethers::core::types::U256,
            ethers::core::types::Address,
        )>,
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
        name = "OrderValidated",
        abi = "OrderValidated(bytes32,address,address)"
    )]
    pub struct OrderValidatedFilter {
        pub order_hash: [u8; 32],
        #[ethevent(indexed)]
        pub offerer: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub zone: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum SeaportEvents {
        CounterIncrementedFilter(CounterIncrementedFilter),
        OrderCancelledFilter(OrderCancelledFilter),
        OrderFulfilledFilter(OrderFulfilledFilter),
        OrderValidatedFilter(OrderValidatedFilter),
    }
    impl ethers::contract::EthLogDecode for SeaportEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = CounterIncrementedFilter::decode_log(log) {
                return Ok(SeaportEvents::CounterIncrementedFilter(decoded));
            }
            if let Ok(decoded) = OrderCancelledFilter::decode_log(log) {
                return Ok(SeaportEvents::OrderCancelledFilter(decoded));
            }
            if let Ok(decoded) = OrderFulfilledFilter::decode_log(log) {
                return Ok(SeaportEvents::OrderFulfilledFilter(decoded));
            }
            if let Ok(decoded) = OrderValidatedFilter::decode_log(log) {
                return Ok(SeaportEvents::OrderValidatedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for SeaportEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                SeaportEvents::CounterIncrementedFilter(element) => element.fmt(f),
                SeaportEvents::OrderCancelledFilter(element) => element.fmt(f),
                SeaportEvents::OrderFulfilledFilter(element) => element.fmt(f),
                SeaportEvents::OrderValidatedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `cancel`function with signature `cancel((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256)[])` and selector `[253, 159, 30, 16]`"]
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
        name = "cancel",
        abi = "cancel((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256)[])"
    )]
    pub struct CancelCall {
        pub orders: ::std::vec::Vec<OrderComponents>,
    }
    #[doc = "Container type for all input parameters for the `fulfillAdvancedOrder`function with signature `fulfillAdvancedOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes),(uint256,uint8,uint256,uint256,bytes32[])[],bytes32,address)` and selector `[231, 172, 171, 36]`"]
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
        name = "fulfillAdvancedOrder",
        abi = "fulfillAdvancedOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes),(uint256,uint8,uint256,uint256,bytes32[])[],bytes32,address)"
    )]
    pub struct FulfillAdvancedOrderCall {
        pub advanced_order: AdvancedOrder,
        pub criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
        pub fulfiller_conduit_key: [u8; 32],
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `fulfillAvailableAdvancedOrders`function with signature `fulfillAvailableAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,address,uint256)` and selector `[135, 32, 27, 65]`"]
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
        name = "fulfillAvailableAdvancedOrders",
        abi = "fulfillAvailableAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,address,uint256)"
    )]
    pub struct FulfillAvailableAdvancedOrdersCall {
        pub advanced_orders: ::std::vec::Vec<AdvancedOrder>,
        pub criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
        pub offer_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
        pub consideration_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
        pub fulfiller_conduit_key: [u8; 32],
        pub recipient: ethers::core::types::Address,
        pub maximum_fulfilled: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `fulfillAvailableOrders`function with signature `fulfillAvailableOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,uint256)` and selector `[237, 152, 165, 116]`"]
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
        name = "fulfillAvailableOrders",
        abi = "fulfillAvailableOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,uint256)"
    )]
    pub struct FulfillAvailableOrdersCall {
        pub orders: ::std::vec::Vec<Order>,
        pub offer_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
        pub consideration_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
        pub fulfiller_conduit_key: [u8; 32],
        pub maximum_fulfilled: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `fulfillBasicOrder`function with signature `fulfillBasicOrder((address,uint256,uint256,address,address,address,uint256,uint256,uint8,uint256,uint256,bytes32,uint256,bytes32,bytes32,uint256,(uint256,address)[],bytes))` and selector `[251, 15, 62, 225]`"]
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
        name = "fulfillBasicOrder",
        abi = "fulfillBasicOrder((address,uint256,uint256,address,address,address,uint256,uint256,uint8,uint256,uint256,bytes32,uint256,bytes32,bytes32,uint256,(uint256,address)[],bytes))"
    )]
    pub struct FulfillBasicOrderCall {
        pub parameters: BasicOrderParameters,
    }
    #[doc = "Container type for all input parameters for the `fulfillOrder`function with signature `fulfillOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes),bytes32)` and selector `[179, 163, 76, 76]`"]
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
        name = "fulfillOrder",
        abi = "fulfillOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes),bytes32)"
    )]
    pub struct FulfillOrderCall {
        pub order: Order,
        pub fulfiller_conduit_key: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getCounter`function with signature `getCounter(address)` and selector `[240, 126, 195, 115]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getCounter", abi = "getCounter(address)")]
    pub struct GetCounterCall {
        pub offerer: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `getOrderHash`function with signature `getOrderHash((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256))` and selector `[121, 223, 114, 189]`"]
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
        name = "getOrderHash",
        abi = "getOrderHash((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256))"
    )]
    pub struct GetOrderHashCall {
        pub order: OrderComponents,
    }
    #[doc = "Container type for all input parameters for the `getOrderStatus`function with signature `getOrderStatus(bytes32)` and selector `[70, 66, 58, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getOrderStatus", abi = "getOrderStatus(bytes32)")]
    pub struct GetOrderStatusCall {
        pub order_hash: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `incrementCounter`function with signature `incrementCounter()` and selector `[91, 52, 185, 102]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "incrementCounter", abi = "incrementCounter()")]
    pub struct IncrementCounterCall;
    #[doc = "Container type for all input parameters for the `information`function with signature `information()` and selector `[244, 123, 119, 64]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "information", abi = "information()")]
    pub struct InformationCall;
    #[doc = "Container type for all input parameters for the `matchAdvancedOrders`function with signature `matchAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],((uint256,uint256)[],(uint256,uint256)[])[])` and selector `[85, 148, 74, 66]`"]
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
        name = "matchAdvancedOrders",
        abi = "matchAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],((uint256,uint256)[],(uint256,uint256)[])[])"
    )]
    pub struct MatchAdvancedOrdersCall {
        pub advanced_orders: ::std::vec::Vec<AdvancedOrder>,
        pub criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
        pub fulfillments: ::std::vec::Vec<Fulfillment>,
    }
    #[doc = "Container type for all input parameters for the `matchOrders`function with signature `matchOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],((uint256,uint256)[],(uint256,uint256)[])[])` and selector `[168, 23, 68, 4]`"]
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
        name = "matchOrders",
        abi = "matchOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],((uint256,uint256)[],(uint256,uint256)[])[])"
    )]
    pub struct MatchOrdersCall {
        pub orders: ::std::vec::Vec<Order>,
        pub fulfillments: ::std::vec::Vec<Fulfillment>,
    }
    #[doc = "Container type for all input parameters for the `name`function with signature `name()` and selector `[6, 253, 222, 3]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    #[doc = "Container type for all input parameters for the `validate`function with signature `validate(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[])` and selector `[136, 20, 119, 50]`"]
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
        name = "validate",
        abi = "validate(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[])"
    )]
    pub struct ValidateCall {
        pub orders: ::std::vec::Vec<Order>,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum SeaportCalls {
        Cancel(CancelCall),
        FulfillAdvancedOrder(FulfillAdvancedOrderCall),
        FulfillAvailableAdvancedOrders(FulfillAvailableAdvancedOrdersCall),
        FulfillAvailableOrders(FulfillAvailableOrdersCall),
        FulfillBasicOrder(FulfillBasicOrderCall),
        FulfillOrder(FulfillOrderCall),
        GetCounter(GetCounterCall),
        GetOrderHash(GetOrderHashCall),
        GetOrderStatus(GetOrderStatusCall),
        IncrementCounter(IncrementCounterCall),
        Information(InformationCall),
        MatchAdvancedOrders(MatchAdvancedOrdersCall),
        MatchOrders(MatchOrdersCall),
        Name(NameCall),
        Validate(ValidateCall),
    }
    impl ethers::core::abi::AbiDecode for SeaportCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) = <CancelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportCalls::Cancel(decoded));
            }
            if let Ok(decoded) =
                <FulfillAdvancedOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportCalls::FulfillAdvancedOrder(decoded));
            }
            if let Ok(decoded) =
                <FulfillAvailableAdvancedOrdersCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(SeaportCalls::FulfillAvailableAdvancedOrders(decoded));
            }
            if let Ok(decoded) =
                <FulfillAvailableOrdersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportCalls::FulfillAvailableOrders(decoded));
            }
            if let Ok(decoded) =
                <FulfillBasicOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportCalls::FulfillBasicOrder(decoded));
            }
            if let Ok(decoded) =
                <FulfillOrderCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportCalls::FulfillOrder(decoded));
            }
            if let Ok(decoded) =
                <GetCounterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportCalls::GetCounter(decoded));
            }
            if let Ok(decoded) =
                <GetOrderHashCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportCalls::GetOrderHash(decoded));
            }
            if let Ok(decoded) =
                <GetOrderStatusCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportCalls::GetOrderStatus(decoded));
            }
            if let Ok(decoded) =
                <IncrementCounterCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportCalls::IncrementCounter(decoded));
            }
            if let Ok(decoded) =
                <InformationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportCalls::Information(decoded));
            }
            if let Ok(decoded) =
                <MatchAdvancedOrdersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportCalls::MatchAdvancedOrders(decoded));
            }
            if let Ok(decoded) =
                <MatchOrdersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportCalls::MatchOrders(decoded));
            }
            if let Ok(decoded) = <NameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref()) {
                return Ok(SeaportCalls::Name(decoded));
            }
            if let Ok(decoded) =
                <ValidateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(SeaportCalls::Validate(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for SeaportCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                SeaportCalls::Cancel(element) => element.encode(),
                SeaportCalls::FulfillAdvancedOrder(element) => element.encode(),
                SeaportCalls::FulfillAvailableAdvancedOrders(element) => element.encode(),
                SeaportCalls::FulfillAvailableOrders(element) => element.encode(),
                SeaportCalls::FulfillBasicOrder(element) => element.encode(),
                SeaportCalls::FulfillOrder(element) => element.encode(),
                SeaportCalls::GetCounter(element) => element.encode(),
                SeaportCalls::GetOrderHash(element) => element.encode(),
                SeaportCalls::GetOrderStatus(element) => element.encode(),
                SeaportCalls::IncrementCounter(element) => element.encode(),
                SeaportCalls::Information(element) => element.encode(),
                SeaportCalls::MatchAdvancedOrders(element) => element.encode(),
                SeaportCalls::MatchOrders(element) => element.encode(),
                SeaportCalls::Name(element) => element.encode(),
                SeaportCalls::Validate(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for SeaportCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                SeaportCalls::Cancel(element) => element.fmt(f),
                SeaportCalls::FulfillAdvancedOrder(element) => element.fmt(f),
                SeaportCalls::FulfillAvailableAdvancedOrders(element) => element.fmt(f),
                SeaportCalls::FulfillAvailableOrders(element) => element.fmt(f),
                SeaportCalls::FulfillBasicOrder(element) => element.fmt(f),
                SeaportCalls::FulfillOrder(element) => element.fmt(f),
                SeaportCalls::GetCounter(element) => element.fmt(f),
                SeaportCalls::GetOrderHash(element) => element.fmt(f),
                SeaportCalls::GetOrderStatus(element) => element.fmt(f),
                SeaportCalls::IncrementCounter(element) => element.fmt(f),
                SeaportCalls::Information(element) => element.fmt(f),
                SeaportCalls::MatchAdvancedOrders(element) => element.fmt(f),
                SeaportCalls::MatchOrders(element) => element.fmt(f),
                SeaportCalls::Name(element) => element.fmt(f),
                SeaportCalls::Validate(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<CancelCall> for SeaportCalls {
        fn from(var: CancelCall) -> Self {
            SeaportCalls::Cancel(var)
        }
    }
    impl ::std::convert::From<FulfillAdvancedOrderCall> for SeaportCalls {
        fn from(var: FulfillAdvancedOrderCall) -> Self {
            SeaportCalls::FulfillAdvancedOrder(var)
        }
    }
    impl ::std::convert::From<FulfillAvailableAdvancedOrdersCall> for SeaportCalls {
        fn from(var: FulfillAvailableAdvancedOrdersCall) -> Self {
            SeaportCalls::FulfillAvailableAdvancedOrders(var)
        }
    }
    impl ::std::convert::From<FulfillAvailableOrdersCall> for SeaportCalls {
        fn from(var: FulfillAvailableOrdersCall) -> Self {
            SeaportCalls::FulfillAvailableOrders(var)
        }
    }
    impl ::std::convert::From<FulfillBasicOrderCall> for SeaportCalls {
        fn from(var: FulfillBasicOrderCall) -> Self {
            SeaportCalls::FulfillBasicOrder(var)
        }
    }
    impl ::std::convert::From<FulfillOrderCall> for SeaportCalls {
        fn from(var: FulfillOrderCall) -> Self {
            SeaportCalls::FulfillOrder(var)
        }
    }
    impl ::std::convert::From<GetCounterCall> for SeaportCalls {
        fn from(var: GetCounterCall) -> Self {
            SeaportCalls::GetCounter(var)
        }
    }
    impl ::std::convert::From<GetOrderHashCall> for SeaportCalls {
        fn from(var: GetOrderHashCall) -> Self {
            SeaportCalls::GetOrderHash(var)
        }
    }
    impl ::std::convert::From<GetOrderStatusCall> for SeaportCalls {
        fn from(var: GetOrderStatusCall) -> Self {
            SeaportCalls::GetOrderStatus(var)
        }
    }
    impl ::std::convert::From<IncrementCounterCall> for SeaportCalls {
        fn from(var: IncrementCounterCall) -> Self {
            SeaportCalls::IncrementCounter(var)
        }
    }
    impl ::std::convert::From<InformationCall> for SeaportCalls {
        fn from(var: InformationCall) -> Self {
            SeaportCalls::Information(var)
        }
    }
    impl ::std::convert::From<MatchAdvancedOrdersCall> for SeaportCalls {
        fn from(var: MatchAdvancedOrdersCall) -> Self {
            SeaportCalls::MatchAdvancedOrders(var)
        }
    }
    impl ::std::convert::From<MatchOrdersCall> for SeaportCalls {
        fn from(var: MatchOrdersCall) -> Self {
            SeaportCalls::MatchOrders(var)
        }
    }
    impl ::std::convert::From<NameCall> for SeaportCalls {
        fn from(var: NameCall) -> Self {
            SeaportCalls::Name(var)
        }
    }
    impl ::std::convert::From<ValidateCall> for SeaportCalls {
        fn from(var: ValidateCall) -> Self {
            SeaportCalls::Validate(var)
        }
    }
    #[doc = "`AdditionalRecipient(uint256,address)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AdditionalRecipient {
        pub amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "`AdvancedOrder((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct AdvancedOrder {
        pub parameters: OrderParameters,
        pub numerator: u128,
        pub denominator: u128,
        pub signature: ethers::core::types::Bytes,
        pub extra_data: ethers::core::types::Bytes,
    }
    #[doc = "`BasicOrderParameters(address,uint256,uint256,address,address,address,uint256,uint256,uint8,uint256,uint256,bytes32,uint256,bytes32,bytes32,uint256,(uint256,address)[],bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct BasicOrderParameters {
        pub consideration_token: ethers::core::types::Address,
        pub consideration_identifier: ethers::core::types::U256,
        pub consideration_amount: ethers::core::types::U256,
        pub offerer: ethers::core::types::Address,
        pub zone: ethers::core::types::Address,
        pub offer_token: ethers::core::types::Address,
        pub offer_identifier: ethers::core::types::U256,
        pub offer_amount: ethers::core::types::U256,
        pub basic_order_type: u8,
        pub start_time: ethers::core::types::U256,
        pub end_time: ethers::core::types::U256,
        pub zone_hash: [u8; 32],
        pub salt: ethers::core::types::U256,
        pub offerer_conduit_key: [u8; 32],
        pub fulfiller_conduit_key: [u8; 32],
        pub total_original_additional_recipients: ethers::core::types::U256,
        pub additional_recipients: ::std::vec::Vec<AdditionalRecipient>,
        pub signature: ethers::core::types::Bytes,
    }
    #[doc = "`ConsiderationItem(uint8,address,uint256,uint256,uint256,address)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ConsiderationItem {
        pub item_type: u8,
        pub token: ethers::core::types::Address,
        pub identifier_or_criteria: ethers::core::types::U256,
        pub start_amount: ethers::core::types::U256,
        pub end_amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
    }
    #[doc = "`CriteriaResolver(uint256,uint8,uint256,uint256,bytes32[])`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct CriteriaResolver {
        pub order_index: ethers::core::types::U256,
        pub side: u8,
        pub index: ethers::core::types::U256,
        pub identifier: ethers::core::types::U256,
        pub criteria_proof: Vec<[u8; 32]>,
    }
    #[doc = "`Execution((uint8,address,uint256,uint256,address),address,bytes32)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Execution {
        pub item: ReceivedItem,
        pub offerer: ethers::core::types::Address,
        pub conduit_key: [u8; 32],
    }
    #[doc = "`Fulfillment((uint256,uint256)[],(uint256,uint256)[])`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Fulfillment {
        pub offer_components: ::std::vec::Vec<FulfillmentComponent>,
        pub consideration_components: ::std::vec::Vec<FulfillmentComponent>,
    }
    #[doc = "`FulfillmentComponent(uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct FulfillmentComponent {
        pub order_index: ethers::core::types::U256,
        pub item_index: ethers::core::types::U256,
    }
    #[doc = "`OfferItem(uint8,address,uint256,uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OfferItem {
        pub item_type: u8,
        pub token: ethers::core::types::Address,
        pub identifier_or_criteria: ethers::core::types::U256,
        pub start_amount: ethers::core::types::U256,
        pub end_amount: ethers::core::types::U256,
    }
    #[doc = "`Order((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Order {
        pub parameters: OrderParameters,
        pub signature: ethers::core::types::Bytes,
    }
    #[doc = "`OrderComponents(address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OrderComponents {
        pub offerer: ethers::core::types::Address,
        pub zone: ethers::core::types::Address,
        pub offer: ::std::vec::Vec<OfferItem>,
        pub consideration: ::std::vec::Vec<ConsiderationItem>,
        pub order_type: u8,
        pub start_time: ethers::core::types::U256,
        pub end_time: ethers::core::types::U256,
        pub zone_hash: [u8; 32],
        pub salt: ethers::core::types::U256,
        pub conduit_key: [u8; 32],
        pub counter: ethers::core::types::U256,
    }
    #[doc = "`OrderParameters(address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct OrderParameters {
        pub offerer: ethers::core::types::Address,
        pub zone: ethers::core::types::Address,
        pub offer: ::std::vec::Vec<OfferItem>,
        pub consideration: ::std::vec::Vec<ConsiderationItem>,
        pub order_type: u8,
        pub start_time: ethers::core::types::U256,
        pub end_time: ethers::core::types::U256,
        pub zone_hash: [u8; 32],
        pub salt: ethers::core::types::U256,
        pub conduit_key: [u8; 32],
        pub total_original_consideration_items: ethers::core::types::U256,
    }
    #[doc = "`ReceivedItem(uint8,address,uint256,uint256,address)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct ReceivedItem {
        pub item_type: u8,
        pub token: ethers::core::types::Address,
        pub identifier: ethers::core::types::U256,
        pub amount: ethers::core::types::U256,
        pub recipient: ethers::core::types::Address,
    }
}
