//!
//! Pyrin value formatting and parsing utilities.
//!

use crate::result::Result;
use pyrin_addresses::Address;
use pyrin_consensus_core::constants::*;
use pyrin_consensus_core::network::NetworkType;
use separator::{separated_float, separated_int, separated_uint_with_output, Separatable};
use workflow_log::style;

pub fn try_pyrin_str_to_sompi<S: Into<String>>(s: S) -> Result<Option<u64>> {
    let s: String = s.into();
    let amount = s.trim();
    if amount.is_empty() {
        return Ok(None);
    }

    Ok(Some(str_to_sompi(amount)?))
}

pub fn try_pyrin_str_to_sompi_i64<S: Into<String>>(s: S) -> Result<Option<i64>> {
    let s: String = s.into();
    let amount = s.trim();
    if amount.is_empty() {
        return Ok(None);
    }

    let amount = amount.parse::<f64>()? * LEOR_PER_PYRIN as f64;
    Ok(Some(amount as i64))
}

#[inline]
pub fn sompi_to_pyrin(sompi: u64) -> f64 {
    sompi as f64 / LEOR_PER_PYRIN as f64
}

#[inline]
pub fn pyrin_to_sompi(pyrin: f64) -> u64 {
    (pyrin * LEOR_PER_PYRIN as f64) as u64
}

#[inline]
pub fn sompi_to_pyrin_string(sompi: u64) -> String {
    sompi_to_pyrin(sompi).separated_string()
}

#[inline]
pub fn sompi_to_pyrin_string_with_trailing_zeroes(sompi: u64) -> String {
    separated_float!(format!("{:.8}", sompi_to_pyrin(sompi)))
}

pub fn pyrin_suffix(network_type: &NetworkType) -> &'static str {
    match network_type {
        NetworkType::Mainnet => "PYI",
        NetworkType::Testnet => "TPYI",
        NetworkType::Simnet => "SPYI",
        NetworkType::Devnet => "DPYI",
    }
}

#[inline]
pub fn sompi_to_pyrin_string_with_suffix(sompi: u64, network_type: &NetworkType) -> String {
    let kas = sompi_to_pyrin_string(sompi);
    let suffix = pyrin_suffix(network_type);
    format!("{kas} {suffix}")
}

#[inline]
pub fn sompi_to_pyrin_string_with_trailing_zeroes_and_suffix(sompi: u64, network_type: &NetworkType) -> String {
    let kas = sompi_to_pyrin_string_with_trailing_zeroes(sompi);
    let suffix = pyrin_suffix(network_type);
    format!("{kas} {suffix}")
}

pub fn format_address_colors(address: &Address, range: Option<usize>) -> String {
    let address = address.to_string();

    let parts = address.split(':').collect::<Vec<&str>>();
    let prefix = style(parts[0]).dim();
    let payload = parts[1];
    let range = range.unwrap_or(6);
    let start = range;
    let finish = payload.len() - range;

    let left = &payload[0..start];
    let center = style(&payload[start..finish]).dim();
    let right = &payload[finish..];

    format!("{prefix}:{left}:{center}:{right}")
}

fn str_to_sompi(amount: &str) -> Result<u64> {
    let Some(dot_idx) = amount.find('.') else {
        return Ok(amount.parse::<u64>()? * LEOR_PER_PYRIN);
    };
    let integer = amount[..dot_idx].parse::<u64>()? * LEOR_PER_PYRIN;
    let decimal = &amount[dot_idx + 1..];
    let decimal_len = decimal.len();
    let decimal = if decimal_len == 0 {
        0
    } else if decimal_len <= 8 {
        decimal.parse::<u64>()? * 10u64.pow(8 - decimal_len as u32)
    } else {
        // TODO - discuss how to handle values longer than 8 decimal places
        // (reject, truncate, ceil(), etc.)
        decimal[..8].parse::<u64>()?
    };
    Ok(integer + decimal)
}
