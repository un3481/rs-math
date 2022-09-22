
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

use crate::arithmetic::{ dec, pow, fac };

//##########################################################################################################################

pub const STD_ITER: usize = 64;

pub const E: Decimal = dec!(2.7182818284590452353602874713);
pub const D1DIVE: Decimal = dec!(0.3678794411714423215955237701);
pub const LN_2: Decimal = dec!(0.6931471805599453094172321214);
pub const SQRT_3DIV2: Decimal = dec!(1.2247448713915890490986420373);

pub const PI: Decimal = dec!(3.1415926535897932384626433832);
pub const PI2: Decimal = dec!(6.2831853071795864769252867665);
pub const PIDIV2: Decimal = dec!(1.5707963267948966192313216916);
pub const PI3DIV2: Decimal = dec!(4.7123889803846898576939650749);
pub const PIDIV4: Decimal = dec!(0.7853981633974483096156608458);

pub const PIDIV6: Decimal = dec!(0.5235987755982988730771072305);
pub const PIDIV18: Decimal = dec!(0.1745329251994329576923690768);
pub const PIDIV36: Decimal = dec!(0.0872664625997164788461845384);

pub const TAN_PIDIV6: Decimal = dec!(0.5773502691896257645091487805);
pub const TAN_PIDIV18: Decimal = dec!(0.1763269807084649734710903868);
pub const TAN_PIDIV36: Decimal = dec!(0.0874886635259240052220186694);

//##########################################################################################################################

const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D3: Decimal = dec!(3);
const D4: Decimal = dec!(4);
const D5: Decimal = dec!(5);
const D239: Decimal = dec!(239);

//##########################################################################################################################

/// e = sum(n=1; 1 / n!)
pub fn euler(
    terms: usize
) -> Decimal {
    (1..=terms).into_par_iter()
        .map(|n| D1 / fac(n))
        .reduce(|| D0, |u, d| u + d)
}

//##########################################################################################################################

/// pi_term(x) = sum(n=1; -1^(n + 1) * x^(2n - 1) / (2n - 1))
fn pi_term(
    value: Decimal,
    terms: usize
) -> Decimal {
    (1..=terms).into_par_iter()
        .map(|n|
            pow(-D1, n + 1) * (
                pow(
                    value,
                    (2 * n) - 1
                ) /
                ((D2 * dec(n)) - D1)
            )
        )
        .reduce(|| D0, |u, d| u + d)
}

/// pi = 4 * ((4 * pi_term(1 / 5)) - pi_term(1 / 239))
pub fn pi(
    terms: usize
) -> Decimal {
    let term1 = pi_term(D1 / D5, terms);
    let term2 = pi_term(D1 / D239, terms);
    D4 * ((D4 * term1) - term2)
}

//##########################################################################################################################

/// ln(2) = 1 + sum(n=1; -1^(n + 1) * ((2 - e)^n / (n * e^n)))
pub fn ln_2(
    terms: usize
) -> Decimal {
    D1 + (1..=terms).into_par_iter()
        .map(|n|
            pow(-D1, n + 1) * (
                pow(D2 - E, n) /
                (pow(E, n) * dec(n))
            )
        )
        .reduce(|| D0, |u, d| u + d)
}

//##########################################################################################################################

/// sqrt(3 / 2) = sum(n=1; ((3 / 2) * (2 * (n - 1))! * (1 - (3 / 2))^(n - 1)) / ((n - 1)! * 2^(n - 1))^2)
pub fn sqrt_3div2(
    terms: usize
) -> Decimal {
    (1..=terms).into_par_iter()
        .map(|n|
            (
                (D3 / D2) *
                fac(2 * (n - 1)) *
                pow(D1 / D2, n - 1)
            ) /
            pow(
                fac(n - 1) *
                pow(D2, n - 1),
                2
            )
        )
        .reduce(|| D0, |u, d| u + d)
}

//##########################################################################################################################
