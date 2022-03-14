
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

//##########################################################################################################################

pub fn pow(
    value: Decimal,
    exp: usize
) -> Result<Decimal, Error> {
    Ok(
        match exp {
            0 => dec!(1),
            _ => match value {
                dec!(1) => dec!(1),
                _ => (1..=exp).par_iter()
                    .map(|_| value)
                    .reduce(|| dec!(1), |u, d| u * d),
            },
        }
    )
}

//##########################################################################################################################

pub fn fac(
    value: usize,
) -> Result<Decimal, Error> {
    Ok(
        match value {
            0 => dec!(1),
            _ => (1..=value).par_iter()
                .map(|x| dec!(x))
                .reduce(|| dec!(1), |u, d| u * d),
        }
    )
}

//##########################################################################################################################

