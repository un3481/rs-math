
// Standard Iterations
pub const STD_ITER: usize = 100;

//##########################################################################################################################

pub const fn pow(
    value: f64,
    exp: usize
) -> f64 {
    let mut acc: f64 = 1.0;
    let mut n: usize = 1;
    match exp {
        0 => 1.0,
        1 => value,
        _ => match value {
            0.0 => 0.0,
            1.0 => 1.0,
            _ => loop {
                if n > exp {break acc};
                acc = acc * value;
                n = n + 1;
            },
        },
    }
}

//##########################################################################################################################

pub const fn fac(
    value: usize,
) -> u128 {
    let mut acc: u128 = 1;
    let mut n: usize = 1;
    match value {
        0 => 1,
        1 => 1,
        _ => loop {
            if n > value {break acc};
            acc = acc * (n as u128);
            n = n + 1;
        },
    }
}

//##########################################################################################################################

const fn euler(
    terms: usize
) -> f64 {
    let mut acc: f64 = 0.0;
    let mut n: usize = 1;
    loop {
        if n > terms {break acc};
        let term: f64 =
            1.0 /
            (fac(n) as f64)
        ;
        acc = acc + term;
        n = n + 1;
    }
}

pub const EULER: f64 = euler(STD_ITER);

//##########################################################################################################################

const fn ln_of_two(
    terms: usize
) -> f64 {
    let mut acc: f64 = 1.0;
    let mut n: usize = 1;
    loop {
        if n > terms {break acc};
        let term: f64 = 
            pow(-1.0, n + 1) * (
                pow(2.0 - EULER, n) /
                (pow(EULER, n) * (n as f64))
            )
        ;
        acc = acc + term;
        n = n + 1;
    }
}

pub const LN_OF_TWO: f64 = ln_of_two(STD_ITER);

//##########################################################################################################################

const fn pi(
    terms: usize
) -> f64 {
    let mut term1: f64 = 0.0;
    let mut n: usize = 1;
    loop {
        if n > terms {break};
        let term: f64 =
            pow(-1.0, n + 1) * (
                pow(
                    1.0 / 5.0,
                    (2 * n) - 1
                ) /
                ((2.0 * (n as f64)) - 1.0)
            )
        ;
        term1 = term1 + term;
        n = n + 1;
    };
    let mut term2: f64 = 0.0;
    let mut n: usize = 1;
    loop {
        if n > terms {break};
        let term: f64 =
            pow(-1.0, n + 1) * (
                pow(
                    1.0 / 239.0,
                    (2 * n) - 1
                ) /
                ((2.0 * (n as f64)) - 1.0)
            )
        ;
        term2 = term2 + term;
        n = n + 1;
    };
    4.0 * ((4.0 * term1) - term2)
}

pub const PI: f64 = pi(STD_ITER);

//##########################################################################################################################

const fn sqrt_of_three_halfs(
    terms: usize
) -> f64 {
    let mut acc: f64 = 0.0;
    let mut n: usize = 1;
    loop {
        if n > terms {break acc};
        let term: f64 = (
            (3.0 / 2.0) *
            (fac(2 * (n - 1)) as f64) *
            pow(1.0 / 2.0, n - 1)
        ) /
        pow(
            (fac(n - 1) as f64) *
            pow(2.0, n - 1),
            2
        );
        acc = acc + term;
        n = n + 1;
    }
}

pub const SQRT_OF_THREE_HALFS: f64 = sqrt_of_three_halfs(STD_ITER);

//##########################################################################################################################
