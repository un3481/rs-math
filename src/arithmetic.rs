
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::multiplex::types::{ Multiplex };
use crate::error::Error;

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

fn pow_series(
    value: Decimal,
    power: usize
) -> Result<Decimal, Error> {
    Ok(
        (1..=power).into_iter()
            .map(|_| Ok(value))
            .reduce(|u, d| {
                let (_u, _d) = (u?, d?);
                let res = _u.checked_mul(_d).ok_or(Error::MultiplyOverflow)?;
                Ok(res)
            })
            .unwrap_or(Err(Error::IteratorError))?
    )
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

fn m_pow_series(
    value: Multiplex,
    power: usize
) -> Result<Multiplex, Error> {
    (1..=power).into_iter()
        .map(|_| Ok(value.clone()))
        .reduce(|u, d| {
            let (_u, _d) = (u?, d?);
            let res = _u * _d;
            Ok(res)
        })
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
    if base.1 > power { base.1 = power };
    let dif = pow(value, power - base.1)?;
    let result = dif.checked_mul(base.0).ok_or(Error::MultiplyOverflow)?;
    base.0 = result;
    base.1 = power;
    Ok(result)
}

//##########################################################################################################################

#[inline]
pub fn am_pow(
    value: Decimal,
    power: usize,
    base: &mut (Multiplex, usize)
) -> Result<Multiplex, Error> {
    if base.1 > power { base.1 = power };
    let mut dif = m_pow(Multiplex::new() * value, power - base.1)?;
    let mut result = base.0.clone();
    match dif.squash() {
        Err(_) => result.mul.append(&mut dif.mul),
        Ok(v) => result.mul.push(v),
    };
    base.0 = result.clone();
    base.1 = power;
    Ok(result)
}

//##########################################################################################################################
