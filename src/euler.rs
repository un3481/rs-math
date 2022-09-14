
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::basic::{ dec, pow, fac };
use crate::constants::{ EULER, LN_OF_TWO };

//##########################################################################################################################

// Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D4: Decimal = dec!(4);
const D1NEG: Decimal = dec!(-1);

//##########################################################################################################################

fn power_series(
    terms: usize,
    value: Decimal
) -> Decimal {
    (0..terms).into_par_iter()
        .map(|n| pow(value, n) / fac(n))
        .reduce(|| D0, |u, d| u + d)
}

//##########################################################################################################################

pub fn power(
    terms: usize,
    value: Decimal
) -> Result<Decimal, ()> {
    Ok(
        match value {
            D1NEG => D1 / EULER,
            D0 => D1,
            D1 => EULER,
            _  => power_series(terms, value),
        }
    )
}

//##########################################################################################################################

const fn ln_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let mut rem = value;
    let mut exp = D0;
    loop {
        if rem > D4 {
            rem = rem / D2;
            exp = exp + LN_OF_TWO;
        }
        else if rem < D2 {
            rem = rem * D2;
            exp = exp - LN_OF_TWO;
        }
        else {break}
    };
    (exp, rem)
}

//##########################################################################################################################

fn ln_series(
    terms: usize,
    value: Decimal
) -> Decimal {
    D1 + (
        (1..=terms).into_par_iter()
            .map(|n| (
                pow(D1NEG, n + 1) * (
                    pow(value - EULER, n) /
                    (pow(EULER, n) * dec(n))
                )
            ))
            .reduce(|| D0, |u, d| u + d)
    )
}

//##########################################################################################################################

pub fn ln(
    terms: usize,
    value: Decimal
) -> Result<Decimal, &'static str> {
    if value <= D0 {
        panic!("cannot calc ln(x) for x <= 0");
    };
    Ok(
        match value {
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
