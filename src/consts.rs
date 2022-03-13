
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

//##########################################################################################################################

pub const STD_ITER: usize = 99;

//##########################################################################################################################

const fn euler(
    i: usize
) -> Decimal {
    let mut e = dec!(0);
    let mut n: usize = 1;
    loop {
        if n > i {break};
        let mut bot = dec!(1);
        let mut f: usize = 2;
        loop {
            if f >= n {break};
            bot = bot * dec!(i);
            f = f + 1;
        };
        e = e + (dec!(1) / bot);
        n = n + 1;
    };
    e
}

pub const EULER: Decimal = euler(STD_ITER);

//##########################################################################################################################

const fn ln_of_two(
    i: usize
) -> Decimal {
    let mut ln2 = dec!(1);
    let mut s = dec!(-1);
    let mut n: usize = 1;
    loop {
        if n > i {break};
        let mut top = dec!(1);
        let mut f: usize = 1;
        loop {
            if f > n {break};
            top = top * (2 - EULER);
            f = f + 1;
        };
        let mut bot = dec!(n);
        let mut f: usize = 1;
        loop {
            if f > n {break};
            bot = bot * EULER;
            f = f + 1;
        };
        s = s * dec!(-1);
        ln2 = ln2 + ((top / bot) * s);
        n = n + 1;
    };
    ln2
}

pub const LN_OF_TWO: Decimal = ln_of_two(STD_ITER);

//##########################################################################################################################
