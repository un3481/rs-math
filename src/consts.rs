
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

//##########################################################################################################################

pub const STD_ITER: usize = 99;

//##########################################################################################################################

const fn euler() -> Decimal {
        (1..=STD_ITER).par_iter()
            .map(|n| (2..n).par_iter()
                .map(|x| dec!(x))
                .reduce(|| dec!(1), |u, d| u * d)
            )
            .map(|x| dec!(1) / x)
            .reduce(|| dec!(0), |u, d| u + d)
}

pub const EULER: Decimal = euler();

//##########################################################################################################################

const fn ln2() -> Decimal {
        (1..=STD_ITER).par_iter()
            .map(|n| (n, [
                || (1..=n).par_iter()
                    .map(|_| x - EULER)
                    .reduce(|| dec!(1), |u, d| u * d),
                || (1..=n).par_iter()
                    .map(|_| EULER)
                    .reduce(|| dec!(2), |u, d| u * d)
            ]))
            .map(|(n, t)| (n, t.par_iter().map(|f| f()).collect()))
            .map(|(n, t)| (if let 0=n%2 {-1} else {1}, t))
            .map(|(s, t)| (t[0] / t[1]) * dec!(s))
            .reduce(|| dec!(1), |u, d| u + d)
}

pub const LN_OF_TWO: Decimal = ln2();

//##########################################################################################################################
