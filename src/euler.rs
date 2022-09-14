
// Imports
use rayon::prelude::*;

// Modules
use crate::constants::{ EULER, LN_OF_TWO };
use crate::basic::{ pow, fac };

//##########################################################################################################################

const fn power_series(
    terms: usize,
    value: f64
) -> f64 {
    let mut acc: f64 = 0.0;
    let mut n: usize = 1;
    loop {
        if n > terms {break acc};
        let term =
            pow(value, n) /
            (fac(n) as f64)
        ;
        acc = acc + term;
        n = n + 1;
    }
}

//##########################################################################################################################

pub fn power(
    terms: usize,
    value: f64
) -> f64 {
    match value {
        -1.0 => 1.0 / EULER,
        0.0 => 1.0,
        1.0 => EULER,
        _ => power_series(terms, value),
    }
}

//##########################################################################################################################

const fn ln_prepare(
    value: f64
) -> (f64, f64) {
    let mut rem: f64 = value;
    let mut exp: f64 = 0.0;
    loop {
        if rem > 4.0 {
            rem = rem / 2.0;
            exp = exp + LN_OF_TWO;
        }
        else if rem < 2.0 {
            rem = rem * 2.0;
            exp = exp - LN_OF_TWO;
        }
        else {break}
    };
    (exp, rem)
}

//##########################################################################################################################

fn ln_series(
    terms: usize,
    value: f64
) -> f64 {
    1.0 + (
        (1..=terms).into_par_iter()
            .map(|n|
                pow(-1.0, n + 1) * (
                    pow(value - EULER, n) /
                    (pow(EULER, n) * (n as f64))
                )
            )
            .reduce(|| 0.0, |u, d| u + d)
    )
}

//##########################################################################################################################

pub fn ln(
    terms: usize,
    value: f64
) -> Result<f64, &'static str> {
    if value <= 0.0 {
        return Err("cannot calc ln(x) for x <= 0")
    };
    Ok(
        match value {
            1.0 => 0.0,
            EULER => 1.0,
            _ => {
                let (exp, rem) = ln_prepare(value);
                exp + ln_series(terms, rem)
            },
        }
    )
}

//##########################################################################################################################
