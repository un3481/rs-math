
// Imports
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::fmt;

use rust_decimal::prelude::*;

// Modules
use crate::constants::{ PI, PI2, PIDIV2 };

use crate::error::Error;
use crate::sqrt::{ d_sqrt };
use crate::trigonometry::{ d_cos, d_sin, d_atan2 };

//##########################################################################################################################

// Constants
const D0: Decimal = Decimal::ZERO;
const D1: Decimal = Decimal::ONE;
const D2: Decimal = Decimal::TWO;
const D10: Decimal = Decimal::TEN;
const D100: Decimal = Decimal::ONE_HUNDRED;
const D1000: Decimal = Decimal::ONE_THOUSAND;
const DN1: Decimal = Decimal::NEGATIVE_ONE;

// Standard Iterations
const STD_ITER: usize = 32;

//##########################################################################################################################

/// A complex number in Cartesian form. `z = a + i * b`
#[derive(Copy, Clone, Hash, Debug)]
pub struct Complex {
    /// Real portion of the complex number
    _re: Decimal,
    /// Imaginary portion of the complex number
    _im: Decimal,
    // Radius of complex number |self|
    _radius: Option<Decimal>,
    /// Angle of complex number
    _arg: Option<Decimal>
}

//##########################################################################################################################

impl Complex {
    /// Create a new Complex
    #[inline]
    pub const fn new(re: Decimal, im: Decimal) -> Self {
        Self {
            _re: re,
            _im: im,
            _radius: None,
            _arg: None
        }
    }

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

    /// Reset hidden properties
    #[inline]
    pub const fn reset(&self) -> Self {
        let re = self._re;
        let im = self._im;
        Self::new(re, im)
    }
}

//##########################################################################################################################

impl Complex {
    /// Calculate Radius of Complex number.
    #[inline]
    fn calc_radius(&self, terms: usize) -> Result<Decimal, Error> {
        Ok(
                 if self.is_zero() { D0             }
            else if self._im == D0 { self._re.abs() }
            else if self._re == D0 { self._im.abs() }
            else {
                let _sqr = self.radius_sqr();
                d_sqrt(_sqr, terms)?
            }
        )
    }

    /// Get Radius of Complex number.
    #[inline]
    pub fn radius(&mut self, terms: usize) -> Result<Decimal, Error> {
        if self._radius == None { self._radius = Some(self.calc_radius(terms)?) };
        Ok(self._radius.ok_or(Error::OptionInvalid)?.clone())
    }
}

//##########################################################################################################################

impl Complex {
    // Calculate Angle of Complex number.
    #[inline]
    fn calc_arg(&mut self, terms: usize) -> Result<Decimal, Error> {
        Ok(
            if self.is_zero() { self.radius(terms)? }
            else {
                let radius  = self.radius(terms)?;
                let cos_arg = self._re / radius;
                let sin_arg = self._im / radius;
                d_atan2(cos_arg, sin_arg, terms)?
            }
        )
    }

    /// Get Angle of complex number.
    #[inline]
    pub fn arg(&mut self, terms: usize) -> Result<Decimal, Error> {
        if self._arg == None { self._arg = Some(self.calc_arg(terms)?) };
        Ok(self._arg.ok_or(Error::OptionInvalid)?.clone())
    }
}

//##########################################################################################################################

impl Complex {
    /// Convert to polar form radius (cos(arg) + i sin(arg))
    #[inline]
    pub fn to_polar(&mut self, terms: usize) -> Result<Polar, Error> {
        Ok(
            Polar {
                _radius: self.radius(terms)?,
                _arg:    self.arg(terms)?,
                _re:     Some(self.re()),
                _im:     Some(self.im())
            }
        )
    }
}

//##########################################################################################################################

impl Complex {
    /// Multiplies `self` by the scalar `t`.
    #[inline]
    pub fn scale(&self, value: Decimal) -> Self {
        (*self) * value
    }

    /// Divides `self` by the scalar `t`.
    #[inline]
    pub fn unscale(&self, value: Decimal) -> Self {
        (*self) / value
    }

    /// Returns the complex conjugate. 
    /// conj(a + bi) = a - bi
    #[inline]
    pub fn conj(&self) -> Self {
        let re = self._re;
        let im = -self._im;
        Self::new(re, im)
    }

    /// Returns `1/self`
    #[inline]
    pub fn inv(&self) -> Self {
        D1 / (*self)
    }

