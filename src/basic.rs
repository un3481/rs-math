
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
    value: Decimal
) -> (Decimal, Decimal) {
    fn prepare(
        ratio: Decimal,
        value: Decimal
    ) -> (Decimal, Decimal) {
        match true {
            (value > dec!(1.5)) => prepare(
                ratio * consts::SQRT_OF_THREE_HALFS,
                value / dec!(1.5)
            ),
            (value < dec!(0.5)) => prepare(
                ratio / consts::SQRT_OF_THREE_HALFS,
                value * dec!(1.5)
            ),
            _ => (ratio, value),
        }
    };
    prepare(dec!(1), value)
}

//##########################################################################################################################

pub fn sqrt(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    Ok(
        match true {
            (value < dec!(0)) => decimal::NAN,
            (value == dec!(0)) => dec!(0),
            _ => {
                let (ratio, rem) = sqrt_prepare(value);
                sqrt_series(terms, rem)? * ratio
            },
        }
    )
}

//##########################################################################################################################

