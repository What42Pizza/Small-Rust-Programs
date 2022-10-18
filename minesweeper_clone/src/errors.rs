use std::error::Error;
use std::fmt;



#[derive(Debug)]
pub struct ForcedExitError {}

impl ForcedExitError {
    pub fn new<T>() -> Result<T, Box<dyn Error>> {
        Err(Box::new(ForcedExitError {}))
    }
}

impl fmt::Display for ForcedExitError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter,"{}", "Exit forced by user.")
    }
}

impl Error for ForcedExitError {
    fn description (&self) -> &str {
        "Exit forced by user."
    }
}