use range::Range;

fn main() {
    let mut pi = 0.0;
    let mut numerator = 1.0;

    for k in (Range {
        start: 0,
        end: 1000,
    }) {
        pi += numerator / (2 * k + 1) as f64;
        numerator /= -3.0;
    }
    pi *= f64::sqrt(12.0);

    println!("pi = {pi}");
}
