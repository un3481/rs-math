
fn euler(
    i: usize = 99
) -> Result<f64, Error> {
    Ok(
        (1..=i).par_iter()
            .map(|n| (2..n).par_iter()
                .map(|x| dec!(x))
                .reduce(|| dec!(1), |u, d| u * d)
            )
            .map(|x| dec!(1) / x)
            .reduce(|| dec!(2), |u, d| u + d)
    )
}
