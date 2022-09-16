
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::complex::types::{ Complex };
use crate::complex::utils::{ arg };
use crate::trigonometry::{ cos, sin };
use crate::euler::{ exp, ln };

//##########################################################################################################################

// Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);

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
    let normsq = (
        (value.re * value.re) +
        (value.im * value.im)
    );
    let normln = ln(normsq, terms).unwrap() / D2;
    let _arg = arg(value, terms);
    Complex::new(normln, _arg)
}

//##########################################################################################################################
