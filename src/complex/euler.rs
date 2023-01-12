
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

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
    let results = [
        || exp(value.re, terms),
        || cos(value.im, terms),
        || sin(value.im, terms)
    ].par_iter().map(|f| f()).collect();
    // Extract Variables
    let exp_re = results[0];
    let cos_im = results[1];
    let sin_im = results[2];
    // Calculate Complex
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
    let results = [
        || value.arg(terms),
        || ln(value.norm_sqr(), terms).unwrap_or(D0)
    ].par_iter().map(|f| f()).collect();
    // Extract Variables
    let arg = results[0];
    let ln_norm_sqr = results[1];
    // Calculate Complex
    let ln_norm = ln_norm_sqr / D2;
    Complex::new(ln_norm, arg)
}

//##########################################################################################################################
