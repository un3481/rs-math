
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::constants::{ SQRT_OF_THREE_HALFS };
pub use crate::constants::{ pow, fac };

//##########################################################################################################################

// Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D1DIV2: Decimal = dec!(0.5);
const D3DIV2: Decimal = dec!(1.5);

//##########################################################################################################################

const fn sqrt_prepare(
    value: Decimal
) -> (Decimal, Decimal) {
    let mut rem = value.copy();
    let mut ratio = D1;
    loop {
        if rem > D3DIV2 {
            rem = rem / D3DIV2;
            ratio = ratio * SQRT_OF_THREE_HALFS;
        }
        else if rem < D1DIV2 {
            rem = rem * D3DIV2;
            ratio = ratio / SQRT_OF_THREE_HALFS;
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
    (1..=terms).par_iter()
        .map(|n| (
            (fac(2 * (n - 1)) * pow(D1 - value, n - 1) * value) /
            pow(fac(n - 1) * pow(D2, n - 1), 2)
        ))
        .reduce(|| D0, |u, d| u + d)
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

