
// Imports
use rayon::prelude::*;

// Modules
use crate::constants::{ PI };
use crate::basic::{ pow, fac };

//##########################################################################################################################

// Constants
const PIHALF: f64 = PI / 2.0;
const PIHNEG: f64 = -PIHALF;
const PINEG: f64 = -PI;

//##########################################################################################################################

const fn trig_prepare(
    value: f64
) -> f64 {
    let mut rem: f64 = value;
    if rem > PI {
        rem = rem - (
            (rem / (PI * 2.0)).floor() * PI * 2.0
        )
    }
    else if rem < -PI {
        rem = rem - (
            (rem / (PI * 2.0)).floor() * PI * 2.0
        )
    }
    if rem > PI { rem = rem - (PI * 2.0) }
    else if rem < -PI { rem = rem + (PI * 2.0) };
    rem
}

//##########################################################################################################################

fn cos_series(
    terms: usize,
    value: f64
) -> f64 {
    (0..terms).into_par_iter()
        .map(|n|
            pow(-1.0, n) * (
                pow(value, 2 * n) /
                (fac(2 * n) as f64)
            )
        )
        .reduce(|| 0.0, |u, d| u + d)
}

//##########################################################################################################################

fn sin_series(
    terms: usize,
    value: f64
) -> f64 {
    (0..terms).into_par_iter()
        .map(|n|
            pow(-1.0, n) * (
                pow(value, (2 * n) + 1) /
                (fac((2 * n) + 1) as f64)
            )
        )
        .reduce(|| 0.0, |u, d| u + d)
}

//##########################################################################################################################

pub fn cos(
    terms: usize,
    value: f64
) -> Result<f64, ()> {
    let rem: f64 = trig_prepare(value);
    Ok(
        match rem {
            PI => -1.0,
            PIHALF => 0.0,
            0 => 1.0,
            PIHNEG => 0.0,
            PINEG => -1.0,
            _ => cos_series(terms, rem),
        }
    )
}

//##########################################################################################################################

pub fn sin(
    terms: usize,
    value: f64
) -> Result<f64, ()> {
    let rem: f64 = trig_prepare(value);
    Ok(
        match rem {
            PI => 0.0,
            PIHALF => 1.0,
            0 => 0.0,
            PIHNEG => -1.0,
            PINEG => 0.0,
            _ => sin_series(terms, rem),
        }
    )
}

//##########################################################################################################################
