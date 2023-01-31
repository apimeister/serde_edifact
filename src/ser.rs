use std::fmt::Display;

use crate::error::Error;
use serde::Serialize;

pub fn to_string<T>(value: &T) -> Result<String, Error>
where
    T: Serialize + Display,
{
    let final_string = format!("{}",value);
    #[cfg(feature = "debug")]
    println!("debug vec: {final_string}");
    Ok(final_string)
}