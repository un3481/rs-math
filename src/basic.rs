
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::multiplex::types::{ Multiplex };
use crate::constants::{ SQRT_EXP_VAL, SQRT_EXP_BD, SQRT_UPPER_BD, SQRT_LOWER_BD };
use crate::factorial::{ m_fac };
use crate::arithmetic::{ m_pow, am_pow };
use crate::euler::{ exp, ln };
use crate::error::Error;

//##########################################################################################################################

// Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D4: Decimal = dec!(4);
const D1DIV4: Decimal = dec!(0.25);

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

#[inline]
fn sqrt_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let mut rem: Decimal = value;
    let mut exp: Decimal = D1;
    loop {
        if rem > D4 {
            rem = rem / D4;
            exp = exp * D2;
        }
        else if rem < D1DIV4 {
            rem = rem * D4;
            exp = exp / D2;
        }
        else {break}
    };
    loop {
        if rem > SQRT_UPPER_BD {
            rem = rem / SQRT_EXP_BD;
            exp = exp * SQRT_EXP_VAL;
        }
        else if rem < SQRT_LOWER_BD {
            rem = rem * SQRT_EXP_BD;
            exp = exp / SQRT_EXP_VAL;
        }
        else {break}
    };
    (exp, rem)
}

//##########################################################################################################################

/// sqrt(x) = sum(n=1; (x * (2 * (n - 1))! * (1 - x)^(n - 1)) / ((n - 1)! * 2^(n - 1))^2)
#[inline]
fn sqrt_series(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    let mut acc1: (Multiplex, usize) = (Multiplex::new(), 0);
    let mut acc2: (Multiplex, usize) = (Multiplex::new(), 0);
    // Iterate over Taylor Series
    (1..=terms).into_iter()
        .map(|n| Ok(
            (
                (
                    value *
                    m_fac(2 * (n - 1))? *
                    am_pow(D1 - value, n - 1, &mut acc1)?
                ) /
                m_pow(
                    m_fac(n - 1)? *
                    am_pow(D2, n - 1, &mut acc2)?,
                    2
                )?
            ).squash()?
        ))
        .reduce(|u, d| Ok(
            u?.checked_add(d?).ok_or(Error::AddOverflow)?
        ))
        .unwrap_or(Ok(D0))
}
 
//##########################################################################################################################

#[inline]
pub fn sqrt(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    if value < D0 { return Err(Error::InputOutOfRange) };
    Ok(
             if value == D0 {D0}
        else if value == D1 {D1}
        else {
            let (exp, rem) = sqrt_prepare(value);
            exp * sqrt_series(rem, terms)?
        }
    )
}

//##########################################################################################################################
