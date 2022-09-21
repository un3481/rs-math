
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use lazy_static::lazy_static;
use rayon::prelude::*;

// Modules
use crate::constants::{ STD_ITER, PI, PIDIV2, PIDIV2N, PI3DIV2, PIN, PI2 };
use crate::arithmetic::{ dec, pow, fac };
use crate::error::Error;

type Pair = (Decimal, Decimal);

//##########################################################################################################################

// Constants
const D1N: Decimal = dec!(-1);
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D6: Decimal = dec!(6);
const D18: Decimal = dec!(18);
const D36: Decimal = dec!(36);
const D1DIV5: Decimal = dec!(0.2);
const D2DIV5: Decimal = dec!(0.4);
const TRIG_LOWER: Decimal = dec!(0.999);
const TRIG_UPPER: Decimal = dec!(1.001);
const PI_PAIR: Pair = (D1N, D0);
const PIDIV2_PAIR: Pair = (D0, D1);
const PI3DIV2_PAIR: Pair = (D0, D1N);

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

lazy_static! {
    static ref PIDIV6: Decimal = (*PI) / D6;
    static ref PIDIV18: Decimal = (*PI) / D18;
    static ref PIDIV36: Decimal = (*PI) / D36;
    static ref TAN_PIDIV6: Decimal = sin(*PIDIV6, STD_ITER) / cos(*PIDIV6, STD_ITER);
    static ref TAN_PIDIV18: Decimal = sin(*PIDIV18, STD_ITER) / cos(*PIDIV18, STD_ITER);
    static ref TAN_PIDIV36: Decimal = sin(*PIDIV36, STD_ITER) / cos(*PIDIV36, STD_ITER);
}

//##########################################################################################################################

#[inline]
fn is_valid_pair(icos: Decimal, isin: Decimal) -> bool {
    let module = (icos * icos) + (isin * isin);
    (TRIG_LOWER < module) && (module < TRIG_UPPER)
}

/// cos(a - b) = (cos(a) * cos(b)) + (sin(a) * sin(b))
#[inline]
pub fn cos_sub(arg: Pair, sub: Pair) -> Decimal {
    (arg.0 * sub.0) + (arg.1 * sub.1)
}

/// sin(a - b) = (sin(a) * cos(b)) - (sin(b) * cos(a))
#[inline]
pub fn sin_sub(arg: Pair, sub: Pair) -> Decimal {
    (arg.1 * sub.0) - (sub.1 * arg.0)
}

/// tan(a - b) = sin(a - b) / cos(a - b)
#[inline]
pub fn tan_sub2(arg: Pair, sub: Pair) -> Decimal {
    cos_sub(arg, sub) / sin_sub(arg, sub)
}

/// tan(a - b) = (tan(a) - tan(b)) / (1 + (tan(a) * tan(b)))
#[inline]
pub fn tan_sub(arg: Decimal, sub: Decimal) -> Decimal {
    (arg - sub) / (D1 + (arg * sub))
}

//##########################################################################################################################

fn tan_prepare(
    icos: Decimal,
    isin: Decimal
) -> (Decimal, Decimal) {
    let pair = (icos, isin);
    let mut rem: Decimal = D0;
    let mut tansub =
             if (isin <  D0) && (icos >  D0) { rem = *PI3DIV2; tan_sub2(pair, PI3DIV2_PAIR) }
        else if (isin <  D0) && (icos <= D0) { rem = *PI;      tan_sub2(pair, PI_PAIR)      }
        else if (isin >= D0) && (icos >  D0) { rem = *PIDIV2;  tan_sub2(pair, PIDIV2_PAIR)  }
        else                                 {                 isin / icos                  }
    ;
    loop {
        if tansub < D1DIV5 {break};
        tansub =
                 if tansub > D1     { rem = rem + (*PIDIV6);  tan_sub(tansub, *TAN_PIDIV6)  }
            else if tansub > D2DIV5 { rem = rem + (*PIDIV18); tan_sub(tansub, *TAN_PIDIV18) }
            else                    { rem = rem + (*PIDIV36); tan_sub(tansub, *TAN_PIDIV36) }
        ;
    };
    (tansub, rem)
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
            let (tan, rem) = tan_prepare(icos, isin);
            rem + atan_series(tan, terms)
        }
    )
}

//##########################################################################################################################
