
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;

// Modules
use crate::error::Error;

//##########################################################################################################################

// Constants
const D0 = dec!(0);
const D1 = dec!(1);
const D1NEG = dec!(-1);

//##########################################################################################################################

fn mobius(
    value: usize
) -> Result<Decimal, Error> {
    if value <= 0 { return Err(Error::InputOutOfRange) };
    // Set Variables
    let mut acc: usize = value;
    let mut primes: usize = 0;
    // Check 2
    if let 0=acc%2 {
        primes = primes + 1;
        acc = acc / 2;
        if let 0=acc%2 {return Ok(D0)};
    };
    // Check All Primes
    let i: usize = 3;
    loop {
        if (i * i) > n {break};
        if let 0=acc%i {
            primes = primes + 1;
            acc = acc / i;
            if let 0=acc%i {return Ok(D0)};
        };
        i = i + 2;
    };
    // Return Even or Odd
    Ok(
        if let 0=primes%2 {D1}
        else {D1NEG}
    )
}

//##########################################################################################################################
