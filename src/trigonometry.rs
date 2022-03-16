
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::basic;
use crate::constants::{
    PI
};

//##########################################################################################################################

fn trig_prepare(
    value: Decimal
) -> Decimal {
    const PINEG = dec!(-1) * PI;
    const PI2 = dec!(2) * PI;
    let mut rem = dec!(0) + value;
    rem = match true {
        rem > PI => rem - (
            (rem / PI2).floor() * PI2
        ),
        rem < PINEG => rem - (
            (rem / PI2).floor() * PI2
        ),
        _ => rem,
    };
    match true {
        rem > PI => rem - PI2,
        rem < PINEG => rem + PI2,
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
            .map(|t| t.map(|f| f()).collect())
            .map(|t| (t[0] / t[1]) * t[2])
            .reduce(|| dec!(0), |u, d| u + d)
    )
}

//##########################################################################################################################

pub fn cos(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    const PINEG = dec!(-1) * PI;
    const PIHALF = PI / dec!(2);
    const PIHNEG = PINEG / dec!(2);
    let rem = trig_prepare(value);
    Ok(
        match rem {
            PI => dec!(-1),
            PIHALF => dec!(0),
            dec!(0) => dec!(1),
            PIHNEG => dec!(0),
            PINEG => dec!(-1),
            _ => cos_series(terms, rem)?,
        }
    )
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
            .map(|t| t.map(|f| f()).collect())
            .map(|t| (t[0] / t[1]) * t[2])
            .reduce(|| dec!(0), |u, d| u + d)
    )
}

//##########################################################################################################################

pub fn sin(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    const PINEG = dec!(-1) * PI;
    const PIHALF = PI / dec!(2);
    const PIHNEG = PINEG / dec!(2);
    let rem = trig_prepare(value);
    Ok(
        match rem {
            PI => dec!(0),
            PIHALF => dec!(1),
            dec!(0) => dec!(0),
            PIHNEG => dec!(-1),
            PINEG => dec!(0),
            _ => sin_series(terms, rem)?,
        }
    )
}

//##########################################################################################################################
