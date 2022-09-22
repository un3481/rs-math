
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::constants::{ PI, PIDIV2, PI3DIV2, PI2 };
use crate::constants::{ PIDIV4, PIDIV6, PIDIV18, PIDIV36 };
use crate::constants::{ TAN_PIDIV6, TAN_PIDIV18, TAN_PIDIV36 };
use crate::arithmetic::{ dec, pow, fac };
use crate::error::Error;

//##########################################################################################################################

type Pair = (Decimal, Decimal);

// Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D1DIV5: Decimal = dec!(0.2);
const D2DIV5: Decimal = dec!(0.4);
const TRIG_LOWER: Decimal = dec!(0.999);
const TRIG_UPPER: Decimal = dec!(1.001);
const PI_PAIR: Pair = (dec!(-1), dec!(0));
const PIDIV2_PAIR: Pair = (dec!(0), dec!(1));
const PI3DIV2_PAIR: Pair = (dec!(0), dec!(-1));

//##########################################################################################################################

fn trig_prepare(
    value: Decimal
) -> Decimal {
    let mut rem: Decimal = value;
    if rem > PI {
        rem = rem - (
            (rem / PI2).floor() * PI2
        )
    }
    else if rem < -PI {
        rem = rem - (
            (rem / PI2).floor() * PI2
        )
    }
         if rem >  PI { rem = rem - PI2 }
    else if rem < -PI { rem = rem + PI2 };
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
            pow(-D1, n) * (
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
            pow(-D1, n) * (
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
         if rem == PI      {-D1}
    else if rem == PIDIV2  {D0}
    else if rem == D0      {D1}
    else if rem == -PIDIV2 {D0}
    else if rem == -PI     {-D1}
    else
        { cos_series(rem, terms) }
}

//##########################################################################################################################

pub fn sin(
    value: Decimal,
    terms: usize
) -> Decimal {
    let rem: Decimal = trig_prepare(value);
         if rem == PI      {D0}
    else if rem == PIDIV2  {D1}
    else if rem == D0      {D0}
    else if rem == -PIDIV2 {-D1}
    else if rem == -PI     {D0}
    else
        { sin_series(rem, terms) }
}

//##########################################################################################################################

#[inline]
fn is_valid_pair(icos: Decimal, isin: Decimal) -> bool {
    let module = (icos * icos) + (isin * isin);
    (TRIG_LOWER < module) && (module < TRIG_UPPER)
}

/// cos(a - b) = (cos(a) * cos(b)) + (sin(a) * sin(b))
#[inline]
fn cos_sub2(arg: Pair, sub: Pair) -> Decimal {
    (arg.0 * sub.0) + (arg.1 * sub.1)
}

/// sin(a - b) = (sin(a) * cos(b)) - (sin(b) * cos(a))
#[inline]
fn sin_sub2(arg: Pair, sub: Pair) -> Decimal {
    (arg.1 * sub.0) - (sub.1 * arg.0)
}

/// tan(a - b) = sin(a - b) / cos(a - b)
#[inline]
fn tan_sub2(arg: Pair, sub: Pair) -> Decimal {
    sin_sub2(arg, sub) / cos_sub2(arg, sub)
}

/// tan(a - b) = (tan(a) - tan(b)) / (1 + (tan(a) * tan(b)))
#[inline]
fn tan_sub(arg: Decimal, sub: Decimal) -> Decimal {
    (arg - sub) / (D1 + (arg * sub))
}

//##########################################################################################################################

fn tan_lower(
    itan: Decimal,
    offset: Decimal
) -> (Decimal, Decimal) {
    let mut rem: Decimal = offset;
    let mut tansub: Decimal = itan;
    loop {
        if tansub < D1DIV5 {break};
        tansub =
                 if tansub > D1     { rem = rem + PIDIV6;  tan_sub(tansub, TAN_PIDIV6)  }
            else if tansub > D2DIV5 { rem = rem + PIDIV18; tan_sub(tansub, TAN_PIDIV18) }
            else                    { rem = rem + PIDIV36; tan_sub(tansub, TAN_PIDIV36) }
        ;
    };
    (tansub, rem)
}

//##########################################################################################################################

fn tan_prepare(
    itan: Decimal
) -> (Decimal, Decimal) {
    let rem: Decimal;
    let tansub: Decimal =
             if itan >= D0  { rem = D0;      itan                             }
        else if itan >  -D1 { rem = -PIDIV4; tan_sub(itan, -D1)               }
        else if itan <= -D1 { rem = -PIDIV2; tan_sub(tan_sub(itan, -D1), -D1) }
        else                { rem = D0;      itan                             }
    ;
    tan_lower(tansub, rem)
}

//##########################################################################################################################

fn tan2_prepare(
    icos: Decimal,
    isin: Decimal
) -> (Decimal, Decimal) {
    let pair = (icos, isin);
    let rem: Decimal;
    let tansub: Decimal =
             if (isin <  D0) && (icos >  D0) { rem = PI3DIV2; tan_sub2(pair, PI3DIV2_PAIR) }
        else if (isin <  D0) && (icos <= D0) { rem = PI;      tan_sub2(pair, PI_PAIR)      }
        else if (isin >= D0) && (icos >  D0) { rem = PIDIV2;  tan_sub2(pair, PIDIV2_PAIR)  }
        else                                 { rem = D0;      isin / icos                  }
    ;
    tan_lower(tansub, rem)
}

//##########################################################################################################################

/// atan(x) = sum(n=1; -1^n * (x^(2n + 1) / (2n + 1)))
fn atan_series(
    value: Decimal,
    terms: usize
) -> Decimal {
    (1..terms).into_par_iter()
        .map(|n|
            pow(-D1, n) * (
                pow(value, (2 * n) + 1) /
                ((D2 * dec(n)) + D1)
            )
        )
        .reduce(|| D0, |u, d| u + d)
}

//##########################################################################################################################

pub fn atan(
    itan: Decimal,
    terms: usize
) -> Decimal {
         if itan == D0  {D0}
    else if itan == D1  {PIDIV4}
    else if itan == -D1 {-PIDIV4}
    else {
        let (tan, rem) = tan_prepare(itan);
        rem + atan_series(tan, terms)
    }
}

//##########################################################################################################################

pub fn atan2(
    icos: Decimal,
    isin: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    if !is_valid_pair(icos, isin) { return Err(Error::InvalidSineCosinePair) };
    Ok(
             if isin == D0 && icos > D0 {D0}
        else if icos == D0 && isin > D0 {PIDIV2}
        else if isin == D0 && icos < D0 {PI}
        else if icos == D0 && isin < D0 {PI3DIV2}
        else {
            let (tan, rem) = tan2_prepare(icos, isin);
            let arg = rem + atan_series(tan, terms);
            arg - (if arg > PI {PI2} else {D0})
        }
    )
}

//##########################################################################################################################
