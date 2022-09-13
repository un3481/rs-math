
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::constants::{
    SQRT_OF_THREE_HALFS,
    pow,
    fac
};

//##########################################################################################################################

// Constants
const D0 = dec!(0);
const D1 = dec!(1);
const D2 = dec!(2);
const D1DIV2 = dec!(0.5);
const D3DIV2 = dec!(1.5);

//##########################################################################################################################

// Re-export
pub const pow = pow;
pub const fac = fac;

//##########################################################################################################################

fn sqrt_series(
    terms: usize,
    value: Decimal
) -> Decimal {
    (1..=terms).par_iter()
        .map(|n| [
            || (
                value *
                fac(2 * (n - 1)) *
                pow(D1 - value, n - 1)
            ),
            || pow(
                fac(n - 1) *
                pow(D2, n - 1),
                2
            )
        ].par_iter())
        .map(|t| t.map(|f| f()).collect())
        .map(|t| t[0] / t[1])
        .reduce(|| D0, |u, d| u + d)
}

//##########################################################################################################################

fn sqrt_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let mut rem = value.copy();
    let mut ratio = D1;
    loop {
        match true {
            (rem > D3DIV2) => {
                rem = rem / D3DIV2;
                ratio = ratio * SQRT_OF_THREE_HALFS;
            },
            (rem < D1DIV2) => {
                rem = rem * D3DIV2;
                ratio = ratio / SQRT_OF_THREE_HALFS;
            },
            _ => {break},
        }
    };
    (ratio, rem)
}

//##########################################################################################################################

pub fn sqrt(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    if value < D0 {
        panic!("cannot calc sqrt(x) for x < 0");
    };
    Ok(
        match value {
            D0 => D0,
            D1 => D1,
            _ => {
                let (ratio, rem) = sqrt_prepare(value);
                ratio * sqrt_series(terms, rem)
            },
        }
    )
}

//##########################################################################################################################

