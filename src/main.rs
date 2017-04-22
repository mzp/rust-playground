use std::io::prelude::*;
use std::io::Result;
use std::fs::File;
use std::env;

fn read_all(path : String) -> Result<String> {
    let mut buffer = String::new();
    return File::open(path)
        .and_then(|mut f| f.read_to_string(&mut buffer))
        .map(|_| buffer);
}

fn main() {
    for path in env::args().skip(1) {
        match read_all(path) {
            Ok(content) => { print!("{}", content) }
            Err(why) => { println!("ERR: {}", why.to_string()) }
        }
    }
}
