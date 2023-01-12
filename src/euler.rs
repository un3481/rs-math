
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::constants::{ E, D1DIVE, LN_2 };
use crate::arithmetic::{ dec, fac, pow, a_pow };
use crate::error::Error;

//##########################################################################################################################

// Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D4: Decimal = dec!(4);

//##########################################################################################################################

/// e^x = sum(n=0; x^n / n!)
fn power_series(
    value: Decimal,
    terms: usize
) -> Decimal {
    let mut acc1: (Decimal, usize) = (D1, 0);
    // Iterate over Series
    (1..=terms).into_iter()
        .map(|n|
            a_pow(value, n, acc1) /
            fac(n)
        )
        .reduce(|u, d| u + d)
        .unwrap_or(D0)
}

//##########################################################################################################################

#[inline]
pub fn exp(
    value: Decimal,
    terms: usize
) -> Decimal {
         if value == D0  {D1}
    else if value == D1  {E}
    else if value == -D1 {D1DIVE}
    else
        { power_series(value, terms) }
}

//##########################################################################################################################

fn ln_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let mut rem: Decimal = value;
    let mut exp: Decimal = D0;
    loop {
        if rem > D4 {
            rem = rem / D2;
            exp = exp + LN_2;
        }
        else if rem < D2 {
            rem = rem * D2;
            exp = exp - LN_2;
        }
        else {break}
    };
    (exp, rem)
}

//##########################################################################################################################

/// ln(x) = 1 + sum(n=0; -1^(n + 1) * ((x - e)^n / (n * e^n)))
fn ln_series(
    value: Decimal,
    terms: usize
) -> Decimal {
    let mut acc1: (Decimal, usize) = (D1, 0);
    let mut acc2: (Decimal, usize) = (D1, 0);
    let mut acc3: (Decimal, usize) = (D1, 0);
    // Iterate over Series
    D1 + (
        (1..=terms).into_iter()
            .map(|n|
                a_pow(-D1, n + 1, acc1) * (
                    a_pow(value - E, n, acc2) /
                    (a_pow(E, n, acc3) * dec(n))
                )
            )
            .reduce(|u, d| u + d)
            .unwrap_or(D0)
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
            exp + ln_series(rem, terms)
        }
    )
}

//##########################################################################################################################
