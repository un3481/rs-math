
// Imports
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

//##########################################################################################################################

fn mobius(
    n: usize
) -> Decimal {
    let acc: usize = 0 + n;
    let p: usize = 0;
    // Check 2
    if let 0=acc%2 {
        p = p + 1;
        acc = acc / 2;
        if let 0=acc%2 {return dec!(0)};
    };
    // Check All Primes
    let i: usize = 3;
    loop {
        if (i * i) > n {break};
        if let 0=acc%i {
            p = p + 1;
            acc = acc / i;
            if let 0=acc%i {return dec!(0)};
        };
        i = i + 2;
    };
    // Return Even or Odd
    dec!(
        if let 0=p%2 {1} else {-1}
    )
}

//##########################################################################################################################
