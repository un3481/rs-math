
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::factorial::{ LFAC };

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
    if value < D0 {-value} else {value}
}

//##########################################################################################################################

#[inline]
pub fn fac(value: usize) -> Decimal {
    if value > 27 { panic!("factorial too large for: {}", value) };
    LFAC[value]
}

//##########################################################################################################################

fn pow_series(
    value: Decimal,
    power: usize
) -> Decimal {
    (1..=power).into_iter()
        .map(|_| value)
        .reduce(|u, d| u * d)
        .unwrap()
}

#[inline]
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
            else { pow_series(value, power) }
        },
    }
}

//##########################################################################################################################

#[inline]
pub fn a_pow(
    value: Decimal,
    power: usize,
    base: mut (Decimal, usize)
) -> Decimal {
    if (base.1 > power) { base.1 = power };
    let dif = pow(value, power - base.1);
    let result = dif * base.0;
    base = (result, power);
    result
}

//##########################################################################################################################
