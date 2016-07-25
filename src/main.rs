use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn read_file_to_string<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let mut file = try!(File::open(path));
    let mut data = String::new();
    try!(file.read_to_string(&mut data));
    Ok(data)
}

fn run(path: &str) -> Result<(), Box<std::error::Error>> {
    let file_content = try!(read_file_to_string(&path));
    let mut paths: Vec<_> = file_content.lines().collect();

    paths.sort();

    let max_path_len = paths.iter().map(|s| s.len()).max().unwrap_or(0);

    for path in &paths {
        let stat = read_file_to_string(path).unwrap_or_else(|e| format!("<{}>", e));
        println!("{:<pad$} = {}", path, stat.trim(), pad = max_path_len);
    }

    Ok(())
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next().expect("unknown argument environment...");
    let filepath = args.next().expect("no monitor file specified...");

    match run(&filepath) {
        Ok(_) => (),
        Err(e) => println!("error: {}", e),
    }
}
