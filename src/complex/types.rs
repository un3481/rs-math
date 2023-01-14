
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

// Modules
use crate::trigonometry::{ cos, sin, atan2 };
use crate::arithmetic::{ pos };
use crate::basic::{ sqrt };
use crate::error::Error;

//##########################################################################################################################

// Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);

//##########################################################################################################################

/// A complex number in Polar form. `z = r * exp(i * theta)`
#[derive(PartialEq, Copy, Clone, Hash, Debug)]
pub struct Polar {
    /// Modulus of complex number |self|
    pub radius: Decimal,
    /// Angle of complex number
    pub theta: Decimal
}

impl Polar {
    /// Create a new Complex in Polar form
    #[inline]
    pub fn new(radius: Decimal, theta: Decimal) -> Polar {
        Polar { radius: radius, theta: theta }
    }

    /// Convert a polar representation into a complex number.
    #[inline]
    pub fn to_cartesian(&self, terms: usize) -> Result<Complex, Error> {
        let costheta = cos(self.theta.clone(), terms)?;
        let sintheta = sin(self.theta.clone(), terms)?;
        let re = self.radius.clone().checked_mul(costheta).ok_or(Error::MultiplyOverflow)?;
        let im = self.radius.clone().checked_mul(sintheta).ok_or(Error::MultiplyOverflow)?;
        Ok(Complex::new(re, im))
    }
}

//##########################################################################################################################

/// A complex number in Cartesian form. `z = a + i * b`
#[derive(PartialEq, Copy, Clone, Hash, Debug)]
pub struct Complex {
    /// Real portion of the complex number
    pub re: Decimal,
    /// Imaginary portion of the complex number
    pub im: Decimal
}

impl Complex {
    /// Create a new Complex
    #[inline]
    pub fn new(re: Decimal, im: Decimal) -> Complex {
        Complex { re: re, im: im }
    }

    /// Returns imaginary unit
    #[inline]
    pub fn i() -> Complex {
        Self::new(D0, D1)
    }

    /// Multiplies `self` by the scalar `t`.
    #[inline]
    pub fn scale(&self, t: Decimal) -> Result<Complex, Error> {
        let re = self.re.clone().checked_mul(t.clone()).ok_or(Error::MultiplyOverflow)?;
        let im = self.im.clone().checked_mul(t.clone()).ok_or(Error::MultiplyOverflow)?;
        Ok(Complex::new(re, im))
    }

    /// Divides `self` by the scalar `t`.
    #[inline]
    pub fn unscale(&self, t: Decimal) -> Complex {
        Complex::new(
            self.re.clone() / t.clone(),
            self.im.clone() / t.clone()
        )
    }

    /// Returns the square of the norm (since `T` doesn't necessarily
    /// have a sqrt function), i.e. `re^2 + im^2`.
    #[inline]
    pub fn norm_sqr(&self) -> Result<Decimal, Error> {
        let re_sqr = self.re.clone().checked_mul(self.re.clone()).ok_or(Error::MultiplyOverflow)?;
        let im_sqr = self.im.clone().checked_mul(self.im.clone()).ok_or(Error::MultiplyOverflow)?;
        re_sqr.checked_add(im_sqr).ok_or(Error::AddOverflow)
    }

    /// Calculate |self|
    #[inline]
    pub fn norm(&self, terms: usize) -> Result<Decimal, Error> {
        Ok(
                 if self.is_zero() {D0}
            else if self.im == D0 { pos(self.re.clone()) }
            else if self.re == D0 { pos(self.im.clone()) }
            else {
                let _sqr = self.norm_sqr()?;
                sqrt(_sqr, terms)?
            }
        )
    }

    /// Calculate the principal Arg of self.
    #[inline]
    pub fn arg(&self, terms: usize) -> Result<Decimal, Error> {
        Ok(
            if self.is_zero() {D0}
            else {
                let _norm = self.norm(terms)?;
                let _cos = self.re.clone() / _norm;
                let _sin = self.im.clone() / _norm;
                atan2(_cos, _sin, terms)?
            }
        )
    }

    /// Convert to polar form (r, theta)
    #[inline]
    pub fn to_polar(&self, terms: usize) -> Result<Polar, Error> {
        Ok(
            Polar::new(
                self.norm(terms)?,
                self.arg(terms)?
            )
        )
    }
}

//##########################################################################################################################

impl Complex {
    /// Returns the complex conjugate. i.e. `re - i im`
    #[inline]
    pub fn conj(&self) -> Complex {
        Complex::new(self.re.clone(), -self.im.clone())
    }

    /// Returns `1/self`
    #[inline]
    pub fn inv(&self) -> Result<Complex, Error> {
        let norm_sqr = self.norm_sqr()?;
        let re = self.re.clone() / norm_sqr.clone();
        let im = -self.im.clone() / norm_sqr.clone();
        Ok(Complex::new(re, im))
    }
}

/*

impl Complex {
    /// Checks if the given complex number is NaN
    #[inline]
    pub fn is_nan(self) -> bool {
        self.re.is_nan() || self.im.is_nan()
    }

    /// Checks if the given complex number is infinite
    #[inline]
    pub fn is_infinite(self) -> bool {
        !self.is_nan() && (self.re.is_infinite() || self.im.is_infinite())
    }

    /// Checks if the given complex number is finite
    #[inline]
    pub fn is_finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }

    /// Checks if the given complex number is normal
    #[inline]
   pub fn is_normal(self) -> bool {
        self.re.is_normal() && self.im.is_normal()
    }
}

*/

//##########################################################################################################################

// (a + i b) + (c + i d) == (a + c) + i (b + d)
impl Add<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn add(self, other: Complex) -> Complex {
        Complex::new(self.re + other.re, self.im + other.im)
    }
}

// (a + i b) - (c + i d) == (a - c) + i (b - d)
impl Sub<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.re - other.re, self.im - other.im)
    }
}

// (a + i b) * (c + i d) == (a*c - b*d) + i (a*d + b*c)
impl Mul<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn mul(self, other: Complex) -> Complex {
        let re = self.re.clone() * other.re.clone() - self.im.clone() * other.im.clone();
        let im = self.re * other.im + self.im * other.re;
        Complex::new(re, im)
    }
}

// (a + i b) / (c + i d) == [(a + i b) * (c - i d)] / (c*c + d*d)
//   == [(a*c + b*d) / (c*c + d*d)] + i [(b*c - a*d) / (c*c + d*d)]
impl Div<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn div(self, other: Complex) -> Complex {
        let norm_sqr = other.norm_sqr().unwrap();
        let re = self.re.clone() * other.re.clone() + self.im.clone() * other.im.clone();
        let im = self.im * other.re - self.re * other.im;
        Complex::new(re / norm_sqr.clone(), im / norm_sqr)
    }
}

impl Neg for Complex {
    type Output = Complex;

    #[inline]
    fn neg(self) -> Complex {
        Complex::new(-self.re, -self.im)
    }
}

impl<'a> Neg for &'a Complex {
    type Output = Complex;

    #[inline]
    fn neg(self) -> Complex {
        -self.clone()
    }
}

//##########################################################################################################################

/* constants */
impl Zero for Complex {
    #[inline]
    fn zero() -> Complex {
        Complex::new(D0, D0)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        (self.re == D0) && (self.im == D0)
    }
}

/* string conversions */
impl fmt::Display for Complex {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < Zero::zero() {
            write!(f, "{}-{}i", self.re, D0 - self.im.clone())
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}

//##########################################################################################################################
