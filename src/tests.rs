
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::arithmetic::{ dec, pow };
use crate::basic::{ sqrt, d_pow };
use crate::complex::basic::{ c_pow, cc_pow };
use crate::error::Error;

//##########################################################################################################################

#[test]
    fn test_sqrt() -> Result<(), Error> {
        let terms = 24;
        let x = dec!(4);
        let sqrt_x = sqrt(x, terms)?;
        let pow_x = d_pow(x, dec!(0.5), terms)?;
        assert_eq!(sqrt_x, pow_x);
        Ok(())
    }

//##########################################################################################################################
