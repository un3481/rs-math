
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
    pub E: Box<Decimal>,
    pub PI: Box<Decimal>,
    pub LN_2: Box<Decimal>,
    pub SQRT_3DIV2: Box<Decimal>,
    pub PI2: Box<Decimal>,
    pub PIDIV2: Box<Decimal>,
    pub PIDIV2N: Box<Decimal>,
    pub PIN: Box<Decimal>
}

pub static consts: Consts = Consts{
    E: Box::new(D0),
    PI: Box::new(D0),
    LN_2: Box::new(D0),
    SQRT_3DIV2: Box::new(D0),
    PI2: Box::new(D0),
    PIDIV2: Box::new(D0),
    PIDIV2N: Box::new(D0),
    PIN: Box::new(D0)
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
    let E = *consts.E;
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

pub unsafe fn init(
    terms: usize
) {
    consts.E.as_mut_ptr().write(euler(terms));
    consts.PI.as_mut_ptr().write(pi(terms));
    consts.LN_2.as_mut_ptr().write(ln_2(terms));
    consts.SQRT_3DIV2.as_mut_ptr().write(sqrt_3div2(terms));

    consts.PI2.as_mut_ptr().write(consts.PI * D2);
    consts.PIDIV2.as_mut_ptr().write(consts.PI / D2);
    consts.PIDIV2N.as_mut_ptr().write(consts.PIDIV2 * D1N);
    consts.PIN.as_mut_ptr().write(consts.PI * D1N);
}

//##########################################################################################################################
