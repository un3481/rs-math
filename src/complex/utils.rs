
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::complex::types::{ Complex };
use crate::trigonometry::{ cos, sin, atan };
use crate::basic::{ sqrt };

/// Calculate |self|
#[inline]
pub fn norm(
    value: Complex,
    terms: usize
) -> Decimal {
    sqrt(
        (value.re * value.re) +
        (value.im * value.im),
        terms
    ).unwrap()
}

/// Calculate the principal Arg of self.
#[inline]
pub fn arg(
    value: Complex,
    terms: usize
) -> Decimal {
    if value.is_zero() { return D0 }
    let normal = norm(value, terms);
    atan(
        value.re / normal,
        value.im / normal,
        terms
    ).unwrap()
}

/// Convert to polar form (r, theta), such that `self = r * exp(i * theta)`
#[inline]
pub fn to_polar(
    value: Complex,
    terms: usize
) -> (Decimal, Decimal) {
    (norm(value, terms), arg(value, terms))
}

/// Convert a polar representation into a complex number.
#[inline]
pub fn from_polar(
    r: Decimal,
    theta: Decimal,
    terms: usize
) -> Complex {
    let cost = cos(theta, terms);
    let sint = sin(theta, terms);
    Complex::new(r * cost, r * sint)
}