    /// Round complex terms to integer.
    #[inline]
    pub fn round(&self) -> Self {
        let re = self._re.round();
        let im = self._im.round();
        Self::new(re, im)
    }

    /// Round complex terms to specified precision.
    #[inline]
    pub fn round_dp(&self, dp: u32) -> Self {
        let re = self._re.round_dp(dp);
        let im = self._im.round_dp(dp);
        Self::new(re, im)
    }

    /// Returns the square of the radius (since `T` doesn't necessarily
    /// have a d_sqrt function), i.e. `re^2 + im^2`.
    #[inline]
    pub fn radius_sqr(&self) -> Decimal {
        (self._re * self._re) +
        (self._im * self._im)
    }

    /*
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
    */
}

//##########################################################################################################################

impl Complex {
    /// A constant representing the Imaginary unit - sqrt(-1).
    pub const I: Self = Complex::new(D0, D1);
    /// A constant representing 0.
    pub const ZERO: Self = Complex::new(D0, D0);
    /// A constant representing 1.
    pub const ONE: Self = Complex::new(D1, D0);
    /// A constant representing -1.
    pub const NEGATIVE_ONE: Self = Complex::new(DN1, D0);
    /// A constant representing 2.
    pub const TWO: Self = Complex::new(D2, D0);
    /// A constant representing 10.
    pub const TEN: Self = Complex::new(D10, D0);
    /// A constant representing 100.
    pub const ONE_HUNDRED: Self = Complex::new(D100, D0);
    /// A constant representing 1000.
    pub const ONE_THOUSAND: Self = Complex::new(D1000, D0);
}

//##########################################################################################################################

impl fmt::Display for Complex {
    /// Format string
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
             if self._im > D0 { write!(f, "{}+{}i", self._re, self._im)  }
        else if self._im < D0 { write!(f, "{}-{}i", self._re, -self._im) }
        else                  { write!(f, "{}", self._re)                }
    }
}

//##########################################################################################################################

impl Zero for Complex {
    /// Returns Zero
    #[inline]
    fn zero() -> Self {
        Self::ZERO
    }

    /// Check if value is Zero
    #[inline]
    fn is_zero(&self) -> bool {
        (self._re == D0) && (self._im == D0)
    }
}

//##########################################################################################################################

impl PartialEq for Complex {
    fn eq(&self, other: &Self) -> bool {
        ( self._re == other.re() ) &&
        ( self._im == other.im() )
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

impl PartialEq<Polar> for Complex {
    fn eq(&self, other: &Polar) -> bool {
        other == &self.clone().to_polar(STD_ITER).unwrap()
    }
}

//##########################################################################################################################

impl Neg for Complex {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        let re = -self._re;
        let im = -self._im;
        Self::new(re, im)
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

// (a + i b) + (c + i d) == (a + c) + i (b + d)
impl Add<Complex> for Complex {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        let re = self._re + other.re();
        let im = self._im + other.im();
        Self::new(re, im)
    }
}

impl Add<Decimal> for Complex {
    type Output = Self;

    #[inline]
    fn add(self, other: Decimal) -> Self {
        self + Self::new(other, D0)
    }
}

impl Add<Complex> for Decimal {
    type Output = Complex;

    #[inline]
    fn add(self, other: Complex) -> Complex {
        Complex::new(self, D0) + other
    }
}

//##########################################################################################################################

// (a + i b) - (c + i d) == (a - c) + i (b - d)
impl Sub<Complex> for Complex {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        let re = self._re - other.re();
        let im = self._im - other.im();
        Self::new(re, im)
    }
}

impl Sub<Decimal> for Complex {
    type Output = Self;

    #[inline]
    fn sub(self, other: Decimal) -> Self {
        self - Self::new(other, D0)
    }
}

impl Sub<Complex> for Decimal {
    type Output = Complex;

    #[inline]
    fn sub(self, other: Complex) -> Complex {
        Complex::new(self, D0) - other
    }
}

//##########################################################################################################################

// (a + i b) * (c + i d) == (a*c - b*d) + i (a*d + b*c)
impl Mul<Complex> for Complex {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        let re = (self._re * other.re()) - (self._im * other.im());
        let im = (self._re * other.im()) + (self._im * other.re());
        Self::new(re, im)
    }
}

impl Mul<Decimal> for Complex {
    type Output = Self;

    #[inline]
    fn mul(self, other: Decimal) -> Self {
        self * Self::new(other, D0)
    }
}

