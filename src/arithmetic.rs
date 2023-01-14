
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::multiplex::types::{ Multiplex };

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

fn m_pow_series(
    value: Multiplex,
    power: usize
) -> Multiplex {
    (1..=power).into_iter()
        .map(|_| value.clone())
        .reduce(|u, d| u * d)
        .unwrap()
}

#[inline]
pub fn m_pow(
    value: Multiplex,
    power: usize
) -> Multiplex {
    match power {
        0 => Multiplex::new(),
        1 => value,
        _ => { m_pow_series(value, power) },
    }
}

//##########################################################################################################################

#[inline]
pub fn a_pow(
    value: Decimal,
    power: usize,
    base: &mut (Decimal, usize)
) -> Decimal {
    if base.1 > power { base.1 = power };
    let dif = pow(value, power - base.1);
    let result = dif * base.0;
    base.0 = result;
    base.1 = power;
    result
}

//##########################################################################################################################
