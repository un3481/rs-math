
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::constants::{
    SQRT_OF_THREE_HALFS
};

//##########################################################################################################################

pub fn pow(
    value: Decimal,
    exp: usize
) -> Result<Decimal, Error> {
    Ok(
        match exp {
            0 => dec!(1),
            _ => match value {
                dec!(1) => dec!(1),
                _ => (1..=exp).par_iter()
                    .map(|_| value)
                    .reduce(|| dec!(1), |u, d| u * d),
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
            0 => dec!(1),
            _ => (1..=value).par_iter()
                .map(|x| dec!(x))
                .reduce(|| dec!(1), |u, d| u * d),
        }
    )
}

//##########################################################################################################################

pub fn sqrt_series(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    Ok(
        (1..=terms).par_iter()
            .map(|n| [
                || (
                    value * [
                        || basic::fac(2 * (n - 1)).unwrap(),
                        || basic::pow(dec!(1) - value, n - 1).unwrap()
                    ].par_iter().map(|f| f())
                    .reduce(|| dec!(1), |u, d| u * d)
                ),
                || (
                    basic::pow([
                        || basic::fac(n - 1).unwrap(),
                        || basic::pow(dec!(2), n - 1).unwrap()
                    ].par_iter().map(|f| f())
                    .reduce(|| dec!(1), |u, d| u * d), 2)
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
    let mut rem = dec!(0) + value;
    let mut ratio = dec!(1);
    loop {
        match true {
            (rem > dec!(1.5)) => {
                rem = rem / dec!(1.5);
                ratio = ratio * SQRT_OF_THREE_HALFS;
            },
            (rem < dec!(0.5)) => {
                rem = rem * dec!(1.5);
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
    Ok(
        match true {
            (value < dec!(0)) => panic!("cannot calc sqrt(x) for x < 0"),
            (value == dec!(0)) => dec!(0),
            _ => {
                let (ratio, rem) = sqrt_prepare(value);
                sqrt_series(terms, rem)? * ratio
            },
        }
    )
}

//##########################################################################################################################

