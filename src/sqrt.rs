
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::constants::{ SQRT_UPPER_BD, SQRT_UPPER_VAL, SQRT_LOWER_BD, SQRT_LOWER_VAL };

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
const D100: Decimal = dec!(100);
const D1DIV4: Decimal = dec!(0.25);

//##########################################################################################################################

#[inline]
fn sqrt_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let mut rem: Decimal = value;
    let mut base: Decimal = D1;
    loop {
        if rem > D4 {
            rem = rem / D4;
            base = base * D2;
        }
        else if rem < D1DIV4 {
            rem = rem * D4;
            base = base / D2;
        }
        else {break}
    };
    loop {
        if rem > SQRT_UPPER_BD {
            rem = rem / SQRT_UPPER_BD;
            base = base * SQRT_UPPER_VAL;
        }
        else if rem < SQRT_LOWER_BD {
            rem = rem / SQRT_LOWER_BD;
            base = base * SQRT_LOWER_VAL;
        }
        else {break}
    };
    (rem, base)
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
    if value < D0 { Err(Error::InputOutOfRange)? };
    Ok(
             if value == D0 {D0}
        else if value == D1 {D1}
        else {
            let (rem, base) = sqrt_prepare(value);
            base * sqrt_series(rem, terms)?
        }
    )
}

//##########################################################################################################################

#[inline]
fn int_sqrt_100(
    value: Decimal
) -> Result<Decimal, Error> {
    let vu: u8 = value.to_u8().ok_or(Error::InputOutOfRange)?;
    Ok(
        match vu {
              0 => { D0  },
              1 => { D1  },
              4 => { D2  },
              9 => { D3  },
             16 => { D4  },
             25 => { D5  },
             36 => { D6  },
             49 => { D7  },
             64 => { D8  },
             81 => { D9  },
            100 => { D10 },
              _ => { Err(Error::InputOutOfRange)? },
        }
    )
}

//##########################################################################################################################

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
    if value.fract() != D0 { Err(Error::InputOutOfRange)? };
    if value <= D100 { int_sqrt_100(value) }
    else { int_sqrt_helper(value) }
}

//##########################################################################################################################
