
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::multiplex::types::{ Multiplex };
use crate::constants::{ E, D1DIVE, E_SQR };
use crate::constants::{ LN_UPPER_BD, LN_LOWER_BD, LN_UPPER_EXP_VAL, LN_LOWER_EXP_VAL };
use crate::factorial::{ m_fac };
use crate::arithmetic::{ dec, a_pow, am_pow };
use crate::error::Error;

//##########################################################################################################################

// Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);

//##########################################################################################################################

/// e^x = sum(n=0; x^n / n!)
#[inline]
fn power_series(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    let mut acc1: (Multiplex, usize) = (Multiplex::new(), 0);
    // Iterate over Series
    Ok(
        D1 + (
            (1..=terms).into_iter()
                .map(|n| Ok(
                    (am_pow(value, n, &mut acc1)? / m_fac(n)?).squash()?
                ))
                .reduce(|u, d| Ok(
                    u?.checked_add(d?).ok_or(Error::AddOverflow)?
                ))
                .unwrap_or(Ok(D0))
        )?
    )
}

//##########################################################################################################################

#[inline]
pub fn exp(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    Ok(
             if value == D0  {D1}
        else if value == D1  {E}
        else if value == -D1 {D1DIVE}
        else
            { power_series(value, terms)? }
    )
}

//##########################################################################################################################

#[inline]
fn ln_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let mut rem: Decimal = value;
    let mut exp: Decimal = D0;
    loop {
        if rem > E_SQR {
            rem = rem / E;
            exp = exp + D1;
        }
        else if rem < D1 {
            rem = rem * E;
            exp = exp - D1;
        }
        else {break}
    };
    loop {
        if rem > LN_UPPER_BD {
            rem = rem / LN_UPPER_BD;
            exp = exp + LN_UPPER_EXP_VAL;
        }
        else if rem < LN_LOWER_BD {
            rem = rem * LN_LOWER_BD;
            exp = exp - LN_LOWER_EXP_VAL;
        }
        else {break}
    };
    (exp, rem)
}

//##########################################################################################################################

/// ln(x) = 1 + sum(n=0; -1^(n + 1) * ((x - e)^n / (n * e^n)))
#[inline]
fn ln_series(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    let mut acc1: (Decimal, usize) = (D1, 0);
    let mut acc2: (Multiplex, usize) = (Multiplex::new(), 0);
    let mut acc3: (Multiplex, usize) = (Multiplex::new(), 0);
    // Iterate over Series
    Ok(
        D1 + (
            (1..=terms).into_iter()
                .map(|n| Ok(
                    a_pow(-D1, n + 1, &mut acc1)? * (
                        am_pow(value - E, n, &mut acc2)? / (dec(n) * am_pow(E, n, &mut acc3)?)
                    ).squash()?
                ))
                .reduce(|u, d| Ok(
                    u?.checked_add(d?).ok_or(Error::AddOverflow)?
                ))
                .unwrap_or(Ok(D0))
        )?
    )
}

//##########################################################################################################################

#[inline]
pub fn ln(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    if value <= D0 { return Err(Error::InputOutOfRange) };
    Ok(
             if value == D1     {D0}
        else if value == E      {D1}
        else if value == D1DIVE {-D1}
        else {
            let (exp, rem) = ln_prepare(value);
            exp + ln_series(rem, terms)?
        }
    )
}

//##########################################################################################################################
