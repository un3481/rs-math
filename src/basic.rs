
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
mod consts;

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
    rsn: Decimal,
    x: Decimal
) -> (Decimal, Decimal) {
    let (d05, d15) = (dec!(0.5), dec!(1.5));
    let sqrt15 = consts::SQRT_OF_THREE_HALFS;
    match true {
        (x > d15) => sqrt_prepare(rsn * sqrt15, x / d15),
        (x < d05) => sqrt_prepare(rsn / sqrt15, x * d15),
        _ => (rsn, x),
    }
}

//##########################################################################################################################

pub fn sqrt(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    let (rsn, rem) = sqrt_prepare(dec!(1), value);
    Ok(
        sqrt_series(terms, rem)? * rsn
    )
}

//##########################################################################################################################

