
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::basic;
use crate::constants::{
    EULER,
    LN_OF_TWO
};

//##########################################################################################################################

fn power_series(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    Ok(
        (0..terms).par_iter()
            .map(|n| [
                || basic::pow(value, n).unwrap(),
                || basic::fac(n).unwrap()
            ].par_iter())
            .map(|t| t.map(|f| f()).collect())
            .map(|t| t[0] / t[1])
            .reduce(|| dec!(0), |u, d| u + d)
    )
}

//##########################################################################################################################

pub fn power(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    Ok(
        match value {
            dec!(-1) => (dec!(1) / EULER),
            dec!(0) => dec!(1),
            dec!(1) => EULER,
            _  => power_series(terms, value)?,
        }
    )
}

//##########################################################################################################################

fn ln_series(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    Ok(
        dec!(1) + (
            (1..=terms).par_iter()
                .map(|n| (if let 0=n%2 {-1} else {1}, [
                    || basic::pow(value - EULER, n).unwrap(),
                    || basic::pow(EULER, n).unwrap() * dec!(n)
                ].par_iter()))
                .map(|(s, t)| (s, t.map(|f| f()).collect()))
                .map(|(s, t)| (t[0] / t[1]) * dec!(s))
                .reduce(|| dec!(0), |u, d| u + d)
        )
    )
}

//##########################################################################################################################

fn ln_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let mut rem = dec!(0) + value;
    let mut exp = dec!(0);
    loop {
        match true {
            (rem > dec!(4)) => {
                rem = rem / dec!(2);
                exp = exp + LN_OF_TWO;
            },
            (rem < dec!(2)) => {
                rem = rem * dec!(2);
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
    Ok(
        match true {
            (value <= dec!(0)) => panic!("cannot calc ln(x) for x <= 0"),
            EULER => dec!(1),
            _ => {
                let (exp, rem) = ln_prepare(value);
                ln_series(terms, rem)? + exp
            },
        }
    )
}

//##########################################################################################################################
