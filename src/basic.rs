
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::constants::{ SQRT_3DIV2 };
use crate::arithmetic::{ pow, fac };

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

fn sqrt_series(
    terms: usize,
    value: Decimal
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
    terms: usize,
    value: Decimal
) -> Result<Decimal, &'static str> {
    if value < D0 {
        return Err("cannot calc sqrt(x) for x < 0")
    };
    Ok(
             if value == D0 {D0}
        else if value == D1 {D1}
        else {
            let (ratio, rem) = sqrt_prepare(value);
            ratio * sqrt_series(terms, rem)
        }
    )
}

//##########################################################################################################################
