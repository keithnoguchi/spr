//! std::iter::try_fold exercise
use std::error::Error;
use std::io::BufRead;
use std::result;
use std::str::FromStr;

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let sum = stdin
        .lock()
        .lines()
        .try_fold(0, |sum, line| -> Result<u64> {
            Ok(sum + u64::from_str(line?.trim())?)
        })?;
    println!("{sum}");
    Ok(())
}
