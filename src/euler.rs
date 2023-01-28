
// Imports
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

// Modules
use crate::constants::{ E, D1DIVE, E_SQR, E_POW1DIV5, };
use crate::constants::{ LN_UPPER_BD, LN_UPPER_MUL, LN_UPPER_VAL };
use crate::constants::{ LN_LOWER_BD, LN_LOWER_MUL, LN_LOWER_VAL };
use crate::constants::{ LN_UPPER_BD_P, LN_UPPER_MUL_P, LN_UPPER_VAL_P };
use crate::constants::{ LN_LOWER_BD_P, LN_LOWER_MUL_P, LN_LOWER_VAL_P };

use crate::error::Error;

use crate::factorial::{ m_fac };
use crate::basic::{ dec, d_pow, da_pow, };

use crate::multiplex::types::{ Multiplex };
use crate::multiplex::basic::{ ma_pow };

//##########################################################################################################################

// Constants
const D0: Decimal = Decimal::ZERO;
const D1: Decimal = Decimal::ONE;

const D5: Decimal = dec!(5);
const D1DIV5: Decimal = dec!(0.2);

//##########################################################################################################################

#[inline]
fn d_exp_prepare(
    value: Decimal
) -> Result<(Decimal, Decimal), Error> {
    let mut rem: Decimal = value.abs().fract();
    let fract_pow: usize = (rem * D5).floor().to_usize().ok_or(Error::OptionInvalid)?;
    let int_pow: usize = value.abs().floor().to_usize().ok_or(Error::OptionInvalid)?;
    let base: Decimal = d_pow(E, int_pow)? * d_pow(E_POW1DIV5, fract_pow)?;
    rem = rem - (D1DIV5 * dec(fract_pow));
    Ok((rem, base))
}

//##########################################################################################################################

/// e^x = sum(n=0; x^n / n!)
#[inline]
fn d_exp_series(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    let mut acc1: (Multiplex, usize) = (Multiplex::new(), 0);
    // Iterate over Series
    Ok(
        D1 + (
            (1..=terms).into_iter()
                .map(|n| Ok(
                    (ma_pow(value, n, &mut acc1)? / m_fac(n)?).squash()?
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
pub fn d_exp(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    Ok(
             if value ==  D0 { D1     }
        else if value ==  D1 { E      }
        else if value == -D1 { D1DIVE }
        else {
            let (rem, base) = d_exp_prepare(value)?;
            let res = base * if rem == D0 {D1} else { d_exp_series(rem, terms)? };
            if value.is_sign_negative() {D1 / res} else {res}
        }
    )
}

//##########################################################################################################################

#[inline]
fn d_ln_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let mut rem: Decimal = value;
    let mut base: Decimal = D0;
    loop {
        if rem > E_SQR {
            rem = rem / E;
            base = base + D1;
        }
        else if rem < D1 {
            rem = rem * E;
            base = base - D1;
        }
        else {break}
    };
    loop {
        if rem > LN_UPPER_BD {
            rem = rem / LN_UPPER_MUL;
            base = base + LN_UPPER_VAL;
        }
        else if rem < LN_LOWER_BD {
            rem = rem / LN_LOWER_MUL;
            base = base + LN_LOWER_VAL;
        }
        else {break}
    };
    loop {
        if rem > LN_UPPER_BD_P {
            rem = rem / LN_UPPER_MUL_P;
            base = base + LN_UPPER_VAL_P;
        }
        else if rem < LN_LOWER_BD_P {
            rem = rem / LN_LOWER_MUL_P;
            base = base + LN_LOWER_VAL_P;
        }
        else {break}
    };
    (rem, base)
}

//##########################################################################################################################

/// ln(x) = 1 + sum(n=0; -1^(n + 1) * ((x - e)^n / (n * e^n)))
#[inline]
fn d_ln_series(
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
                    da_pow(-D1, n + 1, &mut acc1)? * (
                        ma_pow(value - E, n, &mut acc2)? / (dec(n) * ma_pow(E, n, &mut acc3)?)
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
pub fn d_ln(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    if value <= D0 { Err(Error::InputOutOfRange)? };
    Ok(
             if value == D1     {  D0 }
        else if value == E      {  D1 }
        else if value == D1DIVE { -D1 }
        else {
            let (rem, base) = d_ln_prepare(value);
            base + d_ln_series(rem, terms)?
        }
    )
}

//##########################################################################################################################
