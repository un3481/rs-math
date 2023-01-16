
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::constants::{ E_SQR };

use crate::error::Error;
use crate::sqrt::{ sqrt, int_sqrt };
use crate::euler::{ exp, ln };
use crate::basic::{ d_pow };
use crate::trigonometry::{ sin, cos, atan, atan2 };

use crate::complex::types::{ Complex };
use crate::complex::basic::{ cc_pow };

//##########################################################################################################################

// Constants
const DN1: Decimal = dec!(-1);
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D3: Decimal = dec!(3);
const D4: Decimal = dec!(4);
const D7: Decimal = dec!(7);
const D24: Decimal = dec!(24);
const D1DIV2: Decimal = dec!(0.5);

const C1: Complex = Complex{ re: D1, im: D0 };
const C2: Complex = Complex{ re: D2, im: D0 };
const C4: Complex = Complex{ re: D4, im: D0 };
const CI1: Complex = Complex{ re: D0, im: D1 };
const CN1I2: Complex = Complex{ re: DN1, im: D2 };
const C4I3: Complex = Complex{ re: D4, im: D3 };
const C7I24: Complex = Complex{ re: D7, im: D24 };
const C1DIV2: Complex = Complex{ re: D1DIV2, im: D0 };

const SIN1: Decimal = dec!(0.8414709848078965066525023216);
const COS1: Decimal = dec!(0.5403023058681397174009366074);
const TAN1: Decimal = dec!(1.5574077246549022305069748075);

const CC_POW_TEST_RE: Decimal = dec!(-0.0466016701047362131681326903);
const CC_POW_TEST_IM: Decimal = dec!(0.0296221438823630242149182959);
const CC_POW_TEST: Complex = Complex{ re: CC_POW_TEST_RE, im: CC_POW_TEST_IM };

// Decimal Precision
const STD_DIG: u32 = 16;

// Iteration Terms
const STD_ITER: usize = 16;
const STD_ITER_LONG: usize = 24;
const STD_ITER_DOUBLE: usize = 32;

//##########################################################################################################################

#[test]
fn test_sqrt() -> Result<(), Error> {
    // int_sqrt(4) == 2
    let res1 = int_sqrt(D4)?;
    assert_eq!(res1, D2);
    // sqrt(4) == 2
    let res2 = sqrt(D4, STD_ITER)?.round_dp(STD_DIG);
    assert_eq!(res2, D2);
    // pow(4, 1/2) == 2
    let res3 = d_pow(D4, D1DIV2, STD_ITER)?.round_dp(STD_DIG);
    assert_eq!(res3, D2);
    // sqrt(4) == int_sqrt(4) == pow(4, 1/2)
    assert_eq!(res1, res2);
    assert_eq!(res2, res3);
    // Return Ok
    Ok(())
}

//##########################################################################################################################

#[test]
fn test_euler() -> Result<(), Error> {
    // exp(2) == e^2
    let res1 = exp(D2, STD_ITER_LONG)?.round_dp(STD_DIG);
    let e_sqr_std = E_SQR.round_dp(STD_DIG);
    assert_eq!(res1, e_sqr_std);
    // ln(e ^ 2) == 2
    let res2 = ln(E_SQR, STD_ITER)?.round_dp(STD_DIG);
    assert_eq!(res2, D2);
    // Return Ok
    Ok(())
}

//##########################################################################################################################

#[test]
fn test_trigonometry() -> Result<(), Error> {
    // sin(1) == sin(1)
    let res1 = sin(D1, STD_ITER)?.round_dp(STD_DIG);
    let sin1_std = SIN1.round_dp(STD_DIG);
    assert_eq!(res1, sin1_std);
    // cos(1) == cos(1)
    let res2 = cos(D1, STD_ITER)?.round_dp(STD_DIG);
    let cos1_std = COS1.round_dp(STD_DIG);
    assert_eq!(res2, cos1_std);
    // atan(tan(1)) == 1
    let res3 = atan(TAN1, STD_ITER)?.round_dp(STD_DIG);
    assert_eq!(res3, D1);
    // atan2(cos(1), sin(1)) == 1
    let res4 = atan2(COS1, SIN1, STD_ITER)?.round_dp(STD_DIG);
    assert_eq!(res4, D1);
    // Return Ok
    Ok(())
}

//##########################################################################################################################

#[test]
fn test_complex() -> Result<(), Error> {
    // cc_pow(4, 1/2) == 2
    let mut res1 = cc_pow(C4, C1DIV2, STD_ITER)?;
    res1.re = res1.re.round_dp(STD_DIG);
    assert_eq!(res1, C2);
    // cc_pow(-1, 1/2) == i
    let res2 = cc_pow(-C1, C1DIV2, STD_ITER)?;
    assert_eq!(res2, CI1);
    // cc_pow(7 + 24i, 1/2) == 4 + 3i
    let mut res3 = cc_pow(C7I24, C1DIV2, STD_ITER_LONG)?;
    res3.re = res3.re.round_dp(STD_DIG);
    res3.im = res3.im.round_dp(STD_DIG);
    assert_eq!(res3, C4I3);
    // cc_pow(4 + 3i, -1 + 2i) == (4 + 3i) ^ (-1 + 2i)
    let mut res4 = cc_pow(C4I3, CN1I2, STD_ITER_DOUBLE)?;
    res4.re = res4.re.round_dp(STD_DIG);
    res4.im = res4.im.round_dp(STD_DIG);
    let mut cc_pow_test_std = CC_POW_TEST;
    cc_pow_test_std.re = cc_pow_test_std.re.round_dp(STD_DIG);
    cc_pow_test_std.im = cc_pow_test_std.im.round_dp(STD_DIG);
    assert_eq!(res4, cc_pow_test_std);
    // Return Ok
    Ok(())
}

//##########################################################################################################################
