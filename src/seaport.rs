use anyhow::Result;
use ethers::contract::abigen;
use ethers::providers::Http;
use ethers::providers::Provider;
use ethers::types::Address;
use std::sync::Arc;

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

// We don't need signers here because we are doing read only operations in the backend
pub struct SeaportMarket {
    pub seaport: Seaport<Provider<Http>>,
    pub conduit_controller: ConduitController<Provider<Http>>,
}

// TODO(Deal with these clones better)
impl SeaportMarket {
    pub fn new(provider: Arc<Provider<Http>>) -> Result<SeaportMarket> {
        // Setup woolf
        let seaport_address: Address = SEAPORT.parse().unwrap();
        let seaport = Seaport::new(seaport_address, provider.clone());

        // Setup woolf
        let conduit_controller_address: Address = CONDUITCONTROLLER.parse().unwrap();
        let conduit_controller =
            ConduitController::new(conduit_controller_address, provider.clone());
        Ok(SeaportMarket {
            seaport,
            conduit_controller,
        })
    }
}
