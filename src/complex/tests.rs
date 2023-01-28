
// Imports
use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

// Modules
use crate::constants::{ PI };

use crate::error::Error;

use crate::complex::types::{ Complex };
use crate::complex::basic::{ cc_pow };
use crate::complex::trigonometry::{ c_cos, c_sin, c_tan, c_atan };
use crate::complex::riemann::{ zeta };

//##########################################################################################################################

// Constants
const D1: Decimal = Decimal::ONE;
const D2: Decimal = Decimal::TWO;

const D3: Decimal = dec!(3);
const D4: Decimal = dec!(4);
const D6: Decimal = dec!(6);
const D7: Decimal = dec!(7);
const D24: Decimal = dec!(24);

const CI: Complex = Complex::I;
const C1: Complex = Complex::ONE;

//##########################################################################################################################

const CC_POW_TEST_RE: Decimal = dec!(-0.0466016701047362131681326903);
const CC_POW_TEST_IM: Decimal = dec!(0.0296221438823630242149182959);
const CC_POW_TEST: Complex = Complex::new(CC_POW_TEST_RE, CC_POW_TEST_IM);

const ASIN_2_RE: Decimal = dec!(1.5707963267948966192313216916);
const ASIN_2_IM: Decimal = dec!(-1.316957896924816708625046347);
const ASIN_2: Complex = Complex::new(ASIN_2_RE, ASIN_2_IM);

const ACOS_3I1_RE: Decimal = dec!(0.3377011092655525270851559001);
const ACOS_3I1_IM: Decimal = dec!(-1.824198702193882734419662159);
const ACOS_3I1: Complex = Complex::new(ACOS_3I1_RE, ACOS_3I1_IM);

const ATAN_1IN3_RE: Decimal = dec!(1.4614618538579256382102348165);
const ATAN_1IN3_IM: Decimal = dec!(-0.3059438579055289264121938212);
const ATAN_1IN3: Complex = Complex::new(ATAN_1IN3_RE, ATAN_1IN3_IM);

//##########################################################################################################################

// Iteration Terms
const TEST_ITER: usize = 16;

// Decimal Precision
const TEST_DIG: u32 = 24;

//##########################################################################################################################

#[test]
fn basic() -> Result<(), Error> {
    // Set Variables
    let _cc_pow_test_std = CC_POW_TEST.round_dp(TEST_DIG);
    let mut _cn1 = -C1;
    let _c1div2 = C1 / D2;
    let _c2 = C1 * D2;
    let mut _c4 = C1 * D4;
    let mut _c4i3 = D4 + (D3 * CI);
    let mut _c7i24 = D7 + (D24 * CI);
    let _cn1i2 = -D1 + (D2 * CI);
    // cc_pow(-1, 1/2) == i
    let res1 = cc_pow(&mut _cn1, _c1div2, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res1, CI);
    // cc_pow(4, 1/2) == 2
    let res2 = cc_pow(&mut _c4, _c1div2, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res2, _c2);
    // cc_pow(7 + 24i, 1/2) == 4 + 3i
    let res3 = cc_pow(&mut _c7i24, _c1div2, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res3, _c4i3);
    // cc_pow(7 + 24i, 1/2) == 4 + 3i
    let res4 = cc_pow(&mut _c4i3, _c2, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res4, _c7i24);
    // cc_pow(4 + 3i, -1 + 2i) == (4 + 3i) ^ (-1 + 2i)
    let res5 = cc_pow(&mut _c4i3, _cn1i2, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res5, _cc_pow_test_std);
    // Return Ok
    Ok(())
}

//##########################################################################################################################

#[test]
fn trigonometry() -> Result<(), Error> {
    // Set Variables
    let _atan_1in3_std = ATAN_1IN3.round_dp(TEST_DIG);
    let _c2 = C1 * D2;
    let _c3i1 = D3 + CI;
    let _c1in3 = D1 - (D3 * CI);
    // cc_sin(asin(2)) == 2
    let res1 = c_sin(ASIN_2, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res1, _c2);
    // cc_cos(asin(3 + i)) == 3 + i
    let res2 = c_cos(ACOS_3I1, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res2, _c3i1);
    // cc_tan(atan(1 - 3i)) == 1 - 3i
    let res3 = c_tan(ATAN_1IN3, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res3, _c1in3);
    // cc_atan(1 - 3i) == atan(1 - 3i)
    let res4 = c_atan(_c1in3, TEST_ITER)?.round_dp(TEST_DIG);
    assert_eq!(res4, _atan_1in3_std);
    // Return Ok
    Ok(())
}

//##########################################################################################################################

#[test]
fn riemann() -> Result<(), Error> {
    // Set Variables
    let _c2 = C1 * D2;
    let _pi_sqr_div6 = (C1 * ((PI * PI) / D6)).round_dp(3);
    // zeta(2) == (pi ^ 2) / 6
    let res1 = zeta(_c2, 10_000, TEST_ITER)?.round_dp(3);
    assert_eq!(res1, _pi_sqr_div6);
    // Return Ok
    Ok(())
}

//##########################################################################################################################
