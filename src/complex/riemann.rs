
// Imports
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::error::Error;

use crate::complex::types::{ Complex };
use crate::complex::basic::{ cpx, cc_pow };

//##########################################################################################################################

// Constants
const D1: Decimal = Decimal::ONE;

const C0: Complex = Complex::ZERO;

//##########################################################################################################################

/// Z(x) = 1/(1^x) + 1/(2^x) + 1/(3^x) + 1/(4^x) + 1/(5^x) ...
/// Z(x) = sum(n=1; n^(-x))
fn zeta_series(
    value: Complex,
    zeta_terms: usize,
    terms: usize
) -> Result<Complex, Error> {
    // Iterate over Zeta Series
    (1..=zeta_terms).into_par_iter()
        .map(|n| Ok(
            D1 / cc_pow(&mut cpx(n), value, terms)?
        ))
        .reduce(|| Ok(C0), |u, d| Ok(u? + d?))
}

/// Riemann Zeta Function
pub fn zeta(
    value: Complex,
    zeta_terms: usize,
    terms: usize
) -> Result<Complex, Error> {
    if value.re() <= D1 { Err(Error::InputOutOfRange)? };
    zeta_series(value, zeta_terms, terms)
}

//##########################################################################################################################
