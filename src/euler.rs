
use rust_decimal_macros::dec;
use rust_decimal::prelude::*;
use rayon::prelude::*;

//##########################################################################################################################

const fn euler(
    i: usize
) -> Result<Decimal, Error> {
    Ok(
        (1..=i).par_iter()
            .map(|n| (2..n).par_iter()
                .map(|x| dec!(x))
                .reduce(|| dec!(1), |u, d| u * d)
            )
            .map(|x| dec!(1) / x)
            .reduce(|| dec!(0), |u, d| u + d)
    )
}

const EULER: Decimal = euler(99).unwrap();

//##########################################################################################################################

const fn ln2(
    i: usize
) -> Result<Decimal, Error> {
    Ok(
        (1..=i).par_iter()
            .map(|x| (x, if let 0=x%2 {1} else {-1}))
            .map(|(n, s)| (s, [
                || (1..=n).par_iter()
                    .map(|_| dec!(2) - EULER)
                    .reduce(|| dec!(1), |u, d| u * d),
                || (1..=n).par_iter()
                    .map(|_| EULER)
                    .reduce(|| dec!(n), |u, d| u * d)
            ]))
            .map(|(s, t)| (s, t.par_iter()
                .map(|f| f()).collect()
            ))
            .map(|(s, t)| (t[0] / t[1]) * s)
            .reduce(|| dec!(1), |u, d| u + d)
    )
}

const LN_OF_TWO: Decimal = ln2(99).unwrap();

//##########################################################################################################################
