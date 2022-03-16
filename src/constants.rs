
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

//##########################################################################################################################

pub const STD_ITER: usize = 99;

//##########################################################################################################################

const fn pow(
    value: Decimal,
    exp: usize
) -> Decimal {
    let mut acc = dec!(1);
    let mut i: usize = 1;
    match exp {
        0 => dec!(1),
        1 => value,
        _ => match value {
            dec!(0) => dec!(0),
            dec!(1) => dec!(1),
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
    let mut acc = dec!(1);
    let mut i: usize = 1;
    match value {
        0 => dec!(1),
        1 => dec!(1),
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
    let param = dec!(1);
    let mut e = dec!(0);
    let mut n: usize = 1;
    loop {
        if n > terms {break e};
        let top = pow(param, n);
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
    let param = dec!(2);
    let mut ln = dec!(1);
    let mut n: usize = 1;
    loop {
        if n > terms {break ln};
        let top = pow(param - EULER, n);
        let bot = pow(EULER, n) * dec!(n);
        let sig = pow(dec!(-1), n + 1);
        ln = ln + ((top / bot) * sig);
        n = n + 1;
    }
}

pub const LN_OF_TWO: Decimal = ln_of_two(STD_ITER);

//##########################################################################################################################

const fn pi(
    terms: usize
) -> Decimal {
    let mut term1 = dec!(0);
    let mut n: usize = 1;
    loop {
        if n > terms {break};
        let top = pow(dec!(1) / dec!(5), (2 * n) - 1);
        let bot = dec!((2 * n) - 1);
        let sig = pow(dec!(-1), n + 1);
        term1 = term1 + ((top / bot) * sig);
        n = n + 1;
    };
    let mut term2 = dec!(0);
    let mut n: usize = 1;
    loop {
        if n > terms {break};
        let top = pow(dec!(1) / dec!(239), (2 * n) - 1);
        let bot = dec!((2 * n) - 1);
        let sig = pow(dec!(-1), n + 1);
        term2 = term2 + ((top / bot) * sig);
        n = n + 1;
    };
    (
        dec!(4) * (
            (dec!(4) * term1) - term2
        )
    )
}

pub const PI: Decimal = pi(STD_ITER);

//##########################################################################################################################

const fn sqrt_of_three_halfs(
    terms: usize
) -> Decimal {
    let param = dec!(1.5);
    let mut sqrt = dec!(0);
    let mut n: usize = 1;
    loop {
        if n > terms {break sqrt};
        let top = param * (
            fac(2 * (n - 1)) *
            pow(dec!(1) - param, n - 1)
        );
        let bot = pow((
            fac(n - 1) *
            pow(dec!(2), n - 1)
        ), 2);
        sqrt = sqrt + (top / bot);
        n = n + 1;
    }
}

pub const SQRT_OF_THREE_HALFS: Decimal = sqrt_of_three_halfs(STD_ITER);

//##########################################################################################################################
