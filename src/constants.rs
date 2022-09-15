
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

use crate::arithmetic::{ dec, pow, fac };

//##########################################################################################################################

pub const STD_ITER: usize = 100;

const D1N: Decimal = dec!(-1);
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D3: Decimal = dec!(3);
const D4: Decimal = dec!(4);
const D5: Decimal = dec!(5);
const D239: Decimal = dec!(239);

//##########################################################################################################################

struct Consts {
    pub E: Decimal,
    pub PI: Decimal,
    pub LN_2: Decimal,
    pub SQRT_3DIV2: Decimal,
    pub PI2: Decimal,
    pub PIDIV2: Decimal,
    pub PIDIV2N: Decimal,
    pub PIN: Decimal
}

pub static consts: Consts = Consts{
    E: D0,
    PI: D0,
    LN_2: D0,
    SQRT_3DIV2: D0,
    PI2: D0,
    PIDIV2: D0,
    PIDIV2N: D0,
    PIN: D0
};

//##########################################################################################################################

pub fn euler(
    terms: usize
) -> Decimal {
    (1..=terms).into_iter()
        .map(|n| D1 / fac(n))
        .reduce(|u, d| u + d)
        .unwrap_or(D0)
}

//##########################################################################################################################

pub fn pi(
    terms: usize
) -> Decimal {
    let term1: Decimal =
        (1..=terms).into_iter()
            .map(|n|
                pow(D1N, n + 1) * (
                    pow(
                        D1 / D5,
                        (2 * n) - 1
                    ) /
                    ((D2 * dec(n)) - D1)
                )
            )
            .reduce(|u, d| u + d)
            .unwrap_or(D0);
    
    let term2: Decimal =
        (1..=terms).into_iter()
            .map(|n|
                pow(D1N, n + 1) * (
                    pow(
                        D1 / D239,
                        (2 * n) - 1
                    ) /
                    ((D2 * dec(n)) - D1)
                )
            )
            .reduce(|u, d| u + d)
            .unwrap_or(D0);
    
    D4 * ((D4 * term1) - term2)
}

//##########################################################################################################################

pub fn ln_2(
    terms: usize
) -> Decimal {
    let E = consts.E;
    D1 + (1..=terms).into_iter()
        .map(|n|
            pow(D1N, n + 1) * (
                pow(D2 - E, n) /
                (pow(E, n) * dec(n))
            )
        )
        .reduce(|u, d| u + d)
        .unwrap_or(D0)
}

//##########################################################################################################################

pub fn sqrt_3div2(
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

//##########################################################################################################################

pub fn init(
    terms: usize
) {
    consts.E = euler(terms);
    consts.PI = pi(terms);
    consts.LN_2 = ln_2(terms);
    consts.SQRT_3DIV2 = sqrt_3div2(terms);
    consts.PI2 = consts.PI * D2;
    consts.PIDIV2 = consts.PI / D2;
    consts.PIDIV2N = consts.PIDIV2 * D1N;
    consts.PIN = consts.PI * D1N;
}

//##########################################################################################################################
