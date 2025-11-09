use anyhow::{Context, Result};
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String> {
    let mut s = String::new();
    File::open(path)
        .with_context( || format!("failed to open file: {}", path) ) ?
        .read_to_string(&mut s)
        .context("failed to read file content") ?;
    Ok(s)
}

pub fn anyhow_test() -> Result<()> {
    let content = read_file("README.md") ?;
    println!("read_file: {:?}", content);
    Ok( () )
}