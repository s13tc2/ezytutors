use std::fmt;
use std::fs::File;
use std::io::Write;
#[derive(Debug)]
pub enum MyError {
    ParseError,
    IOError,
}

impl std::error::Error for MyError {}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::ParseError => write!(f, "Parse Error"),
            MyError::IOError => write!(f, "IO Error"),
        }
    }
}

fn main() {
    let result = square("INVALID");
    match result {
        Ok(res) => println!("Result is {:?}", res),
        Err(e) => println!("Error in parsing: {:?}", e),
    };
    // println!("{:?}", square("2"));
    // println!("{:?}", square("INVALID"));
}

fn square(val: &str) -> Result<i32, MyError> {
    // match val.parse::<i32>() {
    //     Ok(num) => Ok(i32::pow(num, 2)),
    //     Err(e) => Err(e),
    // }
    let num = val.parse::<i32>().map_err(|_| MyError::ParseError)?;
    let mut f = File::open("fictionalfile.txt").map_err(|_| MyError::IOError)?;
    let string_to_write = format!("Square of {:?} is {:?}", num, i32::pow(num, 2));
    f.write_all(string_to_write.as_bytes())
        .map_err(|_| MyError::IOError)?;
    Ok(i32::pow(num, 2))
}
