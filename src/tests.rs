
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::error::Error;
use crate::basic::{ d_pow };
use crate::sqrt::{ sqrt, int_sqrt };

use crate::complex::types::{ Complex };
use crate::complex::basic::{ cc_pow };

//##########################################################################################################################

// Decimal Constants
const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D4: Decimal = dec!(4);
const D1DIV2: Decimal = dec!(0.5);

// Complex Constants
const C1: Complex = Complex{ re: D1, im: D0 };
const C2: Complex = Complex{ re: D2, im: D0 };
const C4: Complex = Complex{ re: D4, im: D0 };
const CI1: Complex = Complex{ re: D0, im: D1 };
const C1DIV2: Complex = Complex{ re: D1DIV2, im: D0 };

// Iteration Term
const TEST_ITER: usize = 16;

//##########################################################################################################################

#[test]
fn test_sqrt() -> Result<(), Error> {
    // int_sqrt(4)
    let res1 = int_sqrt(D4)?;
    assert_eq!(res1, D2);
    // sqrt(4)
    let res2 = sqrt(D4, TEST_ITER)?.round_dp(16);
    assert_eq!(res2, D2);
    // pow(4, 1/2)
    let res3 = d_pow(D4, D1DIV2, TEST_ITER)?.round_dp(16);
    assert_eq!(res3, D2);
    // sqrt(4) == int_sqrt(4) == pow(4, 1/2)
    assert_eq!(res1, res2);
    assert_eq!(res2, res3);
    // Return Ok
    Ok(())
}

//##########################################################################################################################

#[test]
fn test_cc_pow() -> Result<(), Error> {
    // cc_pow(4, 1/2)
    let mut res1 = cc_pow(C4, C1DIV2, TEST_ITER)?;
    res1.re = res1.re.round_dp(16);
    assert_eq!(res1, C2);
    // cc_pow(-1, 1/2)
    let res2 = cc_pow(-C1, C1DIV2, TEST_ITER)?;
    assert_eq!(res2, CI1);
    // Return Ok
    Ok(())
}

//##########################################################################################################################
