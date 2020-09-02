extern crate libwave;
use libwave::chunks;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Debug)]
struct ArgError;

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "not enough arguments")
    }
}

impl Error for ArgError {}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(ArgError));
    }
    let filename = args[1].clone();
    let data: Vec<u8> = fs::read(filename)?;
    let ret = chunks::parse_wave(&data);
    let (_, wav) = ret.unwrap();
    dbg!(wav.master);
    dbg!(wav.fmt);
    Ok(())
}
