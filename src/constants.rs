
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use lazy_static::lazy_static;

use crate::arithmetic::{ dec, pow, fac };

//##########################################################################################################################

pub const STD_ITER: usize = 64;

const D1N: Decimal = dec!(-1);
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D3: Decimal = dec!(3);
const D4: Decimal = dec!(4);
const D5: Decimal = dec!(5);
const D239: Decimal = dec!(239);

//##########################################################################################################################

/// e = sum(n=1; 1 / n!)
fn euler(
    terms: usize
) -> Decimal {
    (1..=terms).into_iter()
        .map(|n| D1 / fac(n))
        .reduce(|u, d| u + d)
        .unwrap_or(D0)
}

lazy_static! {
    pub static ref E: Decimal = euler(STD_ITER);
}

//##########################################################################################################################

/// pi_term(x) = sum(n=1; -1^(n + 1) * x^(2n - 1) / (2n - 1))
fn pi_term(
    value: Decimal,
    terms: usize
) -> Decimal {
    (1..=terms).into_iter()
        .map(|n|
            pow(D1N, n + 1) * (
                pow(
                    value,
                    (2 * n) - 1
                ) /
                ((D2 * dec(n)) - D1)
            )
        )
        .reduce(|u, d| u + d)
        .unwrap_or(D0)
}

/// pi = 4 * ((4 * pi_term(1 / 5)) - pi_term(1 / 239))
fn pi(
    terms: usize
) -> Decimal {
    let term1 = pi_term(D1 / D5, terms);
    let term2 = pi_term(D1 / D239, terms);
    D4 * ((D4 * term1) - term2)
}

lazy_static! {
    pub static ref PI: Decimal = pi(STD_ITER);
}

//##########################################################################################################################

/// ln(2) = 1 + sum(n=1; -1^(n + 1) * ((2 - e)^n / (n * e^n)))
fn ln_2(
    terms: usize
) -> Decimal {
    let _e = *E;
    D1 + (1..=terms).into_iter()
        .map(|n|
            pow(D1N, n + 1) * (
                pow(D2 - _e, n) /
                (pow(_e, n) * dec(n))
            )
        )
        .reduce(|u, d| u + d)
        .unwrap_or(D0)
}

lazy_static! {
    pub static ref LN_2: Decimal = ln_2(STD_ITER);
}

//##########################################################################################################################

/// sqrt(3 / 2) = sum(n=1; ((3 / 2) * (2 * (n - 1))! * (1 - (3 / 2))^(n - 1)) / ((n - 1)! * 2^(n - 1))^2)
fn sqrt_3div2(
    terms: usize
) -> Decimal {
    (1..=terms).into_iter()
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
        .reduce(|u, d| u + d)
        .unwrap_or(D0)
}

lazy_static! {
    pub static ref SQRT_3DIV2: Decimal = sqrt_3div2(STD_ITER);
}

//##########################################################################################################################

lazy_static! {
    pub static ref PI2: Decimal = (*PI) * D2;
    pub static ref PIDIV2: Decimal = (*PI) / D2;
    pub static ref PIDIV2N: Decimal = (*PIDIV2) * D1N;
    pub static ref PI3DIV2: Decimal = (D3 * (*PI)) / D2;
    pub static ref PIN: Decimal = (*PI) * D1N;
}

//##########################################################################################################################
