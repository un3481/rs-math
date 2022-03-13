
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

//##########################################################################################################################

const STD_ITER: usize = 99;

//##########################################################################################################################

const fn euler_series(
    i: usize
) -> Result<Decimal, Error> {
    Ok(
        (1..=i).par_iter()
            .map(|n| (2..n).par_iter()
                .map(|x| dec!(x))
                .reduce(|| dec!(1), |u, d| u * d)
            )
            .map(|x| dec!(1) / x)
            .reduce(|| dec!(0), |u, d| u + d)
    )
}

const EULER: Decimal = euler_series(STD_ITER).unwrap();

//##########################################################################################################################

const fn ln_series(
    i: usize,
    x: Decimal
) -> Result<Decimal, Error> {
    Ok(
        (1..=i).par_iter()
            .map(|n| (n, [
                || (1..=n).par_iter()
                    .map(|_| x - EULER)
                    .reduce(|| dec!(1), |u, d| u * d),
                || (1..=n).par_iter()
                    .map(|_| EULER)
                    .reduce(|| dec!(n), |u, d| u * d)
            ]))
            .map(|(n, t)| (n, t.par_iter().map(|f| f()).collect()))
            .map(|(n, t)| (if let 0=n%2 {-1} else {1}, t))
            .map(|(s, t)| (t[0] / t[1]) * dec!(s))
            .reduce(|| dec!(1), |u, d| u + d)
    )
}

const LN_OF_TWO: Decimal = ln_series(STD_ITER, dec!(2)).unwrap();

//##########################################################################################################################

const fn dec_by2(
    cnt: isize,
    value: Decimal
) -> Result<(isize, Decimal), Error> {
    Ok(
        match true {
            (value > 4) => dec_by2(cnt + 1, value / 2),
            (value < 2) => dec_by2(cnt - 1, value * 2),
            _ => (cnt, value),
        }
    )
}

//##########################################################################################################################
