
// Imports
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::fmt;

use rust_decimal::prelude::*;

// Modules
use crate::constants::{ PI, PI2, PIDIV2, PI3DIV2 };

use crate::error::Error;
use crate::sqrt::{ sqrt };
use crate::trigonometry::{ cos, sin, atan2 };

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
    _radius: Option<(usize, Decimal)>,
    /// Angle of complex number
    _arg: Option<(usize, Decimal)>
}

//##########################################################################################################################

impl Complex {
    /// Create a new Complex
    #[inline]
    pub const fn new(re: Decimal, im: Decimal) -> Complex {
        Complex {
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
                sqrt(_sqr, terms)?
            }
        )
    }

    /// Get Radius of Complex number.
    #[inline]
    pub fn radius(&mut self, terms: usize) -> Result<Decimal, Error> {
        match &self._radius {
            None => {
                self._radius = Some((terms, self.calc_radius(terms)?));
            },
            Some(v) => if terms > v.0 {
                self._radius = Some((terms, self.calc_radius(terms)?));
            },
        };
        Ok(self._radius.ok_or(Error::OptionInvalid)?.1.clone())
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
                let _radius = self.radius(terms)?;
                let _cos = self._re / _radius;
                let _sin = self._im / _radius;
                atan2(_cos, _sin, terms)?
            }
        )
    }

    /// Get Angle of complex number.
    #[inline]
    pub fn arg(&mut self, terms: usize) -> Result<Decimal, Error> {
        match &self._arg {
            None => {
                self._arg = Some((terms, self.calc_arg(terms)?));
            },
            Some(v) => if terms > v.0 {
                self._arg = Some((terms, self.calc_arg(terms)?));
            },
        };
        Ok(self._arg.ok_or(Error::OptionInvalid)?.1.clone())
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
                _re:     Some((terms, self.re())),
                _im:     Some((terms, self.im()))
            }
        )
    }
}

//##########################################################################################################################

impl Complex {
    /// Multiplies `self` by the scalar `t`.
    #[inline]
    pub fn scale(&self, value: Decimal) -> Complex {
        (*self) * value
    }

    /// Divides `self` by the scalar `t`.
    #[inline]
    pub fn unscale(&self, value: Decimal) -> Complex {
        (*self) / value
    }

    /// Returns the complex conjugate. 
    /// conj(a + bi) = a - bi
    #[inline]
    pub fn conj(&self) -> Complex {
        let re = self._re;
        let im = -self._im;
        Complex::new(re, im)
    }

    /// Returns `1/self`
    #[inline]
    pub fn inv(&self) -> Complex {
        D1 / (*self)
    }

    /// Round complex terms to integer.
    #[inline]
    pub fn round(&self) -> Complex {
        let re = self._re.round();
        let im = self._im.round();
        Complex::new(re, im)
    }

    /// Round complex terms to specified precision.
    #[inline]
    pub fn round_dp(&self, dp: u32) -> Complex {
        let re = self._re.round_dp(dp);
        let im = self._im.round_dp(dp);
        Complex::new(re, im)
    }

    /// Returns the square of the radius (since `T` doesn't necessarily
    /// have a sqrt function), i.e. `re^2 + im^2`.
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
    pub const I: Complex = Complex::new(D0, D1);
    /// A constant representing 0.
    pub const ZERO: Complex = Complex::new(D0, D0);
    /// A constant representing 1.
    pub const ONE: Complex = Complex::new(D1, D0);
    /// A constant representing -1.
    pub const NEGATIVE_ONE: Complex = Complex::new(DN1, D0);
    /// A constant representing 2.
    pub const TWO: Complex = Complex::new(D2, D0);
    /// A constant representing 10.
    pub const TEN: Complex = Complex::new(D10, D0);
    /// A constant representing 100.
    pub const ONE_HUNDRED: Complex = Complex::new(D100, D0);
    /// A constant representing 1000.
    pub const ONE_THOUSAND: Complex = Complex::new(D1000, D0);
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
    fn zero() -> Complex {
        Complex::ZERO
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
        let t_radius = match &self._radius { None => STD_ITER, Some(v) => v.0, };
        let t_arg    = match &self._arg    { None => STD_ITER, Some(v) => v.0, };
        let terms = if t_radius < t_arg {t_radius} else {t_arg};
        other == &self.clone().to_polar(terms).unwrap()
    }
}

