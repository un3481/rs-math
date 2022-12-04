
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

//##########################################################################################################################

// Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);

//##########################################################################################################################

/// A multiplex number in iterable form. `z = a * b * c * ...`
#[derive(PartialEq, Clone, Hash, Debug)]
pub struct Multiplex {
    /// Terms of the multiplex number
    list: Vec<Decimal>
}

impl Multiplex {
    /// Create a new Multiplex
    #[inline]
    pub fn new() -> Multiplex {
        Multiplex { list: Vec::new() }
    }
}

//##########################################################################################################################

impl Multiplex {
    /// Returns `1/self`
    #[inline]
    pub fn inv(&mut self) -> &Multiplex {
        let top = self.list.pop();
        self
    }
}

/*

impl Multiplex {
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
impl Add<Multiplex> for Multiplex {
    type Output = Multiplex;

    #[inline]
    fn add(self, other: Multiplex) -> Multiplex {
        Multiplex::new(self.re + other.re, self.im + other.im)
    }
}

// (a + i b) - (c + i d) == (a - c) + i (b - d)
impl Sub<Multiplex> for Multiplex {
    type Output = Multiplex;

    #[inline]
    fn sub(self, other: Multiplex) -> Multiplex {
        Multiplex::new(self.re - other.re, self.im - other.im)
    }
}

// (a + i b) * (c + i d) == (a*c - b*d) + i (a*d + b*c)
impl Mul<Multiplex> for Multiplex {
    type Output = Multiplex;

    #[inline]
    fn mul(self, other: Multiplex) -> Multiplex {
        let re = self.re.clone() * other.re.clone() - self.im.clone() * other.im.clone();
        let im = self.re * other.im + self.im * other.re;
        Multiplex::new(re, im)
    }
}

// (a + i b) / (c + i d) == [(a + i b) * (c - i d)] / (c*c + d*d)
//   == [(a*c + b*d) / (c*c + d*d)] + i [(b*c - a*d) / (c*c + d*d)]
impl Div<Multiplex> for Multiplex {
    type Output = Multiplex;

    #[inline]
    fn div(self, other: Multiplex) -> Multiplex {
        let norm_sqr = other.norm_sqr();
        let re = self.re.clone() * other.re.clone() + self.im.clone() * other.im.clone();
        let im = self.im * other.re - self.re * other.im;
        Multiplex::new(re / norm_sqr.clone(), im / norm_sqr)
    }
}

impl Neg for Multiplex {
    type Output = Multiplex;

    #[inline]
    fn neg(self) -> Multiplex {
        Multiplex::new(-self.re, -self.im)
    }
}

impl<'a> Neg for &'a Multiplex {
    type Output = Multiplex;

    #[inline]
    fn neg(self) -> Multiplex {
        -self.clone()
    }
}

//##########################################################################################################################

/* constants */
impl Zero for Multiplex {
    #[inline]
    fn zero() -> Multiplex {
        Multiplex::new(D0, D0)
    }

    #[inline]
    fn is_zero(&self) -> bool {
        (self.re == D0) && (self.im == D0)
    }
}

/* string conversions */
impl fmt::Display for Multiplex {
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
