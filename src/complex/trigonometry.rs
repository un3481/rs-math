
// Imports
use rust_decimal::prelude::*;

// Modules
use crate::error::Error;

use crate::complex::types::{ Complex };
use crate::complex::euler::{ c_exp, c_ln };

//##########################################################################################################################

const D0: Decimal = Decimal::ZERO;
const D1: Decimal = Decimal::ONE;
const D2: Decimal = Decimal::TWO;

const CI: Complex = Complex::I;
const CI2: Complex = Complex::new(D0, D2);

//##########################################################################################################################

/// cos(z) = (e^zi + e^-zi) / 2
#[inline]
pub fn c_cos(
    value: Complex,
    terms: usize
) -> Result<Complex, Error> {
    let ei = c_exp(value * CI, terms)?;
    let ein = D1 / ei;
    Ok((ei + ein) / D2)
}

//##########################################################################################################################

/// sin(z) = (e^zi - e^-zi) / 2i
#[inline]
pub fn c_sin(
    value: Complex,
    terms: usize
) -> Result<Complex, Error> {
    let ei = c_exp(value * CI, terms)?;
    let ein = D1 / ei;
    Ok((ei - ein) / CI2)
}

//##########################################################################################################################

/// tan(z) = sin(z) / cos(z) = ((e^zi - e^-zi) / 2i) / ((e^zi + e^-zi) / 2)
#[inline]
pub fn c_tan(
    value: Complex,
    terms: usize
) -> Result<Complex, Error> {
    let ei = c_exp(value * CI, terms)?;
    let ein = D1 / ei;
    Ok(
        ((ei - ein) / CI2) /
        ((ei + ein) / D2)
    )
}

//##########################################################################################################################

/// atan(z) = (ln(1 + zi) - ln(1 - zi)) / 2i = ln((1 + zi) / (1 - zi)) / 2i
#[inline]
pub fn c_atan(
    value: Complex,
    terms: usize
) -> Result<Complex, Error> {
    let term1 = D1 + (CI * value);
    let term2 = D1 - (CI * value);
    let ln_term = c_ln(&mut (term1 / term2), terms)?;
    Ok(ln_term / CI2)
}

//##########################################################################################################################

