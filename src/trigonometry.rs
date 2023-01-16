
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::constants::{ PI, PIDIV2, PI3DIV2, PI2 };
use crate::constants::{ PIDIV4, PIDIV6, PIDIV18, PIDIV36 };
use crate::constants::{ TAN_PIDIV6, TAN_PIDIV18, TAN_PIDIV36 };

use crate::error::Error;
use crate::multiplex::types::{ Multiplex };
use crate::factorial::{ m_fac };
use crate::basic::{ dec, a_pow, am_pow };

//##########################################################################################################################

type Pair = (Decimal, Decimal);

// Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const DN1: Decimal = dec!(-1);
const D1DIV5: Decimal = dec!(0.2);
const D2DIV5: Decimal = dec!(0.4);
const PI_PAIR: Pair = (DN1, D0);
const PIDIV2_PAIR: Pair = (D0, D1);
const PI3DIV2_PAIR: Pair = (D0, DN1);

//##########################################################################################################################

#[inline]
fn trig_prepare(
    value: Decimal
) -> Decimal {
    let mut rem: Decimal = value;
    if (rem < -PI) || (PI < rem) {
        rem = rem - ((rem / PI2).floor() * PI2);
    };
         if rem >  PI { rem = rem - PI2; }
    else if rem < -PI { rem = rem + PI2; };
    rem
}

//##########################################################################################################################

/// sin(x) = sum(n=0; -1^n * (x^2n / 2n!))
#[inline]
fn cos_series(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    let mut acc1: (Decimal, usize) = (D1, 0);
    let mut acc2: (Multiplex, usize) = (Multiplex::new(), 0);
    // Iterate over Series
    (0..terms).into_iter()
        .map(|n| Ok(
            a_pow(-D1, n, &mut acc1)? * (
                am_pow(value, 2 * n, &mut acc2)? / m_fac(2 * n)?
            ).squash()?
        ))
        .reduce(|u, d| Ok(
            u?.checked_add(d?).ok_or(Error::AddOverflow)?
        ))
        .unwrap_or(Ok(D0))
}

//##########################################################################################################################

/// sin(x) = sum(n=0; -1^n * (x^(2n + 1) / (2n + 1)!))
#[inline]
fn sin_series(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    let mut acc1: (Decimal, usize) = (D1, 0);
    let mut acc2: (Multiplex, usize) = (Multiplex::new(), 0);
    // Iterate over Series
    (0..terms).into_iter()
        .map(|n| Ok(
            a_pow(-D1, n, &mut acc1)? * (
                am_pow(value, (2 * n) + 1, &mut acc2)? / m_fac((2 * n) + 1)?
            ).squash()?
        ))
        .reduce(|u, d| Ok(
            u?.checked_add(d?).ok_or(Error::AddOverflow)?
        ))
        .unwrap_or(Ok(D0))
}

//##########################################################################################################################

#[inline]
pub fn cos(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    let rem: Decimal = trig_prepare(value);
    Ok(
             if rem == PI      {-D1}
        else if rem == PIDIV2  {D0}
        else if rem == D0      {D1}
        else if rem == -PIDIV2 {D0}
        else if rem == -PI     {-D1}
        else
            { cos_series(rem, terms)? }
    )
}

//##########################################################################################################################

#[inline]
pub fn sin(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    let rem: Decimal = trig_prepare(value);
    Ok(
             if rem == PI      {D0}
        else if rem == PIDIV2  {D1}
        else if rem == D0      {D0}
        else if rem == -PIDIV2 {-D1}
        else if rem == -PI     {D0}
        else
            { sin_series(rem, terms)? }
    )
}

//##########################################################################################################################

/// cos(a - b) = (cos(a) * cos(b)) + (sin(a) * sin(b))
#[inline]
fn cos_sub2(val: Pair, sub: Pair) -> Decimal {
    (val.0 * sub.0) + (val.1 * sub.1)
}

/// sin(a - b) = (sin(a) * cos(b)) - (sin(b) * cos(a))
#[inline]
fn sin_sub2(val: Pair, sub: Pair) -> Decimal {
    (val.1 * sub.0) - (sub.1 * val.0)
}

/// tan(a - b) = sin(a - b) / cos(a - b)
#[inline]
fn tan_sub2(val: Pair, sub: Pair) -> Decimal {
    sin_sub2(val, sub) / cos_sub2(val, sub)
}

