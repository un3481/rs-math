
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

//##########################################################################################################################

fn mobius(
    value: usize
) -> Result<Decimal, Error> {
    if value <= 0 {
        panic!("cannot calc mobius(x) for x <= 0");
    };
    let d0 = dec!(0);
    let mut acc: usize = 0 + value;
    let mut primes: usize = 0;
    // Check 2
    if let 0=acc%2 {
        primes = primes + 1;
        acc = acc / 2;
        if let 0=acc%2 {return Ok(d0)};
    };
    // Check All Primes
    let i: usize = 3;
    loop {
        if (i * i) > n {break};
        if let 0=acc%i {
            primes = primes + 1;
            acc = acc / i;
            if let 0=acc%i {return Ok(d0)};
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
