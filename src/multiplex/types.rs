
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

use std::fmt;
use std::ops::{ Mul, Div, Neg };

// Modules
use crate::error::Error;

//##########################################################################################################################

// Constants
const D1: Decimal = dec!(1);

//##########################################################################################################################

/// A multiplex number in iterable form. `z = a * b * c * ...`
#[derive(PartialEq, Clone, Hash, Debug)]
pub struct Multiplex {
    /// Terms of the multiplex number
    pub mul: Vec<Decimal>,
    pub div: Vec<Decimal>
}

impl Multiplex {
    /// Create a new Multiplex
    #[inline]
    pub fn new() -> Multiplex {
        Multiplex { mul: Vec::new(), div: Vec::new() }
    }
}

//##########################################################################################################################

impl Multiplex {
    /// Returns `1/self`
    #[inline]
    pub fn inv(&mut self) -> Multiplex {
        let mut target = Multiplex::new();
        let mul = self.mul.clone();
        let div = self.div.clone();
        target.mul = div;
        target.div = mul;
        target
    }
}

impl Multiplex {
    #[inline]
    fn squash_helper(target: &mut Vec<Decimal>) -> bool {
        let mut squashed: bool = false;
        loop {
            if target.len() <= 1 {break};
            let v1 = target.pop().unwrap_or(D1);
            let v2 = target.pop().unwrap_or(D1);
            match v1.checked_mul(v2) {
                None => {
                    target.push(v2);
                    target.push(v1);
                    break;
                },
                Some(v) => {
                    target.push(v);
                    squashed = true;
                },
            };
        };
        squashed
    }

    /// Returns Decimal value
    #[inline]
    pub fn squash(&mut self) -> Result<Decimal, Error> {
        let mut mul = self.mul.clone();
        let mut div = self.div.clone();
        loop {
            // Squash until overflow
            let s_mul = Self::squash_helper(&mut mul);
            let s_div = Self::squash_helper(&mut div);
            // Apply division
            let _mul = mul.pop().unwrap_or(D1);
            let _div = div.pop().unwrap_or(D1);
            let value = _mul.checked_div(_div).ok_or(Error::MultiplyOverflow)?;
            // Check if Multiplex is empty
            if div.is_empty() && mul.is_empty() {break Ok(value)};
            if div.is_empty() && !(s_mul || s_div) {break Err(Error::MultiplyOverflow)};
            // Setup next Iteration
            mul.push(value);
        }
    }
}

//##########################################################################################################################

impl Neg for Multiplex {
    type Output = Multiplex;

    #[inline]
    fn neg(self) -> Multiplex {
        let copy = self.clone();
        copy * -D1
    }
}

//##########################################################################################################################

impl Mul<Multiplex> for Decimal {
    type Output = Multiplex;

    #[inline]
    fn mul(self, other: Multiplex) -> Multiplex {
        let mut target = other.clone();
        target.mul.push(self.clone());
        target
    }
}

impl Mul<Decimal> for Multiplex {
    type Output = Multiplex;

    #[inline]
    fn mul(self, other: Decimal) -> Multiplex {
        let mut target = self.clone();
        target.mul.push(other.clone());
        target
    }
}

impl Mul<Multiplex> for Multiplex {
    type Output = Multiplex;

    #[inline]
    fn mul(self, other: Multiplex) -> Multiplex {
        let mut mul = other.mul.clone();
        let mut div = other.div.clone();
        let mut target = self.clone();
        target.mul.append(&mut mul);
        target.div.append(&mut div);
        target
    }
}

impl Div<Multiplex> for Decimal {
    type Output = Multiplex;

    #[inline]
    fn div(self, other: Multiplex) -> Multiplex {
        let mut target = Multiplex::new();
        target.mul.push(self.clone());
        target = target / other;
        target
    }
}

impl Div<Decimal> for Multiplex {
    type Output = Multiplex;

    #[inline]
    fn div(self, other: Decimal) -> Multiplex {
        let mut target = self.clone();
        target.div.push(other.clone());
        target
    }
}

impl Div<Multiplex> for Multiplex {
    type Output = Multiplex;

    #[inline]
    fn div(self, other: Multiplex) -> Multiplex {
        let mut mul = other.mul.clone();
        let mut div = other.div.clone();
        let mut target = self.clone();
        target.mul.append(&mut div);
        target.div.append(&mut mul);
        target
    }
}

//##########################################################################################################################

/* string conversions */
impl fmt::Display for Multiplex {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut mul = self.mul.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" * ");
        let mut div = self.div.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" * ");
        if mul == "" { mul = "1".to_string() };
        if div == "" { div = "1".to_string() };
        write!(f, "({}) / ({})", mul, div)
    }
}

//##########################################################################################################################
