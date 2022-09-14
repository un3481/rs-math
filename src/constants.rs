
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rust_decimal::Error;

//##########################################################################################################################

// Standard Iterations
pub const STD_ITER: usize = 128;

//##########################################################################################################################

// Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D4: Decimal = dec!(4);
const D1NEG: Decimal = dec!(-1);
const D5: Decimal = dec!(5);
const D239: Decimal = dec!(239);
const D3DIV2: Decimal = dec!(1.5);

//##########################################################################################################################

// To Decimal
pub const fn dec(num: usize) -> Decimal {
    let _i64: i64 = num as i64;
    match Decimal::try_new(_i64, 0) {
        Ok(value) => value,
        Err(_) => panic!("rust_decimal error"),
    }
}

//##########################################################################################################################

pub const fn pow(
    value: Decimal,
    exp: usize
) -> Decimal {
    let mut acc = D0 + D1;
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
    let mut acc = D0 + D1;
    let mut i: usize = 1;
    match value {
        0 => D1,
        1 => D1,
        _ => loop {
            if i > value {break acc};
            acc = acc * dec(i);
            i = i + 1;
        },
    }
}

//##########################################################################################################################

const fn euler(
    terms: usize
) -> Decimal {
    let mut e = D0 + D0;
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
    let mut ln = D0 + D1;
    let mut n: usize = 1;
    loop {
        if n > terms {break ln};
        let term = pow(D1NEG, n + 1) * (
            pow(D2 - EULER, n) /
            (pow(EULER, n) * dec(n))
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
    let mut term1 = D0 + D0;
    let mut n: usize = 1;
    loop {
        if n > terms {break};
        let term = pow(D1NEG, n + 1) * (
            pow(D1 / D5, (2 * n) - 1) /
            dec((2 * n) - 1)
        );
        term1 = term1 + term;
        n = n + 1;
    };
    let mut term2 = D0 + D0;
    let mut n: usize = 1;
    loop {
        if n > terms {break};
        let term = pow(D1NEG, n + 1) * (
            pow(D1 / D239, (2 * n) - 1) /
            dec((2 * n) - 1)
        );
        term2 = term2 + term;
        n = n + 1;
    };
    D4 * ((D4 * term1) - term2)
}

pub const PI: Decimal = pi(STD_ITER);

//##########################################################################################################################

const fn sqrt_of_three_halfs(
    terms: usize
) -> Decimal {
    let mut sqrt = D0 + D0;
    let mut n: usize = 1;
    loop {
        if n > terms {break sqrt};
        let term =
            (fac(2 * (n - 1)) * pow(D1 - D3DIV2, n - 1) * D3DIV2) /
            pow(fac(n - 1) * pow(D2, n - 1), 2)
        ;
        sqrt = sqrt + term;
        n = n + 1;
    }
}

pub const SQRT_OF_THREE_HALFS: Decimal = sqrt_of_three_halfs(STD_ITER);

//##########################################################################################################################
