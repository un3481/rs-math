
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::constants::{ consts };
use crate::arithmetic::{ pow, fac };

//##########################################################################################################################

// Constants
const D1N: Decimal = dec!(-1);
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);

//##########################################################################################################################

fn trig_prepare(
    value: Decimal
) -> Decimal {
    let PI = *consts.PI;
    let mut rem: Decimal = value;
    if rem > PI {
        rem = rem - (
            (rem / (PI * D2)).floor() * PI * D2
        )
    }
    else if rem < -PI {
        rem = rem - (
            (rem / (PI * D2)).floor() * PI * D2
        )
    }
         if rem >  PI { rem = rem - (PI * D2) }
    else if rem < -PI { rem = rem + (PI * D2) };
    rem
}

//##########################################################################################################################

fn cos_series(
    terms: usize,
    value: Decimal
) -> Decimal {
    (0..terms).into_par_iter()
        .map(|n|
            pow(D1N, n) * (
                pow(value, 2 * n) /
                fac(2 * n)
            )
        )
        .reduce(|| D0, |u, d| u + d)
}

//##########################################################################################################################

fn sin_series(
    terms: usize,
    value: Decimal
) -> Decimal {
    (0..terms).into_par_iter()
        .map(|n|
            pow(D1N, n) * (
                pow(value, (2 * n) + 1) /
                fac((2 * n) + 1)
            )
        )
        .reduce(|| D0, |u, d| u + d)
}

//##########################################################################################################################

pub fn cos(
    terms: usize,
    value: Decimal
) -> Result<Decimal, ()> {
    let rem: Decimal = trig_prepare(value);
    Ok(
             if rem == *consts.PI {D1N}
        else if rem == *consts.PIDIV2 {D0}
        else if rem == D0 {D1}
        else if rem == *consts.PIDIV2N {D0}
        else if rem == *consts.PIN {D1N}
        else
            { cos_series(terms, rem) }
    )
}

//##########################################################################################################################

pub fn sin(
    terms: usize,
    value: Decimal
) -> Result<Decimal, ()> {
    let rem: Decimal = trig_prepare(value);
    Ok(
             if rem == *consts.PI {D1N}
        else if rem == *consts.PIDIV2 {D0}
        else if rem == D0 {D1}
        else if rem == *consts.PIDIV2N {D0}
        else if rem == *consts.PIN {D1N}
        else
            { sin_series(terms, rem) }
    )
}

//##########################################################################################################################
