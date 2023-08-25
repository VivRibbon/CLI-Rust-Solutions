use clap::*;

fn main() {
    let interface = Command::new("Echor")
        .author("Moira Oona Morse, giantsilkmoth@proton.me")
        .version("0.1.0")
        .about("Echo rewritten in Rust")
        .help_template("\
    {before-help}{name} {version}
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}")
        .arg(
            Arg::new("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .num_args(1..)
        )
        .arg(
            Arg::new("omit_newline")
            .short('n')
            .help("Do not print newline")
            .action(ArgAction::SetTrue)
        )
        .get_matches();
    let text: Vec<&str> = interface.get_many("text").unwrap().clone().map(String::as_str).collect();
    let omit_newline: bool = interface.get_flag("omit_newline");

    match omit_newline {
        false => println!("{}", text.join(" ")),
        true => print!("{}", text.join(" "))
    };
}

