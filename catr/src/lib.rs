use std::error::Error;
use clap::{Command,Arg,ArgAction};
use std::fs::File;
use std::io::{self,BufRead,BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let interface = Command::new("catr")
        .author("Moira Oona Morse, giantsilkmoth@proton.me")
        .version("0.1.0")
        .about("Cat rewritten in Rust")
        .help_template("\
    {before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}")
        .arg(
            Arg::new("files")
            .value_name("FILES")
            .help("Files to concatenate")
            .default_value("-")
            .num_args(1..)
)
        .arg(
            Arg::new("number_lines")
            .short('n')
            .long("number")
            .help("Number lines")
            .action(ArgAction::SetTrue)
            .conflicts_with("number_nonblank_lines")
        )
        .arg(
            Arg::new("number_nonblank_lines")
            .short('b')
            .long("number-nonblank")
            .help("Number nonblank lines")
            .action(ArgAction::SetTrue)
        )
        .get_matches();

    Ok(Config {
        files: interface.get_many("files").unwrap().cloned().collect(),
        number_lines: interface.get_flag("number_lines"),
        number_nonblank_lines: interface.get_flag("number_nonblank_lines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
