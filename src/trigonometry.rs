// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::basic;
use crate::constants::{PI};

//##########################################################################################################################

fn trig_prepare(
    value: Decimal
) -> Decimal {
    let pi2 = dec!(2) * PI;
    let mut rem = dec!(0) + value;
    rem = match true {
        (rem > dec!(PI)) => rem - (
            (rem / pi2).floor() * pi2
        ),
        (rem < dec!(-PI)) => rem - (
            (rem / pi2).floor() * pi2
        ),
        _ => rem,
    };
    match true {
        (rem > PI) => rem - pi2,
        (rem < (dec!(-1) * PI)) => rem + pi2,
        _ => rem,
    }
}

//##########################################################################################################################

fn cos_series(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    Ok(
        (0..terms).par_iter()
            .map(|n| [
                || basic::pow(value, 2 * n).unwrap(),
                || basic::fac(2 * n).unwrap(),
                || basic::pow(dec!(-1), n).unwrap()
            ].par_iter()))
            .map(|v| v.map(|f| f()).collect())
            .map(|v| (v[0] / v[1]) * v[2])
            .reduce(|| dec!(0), |u, d| u + d)
    )
}

//##########################################################################################################################

pub fn cos(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    let rem = trig_prepare(value);
    Ok(cos_series(terms, rem)?)
}

//##########################################################################################################################

fn sin_series(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    Ok(
        (0..terms).par_iter()
            .map(|n| [
                || basic::pow(value, (2 * n) + 1).unwrap(),
                || basic::fac((2 * n) + 1).unwrap(),
                || basic::pow(dec!(-1), n).unwrap()
            ].par_iter()))
            .map(|v| v.map(|f| f()).collect())
            .map(|v| (v[0] / v[1]) * v[2])
            .reduce(|| dec!(0), |u, d| u + d)
    )
}

//##########################################################################################################################

pub fn sin(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    let rem = trig_prepare(value);
    Ok(sin_series(terms, rem)?)
}

//##########################################################################################################################
