use std::fs::File;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {

    #[error("invalid input")]
    InvalidInput,

    #[error("io error")]
    Io(#[from] std::io::Error),

}

fn open_file() -> Result<File, MyError> {
    let f = File::open("config.toml") ? ;
    Ok(f)
}

pub fn thiserror_test() {
    let content = open_file();
    println!("this_error: {:?}", content);
}