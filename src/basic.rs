
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::constants::{ SQRT_3DIV2 };
use crate::arithmetic::{ pow, fac };
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
    let _sqrt3div2 = *SQRT_3DIV2;
    let mut rem: Decimal = value;
    let mut ratio: Decimal = D1;
    loop {
        if rem > D3DIV2 {
            rem = rem / D3DIV2;
            ratio = ratio * _sqrt3div2;
        }
        else if rem < D1DIV2 {
            rem = rem * D3DIV2;
            ratio = ratio / _sqrt3div2;
        }
        else {break}
    };
    (ratio, rem)
}

//##########################################################################################################################

/// sqrt(x) = sum(1->k; (x * (2 * (n - 1))! * (1 - x)^(n - 1)) / ((n - 1)! * 2^(n - 1))^2)
fn sqrt_series(
    value: Decimal,
    terms: usize
) -> Decimal {
    (1..=terms).into_par_iter()
        .map(|n|
            (
                value *
                fac(2 * (n - 1)) *
                pow(D1 - value, n - 1)
            ) /
            pow(
                fac(n - 1) *
                pow(D2, n - 1),
                2
            )
        )
        .reduce(|| D0, |u, d| u + d)
}
 
//##########################################################################################################################

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
pub fn d_pow(
    value: Decimal,
    power: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    match ln(value, terms) {
        Ok(_ln) => Ok(exp(_ln * power, terms)),
        Err(err) => Err(err),
    }
}

//##########################################################################################################################
