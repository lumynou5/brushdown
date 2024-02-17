mod config;

use crate::config::Config;
use clap::{Parser, Subcommand};
use std::fs::{create_dir_all, remove_dir_all, File, OpenOptions};
use std::io::{BufReader, BufWriter, ErrorKind};
use std::path::Path;
use std::process::ExitCode;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    subcommand: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    New,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.subcommand {
        None => {
            let config_file =
                match File::open("brushdown.yaml").or_else(|_| File::open("brushdown.yml")) {
                    Ok(v) => v,
                    Err(err) if err.kind() == ErrorKind::NotFound => {
                        eprintln!(
                            "Configuration file is not found.  Run `brushdown new` to create one."
                        );
                        return ExitCode::FAILURE;
                    }
                    Err(_) => {
                        eprintln!("Failed to open the configuration file.");
                        return ExitCode::FAILURE;
                    }
                };
            let config_reader = BufReader::new(config_file);
            let config = Config::from_raw(match serde_yaml::from_reader(config_reader) {
                Ok(v) => v,
                Err(_) => {
                    eprintln!("Failed to parse the configuration file.");
                    return ExitCode::FAILURE;
                }
            });
            if !config.src.is_dir() || config.src.is_symlink() {
                eprintln!(
                    "The source path `{}` is not a directory.",
                    config.src.display()
                );
                return ExitCode::FAILURE;
            }
            if config.src == config.dest {
                eprintln!("The source directory and the destination directory are same.");
                return ExitCode::FAILURE;
            }
            if (config.dest.exists() && !config.dest.is_dir()) || config.dest.is_symlink() {
                eprintln!(
                    "The destination path `{}` exists and is not a directory.",
                    config.dest.display()
                );
                return ExitCode::FAILURE;
            }
            if config.dest.exists() {
                if let Err(_) = remove_dir_all(&config.dest) {
                    eprintln!(
                        "Failed to clear the destination directory `{}`.",
                        config.dest.display()
                    );
                    return ExitCode::FAILURE;
                }
            }
            if let Err(_) = create_dir_all(&config.dest) {
                eprintln!(
                    "Failed to create the destination directory `{}`.",
                    config.dest.display()
                );
                return ExitCode::FAILURE;
            }
            ExitCode::SUCCESS
        }
        Some(Command::New) => {
            if Path::new("brushdown.yaml").exists() {
                eprintln!("A configuration file exists already, please remove it first.");
                ExitCode::FAILURE
            } else {
                match OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open("brushdown.yaml")
                {
                    Ok(file) => {
                        match serde_yaml::to_writer(BufWriter::new(file), &Config::default()) {
                            Ok(_) => ExitCode::SUCCESS,
                            Err(_) => {
                                eprintln!("Failed to write to the configuration file");
                                ExitCode::FAILURE
                            }
                        }
                    }
                    Err(_) => {
                        eprintln!("Failed to create a configuration file.");
                        ExitCode::FAILURE
                    }
                }
            }
        }
    }
}
