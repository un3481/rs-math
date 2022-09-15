
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::constants::{ PI, PIDIV2, PIDIV2N, PIN, PI2 };
use crate::arithmetic::{ pow, fac };

//##########################################################################################################################

// Constants
const D1N: Decimal = dec!(-1);
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);

//##########################################################################################################################

fn trig_prepare(
    value: Decimal
) -> Decimal {
    let _pi = *PI;
    let _pi2 = *PI2;
    let _pin = *PIN;
    let mut rem: Decimal = value;
    if rem > _pi {
        rem = rem - (
            (rem / _pi2).floor() * _pi2
        )
    }
    else if rem < _pin {
        rem = rem - (
            (rem / _pi2).floor() * _pi2
        )
    }
         if rem >  _pi { rem = rem - _pi2 }
    else if rem < _pin { rem = rem + _pi2 };
    rem
}

//##########################################################################################################################

fn cos_series(
    value: Decimal,
    terms: usize
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
    value: Decimal,
    terms: usize
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
    value: Decimal,
    terms: usize
) -> Result<Decimal, ()> {
    let rem: Decimal = trig_prepare(value);
    Ok(
             if rem == *PI {D1N}
        else if rem == *PIDIV2 {D0}
        else if rem == D0 {D1}
        else if rem == *PIDIV2N {D0}
        else if rem == *PIN {D1N}
        else
            { cos_series(rem, terms) }
    )
}

//##########################################################################################################################

pub fn sin(
    value: Decimal,
    terms: usize
) -> Result<Decimal, ()> {
    let rem: Decimal = trig_prepare(value);
    Ok(
             if rem == *PI {D1N}
        else if rem == *PIDIV2 {D0}
        else if rem == D0 {D1}
        else if rem == *PIDIV2N {D0}
        else if rem == *PIN {D1N}
        else
            { sin_series(rem, terms) }
    )
}

//##########################################################################################################################
