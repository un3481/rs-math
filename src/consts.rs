
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

//##########################################################################################################################

pub const STD_ITER: usize = 99;

//##########################################################################################################################

const fn euler() -> Decimal {
    let mut e = dec!(0);
    let mut n: usize = 1;
    loop {
        let mut bot = dec!(1);
        let mut i: usize = 2;
        loop { 
            bot = bot * i;
            i = i + 1;
            if i >= n {break};
        };
        e = e + (dec!(1) / bot);
        n = n + 1;
        if n > STD_ITER {break};
    };
    e
}

pub const EULER: Decimal = euler();

//##########################################################################################################################

const fn ln_of_two() -> Decimal {
    let mut ln2 = dec!(1);
    let mut n: usize = 1;
    loop {
        let mut top = dec!(1);
        let mut i: usize = 1;
        loop {
            top = top * (2 - EULER);
            i = i + 1;
            if i > n {break};
        };
        let mut bot = dec!(2);
        let mut i: usize = 1;
        loop {
            bot = bot * EULER;
            i = i + 1;
            if i > n {break};
        };
        let s = if let 0=n%2 {-1} else {1};
        ln2 = ln2 + ((top / bot) * dec!(s));
        n = n + 1;
        if n > STD_ITER {break};
    };
    ln2
}

pub const LN_OF_TWO: Decimal = ln_of_two();

//##########################################################################################################################
