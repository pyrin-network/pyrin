use crate::error::Error;
use crate::result::Result;
use pyrin_consensus_core::constants::LEOR_PER_PYRIN;
use std::fmt::Display;

pub fn try_parse_required_nonzero_pyrin_as_sompi_u64<S: ToString + Display>(pyrin_amount: Option<S>) -> Result<u64> {
    if let Some(pyrin_amount) = pyrin_amount {
        let sompi_amount = pyrin_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Kasapa amount is not valid: '{pyrin_amount}'")))?
            * LEOR_PER_PYRIN as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Pyrin amount is not valid: '{pyrin_amount}'"))
        } else {
            let sompi_amount = sompi_amount as u64;
            if sompi_amount == 0 {
                Err(Error::custom("Supplied required pyrin amount must not be a zero: '{pyrin_amount}'"))
            } else {
                Ok(sompi_amount)
            }
        }
    } else {
        Err(Error::custom("Missing Pyrin amount"))
    }
}

pub fn try_parse_required_pyrin_as_sompi_u64<S: ToString + Display>(pyrin_amount: Option<S>) -> Result<u64> {
    if let Some(pyrin_amount) = pyrin_amount {
        let sompi_amount = pyrin_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Kasapa amount is not valid: '{pyrin_amount}'")))?
            * LEOR_PER_PYRIN as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Pyrin amount is not valid: '{pyrin_amount}'"))
        } else {
            Ok(sompi_amount as u64)
        }
    } else {
        Err(Error::custom("Missing Pyrin amount"))
    }
}

pub fn try_parse_optional_pyrin_as_sompi_i64<S: ToString + Display>(pyrin_amount: Option<S>) -> Result<Option<i64>> {
    if let Some(pyrin_amount) = pyrin_amount {
        let sompi_amount = pyrin_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_e| Error::custom(format!("Supplied Kasapa amount is not valid: '{pyrin_amount}'")))?
            * LEOR_PER_PYRIN as f64;
        if sompi_amount < 0.0 {
            Err(Error::custom("Supplied Pyrin amount is not valid: '{pyrin_amount}'"))
        } else {
            Ok(Some(sompi_amount as i64))
        }
    } else {
        Ok(None)
    }
}
