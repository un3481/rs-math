
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::basic::{ pow, fac };
use crate::constants::{ PI };

//##########################################################################################################################

// Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D1NEG: Decimal = dec!(-1);

//##########################################################################################################################

const fn trig_prepare(
    value: Decimal
) -> Decimal {
    let mut rem = value;
    if rem > PI {
        rem = rem - (
            (rem / (PI * D2)).floor() * PI * D2
        )
    }
    else if rem < (D1NEG * PI) {
        rem = rem - (
            (rem / (PI * D2)).floor() * PI * D2
        )
    }
    if rem > PI { rem = rem - (PI * D2) }
    else if rem < (D1NEG * PI) { rem = rem + (PI * D2) };
    rem
}

//##########################################################################################################################

fn cos_series(
    terms: usize,
    value: Decimal
) -> Decimal {
    (0..terms).into_par_iter()
        .map(|n| (
            pow(D1NEG, n) * (
                pow(value, 2 * n) /
                fac(2 * n)
            )
        ))
        .reduce(|| D0, |u, d| u + d)
}

//##########################################################################################################################

fn sin_series(
    terms: usize,
    value: Decimal
) -> Decimal {
    (0..terms).into_par_iter()
        .map(|n| (
            pow(D1NEG, n) * (
                pow(value, (2 * n) + 1) /
                fac((2 * n) + 1)
            )
        ))
        .reduce(|| D0, |u, d| u + d)
}

//##########################################################################################################################

pub fn cos(
    terms: usize,
    value: Decimal
) -> Result<Decimal, ()> {
    let rem = trig_prepare(value);
    let PIHALF = PI / D2;
    let PIHNEG = D1NEG * PIHALF;
    let PINEG = D1NEG * PI;
    Ok(
        match rem {
            PI => D1NEG,
            PIHALF => D0,
            D0 => D1,
            PIHNEG => D0,
            PINEG => D1NEG,
            _ => cos_series(terms, rem),
        }
    )
}

//##########################################################################################################################

pub fn sin(
    terms: usize,
    value: Decimal
) -> Result<Decimal, ()> {
    let rem = trig_prepare(value);
    let PIHALF = PI / D2;
    let PIHNEG = D1NEG * PIHALF;
    let PINEG = D1NEG * PI;
    Ok(
        match rem {
            PI => D0,
            PIHALF => D1,
            D0 => D0,
            PIHNEG => D1NEG,
            PINEG => D0,
            _ => sin_series(terms, rem),
        }
    )
}

//##########################################################################################################################
