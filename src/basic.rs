
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::error::Error;
use crate::multiplex::types::{ Multiplex };
use crate::euler::{ exp, ln };

//##########################################################################################################################

const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);

//##########################################################################################################################

#[inline]
pub fn dec(value: usize) -> Decimal {
    Decimal::new(value as i64, 0)
}

#[inline]
pub fn pos(value: Decimal) -> Decimal {
    if value < D0 {-value} else {value}
}

//##########################################################################################################################

#[inline]
fn pow_series(
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
pub fn pow(
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
                else { pow_series(value, power)? }
            },
        }
    )
}

//##########################################################################################################################

#[inline]
fn m_pow_series(
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
            _ => { m_pow_series(value, power)? },
        }
    )
}

//##########################################################################################################################

#[inline]
pub fn a_pow(
    value: Decimal,
    power: usize,
    base: &mut (Decimal, usize)
) -> Result<Decimal, Error> {
    // Apply Power
    let exp = if base.1 > power {power} else {base.1};
    let dif = pow(value, power - exp)?;
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

#[inline]
pub fn am_pow(
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
    // Update Base
    base.0 = result.clone();
    base.1 = power;
    // Return Result
    Ok(result)
}

//##########################################################################################################################

/// a^b = e^(ln(a) * b)
#[inline]
pub fn d_pow(
    value: Decimal,
    power: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    let ln_val = ln(value, terms)?;
    exp(ln_val * power, terms)
}

//##########################################################################################################################
