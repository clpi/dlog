use std::{
    ffi::OsString, error::Error,
};
pub enum Either<A, B>{
    A(A),
    B(B)
}

impl<A, B> Either<A, B> {

    pub fn A(a: A) -> Self {
        Either::A(a)
    }

    pub fn B(b: B) -> Self {
        Either::B(b)
    }
}

pub fn get_arg(nth: usize) -> Result<OsString, Box<dyn Error>> {
    match std::env::args_os().nth(nth) {
        None => Err(From::from("No arguments provided")),
        Some(arg) => Ok(arg)
    }
}

// TODO make it so single doesn't coincide with double
pub fn single_flagged(arg: String) -> bool {
    match arg.find("-") {
        Some(idx) => idx == 0,
        None => false,
    }
}

pub fn double_flagged(arg: String) -> bool {
    match arg.find("--") {
        Some(idx) => idx == 0,
        None => false,
    }
}
