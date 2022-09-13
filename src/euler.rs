
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::basic::{ pow, fac };
use crate::constants::{ EULER, LN_OF_TWO };

//##########################################################################################################################

// Constants
const D0 = dec!(0);
const D1 = dec!(1);
const D2 = dec!(2);
const D4 = dec!(4);
const D1NEG = dec!(-1);
const EULERINV = D1 / EULER;

//##########################################################################################################################

fn power_series(
    terms: usize,
    value: Decimal
) -> Decimal {
    (0..terms).par_iter()
        .map(|n| pow(value, n) / fac(n))
        .reduce(|| D0, |u, d| u + d)
}

//##########################################################################################################################

pub fn power(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    Ok(
        match value {
            D1NEG => EULERINV,
            D0 => D1,
            D1 => EULER,
            _  => power_series(terms, value),
        }
    )
}

//##########################################################################################################################

fn ln_series(
    terms: usize,
    value: Decimal
) -> Decimal {
    D1 + (
        (1..=terms).par_iter()
            .map(|n| (
                pow(D1NEG, n + 1) * (
                    pow(value - EULER, n) /
                    (dec!(n) * pow(EULER, n))
                )
            ))
            .reduce(|| D0, |u, d| u + d)
    )
}

//##########################################################################################################################

fn ln_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let mut rem = value;
    let mut exp = D0;
    loop {
        match true {
            (rem > D4) => {
                rem = rem / D2;
                exp = exp + LN_OF_TWO;
            },
            (rem < D2) => {
                rem = rem * D2;
                exp = exp - LN_OF_TWO;
            },
            _ => {break},
        }
    };
    (exp, rem)
}

//##########################################################################################################################

pub fn ln(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    if value <= D0 {
        panic!("cannot calc ln(x) for x <= 0");
    };
    Ok(
        match  {
            D1 => D0,
            EULER => D1,
            _ => {
                let (exp, rem) = ln_prepare(value);
                exp + ln_series(terms, rem)
            },
        }
    )
}

//##########################################################################################################################