impl Mul<Complex> for Decimal {
    type Output = Complex;

    #[inline]
    fn mul(self, other: Complex) -> Complex {
        Complex::new(self, D0) * other
    }
}

//##########################################################################################################################

// (a + i b) / (c + i d) == [(a + i b) * (c - i d)] / (c*c + d*d)
//   == [(a*c + b*d) / (c*c + d*d)] + i [(b*c - a*d) / (c*c + d*d)]
impl Div<Complex> for Complex {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self {
        let radius_sqr = other.radius_sqr();
        let re = ((self._re * other.re()) + (self._im * other.im())) / radius_sqr;
        let im = ((self._im * other.re()) - (self._re * other.im())) / radius_sqr;
        Self::new(re, im)
    }
}

impl Div<Decimal> for Complex {
    type Output = Self;

    #[inline]
    fn div(self, other: Decimal) -> Self {
        self / Self::new(other, D0)
    }
}

impl Div<Complex> for Decimal {
    type Output = Complex;

    #[inline]
    fn div(self, other: Complex) -> Complex {
        Complex::new(self, D0) / other
    }
}

//##########################################################################################################################

/// A complex number in Polar form. `z = |z| * (cos(arg) + i sin(arg))`
#[derive(Copy, Clone, Hash, Debug)]
pub struct Polar {
    /// Radius of complex number |self|
    _radius: Decimal,
    /// Angle of complex number
    _arg: Decimal,
    /// Real portion of the complex number
    _re: Option<Decimal>,
    /// Imaginary portion of the complex number
    _im: Option<Decimal>,
}

//##########################################################################################################################

impl Polar {
    /// Create a new Complex in Polar form
    #[inline]
    pub const fn new(radius: Decimal, arg: Decimal) -> Self {
        Self {
            _radius: radius,
            _arg: arg,
            _re: None,
            _im: None,
        }
    }

    /// Get radius property
    #[inline]
    pub const fn radius(&self) -> Decimal {
        self._radius
    }

    /// Get arg property
    #[inline]
    pub const fn arg(&self) -> Decimal {
        self._arg
    }

    /// Reset hidden properties
    #[inline]
    pub const fn reset(&self) -> Self {
        let radius = self._radius;
        let arg    = self._arg;
        Self::new(radius, arg)
    }
}

//##########################################################################################################################

impl Polar {
    /// Calculate Real part of Complex number.
    #[inline]
    fn calc_re(&self, terms: usize) -> Result<Decimal, Error> {
        Ok(
            d_cos(self._arg, terms)?
                .checked_mul(self._radius)
                .ok_or(Error::MultiplyOverflow)?
        )
    }

    /// Get Real part of Complex number.
    #[inline]
    pub fn re(&mut self, terms: usize) -> Result<Decimal, Error> {
        if self._re == None { self._re = Some(self.calc_re(terms)?) };
        Ok(self._re.ok_or(Error::OptionInvalid)?.clone())
    }
}

//##########################################################################################################################

impl Polar {
    // Calculate Imaginary part of Complex number.
    #[inline]
    fn calc_im(&self, terms: usize) -> Result<Decimal, Error> {
        Ok(
            d_sin(self._arg, terms)?
                .checked_mul(self._radius)
                .ok_or(Error::MultiplyOverflow)?
        )
    }

    /// Get Imaginary part of Complex number.
    #[inline]
    pub fn im(&mut self, terms: usize) -> Result<Decimal, Error> {
        if self._im == None { self._im = Some(self.calc_im(terms)?) };
        Ok(self._im.ok_or(Error::OptionInvalid)?.clone())
    }
}

//##########################################################################################################################

impl Polar {
    /// Convert a Polar form number into Cartesian form.
    #[inline]
    pub fn to_cartesian(&mut self, terms: usize) -> Result<Complex, Error> {
        self.to_std();
        Ok(
            Complex {
                _re:     self.re(terms)?,
                _im:     self.im(terms)?,
                _radius: Some(self.radius()),
                _arg:    Some(self.arg())
            }
        )
    }
}

//##########################################################################################################################

