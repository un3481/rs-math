
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::complex::types::{ Complex };
use crate::complex::euler::{ c_exp, c_ln };
use crate::error::Error;

//##########################################################################################################################

const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const C0: Complex = Complex{ re: D0, im: D0 };
const C1: Complex = Complex{ re: D1, im: D0 };

//##########################################################################################################################

fn c_pow_series(
    value: Complex,
    power: usize
) -> Result<Complex, Error> {
    (1..=power).into_iter()
        .map(|_| Ok(value))
        .reduce(|u, d| Ok(u? * d?))
        .unwrap_or(Err(Error::IteratorError))
}

#[inline]
pub fn c_pow(
    value: Complex,
    power: usize
) -> Result<Complex, Error> {
    Ok(
        match power {
            0 => C1,
            1 => value,
            _ => {
                     if value == C0 {C0}
                else if value == C1 {C1}
                else { c_pow_series(value, power)? }
            },
        }
    )
}

//##########################################################################################################################

/// z^w = e^(ln(z) * w)
#[inline]
pub fn cc_pow(
    value: Complex,
    power: Complex,
    terms: usize
) -> Result<Complex, Error> {
    let ln_val = c_ln(value, terms)?;
    c_exp(ln_val * power, terms)
}

//##########################################################################################################################
