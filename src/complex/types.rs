
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use std::thread::{ spawn };

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

// Modules
use crate::constants::{ PI };

use crate::error::Error;
use crate::sqrt::{ sqrt };
use crate::trigonometry::{ cos, sin, atan2 };

//##########################################################################################################################

// Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);

// Standard Iterations
const STD_ITER: usize = 32;

//##########################################################################################################################

/// A complex number in Polar form. `z = r * exp(i * theta)`
#[derive(Copy, Clone, Hash, Debug)]
pub struct Polar {
    /// Radius of complex number |self|
    _radius: Decimal,
    /// Angle of complex number
    _theta: Decimal,
    // Cartesian form of complex number
    _cartesian: Option<(usize, Complex)>,
}

//##########################################################################################################################

impl Polar {
    /// Get radius property
    #[inline]
    pub const fn radius(&self) -> Decimal {
        self._radius
    }

    /// Get theta property
    #[inline]
    pub const fn theta(&self) -> Decimal {
        self._theta
    }
}

//##########################################################################################################################

impl PartialEq for Polar {
    fn eq(&self, other: &Self) -> bool {
        ( self._radius == other.radius() ) &&
        ( self._theta  == other.theta()  )
    }
}

impl PartialEq<Complex> for Polar {
    fn eq(&self, other: &Complex) -> bool {
        let terms = match &self._cartesian {
            None => STD_ITER,
            Some(v) => v.0,
        };
        &self.clone().to_cartesian(terms).unwrap() == other
    }
}

impl PartialEq<Decimal> for Polar {
    fn eq(&self, other: &Decimal) -> bool {
        let theta = if other >= &D0 {D0} else {PI};
        ( self._radius == other.abs() ) &&
        ( self._theta  == theta       )
    }
}

impl PartialEq<Polar> for Decimal {
    fn eq(&self, other: &Polar) -> bool {
        let theta = if self >= &D0 {D0} else {PI};
        ( self.abs() == other.radius() ) &&
        ( theta      == other.theta()  )
    }
}

//##########################################################################################################################

impl Polar {
    /// Create a new Complex in Polar form
    #[inline]
    pub const fn new(radius: Decimal, theta: Decimal) -> Polar {
        Polar { _radius: radius, _theta: theta, _cartesian: None }
    }

    /// Create a new Complex in Polar form from Cartesian
    #[inline]
    pub const fn from_cartesian(radius: Decimal, theta: Decimal, cartesian: (usize, Complex)) -> Polar {
        Polar { _radius: radius, _theta: theta, _cartesian: Some(cartesian) }
    }

    #[inline]
    fn to_cartesian_helper(&self, terms: usize) -> Result<Complex, Error> {
        // Execute Parallel
        let p_theta = self._theta.clone();
        let p_cost = spawn(move || cos(p_theta, terms));
        let r_sint = sin(self._theta, terms);
        // Extract Variables
        let cost = p_cost.join().unwrap()?;
        let sint = r_sint?;
        // Calculate Result
        let re = self._radius.checked_mul(cost).ok_or(Error::MultiplyOverflow)?;
        let im = self._radius.checked_mul(sint).ok_or(Error::MultiplyOverflow)?;
        Ok(Complex::from_polar(re, im, (terms, self.clone())))
    }

    /// Convert a polar representation into a complex number.
    #[inline]
    pub fn to_cartesian(&mut self, terms: usize) -> Result<Complex, Error> {
        match &self._cartesian {
            None => {
                self._cartesian = Some((terms, self.to_cartesian_helper(terms)?));
            },
            Some(val) => if terms > val.0 {
                self._cartesian = Some((terms, self.to_cartesian_helper(terms)?));
            },
        };
        Ok(self._cartesian.ok_or(Error::OptionInvalid)?.1.clone())
    }
}

//##########################################################################################################################

/// A complex number in Cartesian form. `z = a + i * b`
#[derive(Copy, Clone, Hash, Debug)]
pub struct Complex {
    /// Real portion of the complex number
    _re: Decimal,
    /// Imaginary portion of the complex number
    _im: Decimal,
    // Radius of complex number |self|
    _norm: Option<(usize, Decimal)>,
    /// Angle of complex number
    _arg: Option<(usize, Decimal)>
}

//##########################################################################################################################

