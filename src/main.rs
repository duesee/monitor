use std::fs::File;
use std::io::prelude::*;

fn read_to_string(mut src: Box<Read>) -> std::io::Result<String> {
    let mut data = String::new();
    try!(src.read_to_string(&mut data));
    Ok(data)
}

fn run() -> Result<(), Box<std::error::Error>> {
    let mut args = std::env::args();
    let _ = args.next();
    let path = args.next();

    let src: Box<Read> = match path {
        // If path provided read from file...
        Some(path) => Box::new(try!(File::open(path))),
        // ...else read from stdin.
        None => Box::new(std::io::stdin()),
    };

    let src_content = try!(read_to_string(src));
    let max_path_len = src_content.lines().map(str::len).max().unwrap_or(0);

    for path in src_content.lines() {
        let stat = File::open(path)
            .and_then(|f| read_to_string(Box::new(f)))
            .unwrap_or_else(|e| format!("<{}>", e));
        println!("{:<pad$} = {}", path, stat.trim(), pad = max_path_len);
    }

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(e) => println!("error: {}", e),
    }
}