/// tan(a - b) = (tan(a) - tan(b)) / (1 + (tan(a) * tan(b)))
#[inline]
fn tan_sub(val: Decimal, sub: Decimal) -> Decimal {
    (val - sub) / (D1 + (val * sub))
}

//##########################################################################################################################

#[inline]
fn is_valid_pair(_cos: Decimal, _sin: Decimal, terms: usize) -> bool {
    let digits = if terms > 32 {16} else {terms / 2} as u32;
    let module = (_cos * _cos) + (_sin * _sin);
    D1 == module.round_dp(digits)
}

//##########################################################################################################################

#[inline]
fn tan_lower(
    value: Decimal,
    offset: Decimal
) -> (Decimal, Decimal) {
    let mut rem: Decimal = value;
    let mut base: Decimal = offset;
    loop {
             if rem < D1DIV5 { break;                                                  }
        else if rem > D1     { base = base + PIDIV6;  rem = tan_sub(rem, TAN_PIDIV6);  }
        else if rem > D2DIV5 { base = base + PIDIV18; rem = tan_sub(rem, TAN_PIDIV18); }
        else                 { base = base + PIDIV36; rem = tan_sub(rem, TAN_PIDIV36); };
    };
    (rem, base)
}

//##########################################################################################################################

#[inline]
fn tan_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let mut rem: Decimal = value;
    let mut base: Decimal = D0;
         if rem >=  D0 {                                                        }
    else if rem >  -D1 { base = -PIDIV4; rem = tan_sub(rem, -D1);               }
    else if rem <= -D1 { base = -PIDIV2; rem = tan_sub(tan_sub(rem, -D1), -D1); };
    tan_lower(rem, base)
}

//##########################################################################################################################

#[inline]
fn tan2_prepare(
    _cos: Decimal,
    _sin: Decimal
) -> (Decimal, Decimal) {
    let mut rem: Decimal = _sin / _cos;
    let mut base: Decimal = D0;
         if (_sin >  D0) && (_cos <= D0) { base = PIDIV2;  rem = tan_sub2((_cos, _sin), PIDIV2_PAIR);  }
    else if (_sin <= D0) && (_cos <  D0) { base = PI;      rem = tan_sub2((_cos, _sin), PI_PAIR);      }
    else if (_sin <  D0) && (_cos >= D0) { base = PI3DIV2; rem = tan_sub2((_cos, _sin), PI3DIV2_PAIR); };
    tan_lower(rem, base)
}

//##########################################################################################################################

/// atan(x) = sum(n=0; -1^n * (x^(2n + 1) / (2n + 1)))
#[inline]
fn atan_series(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    let mut acc1: (Decimal, usize) = (D1, 0);
    let mut acc2: (Multiplex, usize) = (Multiplex::new(), 0);
    // Iterate over Series
    (0..terms).into_iter()
        .map(|n| Ok(
            a_pow(-D1, n, &mut acc1)? * (
                am_pow(value, (2 * n) + 1, &mut acc2)? / ((D2 * dec(n)) + D1)
            ).squash()?
        ))
        .reduce(|u, d| Ok(
            u?.checked_add(d?).ok_or(Error::AddOverflow)?
        ))
        .unwrap_or(Ok(D0))
}

//##########################################################################################################################

#[inline]
pub fn atan(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    Ok(
             if value == D0  {D0}
        else if value == D1  {PIDIV4}
        else if value == -D1 {-PIDIV4}
        else {
            let (rem, base) = tan_prepare(value);
            base + atan_series(rem, terms)?
        }
    )
}

//##########################################################################################################################

#[inline]
pub fn atan2(
    _cos: Decimal,
    _sin: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    if !is_valid_pair(_cos, _sin, terms) { Err(Error::InputOutOfRange)? };
    Ok(
             if (_cos >  D0) && (_sin == D0) {D0}
        else if (_cos == D0) && (_sin >  D0) {PIDIV2}
        else if (_cos <  D0) && (_sin == D0) {PI}
        else if (_cos == D0) && (_sin <  D0) {-PIDIV2}
        else {
            let (rem, base) = tan2_prepare(_cos, _sin);
            let arg = base + atan_series(rem, terms)?;
            if arg <= PI {arg} else {arg - PI2}
        }
    )
}

//##########################################################################################################################