impl Complex {
    /// Get re property
    #[inline]
    pub const fn re(&self) -> Decimal {
        self._re
    }

    /// Get im property
    #[inline]
    pub const fn im(&self) -> Decimal {
        self._im
    }
}

//##########################################################################################################################

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        ( self._re == other.re() ) &&
        ( self._im == other.im() )
    }
}

impl PartialEq<Polar> for Complex {
    fn eq(&self, other: &Polar) -> bool {
        let norm_terms = match &self._norm {
            None => STD_ITER,
            Some(v) => v.0,
        };
        let arg_terms = match &self._arg {
            None => STD_ITER,
            Some(v) => v.0,
        };
        let terms = if norm_terms > arg_terms {norm_terms} else {arg_terms};
        &self.clone().to_polar(terms).unwrap() == other
    }
}

impl PartialEq<Decimal> for Complex {
    fn eq(&self, other: &Decimal) -> bool {
        ( &self._re == other ) &&
        ( self._im  == D0    )
    }
}

impl PartialEq<Complex> for Decimal {
    fn eq(&self, other: &Complex) -> bool {
        ( self == &other.re() ) &&
        ( D0   == other.im()  )
    }
}

//##########################################################################################################################

impl Complex {
    /// Create a new Complex
    #[inline]
    pub const fn new(re: Decimal, im: Decimal) -> Complex {
        Complex { _re: re, _im: im, _norm: None, _arg: None }
    }

    /// Create a new Complex from Polar form
    #[inline]
    pub const fn from_polar(re: Decimal, im: Decimal, polar: (usize, Polar)) -> Complex {
        Complex { _re: re, _im: im, _norm: Some((polar.0, polar.1.radius())), _arg: Some((polar.0, polar.1.theta())) }
    }

    /// Convert to polar form (r, theta)
    #[inline]
    pub fn to_polar(&mut self, terms: usize) -> Result<Polar, Error> {
        // Extract Variables
        let theta  = self.arg(terms)?;
        let radius = self.norm(terms)?;
        // Calculate Result
        Ok(Polar::from_cartesian(radius, theta, (terms, self.clone())))
    }
}

//##########################################################################################################################

impl Complex {
    /// Returns imaginary unit
    #[inline]
    pub fn i() -> Complex {
        Complex::new(D0, D1)
    }

    /// Multiplies `self` by the scalar `t`.
    #[inline]
    pub fn scale(&self, value: Decimal) -> Complex {
        Complex::new(
            self._re * value,
            self._im * value
        )
    }

    /// Divides `self` by the scalar `t`.
    #[inline]
    pub fn unscale(&self, value: Decimal) -> Complex {
        Complex::new(
            self._re / value,
            self._im / value
        )
    }

    /// Returns the square of the norm (since `T` doesn't necessarily
    /// have a sqrt function), i.e. `re^2 + im^2`.
    #[inline]
    pub fn norm_sqr(&self) -> Result<Decimal, Error> {
        let re_sqr = self._re.checked_mul(self._re).ok_or(Error::MultiplyOverflow)?;
        let im_sqr = self._im.checked_mul(self._im).ok_or(Error::MultiplyOverflow)?;
        re_sqr.checked_add(im_sqr).ok_or(Error::AddOverflow)
    }
}

//##########################################################################################################################

impl Complex {

    #[inline]
    fn norm_helper(&self, terms: usize) -> Result<Decimal, Error> {
        Ok(
                 if self.is_zero() {D0}
            else if self._im == D0 { self._re.abs() }
            else if self._re == D0 { self._im.abs() }
            else {
                let _sqr = self.norm_sqr()?;
                sqrt(_sqr, terms)?
            }
        )
    }

    /// Calculate |self|
    #[inline]
    pub fn norm(&mut self, terms: usize) -> Result<Decimal, Error> {
        match &self._norm {
            None => {
                self._norm = Some((terms, self.norm_helper(terms)?));
            },
            Some(val) => if terms > val.0 {
                self._norm = Some((terms, self.norm_helper(terms)?));
            },
        };
        Ok(self._norm.ok_or(Error::OptionInvalid)?.1.clone())
    }
}

//##########################################################################################################################

impl Complex {

