use crate::result::Result;
use js_sys::BigInt;
use pyrin_consensus_core::network::{NetworkType, NetworkTypeT};
use wasm_bindgen::prelude::*;
use workflow_wasm::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "bigint | number | HexString")]
    #[derive(Clone, Debug)]
    pub type ISompiToPyrin;
}

/// Convert a Pyrin string to Sompi represented by bigint.
/// This function provides correct precision handling and
/// can be used to parse user input.
/// @category Wallet SDK
#[wasm_bindgen(js_name = "pyrinToSompi")]
pub fn pyrin_to_sompi(pyrin: String) -> Option<BigInt> {
    crate::utils::try_pyrin_str_to_sompi(pyrin).ok().flatten().map(Into::into)
}

///
/// Convert Sompi to a string representation of the amount in Pyrin.
///
/// @category Wallet SDK
///
#[wasm_bindgen(js_name = "sompiToPyrinString")]
pub fn sompi_to_pyrin_string(sompi: ISompiToPyrin) -> Result<String> {
    let sompi = sompi.try_as_u64()?;
    Ok(crate::utils::sompi_to_pyrin_string(sompi))
}

///
/// Format a Sompi amount to a string representation of the amount in Pyrin with a suffix
/// based on the network type (e.g. `PYI` for mainnet, `TPYI` for testnet,
/// `SPYI` for simnet, `DPYI` for devnet).
///
/// @category Wallet SDK
///
#[wasm_bindgen(js_name = "sompiToPyrinStringWithSuffix")]
pub fn sompi_to_pyrin_string_with_suffix(sompi: ISompiToPyrin, network: &NetworkTypeT) -> Result<String> {
    let sompi = sompi.try_as_u64()?;
    let network_type = NetworkType::try_from(network)?;
    Ok(crate::utils::sompi_to_pyrin_string_with_suffix(sompi, &network_type))
}