/*

/// Computes the principal value of the square root of `self`.
///
/// This function has one branch cut:
///
/// * `(-∞, 0)`, continuous from above.
///
/// The branch satisfies `-π/2 ≤ arg(sqrt(z)) ≤ π/2`. 

/// Computes the sine of `self`.
#[inline]
pub fn sin(&self) -> Complex {
    // formula: sin(a + bi) = sin(a)cosh(b) + i*cos(a)sinh(b)
    Complex::new(self.re.sin() * self.im.cosh(), self.re.cos() * self.im.sinh())
}

/// Computes the cosine of `self`.
#[inline]
pub fn cos(&self) -> Complex {
    // formula: cos(a + bi) = cos(a)cosh(b) - i*sin(a)sinh(b)
    Complex::new(self.re.cos() * self.im.cosh(), -self.re.sin() * self.im.sinh())
}

/// Computes the tangent of `self`.
#[inline]
pub fn tan(&self) -> Complex {
    // formula: tan(a + bi) = (sin(2a) + i*sinh(2b))/(cos(2a) + cosh(2b))
    let (two_re, two_im) = (self.re + self.re, self.im + self.im);
    Complex::new(two_re.sin(), two_im.sinh()).unscale(two_re.cos() + two_im.cosh())
}

/// Computes the principal value of the inverse tangent of `self`.
///
/// This function has two branch cuts:
///
/// * `(-∞i, -i]`, continuous from the left.
/// * `[i, ∞i)`, continuous from the right.
///
/// The branch satisfies `-π/2 ≤ Re(atan(z)) ≤ π/2`.
#[inline]
pub fn atan(&self) -> Complex {
    // formula: arctan(z) = (ln(1+iz) - ln(1-iz))/(2i)
    let i = Complex::i();
    let one = Complex::one();
    let two = one + one;
    if *self == i {
        return Complex::new(T::zero(), T::infinity());
    }
    else if *self == -i {
        return Complex::new(T::zero(), -T::infinity());
    }
    ((one + i * self).ln() - (one - i * self).ln()) / (two * i)
}

/// Computes the principal value of the inverse sine of `self`.
///
/// This function has two branch cuts:
///
/// * `(-∞, -1)`, continuous from above.
/// * `(1, ∞)`, continuous from below.
///
/// The branch satisfies `-π/2 ≤ Re(asin(z)) ≤ π/2`.
#[inline]
pub fn asin(&self) -> Complex {
    // formula: arcsin(z) = -i ln(sqrt(1-z^2) + iz)
    let i = Complex::i();
    -i*((Complex::one() - self*self).sqrt() + i*self).ln()
}

/// Computes the principal value of the inverse cosine of `self`.
///
/// This function has two branch cuts:
///
/// * `(-∞, -1)`, continuous from above.
/// * `(1, ∞)`, continuous from below.
///
/// The branch satisfies `0 ≤ Re(acos(z)) ≤ π`.
#[inline]
pub fn acos(&self) -> Complex {
    // formula: arccos(z) = -i ln(i sqrt(1-z^2) + z)
    let i = Complex::i();
    -i*(i*(Complex::one() - self*self).sqrt() + self).ln()
}

/// Computes the hyperbolic sine of `self`.
#[inline]
pub fn sinh(&self) -> Complex {
    // formula: sinh(a + bi) = sinh(a)cos(b) + i*cosh(a)sin(b)
    Complex::new(self.re.sinh() * self.im.cos(), self.re.cosh() * self.im.sin())
}

/// Computes the hyperbolic cosine of `self`.
#[inline]
pub fn cosh(&self) -> Complex {
    // formula: cosh(a + bi) = cosh(a)cos(b) + i*sinh(a)sin(b)
    Complex::new(self.re.cosh() * self.im.cos(), self.re.sinh() * self.im.sin())
}

/// Computes the hyperbolic tangent of `self`.
#[inline]
pub fn tanh(&self) -> Complex {
    // formula: tanh(a + bi) = (sinh(2a) + i*sin(2b))/(cosh(2a) + cos(2b))
    let (two_re, two_im) = (self.re + self.re, self.im + self.im);
    Complex::new(two_re.sinh(), two_im.sin()).unscale(two_re.cosh() + two_im.cos())
}

/// Computes the principal value of inverse hyperbolic sine of `self`.
///
/// This function has two branch cuts:
///
/// * `(-∞i, -i)`, continuous from the left.
/// * `(i, ∞i)`, continuous from the right.
///
/// The branch satisfies `-π/2 ≤ Im(asinh(z)) ≤ π/2`.
#[inline]
pub fn asinh(&self) -> Complex {
    // formula: arcsinh(z) = ln(z + sqrt(1+z^2))
    let one = Complex::one();
    (self + (one + self * self).sqrt()).ln()
}

/// Computes the principal value of inverse hyperbolic cosine of `self`.
///
/// This function has one branch cut:
///
/// * `(-∞, 1)`, continuous from above.
///
/// The branch satisfies `-π ≤ Im(acosh(z)) ≤ π` and `0 ≤ Re(acosh(z)) < ∞`.
#[inline]
pub fn acosh(&self) -> Complex {
    // formula: arccosh(z) = 2 ln(sqrt((z+1)/2) + sqrt((z-1)/2))
    let one = Complex::one();
    let two = one + one;
    two * (((self + one)/two).sqrt() + ((self - one)/two).sqrt()).ln()
}

/// Computes the principal value of inverse hyperbolic tangent of `self`.
///
/// This function has two branch cuts:
///
/// * `(-∞, -1]`, continuous from above.
/// * `[1, ∞)`, continuous from below.
///
/// The branch satisfies `-π/2 ≤ Im(atanh(z)) ≤ π/2`.
#[inline]
pub fn atanh(&self) -> Complex {
    // formula: arctanh(z) = (ln(1+z) - ln(1-z))/2
    let one = Complex::one();
    let two = one + one;
    if *self == one {
        return Complex::new(T::infinity(), T::zero());
    }
    else if *self == -one {
        return Complex::new(-T::infinity(), T::zero());
    }
    ((one + self).ln() - (one - self).ln()) / two
}

*/