    #[inline]
    fn arg_helper(&mut self, terms: usize) -> Result<Decimal, Error> {
        Ok(
            if self.is_zero() {D0}
            else {
                let _norm = self.norm(terms)?;
                let _cos = self._re / _norm;
                let _sin = self._im / _norm;
                atan2(_cos, _sin, terms)?
            }
        )
    }

    /// Calculate the principal Arg of self.
    #[inline]
    pub fn arg(&mut self, terms: usize) -> Result<Decimal, Error> {
        match &self._arg {
            None => {
                self._norm = Some((terms, self.arg_helper(terms)?));
            },
            Some(val) => if terms > val.0 {
                self._norm = Some((terms, self.arg_helper(terms)?));
            },
        };
        Ok(self._norm.ok_or(Error::OptionInvalid)?.1.clone())
    }
}

//##########################################################################################################################

impl Complex {
    /// Returns the complex conjugate. i.e. `re - i im`
    #[inline]
    pub fn conj(&self) -> Complex {
        Complex::new(self._re, -self._im)
    }

    /// Returns `1/self`
    #[inline]
    pub fn inv(&self) -> Result<Complex, Error> {
        let norm_sqr = self.norm_sqr()?;
        let re = self._re / norm_sqr;
        let im = -self._im / norm_sqr;
        Ok(Complex::new(re, im))
    }

    #[inline]
    pub fn round(&self) -> Complex {
        Complex::new(
            self._re.round(),
            self._im.round()
        )
    }

    #[inline]
    pub fn round_dp(&self, dp: u32) -> Complex {
        Complex::new(
            self._re.round_dp(dp),
            self._im.round_dp(dp)
        )
    }
}

/*

impl Complex {
    /// Checks if the given complex number is NaN
    #[inline]
    pub fn is_nan(self) -> bool {
        self._re.is_nan() || self._im.is_nan()
    }

    /// Checks if the given complex number is infinite
    #[inline]
    pub fn is_infinite(self) -> bool {
        !self.is_nan() && (self._re.is_infinite() || self._im.is_infinite())
    }

    /// Checks if the given complex number is finite
    #[inline]
    pub fn is_finite(self) -> bool {
        self._re.is_finite() && self._im.is_finite()
    }

    /// Checks if the given complex number is normal
    #[inline]
   pub fn is_normal(self) -> bool {
        self._re.is_normal() && self._im.is_normal()
    }
}

*/

//##########################################################################################################################

// (a + i b) + (c + i d) == (a + c) + i (b + d)
impl Add<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn add(self, other: Complex) -> Complex {
        Complex::new(self._re + other.re(), self._im + other.im())
    }
}

//##########################################################################################################################

// (a + i b) - (c + i d) == (a - c) + i (b - d)
impl Sub<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn sub(self, other: Complex) -> Complex {
        Complex::new(self._re - other.re(), self._im - other.im())
    }
}

//##########################################################################################################################

// (a + i b) * (c + i d) == (a*c - b*d) + i (a*d + b*c)
impl Mul<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn mul(self, other: Complex) -> Complex {
        let re = self._re * other.re() - self._im * other.im();
        let im = self._re * other.im() + self._im * other.re();
        Complex::new(re, im)
    }
}

//##########################################################################################################################

// (a + i b) / (c + i d) == [(a + i b) * (c - i d)] / (c*c + d*d)
//   == [(a*c + b*d) / (c*c + d*d)] + i [(b*c - a*d) / (c*c + d*d)]
impl Div<Complex> for Complex {
    type Output = Complex;

    #[inline]
    fn div(self, other: Complex) -> Complex {
        let norm_sqr = other.norm_sqr().unwrap();
        let re = self._re * other.re() + self._im * other.im();
        let im = self._im * other.re() - self._re * other.im();
        Complex::new(re / norm_sqr, im / norm_sqr)
    }
}

//##########################################################################################################################

impl Neg for Complex {
    type Output = Complex;

    #[inline]
    fn neg(self) -> Complex {
        Complex::new(-self._re, -self._im)
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
        (self._re == D0) && (self._im == D0)
    }
}

/* string conversions */
impl fmt::Display for Complex {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self._im < Zero::zero() {
            write!(f, "{}-{}i", self._re, D0 - self._im)
        } else {
            write!(f, "{}+{}i", self._re, self._im)
        }
    }
}

//##########################################################################################################################
