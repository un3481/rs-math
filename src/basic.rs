
// Imports
use rayon::prelude::*;

// Modules
use crate::constants::{ SQRT_OF_THREE_HALFS };
pub use crate::constants::{ pow, fac };

//##########################################################################################################################

const fn sqrt_prepare(
    value: f64
) -> (f64, f64) {
    let mut rem: f64 = value;
    let mut ratio: f64 = 1.0;
    loop {
        if rem > 1.5 {
            rem = rem / 1.5;
            ratio = ratio * SQRT_OF_THREE_HALFS;
        }
        else if rem < 0.5 {
            rem = rem * 1.5;
            ratio = ratio / SQRT_OF_THREE_HALFS;
        }
        else {break}
    };
    (ratio, rem)
}

//##########################################################################################################################

fn sqrt_series(
    terms: usize,
    value: f64
) -> f64 {
    (1..=terms).into_par_iter()
        .map(|n|
            (
                (fac(2 * (n - 1)) as f64) *
                pow(1.0 - value, n - 1) *
                value
            ) /
            pow(
                (fac(n - 1) as f64) *
                pow(2.0, n - 1),
                2
            )
        )
        .reduce(|| 0.0, |u, d| u + d)
}
 
//##########################################################################################################################

pub fn sqrt(
    terms: usize,
    value: f64
) -> Result<f64, &'static str> {
    if value < 0.0 {
        return Err("cannot calc sqrt(x) for x < 0")
    };
    Ok(
        match value {
            0 => 0.0,
            1 => 1.0,
            _ => {
                let (ratio, rem) = sqrt_prepare(value);
                ratio * sqrt_series(terms, rem)
            },
        }
    )
}

//##########################################################################################################################