impl Polar {
    /// Format Polar complex number into standard form
    #[inline]
    pub fn to_std(&mut self) -> Self {
        // Fix Angle
        let mut arg = self._arg + if self._radius < D0 {PI} else {D0};
        if (arg < -PI) || (PI < arg) {
            arg = arg - ((arg / PI2).floor() * PI2);
        };
             if arg >  PI { arg = arg - PI2; }
        else if arg < -PI { arg = arg + PI2; };
        // Assign new values
        self._radius = self._radius.abs();
        self._arg    = arg;
        // Return cloned self
        self.clone()
    }
}

//##########################################################################################################################

impl Polar {
    /// A constant representing the Imaginary unit - sqrt(-1).
    pub const I: Self = Polar::new(D1, PIDIV2);
    /// A constant representing 0.
    pub const ZERO: Self = Polar::new(D0, D0);
    /// A constant representing 1.
    pub const ONE: Self = Polar::new(D1, D0);
    /// A constant representing -1.
    pub const NEGATIVE_ONE: Self = Polar::new(D1, PI);
    /// A constant representing 2.
    pub const TWO: Self = Polar::new(D2, D0);
    /// A constant representing 10.
    pub const TEN: Self = Polar::new(D10, D0);
    /// A constant representing 100.
    pub const ONE_HUNDRED: Self = Polar::new(D100, D0);
    /// A constant representing 1000.
    pub const ONE_THOUSAND: Self = Polar::new(D1000, D0);
}

//##########################################################################################################################

impl fmt::Display for Polar {
    /// Format string
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r={};arg={}", self._radius, self._arg)
    }
}

//##########################################################################################################################

impl PartialEq for Polar {
    fn eq(&self, other: &Self) -> bool {
        if self._radius == D0 { other.radius() == D0 }
        else {
            let _self  =  self.clone().to_std();
            let _other = other.clone().to_std();
            ( _self.radius() == _other.radius() ) &&
            ( _self.arg()    == _other.arg()    )
        }
    }
}

impl PartialEq<Decimal> for Polar {
    fn eq(&self, other: &Decimal) -> bool {
        let arg = if other >= &D0 {D0} else {PI};
        (*self) == Self::new(other.abs(), arg)
    }
}

impl PartialEq<Polar> for Decimal {
    fn eq(&self, other: &Polar) -> bool {
        let arg = if self >= &D0 {D0} else {PI};
        Polar::new(self.abs(), arg) == (*other)
    }
}

impl PartialEq<Complex> for Polar {
    fn eq(&self, other: &Complex) -> bool {
        other == &self.clone().to_cartesian(STD_ITER).unwrap()
    }
}

//##########################################################################################################################

impl Neg for Polar {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        let radius = self._radius;
        let arg    = self._arg + if self._arg <= D0 {PI} else {-PI};
        Self::new(radius, arg)
    }
}

impl<'a> Neg for &'a Polar {
    type Output = Polar;

    #[inline]
    fn neg(self) -> Polar {
        -self.clone()
    }
}

//##########################################################################################################################

/// r1(cos(θ1) + i sin(θ1)) * r2(cos(θ2) + i sin(θ2)) =
///      r1*r2 (cos(θ1 + θ2) + i sin(θ1 + θ2))
impl Mul<Polar> for Polar {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self {
        let radius = self._radius * other.radius();
        let arg    = self._arg    + other.arg();
        Self::new(radius, arg).to_std()
    }
}

impl Mul<Decimal> for Polar {
    type Output = Self;

    #[inline]
    fn mul(self, other: Decimal) -> Self {
        self * Self::new(other, D0)
    }
}

impl Mul<Polar> for Decimal {
    type Output = Polar;

    #[inline]
    fn mul(self, other: Polar) -> Polar {
        Polar::new(self, D0) * other
    }
}

//##########################################################################################################################

/// r1(cos(θ1) + i sin(θ1)) / r2(cos(θ2) + i sin(θ2)) =
///      r1/r2 (cos(θ1 - θ2) + i sin(θ1 - θ2))
impl Div<Polar> for Polar {
    type Output = Self;

    #[inline]
    fn div(self, other: Self) -> Self {
        let radius = self._radius / other.radius();
        let arg    = self._arg    - other.arg();
        Self::new(radius, arg).to_std()
    }
}

impl Div<Decimal> for Polar {
    type Output = Self;

    #[inline]
    fn div(self, other: Decimal) -> Self {
        self / Self::new(other, D0)
    }
}

impl Div<Polar> for Decimal {
    type Output = Polar;

    #[inline]
    fn div(self, other: Polar) -> Polar {
        Polar::new(self, D0) / other
    }
}

//##########################################################################################################################
