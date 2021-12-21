use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file =  File::open("/etc/hosts")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    println!("{}", text);
    Ok(())
}