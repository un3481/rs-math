
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::constants::{ E, D1DIVE, LN_2 };
use crate::arithmetic::{ dec, pow, fac };
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
    (1..=terms).into_par_iter()
        .map(|n| pow(value, n) / fac(n))
        .reduce(|| D0, |u, d| u + d)
}

//##########################################################################################################################

pub fn exp(
    value: Decimal,
    terms: usize
) -> Decimal {
         if value == -D1 {D1DIVE}
    else if value == D0  {D1}
    else if value == D1  {E}
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
    D1 + (
        (1..=terms).into_par_iter()
            .map(|n|
                pow(-D1, n + 1) * (
                    pow(value - E, n) /
                    (pow(E, n) * dec(n))
                )
            )
            .reduce(|| D0, |u, d| u + d)
    )
}

//##########################################################################################################################

pub fn ln(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    if value <= D0 { return Err(Error::InputOutOfRange) };
    Ok(
             if value == D1 {D0}
        else if value ==  E {D1}
        else {
            let (exp, rem) = ln_prepare(value);
            exp + ln_series(rem, terms)
        }
    )
}

//##########################################################################################################################
