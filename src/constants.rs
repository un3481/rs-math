
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

//##########################################################################################################################

// Standard Iterations
pub const STD_ITER: usize = 99;

// Private Constants
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

const fn pow(
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

const fn fac(
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
        let top = pow(D1, n);
        let bot = fac(n);
        e = e + (top / bot);
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
        let top = pow(D2 - EULER, n);
        let bot = pow(EULER, n) * dec!(n);
        let sig = pow(D1NEG, n + 1);
        ln = ln + ((top / bot) * sig);
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
        let top = pow(D1DIV5, (2 * n) - 1);
        let bot = dec!((2 * n) - 1);
        let sig = pow(D1NEG, n + 1);
        term1 = term1 + ((top / bot) * sig);
        n = n + 1;
    };
    let mut term2 = D0;
    let mut n: usize = 1;
    loop {
        if n > terms {break};
        let top = pow(D1DIV239, (2 * n) - 1);
        let bot = dec!((2 * n) - 1);
        let sig = pow(D1NEG, n + 1);
        term2 = term2 + ((top / bot) * sig);
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
        let top = D3DIV2 * (
            fac(2 * (n - 1)) *
            pow(D1M3DIV2, n - 1)
        );
        let bot = pow((
            fac(n - 1) *
            pow(D2, n - 1)
        ), 2);
        sqrt = sqrt + (top / bot);
        n = n + 1;
    }
}

pub const SQRT_OF_THREE_HALFS: Decimal = sqrt_of_three_halfs(STD_ITER);

//##########################################################################################################################
