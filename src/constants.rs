
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

//##########################################################################################################################

// Standard Iterations
pub const STD_ITER: usize = 128;

//##########################################################################################################################

// Constants
const D0 = dec!(0);
const D1 = dec!(1);
const D2 = dec!(2);
const D4 = dec!(4);
const D1NEG = dec!(-1);
const D1DIV5 = D1 / dec!(5);
const D1DIV239 = D1 / dec!(239);
const D3DIV2 = dec!(1.5);
const D1M3DIV2 = D1 - D3DIV2;

//##########################################################################################################################

pub const fn pow(
    value: Decimal,
    exp: usize
) -> Decimal {
    let mut acc = D1;
    let mut i: usize = 1;
    match exp {
        0 => D1,
        1 => value,
        _ => match value {
            D0 => D0,
            D1 => D1,
            _ => loop {
                if i > exp {break acc};
                acc = acc * value;
                i = i + 1;
            },
        },
    }
}

//##########################################################################################################################

pub const fn fac(
    value: usize,
) -> Decimal {
    let mut acc = D1;
    let mut i: usize = 1;
    match value {
        0 => D1,
        1 => D1,
        _ => loop {
            if i > value {break acc};
            acc = acc * dec!(i);
            i = i + 1;
        },
    }
}

//##########################################################################################################################

const fn euler(
    terms: usize
) -> Decimal {
    let mut e = D0;
    let mut n: usize = 1;
    loop {
        if n > terms {break e};
        let term = pow(D1, n) / fac(n);
        e = e + term;
        n = n + 1;
    }
}

pub const EULER: Decimal = euler(STD_ITER);

//##########################################################################################################################

const fn ln_of_two(
    terms: usize
) -> Decimal {
    let mut ln = D1;
    let mut n: usize = 1;
    loop {
        if n > terms {break ln};
        let term = pow(D1NEG, n + 1) * (
            pow(D2 - EULER, n) /
            (dec!(n) * pow(EULER, n))
        );
        ln = ln + term;
        n = n + 1;
    }
}

pub const LN_OF_TWO: Decimal = ln_of_two(STD_ITER);

//##########################################################################################################################

const fn pi(
    terms: usize
) -> Decimal {
    let mut term1 = D0;
    let mut n: usize = 1;
    loop {
        if n > terms {break};
        let term = pow(D1NEG, n + 1) * (
            pow(D1DIV5, (2 * n) - 1) /
            dec!((2 * n) - 1)
        );
        term1 = term1 + term;
        n = n + 1;
    };
    let mut term2 = D0;
    let mut n: usize = 1;
    loop {
        if n > terms {break};
        let term = pow(D1NEG, n + 1) * (
            pow(D1DIV239, (2 * n) - 1) /
            dec!((2 * n) - 1)
        );
        term2 = term2 + term;
        n = n + 1;
    };
    (
        D4 * (
            (D4 * term1) - term2
        )
    )
}

pub const PI: Decimal = pi(STD_ITER);

//##########################################################################################################################

const fn sqrt_of_three_halfs(
    terms: usize
) -> Decimal {
    let mut sqrt = D0;
    let mut n: usize = 1;
    loop {
        if n > terms {break sqrt};
        let term = (
            (fac(2 * (n - 1)) * pow(D1M3DIV2, n - 1) * D3DIV2) /
            pow(fac(n - 1) * pow(D2, n - 1), 2)
        );
        sqrt = sqrt + term;
        n = n + 1;
    }
}

pub const SQRT_OF_THREE_HALFS: Decimal = sqrt_of_three_halfs(STD_ITER);

//##########################################################################################################################
