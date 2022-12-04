
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::complex::types::{ Complex };
use crate::complex::euler::{ c_exp, c_ln };

//##########################################################################################################################

const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const C0: Complex = Complex{ re: D0, im: D0 };
const C1: Complex = Complex{ re: D1, im: D0 };

//##########################################################################################################################

pub fn c_pow(
    value: Complex,
    power: usize
) -> Complex {
    match power {
        0 => C1,
        1 => value,
        _ => {
                 if value == C0 {C0}
            else if value == C1 {C1}
            else {
                (1..=power).into_iter()
                    .map(|_| value)
                    .reduce(|u, d| u * d)
                    .unwrap()
            }
        },
    }
}

//##########################################################################################################################

/// z^w = e^(ln(z) * w)
pub fn cc_pow(
    value: Complex,
    power: Complex,
    terms: usize
) -> Complex {
    let ln_val = c_ln(value, terms);
    c_exp(ln_val * power, terms)
}

//##########################################################################################################################
