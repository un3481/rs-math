
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::basic::{ sqrt, d_pow };
use crate::complex::types::{ Complex };
use crate::complex::basic::{ cc_pow };
use crate::error::Error;

//##########################################################################################################################

const D0: Decimal = dec!(0);
const D1: Decimal = dec!(1);
const D2: Decimal = dec!(2);
const D4: Decimal = dec!(4);
const D1DIV2: Decimal = dec!(0.5);

const C1: Complex = Complex{ re: D1, im: D0 };
const C2: Complex = Complex{ re: D2, im: D0 };
const C4: Complex = Complex{ re: D4, im: D0 };
const C1DIV2: Complex = Complex{ re: D1DIV2, im: D0 };
const CI1: Complex = Complex{ re: D0, im: D1 };

//##########################################################################################################################

#[test]
    fn test_sqrt() -> Result<(), Error> {
        let terms = 48;
        let res1 = sqrt(D4, terms)?;
        let res2 = d_pow(D4, D1DIV2, terms)?;
        assert_eq!(res1, D2);
        assert_eq!(res1, D2);
        assert_eq!(res1, res2);
        Ok(())
    }

//##########################################################################################################################

#[test]
    fn test_cc_pow() -> Result<(), Error> {
        let terms = 48;
        let res1 = cc_pow(CI1, C1DIV2, terms);
        let res2 = cc_pow(C4, C1DIV2, terms);
        assert_eq!(res1, -C1);
        assert_eq!(res2, C2);
        assert_eq!(res1, res2);
        Ok(())
    }

//##########################################################################################################################
