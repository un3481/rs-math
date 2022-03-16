// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

// Modules
use crate::basic;
use crate::constants::{PI};

//##########################################################################################################################

fn cos_series(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    Ok(
        (0..terms).par_iter()
            .map(|n| (if let 0=n%2 {-1} else {1}, [
                || basic::pow(value, (n - 1) * 2).unwrap(),
                || basic::fac((n - 1) * 2).unwrap()
            ].par_iter()))
            .map(|(s, t)| (s, t.map(|f| f()).collect()))
            .map(|(s, t)| (t[0] / t[1]) * dec!(s))
            .reduce(|| dec!(0), |u, d| u + d)
    )
}

//##########################################################################################################################

fn trig_prepare(
    value: Decimal
) -> Decimal {
    let pi2 = dec!(2) * PI;
    let mut rem = dec!(0) + value;
    match true {
        (rem > dec!(PI)) => {
            rem = rem - (
                (rem / pi2).floor() * pi2
            );
        },
        (rem < dec!(-PI)) => {
            rem = rem - (
                (rem / pi2).floor() * pi2
            );
        },
        _ => {break rem},
    }
}

//##########################################################################################################################

pub fn cos(
    terms: usize,
    value: Decimal
) -> Result<Decimal, Error> {
    let rem = trig_prepare(value);
    Ok(
    
    )
}

//##########################################################################################################################
