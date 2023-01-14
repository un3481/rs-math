
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::basic::{ sqrt, d_pow };
use crate::complex::types::{ Complex };
use crate::complex::basic::{ cc_pow };
use crate::error::Error;

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

// Iteration Terms
const TEST_ITER: usize = 32;

//##########################################################################################################################

#[test]
    fn test_sqrt() -> Result<(), Error> {
        // Test POW(4, 1/2)
        let res1 = d_pow(D4, D1DIV2, TEST_ITER)?;
        assert_eq!(res1, D2);
        // Test SQRT(4)
        let res2 = sqrt(D4, TEST_ITER)?;
        assert_eq!(res2, D2);
        // Test SQRT(4) = POW(4, 1/2)
        assert_eq!(res1, res2);
        // Return Ok
        Ok(())
    }

//##########################################################################################################################

#[test]
    fn test_cc_pow() -> Result<(), Error> {
        // Test POW(-1, 1/2)
        let res1 = cc_pow(-C1, C1DIV2, TEST_ITER)?;
        assert_eq!(res1, CI1);
        // Test POW(4, 1/2)
        let res2 = cc_pow(C4, C1DIV2, TEST_ITER)?;
        assert_eq!(res2, C2);
        // Return Ok
        Ok(())
    }

//##########################################################################################################################
