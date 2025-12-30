use anyhow::{bail, ensure, Context, Result};
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

fn read_config() -> Result<String> {
    std::fs::read_to_string("config1.toml")
        .context("failed to read config.toml")
}

fn read_file2(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context( || format!("failed to read file: {}", path) )
}

fn validate_input(input: &str) -> Result<()> {
    ensure!(input.len() >5, "输入太短: {}", input.len() );
    if input.contains("invalid") {
        bail!("无效输入: {}", input);
    }
    Ok( () )
}

pub fn anyhow_test() -> Result<()> {
    // let content = read_config();
    // let content = read_file("README.md") ?;
    let content = read_file2("README.md");

    if let Err(e) = content {
        println!("anyhow cause: {}", e);
    } else {
        println!("anyhow: {:?}", content);
    }

    Ok( () )
}