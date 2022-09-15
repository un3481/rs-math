
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::constants::{ PI, PIDIV2, PIDIV2N, PIN, PI2 };
use crate::arithmetic::{ dec, pow, fac };
use crate::basic::{ sqrt }
use crate::error::Error;

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
) -> Decimal {
    let rem: Decimal = trig_prepare(value);
         if rem == *PI {D1N}
    else if rem == *PIDIV2 {D0}
    else if rem == D0 {D1}
    else if rem == *PIDIV2N {D0}
    else if rem == *PIN {D1N}
    else
        { cos_series(rem, terms) }
}

//##########################################################################################################################

pub fn sin(
    value: Decimal,
    terms: usize
) -> Decimal {
    let rem: Decimal = trig_prepare(value);
         if rem == *PI {D1N}
    else if rem == *PIDIV2 {D0}
    else if rem == D0 {D1}
    else if rem == *PIDIV2N {D0}
    else if rem == *PIN {D1N}
    else
        { sin_series(rem, terms) }
}

//##########################################################################################################################

fn tan_prepare(
    icos: Decimal,
    isin: Decimal,
    terms: usize
) -> (Decimal, Decimal) {
    let cosdiv2 = sqrt((D1 + icos) / D2, terms);
    let cosdiv4 = sqrt((D1 + cosdiv2) / D2, terms);
    let sindiv4 = cosdiv4 * if isin < D0 {D1N} else {D1};
    let tandiv8 =
        if sindiv4 == D0 {D0}
        else { (D1 - cosdiv4) / sindiv4 }
    ;
    let mut tandiv = tandiv8;
    let mut divs = D8;
    loop {
        if tandiv < D1DIV5 && tandiv > D1DIV5N {break tandiv}
        tandiv = (sqrt(D1 + pow(tandiv, 2), terms) - D1) / tandiv;
        divs = divs * D2;
    };
    (tandiv, divs)
} 

//##########################################################################################################################

fn atan_series(
    value: Decimal,
    terms: usize
) -> Decimal {
    (1..terms).into_par_iter()
        .map(|n|
            pow(D1N, n + 1) * (
                pow(value, (2 * (n - 1)) + 1) /
                ((D2 * (dec(n) - D1)) + D1)
            )
        )
        .reduce(|| D0, |u, d| u + d)
}

//##########################################################################################################################

pub fn atan(
    icos: Decimal,
    isin: Decimal,
    terms: usize
) -> <Decimal, Error> {
    let modl = sqrt((icos * icos) + (isin * isin), terms);
    if modl != D1 { return Err(Error::InvalidSineOrCosine) };
    Ok(
             if icos == D0 && isin > D0 {PIDIV2}
        else if icos == D0 && isin < D0 {PI3DIV2}
        else if isin == D0 && icos > D0 {D0}
        else if isin == D0 && icos < D0 {PI}
        else {
            let (tan, divs) = tan_prepare(icos, isin, terms);
            divs * atan_series(tan, terms)
        }
    )
}

//##########################################################################################################################
