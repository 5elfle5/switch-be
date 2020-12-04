use std::io::{ Error };
use std::fmt;

#[derive(Debug)]
pub struct BladesError {
    pub details: &'static str
}

impl BladesError {
    pub fn new(msg: &'static str) -> BladesError {
        BladesError{ details: msg }
    }
}

impl fmt::Display for BladesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl From<Error> for BladesError {
    fn from(_: Error) -> BladesError {
        BladesError::new("Blades!")
    }
}
