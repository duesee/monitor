use std::error::Error;
use std::fs::File;
use std::io::{Read, Result as IoResult, stdin};

fn read_string_from_src<T: Read>(mut src: T) -> IoResult<String> {
    let mut data = String::new();
    try!(src.read_to_string(&mut data));
    Ok(data)
}

fn run(path: Option<String>) -> Result<(), Box<Error>> {
    let src: Box<Read> = match path {
        Some(path) => Box::new(try!(File::open(path))),
        None => Box::new(stdin()),
    };

    let content = try!(read_string_from_src(src));
    let max_path_len = content.lines().map(str::len).max().unwrap_or(0);

    for path in content.lines() {
        let stat = File::open(path)
            .and_then(read_string_from_src)
            .unwrap_or_else(|e| format!("<{}>", e));
        println!("{:<pad$} = {}", path, stat.trim(), pad = max_path_len);
    }

    Ok(())
}

fn main() {
    let path = std::env::args().nth(1);
    run(path).unwrap_or_else(|e| println!("error: {}", e));
}
