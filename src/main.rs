use std::io::prelude::*;
use std::fs::File;
use std::env;

fn read_all(path : String, mut buffer : &mut String) -> std::io::Result<usize> {
    return File::open(path)
        .and_then(|mut f| f.read_to_string(&mut buffer));
}

fn main() {
    for path in env::args().skip(1) {
        let mut buffer = String::new();

        match read_all(path, &mut buffer) {
            Ok(_) => { print!("{}", buffer) }
            Err(why) => { println!("ERR: {}", why.to_string()) }
        }
    }
}
