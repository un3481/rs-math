
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

/// e^(a + bi) = e^a * (cos(b) + i*sin(b))
pub fn c_exp(
    value: Complex,
    terms: usize
) -> Complex {
    let ea = exp(value.re, terms);
    let cosb = cos(value.im, terms);
    let sinb = sin(value.im, terms);
    Complex::new(ea * cosb, ea * sinb)
}

//##########################################################################################################################

/// ln(z) = ln(|z|) + i*arg(z) = (ln(|z|^2) / 2) + i*arg(z)
pub fn c_ln(
    value: Complex,
    terms: usize
) -> Complex {
    let arg = value.arg(terms);
    let modsqr = value.norm_sqr();
    let lnmodsqr = ln(modsqr, terms).unwrap_or(D0);
    let lnmod = lnmodsqr / D2;
    Complex::new(lnmod, arg)
}

//##########################################################################################################################
