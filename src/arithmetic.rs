
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

//##########################################################################################################################

const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);

//##########################################################################################################################

#[inline]
pub fn dec(value: usize) -> Decimal {
    Decimal::new(value as i64, 0)
}

#[inline]
pub fn pos(value: Decimal) -> Decimal {
    value * (if value < D0 {-D1} else {D1})
}

//##########################################################################################################################

pub fn pow(
    value: Decimal,
    power: usize
) -> Decimal {
    match power {
        0 => D1,
        1 => value,
        _ => {
                 if value == D0 {D0}
            else if value == D1 {D1}
            else {
                (1..=power).into_iter()
                    .map(|_| value)
                    .reduce(|u, d| u * d)
                    .unwrap_or(D0)
            }
        },
    }
}

//##########################################################################################################################

pub fn fac(
    value: usize,
) -> Decimal {
    match value {
        0 => D1,
        1 => D1,
        _ => (1..=value).into_iter()
            .map(|n| dec(n))
            .reduce(|u, d| u * d)
            .unwrap_or(D0),
    }
}

//##########################################################################################################################
