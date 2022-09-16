
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::complex::types::{ Complex };
use crate::complex::euler::{ c_exp, c_ln };

//##########################################################################################################################

pub fn c_pow(
    value: Complex,
    power: Complex,
    terms: usize
) -> Complex {
    let _ln = c_ln(value, terms);
    c_exp(_ln * power, terms)
}

//##########################################################################################################################
