
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::constants::{ SQRT_3DIV2 };
use crate::factorial::{ m_fac };
use crate::arithmetic::{ a_pow, m_pow };
use crate::euler::{ exp, ln };
use crate::error::Error;

//##########################################################################################################################

// Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D1DIV2: Decimal = dec!(0.5);
const D3DIV2: Decimal = dec!(1.5);

//##########################################################################################################################

fn sqrt_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let mut rem: Decimal = value;
    let mut ratio: Decimal = D1;
    loop {
        if rem > D3DIV2 {
            rem = rem / D3DIV2;
            ratio = ratio * SQRT_3DIV2;
        }
        else if rem < D1DIV2 {
            rem = rem * D3DIV2;
            ratio = ratio / SQRT_3DIV2;
        }
        else {break}
    };
    (ratio, rem)
}

//##########################################################################################################################

/// sqrt(x) = sum(n=1; (x * (2 * (n - 1))! * (1 - x)^(n - 1)) / ((n - 1)! * 2^(n - 1))^2)
fn sqrt_series(
    value: Decimal,
    terms: usize
) -> Decimal {
    let mut acc1: (Decimal, usize) = (D1, 0);
    let mut acc2: (Decimal, usize) = (D1, 0);
    // Iterate over Taylor Series
    (1..=terms).into_iter()
        .map(|n|
            (
                (
                    value *
                    m_fac(2 * (n - 1)) *
                    a_pow(D1 - value, n - 1, &mut acc1)
                ) /
                m_pow(
                    m_fac(n - 1) *
                    a_pow(D2, n - 1, &mut acc2),
                    2
                )
            ).squash().unwrap()
        )
        .reduce(|u, d| u + d)
        .unwrap_or(D0)
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
            let (ratio, rem) = sqrt_prepare(value);
            ratio * sqrt_series(rem, terms)
        }
    )
}

//##########################################################################################################################

/// a^b = e^(ln(a) * b)
#[inline]
pub fn d_pow(
    value: Decimal,
    power: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    match ln(value, terms) {
        Err(err) => Err(err),
        Ok(ln_val) => Ok(
            exp(ln_val * power, terms)
        ),
    }
}

//##########################################################################################################################
