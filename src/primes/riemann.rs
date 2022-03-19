
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

//##########################################################################################################################

// Constants
const D0 = dec!(0);

//##########################################################################################################################

fn mobius(
    value: usize
) -> Result<Decimal, Error> {
    if value <= 0 {
        panic!("cannot calc mobius(x) for x <= 0");
    };
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
        dec!(
            if let 0=primes%2 {1} else {-1}
        )
    )
}

//##########################################################################################################################
