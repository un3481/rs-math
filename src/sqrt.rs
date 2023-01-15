
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::constants::{ SQRT_EXP_VAL, SQRT_EXP_BD, SQRT_UPPER_BD, SQRT_LOWER_BD };

use crate::error::Error;
use crate::multiplex::types::{ Multiplex };
use crate::factorial::{ m_fac };
use crate::basic::{ m_pow, am_pow };

//##########################################################################################################################

// Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D3: Decimal = dec!(3);
const D4: Decimal = dec!(4);
const D5: Decimal = dec!(5);
const D6: Decimal = dec!(6);
const D7: Decimal = dec!(7);
const D8: Decimal = dec!(8);
const D9: Decimal = dec!(9);
const D10: Decimal = dec!(10);
const D1DIV4: Decimal = dec!(0.25);

const D16: Decimal = dec!(16);
const D25: Decimal = dec!(25);
const D36: Decimal = dec!(36);
const D49: Decimal = dec!(49);
const D64: Decimal = dec!(64);
const D81: Decimal = dec!(81);
const D100: Decimal = dec!(100);

//##########################################################################################################################

#[inline]
fn sqrt_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let mut rem: Decimal = value;
    let mut exp: Decimal = D1;
    loop {
        if rem > D4 {
            rem = rem / D4;
            exp = exp * D2;
        }
        else if rem < D1DIV4 {
            rem = rem * D4;
            exp = exp / D2;
        }
        else {break}
    };
    loop {
        if rem > SQRT_UPPER_BD {
            rem = rem / SQRT_EXP_BD;
            exp = exp * SQRT_EXP_VAL;
        }
        else if rem < SQRT_LOWER_BD {
            rem = rem * SQRT_EXP_BD;
            exp = exp / SQRT_EXP_VAL;
        }
        else {break}
    };
    (exp, rem)
}

//##########################################################################################################################

/// sqrt(x) = sum(n=1; (x * (2 * (n - 1))! * (1 - x)^(n - 1)) / ((n - 1)! * 2^(n - 1))^2)
#[inline]
fn sqrt_series(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    let mut acc1: (Multiplex, usize) = (Multiplex::new(), 0);
    let mut acc2: (Multiplex, usize) = (Multiplex::new(), 0);
    // Iterate over Taylor Series
    (1..=terms).into_iter()
        .map(|n| Ok(
            (
                (
                    value *
                    m_fac(2 * (n - 1))? *
                    am_pow(D1 - value, n - 1, &mut acc1)?
                ) /
                m_pow(
                    m_fac(n - 1)? *
                    am_pow(D2, n - 1, &mut acc2)?,
                    2
                )?
            ).squash()?
        ))
        .reduce(|u, d| Ok(
            u?.checked_add(d?).ok_or(Error::AddOverflow)?
        ))
        .unwrap_or(Ok(D0))
}
 
//##########################################################################################################################

#[inline]
pub fn sqrt(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    if value < D0 { return Err(Error::InputOutOfRange) };
    Ok(
             if value == D0 {D0}
        else if value == D1 {D1}
        else {
            let (exp, rem) = sqrt_prepare(value);
            exp * sqrt_series(rem, terms)?
        }
    )
}

//##########################################################################################################################

#[inline]
fn int_sqrt_100(
    value: Decimal
) -> Result<Decimal, Error> {
         if value == D0   {Ok(D0 )}
    else if value == D1   {Ok(D1 )}
    else if value == D4   {Ok(D2 )}
    else if value == D9   {Ok(D3 )}
    else if value == D16  {Ok(D4 )}
    else if value == D25  {Ok(D5 )}
    else if value == D36  {Ok(D6 )}
    else if value == D49  {Ok(D7 )}
    else if value == D64  {Ok(D8 )}
    else if value == D81  {Ok(D9 )}
    else if value == D100 {Ok(D10)}
    else { Err(Error::InputOutOfRange) }
}

#[inline]
fn int_sqrt_helper(
    value: Decimal
) -> Result<Decimal, Error> {
    // Set Variables
    let mut left: Decimal = D100;
    let mut right: Decimal = value;
    // Loop until value is reached
    loop {
        if left > right {break Err(Error::InputOutOfRange)};
        // Set Variables
        let mid = ((left + right) / D2).round();
        let mid_sqr = mid * mid;
        // Check if mid is perfect square
        if mid_sqr == value {break Ok(mid)};
        // Mid is small -> go right to increase mid
        if mid_sqr < value { left = mid + D1; }
        // Mid is large -> go left to decrease mid
        else { right = mid - D1; };
    }
}

//##########################################################################################################################

// Find perfect square of integer.
#[inline]
pub fn int_sqrt(
    value: Decimal
) -> Result<Decimal, Error> {
    if value.fract() != D0 { return Err(Error::InputOutOfRange) };
    if value <= D100 { int_sqrt_100(value) }
    else { int_sqrt_helper(value) }
}

//##########################################################################################################################
