
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::constants::{ E, LN_2 };
use crate::arithmetic::{ dec, pow, fac };
use crate::error::Error;

//##########################################################################################################################

// Constants
const D1N: Decimal = dec!(-1);
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D4: Decimal = dec!(4);

//##########################################################################################################################

fn power_series(
    value: Decimal,
    terms: usize
) -> Decimal {
    (1..=terms).into_iter()
        .map(|n| pow(value, n) / fac(n))
        .reduce(|u, d| u + d)
        .unwrap_or(D0)
}

//##########################################################################################################################

pub fn exp(
    value: Decimal,
    terms: usize
) -> Decimal {
    let _e = *E; 
         if value == D1N {D1 / _e}
    else if value == D0  {D1}
    else if value == D1  {_e}
    else
        { power_series(value, terms) }
}

//##########################################################################################################################

fn ln_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let _ln2 = *LN_2;
    let mut rem: Decimal = value;
    let mut exp: Decimal = D0;
    loop {
        if rem > D4 {
            rem = rem / D2;
            exp = exp + _ln2;
        }
        else if rem < D2 {
            rem = rem * D2;
            exp = exp - _ln2;
        }
        else {break}
    };
    (exp, rem)
}

//##########################################################################################################################

fn ln_series(
    value: Decimal,
    terms: usize
) -> Decimal {
    let _e = *E;
    D1 + (
        (1..=terms).into_par_iter()
            .map(|n|
                pow(D1N, n + 1) * (
                    pow(value - _e, n) /
                    (pow(_e, n) * dec(n))
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
    let _e = *E;
    Ok(
             if value == D1 {D0}
        else if value == _e {D1}
        else {
            let (exp, rem) = ln_prepare(value);
            exp + ln_series(rem, terms)
        }
    )
}

//##########################################################################################################################
