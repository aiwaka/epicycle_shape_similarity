use anyhow::Result;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;

pub fn output_sequences<T>(filename: &str, data: &[T]) -> Result<()>
where
    T: Display,
{
    let mut file = File::create(filename)?;
    for v in data {
        writeln!(file, "{}", v)?;
    }
    Ok(())
}

pub fn output_sequences_with_x<T>(filename: &str, x_data: &[f64], data: &[T]) -> Result<()>
where
    T: Display,
{
    let mut file = File::create(filename)?;
    for (x, y) in x_data.iter().zip(data) {
        writeln!(file, "{}  {}", x, y)?;
    }
    Ok(())
}
