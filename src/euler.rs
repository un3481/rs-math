
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

mod consts;

//##########################################################################################################################

pub fn euler_series(
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

//##########################################################################################################################

pub fn ln_series(
    i: usize,
    x: Decimal
) -> Result<Decimal, Error> {
    Ok(
        (1..=i).par_iter()
            .map(|n| (n, [
                || (1..=n).par_iter()
                    .map(|_| x - consts::EULER)
                    .reduce(|| dec!(1), |u, d| u * d),
                || (1..=n).par_iter()
                    .map(|_| consts::EULER)
                    .reduce(|| dec!(n), |u, d| u * d)
            ]))
            .map(|(n, t)| (n, t.par_iter().map(|f| f()).collect()))
            .map(|(n, t)| (if let 0=n%2 {-1} else {1}, t))
            .map(|(s, t)| (t[0] / t[1]) * dec!(s))
            .reduce(|| dec!(1), |u, d| u + d)
    )
}

//##########################################################################################################################

fn dec_by2(
    exp: isize,
    x: Decimal
) -> (isize, Decimal) {
    let (d2, d4) = (dec!(2), dec!(4));
    match true {
        (x > d4) => dec_by2(exp + 1, x / d2),
        (x < d2) => dec_by2(exp - 1, x * d2),
        _ => (exp, x),
    }
}

//##########################################################################################################################

pub fn ln(
    i: usize,
    x: Decimal
) -> Result<Decimal, Error> {
    let (exp, rem) = dec_by2(0, x);
    let lnx = ln_series(i, rem)?;
    Ok(
        lnx + (
            dec!(exp) * consts::LN_OF_TWO
        )
    )
}

//##########################################################################################################################
