
// Imports
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

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
const DN1: Decimal = Decimal::NEGATIVE_ONE;
const D0: Decimal = Decimal::ZERO;
const D1: Decimal = Decimal::ONE;
const D2: Decimal = Decimal::TWO;

const D3: Decimal = dec!(3);
const D4: Decimal = dec!(4);
const D7: Decimal = dec!(7);
const D24: Decimal = dec!(24);
const D1DIV2: Decimal = dec!(0.5);

const CI: Complex = Complex::I;
const C1: Complex = Complex::ONE;
const C2: Complex = Complex::TWO;

const C4: Complex = Complex::new(D4, D0);
const C4I3: Complex = Complex::new(D4, D3);
const C7I24: Complex = Complex::new(D7, D24);
const CN1I2: Complex = Complex::new(DN1, D2);
const C1DIV2: Complex = Complex::new(D1DIV2, D0);

const SIN1: Decimal = dec!(0.8414709848078965066525023216);
const COS1: Decimal = dec!(0.5403023058681397174009366074);
const TAN1: Decimal = dec!(1.5574077246549022305069748075);

const CC_POW_TEST_RE: Decimal = dec!(-0.0466016701047362131681326903);
const CC_POW_TEST_IM: Decimal = dec!(0.0296221438823630242149182959);
const CC_POW_TEST: Complex = Complex::new(CC_POW_TEST_RE, CC_POW_TEST_IM);

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
    // Return Ok
    Ok(())
}

//##########################################################################################################################

#[test]
fn test_euler() -> Result<(), Error> {
    // Set Variables
    let e_sqr_std = E_SQR.round_dp(STD_DIG);
    // exp(2) == e^2
    let res1 = exp(D2, STD_ITER_LONG)?.round_dp(STD_DIG);
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
    // Set Variables
    let sin1_std = SIN1.round_dp(STD_DIG);
    let cos1_std = COS1.round_dp(STD_DIG);
    // sin(1) == sin(1)
    let res1 = sin(D1, STD_ITER)?.round_dp(STD_DIG);
    assert_eq!(res1, sin1_std);
    // cos(1) == cos(1)
    let res2 = cos(D1, STD_ITER)?.round_dp(STD_DIG);
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
    // Set Variables
    let cc_pow_test_std = CC_POW_TEST.round_dp(STD_DIG);
    let mut _cn1 = -C1;
    let mut _c4 = C4;
    let mut _c7i24 = C7I24;
    let mut _c4i3 = C4I3;
    // cc_pow(-1, 1/2) == i
    let res1 = cc_pow(&mut _cn1, C1DIV2, STD_ITER)?;
    assert_eq!(res1, CI);
    // cc_pow(4, 1/2) == 2
    let res2 = cc_pow(&mut _c4, C1DIV2, STD_ITER)?.round_dp(STD_DIG);
    assert_eq!(res2, C2);
    // cc_pow(7 + 24i, 1/2) == 4 + 3i
    let res3 = cc_pow(&mut _c7i24, C1DIV2, STD_ITER_LONG)?.round_dp(STD_DIG);
    assert_eq!(res3, C4I3);
    // cc_pow(7 + 24i, 1/2) == 4 + 3i
    let res4 = cc_pow(&mut _c4i3, C2, STD_ITER_DOUBLE)?.round_dp(STD_DIG);
    assert_eq!(res4, C7I24);
    // cc_pow(4 + 3i, -1 + 2i) == (4 + 3i) ^ (-1 + 2i)
    let res5 = cc_pow(&mut _c4i3, CN1I2, STD_ITER_DOUBLE)?.round_dp(STD_DIG);
    assert_eq!(res5, cc_pow_test_std);
    // Return Ok
    Ok(())
}

//##########################################################################################################################
