#[macro_use]
extern crate serde_derive;
extern crate docopt;
extern crate csv;

use docopt::Docopt;
use std::error::Error;
use std::fs::{OpenOptions, File};
use std::io::{Read, Write, Result as IoResult, stdin};

const USAGE: &'static str = "
Monitor.

Reads all files specified line-by-line in <file> (or from stdin otherwise) and shows their content structured on the screen.
The gathered values can optionally be appended line-by-line to <outfile> in CSV format.

Usage: monitor [options] [<file>]

Options:
  -o --outfile=<outfile>  Append results in CSV format to outfile.
  -h --help               Show this help screen.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_file: Option<String>,
    flag_outfile: Option<String>,
}

fn read_string_from_src<T: Read>(mut src: T) -> IoResult<String> {
    let mut data = String::new();
    src.read_to_string(&mut data)?;
    Ok(data)
}

fn run(file: Option<String>, outfile: Option<String>) -> Result<(), Box<Error>> {
    let src: Box<Read> = match file {
        Some(path) => Box::new(File::open(path)?),
        None => Box::new(stdin()),
    };

    let content = read_string_from_src(src)?;
    let max_path_len = content.lines().map(str::len).max().unwrap_or(0);

    let stats = {
        let mut tmp = Vec::new();

        for path in content.lines() {
            let stat = File::open(path)
                .and_then(read_string_from_src)
                .unwrap_or_else(|e| format!("<{}>", e));

            tmp.push((path.to_owned(), stat.trim().to_owned()));
        }

        tmp
    };

    // Print human readable results...
    for &(ref path, ref stat) in &stats {
        println!("{:<pad$} = {}", path, stat, pad = max_path_len);
    }

    // ...and, if requested, append CSV line to specified outfile.
    if let Some(outfile) = outfile {
        // Strip paths from vector first...
        let stats = stats.into_iter().map(|(_, stat)| stat).collect::<Vec<_>>();

        let mut writer = csv::Writer::from_memory().double_quote(false);
        writer.encode(stats)?;

        let mut outfile = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(outfile)?;

        outfile.write(writer.as_bytes())?;
    }

    Ok(())
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    run(args.arg_file, args.flag_outfile).unwrap_or_else(|e| println!("error: {}", e));
}
