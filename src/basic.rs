
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::multiplex::types::{ Multiplex };
use crate::constants::{ SQRT_2, SQRT_5DIV4 };
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
const D3DIV4: Decimal = dec!(0.75);
const D5DIV4: Decimal = dec!(1.25);

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
            rem = rem / D2;
            exp = exp * SQRT_2;
        }
        else if rem < D2 {
            rem = rem * D2;
            exp = exp / SQRT_2;
        }
        else {break}
    };
    loop {
        if rem > D5DIV4 {
            rem = rem / D5DIV4;
            exp = exp * SQRT_5DIV4;
        }
        else if rem < D3DIV4 {
            rem = rem * D5DIV4;
            exp = exp / SQRT_5DIV4;
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
            let (ratio, rem) = sqrt_prepare(value);
            ratio * sqrt_series(rem, terms)?
        }
    )
}

//##########################################################################################################################
