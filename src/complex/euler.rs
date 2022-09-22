
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
    let _ea = exp(value.re, terms);
    let _cos = cos(value.im, terms);
    let _sin = sin(value.im, terms);
    Complex::new(_ea * _cos, _ea * _sin)
}

//##########################################################################################################################

/// ln(z) = ln(|z|) + i*arg(z) = (ln(|z|^2) / 2) + i*arg(z)
pub fn c_ln(
    value: Complex,
    terms: usize
) -> Complex {
    let _arg = value.arg(terms);
    let _sqr = value.norm_sqr();
    let _ln_sqr = ln(_sqr, terms).unwrap_or(D0);
    let _ln = _ln_sqr / D2;
    Complex::new(_ln, _arg)
}

//##########################################################################################################################
