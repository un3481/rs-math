
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
    let exp_re = exp(value.re, terms);
    let cos_im = cos(value.im, terms);
    let sin_im = sin(value.im, terms);
    let re = exp_re * cos_im;
    let im = exp_re * sin_im;
    Complex::new(re, im)
}

//##########################################################################################################################

/// ln(z) = ln(|z|) + i*arg(z) = (ln(|z|^2) / 2) + i*arg(z)
pub fn c_ln(
    value: Complex,
    terms: usize
) -> Complex {
    let arg = value.arg(terms);
    let norm_sqr = value.norm_sqr();
    let ln_norm_sqr = ln(norm_sqr, terms).unwrap_or(D0);
    let ln_norm = ln_norm_sqr / D2;
    Complex::new(ln_norm, arg)
}

//##########################################################################################################################
