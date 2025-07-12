use std::error::Error;
use std::fmt;



#[derive(Debug)]
pub struct InvalidDataError {
    details: String
}

impl InvalidDataError {
    
    pub fn new <T> (details: String) -> Result<T, Box<dyn Error>> {
        Err(Box::new(InvalidDataError {
            details: details
        }))
    }
    
}

impl fmt::Display for InvalidDataError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter,"{}",self.details)
    }
}

impl Error for InvalidDataError {
    fn description (&self) -> &str {
        &self.details
    }
}



#[derive(Debug)]
pub struct ForcedExitError {
    details: String
}

impl ForcedExitError {
    
    pub fn new <T> (details: String) -> Result<T, Box<dyn Error>> {
        Err(Box::new(ForcedExitError {
            details: details
        }))
    }
    
}

impl fmt::Display for ForcedExitError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter,"{}",self.details)
    }
}

impl Error for ForcedExitError {
    fn description (&self) -> &str {
        &self.details
    }
}