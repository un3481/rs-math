
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::constants::{ consts };
use crate::arithmetic::{ dec, pow, fac };

//##########################################################################################################################

// Constants
const D1N: Decimal = dec!(-1);
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D4: Decimal = dec!(4);

//##########################################################################################################################

fn power_series(
    terms: usize,
    value: Decimal
) -> Decimal {
    (1..=terms).into_iter()
        .map(|n| pow(value, n) / fac(n))
        .reduce(|u, d| u + d)
        .unwrap_or(D0)
}

//##########################################################################################################################

pub fn power(
    terms: usize,
    value: Decimal
) -> Decimal {
    let E = consts.E; 
         if value == D1N {D1 / E}
    else if value == D0  {D1}
    else if value == D1  {E}
    else
        { power_series(terms, value) }
}

//##########################################################################################################################

fn ln_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let LN_2 = consts.LN_2;
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

fn ln_series(
    terms: usize,
    value: Decimal
) -> Decimal {
    let E = consts.E;
    D1 + (
        (1..=terms).into_par_iter()
            .map(|n|
                pow(D1N, n + 1) * (
                    pow(value - E, n) /
                    (pow(E, n) * dec(n))
                )
            )
            .reduce(|| D0, |u, d| u + d)
    )
}

//##########################################################################################################################

pub fn ln(
    terms: usize,
    value: Decimal
) -> Result<Decimal, &'static str> {
    if value <= D0 {
        return Err("cannot calc ln(x) for x <= 0")
    };
    let E = consts.E;
    Ok(
             if value == D1 {D0}
        else if value == E  {D1}
        else {
            let (exp, rem) = ln_prepare(value);
            exp + ln_series(terms, rem)
        }
    )
}

//##########################################################################################################################