//##########################################################################################################################

impl Neg for Complex {
    type Output = Complex;

    #[inline]
    fn neg(self) -> Complex {
        let re = -self._re;
        let im = -self._im;
        Complex::new(re, im)
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
    type Output = Complex;

    #[inline]
    fn add(self, other: Complex) -> Complex {
        let re = self._re + other.re();
        let im = self._im + other.im();
        Complex::new(re, im)
    }
}

impl Add<Decimal> for Complex {
    type Output = Complex;

    #[inline]
    fn add(self, other: Decimal) -> Complex {
        self + Complex::new(other, D0)
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
    type Output = Complex;

    #[inline]
    fn sub(self, other: Complex) -> Complex {
        let re = self._re - other.re();
        let im = self._im - other.im();
        Complex::new(re, im)
    }
}

impl Sub<Decimal> for Complex {
    type Output = Complex;

    #[inline]
    fn sub(self, other: Decimal) -> Complex {
        self - Complex::new(other, D0)
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
    type Output = Complex;

    #[inline]
    fn mul(self, other: Complex) -> Complex {
        let re = (self._re * other.re()) - (self._im * other.im());
        let im = (self._re * other.im()) + (self._im * other.re());
        Complex::new(re, im)
    }
}

impl Mul<Decimal> for Complex {
    type Output = Complex;

    #[inline]
    fn mul(self, other: Decimal) -> Complex {
        self * Complex::new(other, D0)
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
    type Output = Complex;

    #[inline]
    fn div(self, other: Complex) -> Complex {
        let radius_sqr = other.radius_sqr();
        let re = ((self._re * other.re()) + (self._im * other.im())) / radius_sqr;
        let im = ((self._im * other.re()) - (self._re * other.im())) / radius_sqr;
        Complex::new(re, im)
    }
}

impl Div<Decimal> for Complex {
    type Output = Complex;

