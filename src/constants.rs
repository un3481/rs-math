
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::multiplex::types::{ Multiplex };
use crate::arithmetic::{ dec, a_pow, am_pow };
use crate::error::Error;

//##########################################################################################################################

pub const E: Decimal = dec!(2.7182818284590452353602874714); // (e)
pub const D1DIVE: Decimal = dec!(0.3678794411714423215955237702); // (1 / e)
pub const E_SQR: Decimal = dec!(7.3890560989306502272304274606); // (e ^ 2)

pub const SQRT_UPPER_BD: Decimal = dec!(1.21); // (1.21)
pub const SQRT_LOWER_BD: Decimal = dec!(0.79); // (0.79)
pub const SQRT_EXP_BD: Decimal = dec!(1.21); // (1.21)
pub const SQRT_EXP_VAL: Decimal = dec!(1.1); // sqrt(1.21)

pub const LN_UPPER_BD: Decimal = dec!(3.3201169227365474895307674296); // (e ^ 1.2)
pub const LN_LOWER_BD: Decimal = dec!(2.2255409284924676045795375314); // (e ^ 0.8)
pub const LN_UPPER_EXP_VAL: Decimal = dec!(1.2); // ln(e ^ 1.2)
pub const LN_LOWER_EXP_VAL: Decimal = dec!(0.8); // ln(e ^ 0.8)

pub const PI: Decimal = dec!(3.1415926535897932384626433833);
pub const PI2: Decimal = dec!(6.2831853071795864769252867666);
pub const PIDIV2: Decimal = dec!(1.5707963267948966192313216916);
pub const PI3DIV2: Decimal = dec!(4.7123889803846898576939650749);
pub const PIDIV4: Decimal = dec!(0.7853981633974483096156608458);

pub const PIDIV6: Decimal = dec!(0.5235987755982988730771072305);
pub const PIDIV18: Decimal = dec!(0.1745329251994329576923690768);
pub const PIDIV36: Decimal = dec!(0.0872664625997164788461845384);

pub const TAN_PIDIV6: Decimal = dec!(0.5773502691896257645091487805);
pub const TAN_PIDIV18: Decimal = dec!(0.1763269807084649734710903869);
pub const TAN_PIDIV36: Decimal = dec!(0.0874886635259240052220186694);

//##########################################################################################################################

const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D4: Decimal = dec!(4);
const D5: Decimal = dec!(5);
const D239: Decimal = dec!(239);

//##########################################################################################################################

/// pi_term(x) = sum(n=1; -1^(n + 1) * x^(2n - 1) / (2n - 1))
#[inline]
fn pi_term(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    let mut acc1: (Decimal, usize) = (D1, 0);
    let mut acc2: (Multiplex, usize) = (Multiplex::new(), 0);
    // Iterate over Series
    (1..=terms).into_iter()
        .map(|n| Ok(
            a_pow(-D1, n + 1, &mut acc1)? * (
                am_pow(value, (2 * n) - 1, &mut acc2)? / ((D2 * dec(n)) - D1)
            ).squash()?
        ))
        .reduce(|u, d| Ok(
            u?.checked_add(d?).ok_or(Error::AddOverflow)?
        ))
        .unwrap_or(Ok(D0))
}

/// pi = 4 * ((4 * pi_term(1 / 5)) - pi_term(1 / 239))
#[inline]
pub fn pi(
    terms: usize
) -> Result<Decimal, Error> {
    let term1 = pi_term(D1 / D5, terms)?;
    let term2 = pi_term(D1 / D239, terms)?;
    Ok(D4 * ((D4 * term1) - term2))
}

//##########################################################################################################################
