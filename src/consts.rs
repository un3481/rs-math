
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
        _ => match value {
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
    let mut e = dec!(0);
    let mut n: usize = 1;
    loop {
        if n > terms {break e};
        let bot = fac(n);
        e = e + (dec!(1) / bot);
        n = n + 1;
    }
}

pub const EULER: Decimal = euler(STD_ITER);

//##########################################################################################################################

const fn ln_of_two(
    terms: usize
) -> Decimal {
    let mut ln2 = dec!(1);
    let mut s = dec!(-1);
    let mut n: usize = 1;
    loop {
        if n > terms {break ln2};
        let top = pow(dec!(2) - EULER, n);
        let bot = pow(EULER, n) * dec!(n);
        s = s * dec!(-1);
        ln2 = ln2 + ((top / bot) * s);
        n = n + 1;
    }
}

pub const LN_OF_TWO: Decimal = ln_of_two(STD_ITER);

//##########################################################################################################################
