
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use std::thread::{ spawn };

// Modules
use crate::complex::types::{ Complex };
use crate::trigonometry::{ cos, sin };
use crate::euler::{ exp, ln };
use crate::error::Error;

//##########################################################################################################################

// Constants
const D2: Decimal = dec!(2);

//##########################################################################################################################

/// e^(a + bi) = e^a * (cos(b) + i*sin(b))
pub fn c_exp(
    value: Complex,
    terms: usize
) -> Result<Complex, Error> {
    // Execute Parallel
    let p_exp_re = spawn(move || exp(value.re, terms));
    let p_cos_im = spawn(move || cos(value.im, terms));
    let p_sin_im = spawn(move || sin(value.im, terms));
    // Extract Variables
    let exp_re = p_exp_re.join().unwrap()?;
    let cos_im = p_cos_im.join().unwrap()?;
    let sin_im = p_sin_im.join().unwrap()?;
    // Calculate Complex
    let re = exp_re * cos_im;
    let im = exp_re * sin_im;
    Ok(Complex::new(re, im))
}

//##########################################################################################################################

/// ln(z) = ln(|z|) + i*arg(z) = (ln(|z|^2) / 2) + i*arg(z)
pub fn c_ln(
    value: Complex,
    terms: usize
) -> Result<Complex, Error> {
    // Execute Parallel
    let p_ln_norm = spawn(move || Ok(ln(value.norm_sqr()?, terms)? / D2));
    let p_val_arg = spawn(move || value.arg(terms));
    // Extract Variables
    let re = p_ln_norm.join().unwrap()?;
    let im = p_val_arg.join().unwrap()?;
    // Calculate Complex
    Ok(Complex::new(re, im))
}

//##########################################################################################################################
