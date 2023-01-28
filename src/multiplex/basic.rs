
// Imports
use rust_decimal::prelude::*;

// Modules
use crate::error::Error;
use crate::basic::{ dec };

use crate::multiplex::types::{ Multiplex };

//##########################################################################################################################

#[inline]
pub fn mux(value: usize) -> Multiplex {
    Multiplex::new() * dec(value)
}

//##########################################################################################################################

#[inline]
fn m_pow_helper(
    value: Multiplex,
    power: usize
) -> Result<Multiplex, Error> {
    (1..=power).into_iter()
        .map(|_| Ok(value.clone()))
        .reduce(|u, d| Ok(u? * d?))
        .unwrap_or(Err(Error::IteratorError))
}

#[inline]
pub fn m_pow(
    value: Multiplex,
    power: usize
) -> Result<Multiplex, Error> {
    Ok(
        match power {
            0 => Multiplex::new(),
            1 => value,
            _ => { m_pow_helper(value, power)? },
        }
    )
}

//##########################################################################################################################

#[inline]
pub fn ma_pow(
    value: Decimal,
    power: usize,
    base: &mut (Multiplex, usize)
) -> Result<Multiplex, Error> {
    // Apply Power
    let exp = if base.1 > power {power} else {base.1};
    let mut dif = m_pow(value * Multiplex::new(), power - exp)?;
    // Calculate Result
    let mut result = base.0.clone();
    match dif.squash() {
        Err(_) => result.mul.append(&mut dif.mul),
        Ok(v) => result.mul.push(v),
    };
    match result.squash() {
        Err(_) => {},
        Ok(v) => { result = Multiplex::new() * v },
    };
    // Update Base
    base.0 = result.clone();
    base.1 = power;
    // Return Result
    Ok(result)
}

//##########################################################################################################################
