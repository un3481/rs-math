
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::constants::{ PI, PIDIV2, PIDIV2N, PI3DIV2, PIN, PI2 };
use crate::arithmetic::{ dec, pow, fac };
use crate::basic::{ sqrt };
use crate::error::Error;

//##########################################################################################################################

// Constants
const D1N: Decimal = dec!(-1);
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D8: Decimal = dec!(8);
const D1DIV5: Decimal = dec!(0.2);
const D1DIV5N: Decimal = dec!(-0.2);
const TRIG_LOWER: Decimal = dec!(0.999);
const TRIG_UPPER: Decimal = dec!(1.001);

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

/// sin(x) = sum(n=0; -1^n * (x^2n / 2n!))
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

/// sin(x) = sum(n=0; -1^n * (x^(2n + 1) / (2n + 1)!))
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

#[inline]
fn is_valid_pair(icos: Decimal, isin: Decimal) -> bool {
    let module = (icos * icos) + (isin * isin);
    (TRIG_LOWER < module) && (module < TRIG_UPPER)
}

//##########################################################################################################################

/// cos(x/2) = sqrt((1 + cos(x)) / 2)
/// tan(x/2) = (1 - cos(x)) / sin(x)
/// tan(x/2) = (sqrt(1 + tan(x)^2) - 1) / tan(x)
fn tan_prepare(
    icos: Decimal,
    isin: Decimal,
    terms: usize
) -> Result<(Decimal, Decimal), Error> {
    let cosdiv2 = sqrt((D1 + icos) / D2, terms)?;
    let cosdiv4 = sqrt((D1 + cosdiv2) / D2, terms)?;
    let sindiv4 = cosdiv4 * (if isin < D0 {D1N} else {D1});
    let tandiv8 =
        if sindiv4 == D0 {D0}
        else { (D1 - cosdiv4) / sindiv4 }
    ;
    let mut tandiv = tandiv8;
    let mut divs = D8;
    loop {
        if (D1DIV5N < tandiv) && (tandiv < D1DIV5) {break tandiv};
        tandiv = (sqrt(D1 + pow(tandiv, 2), terms)? - D1) / tandiv;
        divs = divs * D2;
    };
    Ok(
        (tandiv, divs)
    )
}

//##########################################################################################################################

/// atan(x) = sum(n=1; -1^n * (x^(2n + 1) / (2n + 1)))
fn atan_series(
    value: Decimal,
    terms: usize
) -> Decimal {
    (1..terms).into_par_iter()
        .map(|n|
            pow(D1N, n) * (
                pow(value, (2 * n) + 1) /
                ((D2 * dec(n)) + D1)
            )
        )
        .reduce(|| D0, |u, d| u + d)
}

//##########################################################################################################################

pub fn atan(
    icos: Decimal,
    isin: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    if !is_valid_pair(icos, isin) { return Err(Error::InvalidSineCosinePair) };
    Ok(
             if icos == D0 && isin > D0 {*PIDIV2}
        else if icos == D0 && isin < D0 {*PI3DIV2}
        else if isin == D0 && icos > D0 {D0}
        else if isin == D0 && icos < D0 {*PI}
        else {
            let (tan, divs) = tan_prepare(icos, isin, terms)?;
            divs * atan_series(tan, terms)
        }
    )
}

//##########################################################################################################################
