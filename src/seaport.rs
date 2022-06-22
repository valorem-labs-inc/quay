use ethers::contract::abigen;

// Because of CREATE2, these are the same accross networks

pub(crate) const SEAPORT: &str = "0x00000000006c3852cbEf3e08E8dF289169EdE581";
pub(crate) const CONDUITCONTROLLER: &str = "0x00000000F9490004C11Cef243f5400493c00Ad63";

// TODO(Support configurable conduit address)

// These macros generate type safe bindings to the seaport smart contracts

abigen!(
    Seaport,
    "./src/abi/seaport.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

abigen!(
    ConduitController,
    "./src/abi/conduit_controller.json",
    event_derives(serde::Deserialize, serde::Serialize)
);
