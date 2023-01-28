
// Imports
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

// Modules
use crate::constants::{ E_SQR };

use crate::error::Error;
use crate::sqrt::{ i_sqrt, d_sqrt };
use crate::euler::{ d_exp, d_ln };
use crate::basic::{ dd_pow };
use crate::trigonometry::{ d_cos, d_sin, d_atan, d_atan2 };

//##########################################################################################################################

// Constants
const D1: Decimal = Decimal::ONE;
const D2: Decimal = Decimal::TWO;

const D4: Decimal = dec!(4);
const D1DIV2: Decimal = dec!(0.5);

//##########################################################################################################################

const SQRT_2: Decimal = dec!(1.4142135623730950488016887242);

const SIN_1: Decimal = dec!(0.8414709848078965066525023216);
const COS_1: Decimal = dec!(0.5403023058681397174009366074);
const TAN_1: Decimal = dec!(1.5574077246549022305069748075);

//##########################################################################################################################

// Iteration Terms
const TEST_ITER: usize = 16;

// Decimal Precision
const TEST_DIG: u32 = 24;

//##########################################################################################################################

#[test]
fn sqrt() -> Result<(), Error> {
    // Set Variables
    let _sqrt_2_std = SQRT_2.round_dp(TEST_DIG);
    // i_sqrt(4) == 2
    let res1 = i_sqrt(D4)?;
    assert_eq!(res1, D2);
    // sqrt(4) == 2
    let res2 = d_sqrt(D4, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res2, D2);
    // sqrt(2) == sqrt(2)
    let res3 = d_sqrt(D2, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res3, _sqrt_2_std);
    // pow(4, 1/2) == 2
    let res4 = dd_pow(D4, D1DIV2, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res4, D2);
    // pow(2, 1/2) == sqrt(2)
    let res5 = dd_pow(D2, D1DIV2, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res5, _sqrt_2_std);
    // Return Ok
    Ok(())
}

//##########################################################################################################################

#[test]
fn euler() -> Result<(), Error> {
    // Set Variables
    let _e_sqr_std = E_SQR.round_dp(TEST_DIG);
    // exp(2) == e^2
    let res1 = d_exp(D2, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res1, _e_sqr_std);
    // ln(e ^ 2) == 2
    let res2 = d_ln(E_SQR, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res2, D2);
    // Return Ok
    Ok(())
}

//##########################################################################################################################

#[test]
fn trigonometry() -> Result<(), Error> {
    // Set Variables
    let _sin1_std = SIN_1.round_dp(TEST_DIG);
    let _cos1_std = COS_1.round_dp(TEST_DIG);
    // sin(1) == sin(1)
    let res1 = d_sin(D1, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res1, _sin1_std);
    // cos(1) == cos(1)
    let res2 = d_cos(D1, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res2, _cos1_std);
    // atan(tan(1)) == 1
    let res3 = d_atan(TAN_1, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res3, D1);
    // atan2(cos(1), sin(1)) == 1
    let res4 = d_atan2(COS_1, SIN_1, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res4, D1);
    // Return Ok
    Ok(())
}

//##########################################################################################################################
