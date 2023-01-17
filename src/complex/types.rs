
// Imports
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::thread::{ spawn };
use std::fmt;

use rust_decimal::prelude::*;

// Modules
use crate::constants::{ PI, PI2, PIDIV2 };

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

/// A complex number in Polar form. `z = r * exp(i * theta)`
#[derive(Copy, Clone, Hash, Debug)]
pub struct Polar {
    /// Radius of complex number |self|
    _radius: Decimal,
    /// Angle of complex number
    _theta: Decimal,
    /// Cartesian form of complex number
    _cartesian: Option<(usize, Complex)>,
}

//##########################################################################################################################

impl Polar {
    /// Create a new Complex in Polar form
    #[inline]
    pub const fn new(radius: Decimal, theta: Decimal) -> Polar {
        Polar {
            _radius: radius,
            _theta: theta,
            _cartesian: None
        }
    }

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

impl Polar {
    /// Create a new Complex from Polar form.
    #[inline]
    fn new_cartesian(&self, re: Decimal, im: Decimal, terms: usize) -> Complex {
        Complex {
            _re: re,
            _im: im,
            _norm: Some((terms, self._radius.clone())),
            _arg: Some((terms, self._theta.clone()))
        }
    }

    /// Calculate Cartesian form of complex number.
    #[inline]
    fn calc_cartesian(&self, terms: usize) -> Result<Complex, Error> {
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
        // Return Result
        Ok(self.new_cartesian(re, im, terms))
    }

    /// Convert a Polar form number into Cartesian form.
    #[inline]
    pub fn to_cartesian(&mut self, terms: usize) -> Result<Complex, Error> {
        match &self._cartesian {
            None => {
                self._cartesian = Some((terms, self.calc_cartesian(terms)?));
            },
            Some(v) => if terms > v.0 {
                self._cartesian = Some((terms, self.calc_cartesian(terms)?));
            },
        };
        Ok(self._cartesian.ok_or(Error::OptionInvalid)?.1.clone())
    }
}

//##########################################################################################################################

impl Polar {
    /// Format Polar complex number into standard form
    #[inline]
    pub fn to_std(&self) -> Polar {
        let radius = self._radius.abs();
        let mut theta = self._theta + if self._radius < D0 {PI} else {D0};
        if (theta < -PI) || (PI < theta) {
            theta = theta - ((theta / PI2).floor() * PI2);
        };
             if theta >  PI { theta = theta - PI2; }
        else if theta < -PI { theta = theta + PI2; };
        Polar {
            _radius: radius,
            _theta: theta,
            _cartesian: self._cartesian.clone()
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
        write!(f, "r={};arg={}", self._radius, self._theta)
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
            ( self._theta  == other.theta()  )
        }
    }
}

impl PartialEq<Decimal> for Polar {
    fn eq(&self, other: &Decimal) -> bool {
        let theta = if other >= &D0 {D0} else {PI};
        (*self) == Polar::new(other.abs(), theta)
    }
}

impl PartialEq<Polar> for Decimal {
    fn eq(&self, other: &Polar) -> bool {
        let theta = if self >= &D0 {D0} else {PI};
        Polar::new(self.abs(), theta) == (*other)
    }
}

impl PartialEq<Complex> for Polar {
    fn eq(&self, other: &Complex) -> bool {
        let terms = match &self._cartesian { None => STD_ITER, Some(v) => v.0, };
        other == &self.clone().to_cartesian(terms).unwrap()
    }
}

//##########################################################################################################################

impl Neg for Polar {
    type Output = Polar;

    #[inline]
    fn neg(self) -> Polar {
        let radius = self._radius;
        let theta = self._theta + PI;
        Polar::new(radius, theta).to_std()
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
        let c_self  = match &self._cartesian  { None => STD_ITER, Some(v) => v.0, };
        let c_other = match &other._cartesian { None => STD_ITER, Some(v) => v.0, };
        let terms = if c_self < c_other {c_self} else {c_other};
        (
             self.clone().to_cartesian(terms).unwrap() +
            other.clone().to_cartesian(terms).unwrap()
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
        let c_self  = match &self._cartesian  { None => STD_ITER, Some(v) => v.0, };
        let c_other = match &other._cartesian { None => STD_ITER, Some(v) => v.0, };
        let terms = if c_self < c_other {c_self} else {c_other};
        (
             self.clone().to_cartesian(terms).unwrap() -
            other.clone().to_cartesian(terms).unwrap()
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
        let theta  = self._theta  + other.theta();
        Polar::new(radius, theta).to_std()
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
        let theta  = self._theta  - other.theta();
        Polar::new(radius, theta).to_std()
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
    /// Create a new Complex
    #[inline]
    pub const fn new(re: Decimal, im: Decimal) -> Complex {
        Complex {
            _re: re,
            _im: im,
            _norm: None,
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
    /// Create a new Complex in Polar form
    #[inline]
    fn new_polar(&self, radius: Decimal, theta: Decimal, terms: usize) -> Polar {
        Polar {
            _radius: radius,
            _theta: theta,
            _cartesian: Some((terms, self.clone()))
        }.to_std()
    }

    /// Convert to polar form (r, theta)
    #[inline]
    pub fn to_polar(&mut self, terms: usize) -> Result<Polar, Error> {
        // Extract Variables
        let theta  = self.arg(terms)?;
        let radius = self.norm(terms)?;
        // Return Result
        Ok(self.new_polar(radius, theta, terms))
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

    /// Returns the square of the norm (since `T` doesn't necessarily
    /// have a sqrt function), i.e. `re^2 + im^2`.
    #[inline]
    pub fn norm_sqr(&self) -> Decimal {
        (self._re * self._re) +
        (self._im * self._im)
    }
}

//##########################################################################################################################

impl Complex {
    /// Calculate Radius of Complex number.
    #[inline]
    fn calc_norm(&self, terms: usize) -> Result<Decimal, Error> {
        Ok(
                 if self.is_zero() { D0             }
            else if self._im == D0 { self._re.abs() }
            else if self._re == D0 { self._im.abs() }
            else {
                let _sqr = self.norm_sqr();
                sqrt(_sqr, terms)?
            }
        )
    }

    /// Get Radius of Complex number.
    #[inline]
    pub fn norm(&mut self, terms: usize) -> Result<Decimal, Error> {
        match &self._norm {
            None => {
                self._norm = Some((terms, self.calc_norm(terms)?));
            },
            Some(v) => if terms > v.0 {
                self._norm = Some((terms, self.calc_norm(terms)?));
            },
        };
        Ok(self._norm.ok_or(Error::OptionInvalid)?.1.clone())
    }
}

//##########################################################################################################################

impl Complex {
    // Calculate Angle of Complex number.
    #[inline]
    fn calc_arg(&mut self, terms: usize) -> Result<Decimal, Error> {
        Ok(
            if self.is_zero() { self.norm(terms)? }
            else {
                let _norm = self.norm(terms)?;
                let _cos = self._re / _norm;
                let _sin = self._im / _norm;
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
        let t_norm = match &self._norm { None => STD_ITER, Some(v) => v.0, };
        let t_arg  = match &self._arg  { None => STD_ITER, Some(v) => v.0, };
        let terms = if t_norm < t_arg {t_norm} else {t_arg};
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
        let norm_sqr = other.norm_sqr();
        let re = ((self._re * other.re()) + (self._im * other.im())) / norm_sqr;
        let im = ((self._im * other.re()) - (self._re * other.im())) / norm_sqr;
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
