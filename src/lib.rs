//! # beginner_tools
//!
//! Helpful library designed for new Rustacens,
//! provides good examples that are useful in simple projects

use std::io::stdin;
use std::error::Error;
use std::str::FromStr;

/// Reads from stdin and tries to parse it to a specified type.
///
/// # Examples
///
/// ```
/// // Will loop as long as user does not pass in a float,
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

/// Inspired by Python's "input()", this function does (almost)
/// exactly the same thing, takes an input from the user
/// and tries to parse it to the specified type.
/// Note that this function requires a prompt for the user.
///
/// # Examples
/// ```
/// // Will loop as long as user does not pass in a float,
/// // Stores the resulting value in "my_int" for later use
/// let my_int: i64 = loop {
///     if let Ok(n) = input("Type an integer: ") {
///         break n
///     };
///     println!("Wrong Input!");
/// };
///
/// // Shorter, asks once. Crashes if input is invalid.
/// // Passing in an empty &str results in the same
/// // functionality as "get_stdin()".
/// let my_float = input::<f64>("")
///     .expect("Error, not a float");
/// ```
/// 
/// # Errors
///
/// Function returns an Err(error) when user inputs an incorrect type.
pub fn input<T>(prompt: &str) -> Result<T, Box<Error>> 
    where T: FromStr,
          T::Err: 'static + Error 
{
    eprint!("{}", prompt);
    get_stdin::<T>() 
}
