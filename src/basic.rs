
// Imports
use rust_decimal::prelude::*;

// Modules
use crate::error::Error;
use crate::euler::{ d_exp, d_ln };

//##########################################################################################################################

const D0: Decimal = Decimal::ZERO;
const D1: Decimal = Decimal::ONE;

//##########################################################################################################################

#[inline]
pub fn dec(value: usize) -> Decimal {
    Decimal::new(value as i64, 0)
}

//##########################################################################################################################

#[inline]
fn d_pow_helper(
    value: Decimal,
    power: usize
) -> Result<Decimal, Error> {
    (1..=power).into_iter()
        .map(|_| Ok(value))
        .reduce(|u, d| Ok(
            u?.checked_mul(d?).ok_or(Error::MultiplyOverflow)?
        ))
        .unwrap_or(Err(Error::IteratorError))
}

#[inline]
pub fn d_pow(
    value: Decimal,
    power: usize
) -> Result<Decimal, Error> {
    Ok(
        match power {
            0 => D1,
            1 => value,
            _ => {
                     if value == D0 {D0}
                else if value == D1 {D1}
                else { d_pow_helper(value, power)? }
            },
        }
    )
}

//##########################################################################################################################

#[inline]
pub fn da_pow(
    value: Decimal,
    power: usize,
    base: &mut (Decimal, usize)
) -> Result<Decimal, Error> {
    // Apply Power
    let exp = if base.1 > power {power} else {base.1};
    let dif = d_pow(value, power - exp)?;
    // Calculate Result
    let result = base.0.clone()
        .checked_mul(dif)
        .ok_or(Error::MultiplyOverflow)?;
    // Update Base
    base.0 = result.clone();
    base.1 = power;
    // Return Result
    Ok(result)
}

//##########################################################################################################################

/// a^b = e^(ln(a) * b)
#[inline]
pub fn dd_pow(
    value: Decimal,
    power: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    let ln_val = d_ln(value, terms)?;
    d_exp(ln_val * power, terms)
}

//##########################################################################################################################
