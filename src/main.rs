use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Bad number of arguments");
        return ExitCode::FAILURE;
    }
    let file = match File::open(&args[1]) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("{}", err);
            return ExitCode::FAILURE;
        },
    };
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    if let Err(err) = reader.read_to_string(&mut content) {
        eprintln!("{}", err);
        return ExitCode::FAILURE;
    }
    print!("{}", &content);
    ExitCode::SUCCESS
}
