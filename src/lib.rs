//! # beginner_tools
//!
//! Useful library designed for new Rustacens,
//! provides good examples that are useful in simple projects

use std::io::stdin;
use std::error::Error;
use std::str::FromStr;

/// Reads from stdin and tries to parse it to a specified type.
///
/// # Examples
///
/// ```
/// // Will loop as long as user does not pass in a float
/// let my_float = loop {
///     if let Ok(n) = get_stdin::<f64>() {
///         break n
///     };
///     println!("Wrong Input!");
/// };
/// ```
///
/// # Errors
///
/// Function returns an Err(error) when user inputs an incorrect type
pub fn get_stdin<T>() -> Result<T, Box<Error>> 
    where T: FromStr,
          T::Err: 'static + Error 
{
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    Ok(line.trim().parse::<T>()?)
}

