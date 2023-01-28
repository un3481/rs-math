
// Imports
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

// Modules
use crate::error::Error;
use crate::basic::{ dec, da_pow };

use crate::multiplex::types::{ Multiplex };
use crate::multiplex::basic::{ ma_pow };

//##########################################################################################################################

pub const PI: Decimal = dec!(3.1415926535897932384626433833);
pub const PI2: Decimal = dec!(6.2831853071795864769252867666);
pub const PIDIV2: Decimal = dec!(1.5707963267948966192313216916);
pub const PI3DIV2: Decimal = dec!(4.7123889803846898576939650749);
pub const PIDIV4: Decimal = dec!(0.7853981633974483096156608458);

//##########################################################################################################################

pub const PIDIV6: Decimal = dec!(0.5235987755982988730771072305);
pub const PIDIV18: Decimal = dec!(0.1745329251994329576923690768);
pub const PIDIV36: Decimal = dec!(0.0872664625997164788461845384);

pub const TAN_PIDIV6: Decimal = dec!(0.5773502691896257645091487805);
pub const TAN_PIDIV18: Decimal = dec!(0.1763269807084649734710903869);
pub const TAN_PIDIV36: Decimal = dec!(0.0874886635259240052220186694);

//##########################################################################################################################

pub const E: Decimal = dec!(2.7182818284590452353602874714); // (e)
pub const D1DIVE: Decimal = dec!(0.3678794411714423215955237702); // (1 / e)
pub const E_SQR: Decimal = dec!(7.3890560989306502272304274606); // (e ^ 2)
pub const E_POW1DIV5: Decimal = dec!(1.2214027581601698339210719946); // (e ^ (1 / 4))

//##########################################################################################################################

pub const LN_UPPER_BD: Decimal = dec!(3.2619381941508542824323449656); // (e * 1.2)
pub const LN_LOWER_BD: Decimal = dec!(2.1746254627672361882882299771); // (e * 0.8)
pub const LN_UPPER_MUL: Decimal = dec!(1.2214027581601698339210719946); // (e ^ 0.2)
pub const LN_LOWER_MUL: Decimal = dec!(0.8187307530779818586699355086); // (e ^ -0.2)
pub const LN_UPPER_VAL: Decimal = dec!(0.2); // ln(e ^ 0.2)
pub const LN_LOWER_VAL: Decimal = dec!(-0.2); // ln(e ^ -0.2)

pub const LN_UPPER_BD_P: Decimal = dec!(2.8541959198819974971283018449); // (e * 1.05)
pub const LN_LOWER_BD_P: Decimal = dec!(2.5823677370360929735922730978); // (e * 0.95)
pub const LN_UPPER_MUL_P: Decimal = dec!(1.0512710963760240396975176363); // (e ^ 0.05)
pub const LN_LOWER_MUL_P: Decimal = dec!(0.9512294245007140090914253198); // (e ^ -0.05)
pub const LN_UPPER_VAL_P: Decimal = dec!(0.05); // ln(e ^ 0.05)
pub const LN_LOWER_VAL_P: Decimal = dec!(-0.05); // ln(e ^ -0.05)

//##########################################################################################################################

pub const SQRT_UPPER_BD: Decimal = dec!(1.21); // (1.1 ^ 2)
pub const SQRT_LOWER_BD: Decimal = dec!(0.81); // (0.9 ^ 2)
pub const SQRT_UPPER_VAL: Decimal = dec!(1.1); // sqrt(1.1 ^ 2)
pub const SQRT_LOWER_VAL: Decimal = dec!(0.9); // sqrt(0.9 ^ 2)

pub const SQRT_UPPER_BD_P: Decimal = dec!(1.050625); // (1.025 ^ 2)
pub const SQRT_LOWER_BD_P: Decimal = dec!(0.950625); // (0.975 ^ 2)
pub const SQRT_UPPER_VAL_P: Decimal = dec!(1.025); // sqrt(1.025 ^ 2)
pub const SQRT_LOWER_VAL_P: Decimal = dec!(0.975); // sqrt(0.975 ^ 2)

//##########################################################################################################################

const D0: Decimal = Decimal::ZERO;
const D1: Decimal = Decimal::ONE;
const D2: Decimal = Decimal::TWO;

const D4: Decimal = dec!(4);
const D5: Decimal = dec!(5);
const D239: Decimal = dec!(239);

//##########################################################################################################################

/// pi_term(x) = sum(n=1; -1^(n + 1) * x^(2n - 1) / (2n - 1))
#[inline]
fn d_pi_term(
    value: Decimal,
    terms: usize
) -> Result<Decimal, Error> {
    let mut acc1: (Decimal, usize) = (D1, 0);
    let mut acc2: (Multiplex, usize) = (Multiplex::new(), 0);
    // Iterate over Series
    (1..=terms).into_iter()
        .map(|n| Ok(
            da_pow(-D1, n + 1, &mut acc1)? * (
                ma_pow(value, (2 * n) - 1, &mut acc2)? / ((D2 * dec(n)) - D1)
            ).squash()?
        ))
        .reduce(|u, d| Ok(
            u?.checked_add(d?).ok_or(Error::AddOverflow)?
        ))
        .unwrap_or(Ok(D0))
}

/// pi = 4 * ((4 * pi_term(1 / 5)) - pi_term(1 / 239))
#[inline]
pub fn d_pi(
    terms: usize
) -> Result<Decimal, Error> {
    let term1 = d_pi_term(D1 / D5, terms)?;
    let term2 = d_pi_term(D1 / D239, terms)?;
    Ok(D4 * ((D4 * term1) - term2))
}

//##########################################################################################################################
