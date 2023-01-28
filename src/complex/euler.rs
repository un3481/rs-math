
// Imports
use rust_decimal::prelude::*;

// Modules
use crate::error::Error;
use crate::trigonometry::{ cos, sin };
use crate::euler::{ exp, ln };

use crate::complex::types::{ Complex };

//##########################################################################################################################

// Constants
const D2: Decimal = Decimal::TWO;

//##########################################################################################################################

/// e^(a + bi) = e^a * (cos(b) + i*sin(b))
pub fn c_exp(
    value: Complex,
    terms: usize
) -> Result<Complex, Error> {
    // Calculate Variables
    let cos_im = cos(value.im(), terms)?;
    let sin_im = sin(value.im(), terms)?;
    let exp_re = exp(value.re(), terms)?;
    // Calculate Complex
    let re = exp_re * cos_im;
    let im = exp_re * sin_im;
    // Return Result
    Ok(Complex::new(re, im))
}

//##########################################################################################################################

/// ln(z) = ln(|z|) + i*arg(z) = (ln(|z|^2) / 2) + i*arg(z)
pub fn c_ln(
    value: &mut Complex,
    terms: usize
) -> Result<Complex, Error> {
    // Calculate Complex
    let re = ln(value.radius_sqr(), terms)? / D2;
    let im = value.arg(terms)?;
    // Return Result
    Ok(Complex::new(re, im))
}

//##########################################################################################################################
