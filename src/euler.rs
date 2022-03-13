
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
mod consts;
mod basic;

//##########################################################################################################################

pub fn power_series(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    Ok(
        (0..terms).par_iter()
            .map(|n| [
                || basic::pow(value, n).unwrap(),
                || basic::fac(n).unwrap()
            ].par_iter())
            .map(|t| t.map(|f| f()).collect())
            .map(|t| t[0] / t[1])
            .reduce(|| dec!(0), |u, d| u + d)
    )
}

//##########################################################################################################################

pub fn euler(
    terms: usize
) -> Result<Decimal, Error> {
    Ok(power_series(terms, dec!(1)))
}

//##########################################################################################################################

pub fn ln_series(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    Ok(
        (1..=terms).par_iter()
            .map(|n| (n, [
                || basic::pow(value - consts::EULER, n).unwrap(),
                || basic::pow(consts::EULER, n).unwrap() * dec!(n)
            ].par_iter()))
            .map(|(n, t)| (n, t.map(|f| f()).collect()))
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
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    let (exp, rem) = dec_by2(0, value);
    Ok(
        ln_series(terms, rem)? + (
            dec!(exp) * consts::LN_OF_TWO
        )
    )
}

//##########################################################################################################################
