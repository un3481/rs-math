
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::constants::{
    SQRT_OF_THREE_HALFS
};

//##########################################################################################################################

// Constants
const D0 = dec!(0);
const D1 = dec!(1);
const D2 = dec!(2);
const D1DIV2 = dec!(0.5);
const D3DIV2 = dec!(1.5);

//##########################################################################################################################

pub fn pow(
    value: Decimal,
    exp: usize
) -> Result<Decimal, Error> {
    Ok(
        match exp {
            0 => D1,
            1 => value,
            _ => match value {
                D0 => D0,
                D1 => D1,
                _ => (1..=exp).par_iter()
                    .map(|_| value)
                    .reduce(|| D1, |u, d| u * d),
            },
        }
    )
}

//##########################################################################################################################

pub fn fac(
    value: usize,
) -> Result<Decimal, Error> {
    Ok(
        match value {
            0 => D1,
            1 => D1,
            _ => (1..=value).par_iter()
                .map(|x| dec!(x))
                .reduce(|| D1, |u, d| u * d),
        }
    )
}

//##########################################################################################################################

fn sqrt_series(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    Ok(
        (1..=terms).par_iter()
            .map(|n| [
                || (
                    value * [
                        || basic::fac(2 * (n - 1)).unwrap(),
                        || basic::pow(D1 - value, n - 1).unwrap()
                    ].par_iter().map(|f| f())
                    .reduce(|| D1, |u, d| u * d)
                ),
                || (
                    basic::pow([
                        || basic::fac(n - 1).unwrap(),
                        || basic::pow(D2, n - 1).unwrap()
                    ].par_iter().map(|f| f())
                    .reduce(|| D1, |u, d| u * d), 2)
                )
            ].par_iter())
            .map(|t| t.map(|f| f()).collect())
            .map(|t| t[0] / t[1])
            .reduce(|| dec!(0), |u, d| u + d)
    )
}

//##########################################################################################################################

fn sqrt_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let mut rem = value.copy();
    let mut ratio = D1;
    loop {
        match true {
            (rem > D3DIV2) => {
                rem = rem / D3DIV2;
                ratio = ratio * SQRT_OF_THREE_HALFS;
            },
            (rem < D1DIV2) => {
                rem = rem * D3DIV2;
                ratio = ratio / SQRT_OF_THREE_HALFS;
            },
            _ => {break},
        }
    };
    (ratio, rem)
}

//##########################################################################################################################

pub fn sqrt(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    if value < D0 {
        panic!("cannot calc sqrt(x) for x < 0");
    };
    Ok(
        match value {
            D0 => D0,
            D1 => D1,
            _ => {
                let (ratio, rem) = sqrt_prepare(value);
                sqrt_series(terms, rem)? * ratio
            },
        }
    )
}

//##########################################################################################################################

