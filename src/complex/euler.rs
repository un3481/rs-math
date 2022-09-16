
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::complex::types::{ Complex };
use crate::trigonometry::{ cos, sin };
use crate::euler::{ exp, ln };

//##########################################################################################################################

// Constants
const D0: Decimal = dec!(0);
const D2: Decimal = dec!(2);

//##########################################################################################################################

pub fn c_exp(
    value: Complex,
    terms: usize
) -> Complex {
    let ea = exp(value.re, terms);
    Complex::new(
        ea * cos(value.im, terms),
        ea * sin(value.im, terms)
    )
}

//##########################################################################################################################

pub fn c_ln(
    value: Complex,
    terms: usize
) -> Complex {
    let norm_ln = (
        ln(
            value.norm_sqr(),
            terms
        ).unwrap_or(D0) 
    ) / D2;
    let _arg = value.arg(terms);
    Complex::new(norm_ln, _arg)
}

//##########################################################################################################################