    #[inline]
    fn div(self, other: Decimal) -> Complex {
        self / Complex::new(other, D0)
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
    _re: Option<(usize, Decimal)>,
    /// Imaginary portion of the complex number
    _im: Option<(usize, Decimal)>,
}

//##########################################################################################################################

impl Polar {
    /// Create a new Complex in Polar form
    #[inline]
    pub const fn new(radius: Decimal, arg: Decimal) -> Polar {
        Polar {
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
}

//##########################################################################################################################

impl Polar {
    /// Calculate Real part of Complex number.
    #[inline]
    fn calc_re(&self, terms: usize) -> Result<Decimal, Error> {
        Ok(
            if (self._arg == PIDIV2) || (self._arg == PI3DIV2) { D0 }
            else {
                cos(self._arg, terms)?
                    .checked_mul(self._radius)
                    .ok_or(Error::MultiplyOverflow)?
            }
        )
    }

    /// Get Real part of Complex number.
    #[inline]
    pub fn re(&mut self, terms: usize) -> Result<Decimal, Error> {
        match &self._re {
            None => {
                self._re = Some((terms, self.calc_re(terms)?));
            },
            Some(v) => if terms > v.0 {
                self._re = Some((terms, self.calc_re(terms)?));
            },
        };
        Ok(self._re.ok_or(Error::OptionInvalid)?.1.clone())
    }
}

//##########################################################################################################################

impl Polar {
    // Calculate Imaginary part of Complex number.
    #[inline]
    fn calc_im(&self, terms: usize) -> Result<Decimal, Error> {
        Ok(
            if (self._arg == D0) || (self._arg == PI) { D0 }
            else {
                sin(self._arg, terms)?;
                    .checked_mul(self._radius)
                    .ok_or(Error::MultiplyOverflow)?
            }
        )
    }

    /// Get Imaginary part of Complex number.
    #[inline]
    pub fn im(&mut self, terms: usize) -> Result<Decimal, Error> {
        match &self._im {
            None => {
                self._im = Some((terms, self.calc_im(terms)?));
            },
            Some(v) => if terms > v.0 {
                self._im = Some((terms, self.calc_im(terms)?));
            },
        };
        Ok(self._im.ok_or(Error::OptionInvalid)?.1.clone())
    }
}

//##########################################################################################################################

impl Polar {
    /// Convert a Polar form number into Cartesian form.
    #[inline]
    pub fn to_cartesian(&mut self, terms: usize) -> Result<Complex, Error> {
        Ok(
            Complex {
                _re:     self.re(terms)?,
                _im:     self.im(terms)?,
                _radius: Some((terms, self.radius())),
                _arg:    Some((terms, self.arg()))
            }
        )
    }
}

//##########################################################################################################################

impl Polar {
    /// Format Polar complex number into standard form
    #[inline]
    pub fn to_std(&self) -> Polar {
        let radius = self._radius.abs();
        let mut arg = self._arg + if self._radius < D0 {PI} else {D0};
        if (arg < -PI) || (PI < arg) {
            arg = arg - ((arg / PI2).floor() * PI2);
        };
             if arg >  PI { arg = arg - PI2; }
        else if arg < -PI { arg = arg + PI2; };
        Polar {
            _radius: radius,
            _arg: arg,
            _re: self._re.clone(),
            _im: self._im.clone()
        }
    }
}

//##########################################################################################################################

impl Polar {
    /// A constant representing the Imaginary unit - sqrt(-1).
    pub const I: Polar = Polar::new(D1, PIDIV2);
    /// A constant representing 0.
    pub const ZERO: Polar = Polar::new(D0, D0);
    /// A constant representing 1.
    pub const ONE: Polar = Polar::new(D1, D0);
    /// A constant representing -1.
    pub const NEGATIVE_ONE: Polar = Polar::new(D1, PI);
    /// A constant representing 2.
    pub const TWO: Polar = Polar::new(D2, D0);
    /// A constant representing 10.
    pub const TEN: Polar = Polar::new(D10, D0);
    /// A constant representing 100.
    pub const ONE_HUNDRED: Polar = Polar::new(D100, D0);
    /// A constant representing 1000.
    pub const ONE_THOUSAND: Polar = Polar::new(D1000, D0);
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

impl Zero for Polar {
    /// Returns Zero
    #[inline]
    fn zero() -> Polar {
        Polar::ZERO
    }

    /// Check if value is Zero
    #[inline]
    fn is_zero(&self) -> bool {
        self._radius == D0
    }
}

//##########################################################################################################################

impl PartialEq for Polar {
    fn eq(&self, other: &Self) -> bool {
        if self._radius == D0 { other.radius() == D0 }
        else {
            ( self._radius == other.radius() ) &&
            ( self._arg  == other.arg()  )
        }
    }
}

impl PartialEq<Decimal> for Polar {
    fn eq(&self, other: &Decimal) -> bool {
        let arg = if other >= &D0 {D0} else {PI};
        (*self) == Polar::new(other.abs(), arg)
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
        let t_re = match &self._re { None => STD_ITER, Some(v) => v.0, };
        let t_im = match &self._im { None => STD_ITER, Some(v) => v.0, };
        let terms = if t_re < t_im {t_re} else {t_im};
        other == &self.clone().to_cartesian(terms).unwrap()
    }
}

//##########################################################################################################################

impl Neg for Polar {
    type Output = Polar;

    #[inline]
    fn neg(self) -> Polar {
        let radius = self._radius;
        let arg = self._arg + PI;
        Polar::new(radius, arg).to_std()
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

/// r1(cos(θ1) + i sin(θ1)) + r2(cos(θ2) + i sin(θ2)) =
///      r1*r2 (cos(θ1 + θ2) + i sin(θ1 + θ2))
impl Add<Polar> for Polar {
    type Output = Polar;

    #[inline]
    fn add(self, other: Polar) -> Polar {
        // Choose Smaller Iteration from self
        let t_re1 = match &self._re { None => STD_ITER, Some(v) => v.0, };
        let t_im1 = match &self._im { None => STD_ITER, Some(v) => v.0, };
        let terms1 = if t_re1 < t_im1 {t_re1} else {t_im1};
        // Choose Smaller Iteration from other
        let t_re2 = match other._re { None => STD_ITER, Some(v) => v.0, };
        let t_im2 = match other._im { None => STD_ITER, Some(v) => v.0, };
        let terms2 = if t_re2 < t_im2 {t_re2} else {t_im2};
        // Choose Smaller Iteration from both
        let terms = if terms1 < terms2 {terms1} else {terms2};
        (
             self.clone().to_cartesian(terms1).unwrap() +
            other.clone().to_cartesian(terms2).unwrap()
        ).to_polar(terms).unwrap()
    }
}

impl Add<Decimal> for Polar {
    type Output = Polar;

    #[inline]
    fn add(self, other: Decimal) -> Polar {
        self + Polar::new(other, D0)
    }
}

impl Add<Polar> for Decimal {
    type Output = Polar;

    #[inline]
    fn add(self, other: Polar) -> Polar {
        Polar::new(self, D0) + other
    }
}

//##########################################################################################################################

// (a + i b) - (c + i d) == (a - c) + i (b - d)
impl Sub<Polar> for Polar {
    type Output = Polar;

    #[inline]
    fn sub(self, other: Polar) -> Polar {
        // Choose Smaller Iteration from self
        let t_re1 = match &self._re { None => STD_ITER, Some(v) => v.0, };
        let t_im1 = match &self._im { None => STD_ITER, Some(v) => v.0, };
        let terms1 = if t_re1 < t_im1 {t_re1} else {t_im1};
        // Choose Smaller Iteration from other
        let t_re2 = match other._re { None => STD_ITER, Some(v) => v.0, };
        let t_im2 = match other._im { None => STD_ITER, Some(v) => v.0, };
        let terms2 = if t_re2 < t_im2 {t_re2} else {t_im2};
        // Choose Smaller Iteration from both
        let terms = if terms1 < terms2 {terms1} else {terms2};
        (
             self.clone().to_cartesian(terms1).unwrap() -
            other.clone().to_cartesian(terms2).unwrap()
        ).to_polar(terms).unwrap()
    }
}

impl Sub<Decimal> for Polar {
    type Output = Polar;

    #[inline]
    fn sub(self, other: Decimal) -> Polar {
        self - Polar::new(other, D0)
    }
}

impl Sub<Polar> for Decimal {
    type Output = Polar;

    #[inline]
    fn sub(self, other: Polar) -> Polar {
        Polar::new(self, D0) - other
    }
}

//##########################################################################################################################

/// r1(cos(θ1) + i sin(θ1)) * r2(cos(θ2) + i sin(θ2)) =
///      r1*r2 (cos(θ1 + θ2) + i sin(θ1 + θ2))
impl Mul<Polar> for Polar {
    type Output = Polar;

    #[inline]
    fn mul(self, other: Polar) -> Polar {
        let radius = self._radius * other.radius();
        let arg  = self._arg  + other.arg();
        Polar::new(radius, arg).to_std()
    }
}

impl Mul<Decimal> for Polar {
    type Output = Polar;

    #[inline]
    fn mul(self, other: Decimal) -> Polar {
        self * Polar::new(other, D0)
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
    type Output = Polar;

    #[inline]
    fn div(self, other: Polar) -> Polar {
        let radius = self._radius / other.radius();
        let arg  = self._arg  - other.arg();
        Polar::new(radius, arg).to_std()
    }
}

impl Div<Decimal> for Polar {
    type Output = Polar;

    #[inline]
    fn div(self, other: Decimal) -> Polar {
        self / Polar::new(other, D0)
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
