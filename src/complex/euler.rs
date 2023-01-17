
// Imports
use std::thread::{ spawn };

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
    // Execute Parallel
    let p_cos_im = spawn(move || cos(value.im(), terms));
    let p_sin_im = spawn(move || sin(value.im(), terms));
    let r_exp_re = exp(value.re(), terms);
    // Join Threads
    let j_cos_im = p_cos_im.join();
    let j_sin_im = p_sin_im.join();
    // Extract Variables
    let cos_im = j_cos_im.unwrap()?;
    let sin_im = j_sin_im.unwrap()?;
    let exp_re = r_exp_re?;
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
    // Execute Parallel
    let p_value = value.clone();
    let p_ln_norm = spawn(move || Ok(ln(p_value.radius_sqr(), terms)? / D2));
    let r_val_arg = value.arg(terms);
    // Extract Variables
    let re = p_ln_norm.join().unwrap()?;
    let im = r_val_arg?;
    // Return Result
    Ok(Complex::new(re, im))
}

//##########################################################################################################################
