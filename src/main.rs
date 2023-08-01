use std::{io::{self, BufRead}, path::PathBuf, ops::RangeInclusive, fs::File, thread, time::Duration, cmp::min, fmt::Write};

use clap::{Arg, ArgAction, Command, value_parser, builder::OsStr, Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle, ProgressState};

use crate::commands::split::split;
use crate::commands::bind::bind;
pub mod commands;
pub mod crypto;
pub mod utils;
const OWO: &str = r#"
                                     ██████╗ ██╗    ██╗ ██████╗ 
                                    ██╔═══██╗██║    ██║██╔═══██╗
                                    ██║   ██║██║ █╗ ██║██║   ██║
                                    ██║   ██║██║███╗██║██║   ██║
                                    ╚██████╔╝╚███╔███╔╝╚██████╔╝
                                     ╚═════╝  ╚══╝╚══╝  ╚═════╝
                                                                      
"#;
fn main() {
    // let mut downloaded = 0;
    // let total_size = 231231231;

    // let pb = ProgressBar::new(total_size);
    // pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
    //     .unwrap()
    //     .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
    //     .progress_chars("#>-"));

    // while downloaded < total_size {
    //     let new = min(downloaded + 223211, total_size);
    //     downloaded = new;
    //     pb.set_position(new);
    //     thread::sleep(Duration::from_millis(12));
    // }

    // pb.finish_with_message("downloaded");
    let matches = Command::new("horcrust")
        .display_name("horcrust")
        .bin_name("hx")
        .version("0.1") //Todo make this env variable
        .about("Split a file(s) into encrypted shards, no password required - secrecy preserved.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("split")
                .long_flag("split")
                .about("Split the file into shards.")
                .arg(
                    Arg::new("file")
                        .required(false)
                        .index(1)
                        .action(ArgAction::Set)
                )
                .arg(
                    Arg::new("shards")
                        .required(true)
                        .short('s')
                        .long("shards")
                        .help("Number of shards to split the secret into.")
                        .value_parser(value_parser!(u8))
                        .action(ArgAction::Set)
                )
                .arg(
                    Arg::new("threshold")
                        .required(true)
                        .short('t')
                        .long("threshold")
                        .help("Number of shards required to resurrect the original secret.")
                        .value_parser(value_parser!(u8))
                        .action(ArgAction::Set)
                )
                .arg(
                    Arg::new("destination")
                        .required(false)
                        .short('d')
                        .long("destination")
                        .default_value(".")
                        .help("Where to save the horcruxes to, a new directory will be created if specified one does not exist.")
                        .action(ArgAction::Set)
                ),
        )
        .subcommand(
            Command::new("bind")
                .long_flag("bind")
                .about("Recovers the secret from given shards.")
                .arg(
                    Arg::new("source")
                        .required(false)
                        .help("Source directory that contains the horcruxes.")
                        .short('s')
                        .long("source")
                        .action(ArgAction::Set)
                )
                .arg(
                    Arg::new("destination")
                        .required(false)
                        .short('d')
                        .long("destination")
                        .default_value(".")
                        .help("Directory of where to place the recovered secret.")
                        .action(ArgAction::Set)
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("split", sub_matches)) => {
            let file = sub_matches.get_one::<String>("file").map(|s| s.as_str());
            let shards: Option<&u8> = sub_matches.get_one("shards");
            let threshold: Option<&u8> = sub_matches.get_one("threshold");
            if threshold.unwrap() > shards.unwrap() {
                println!("Threshold cannot be larger than shards");
                std::process::exit(1);
            }
            let destination = sub_matches.get_one::<String>("destination").map(|s| s.as_str());

            //If file arg not found then check std in.
            if file.is_some() {
                let path = PathBuf::from(file.unwrap());
                let x = shards.unwrap().to_owned();
                if path.is_file() {
                    split(&path, destination.unwrap(), x, threshold.unwrap().to_owned()).expect("Sassaas");
                } else {
                    println!("Not a file!")
                }
            } else {
                let input_file = io::stdin()
                    .lock()
                    .lines()
                    .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");
                let term_file = PathBuf::from(input_file);
                split(&term_file, destination.unwrap(), shards.unwrap().to_owned(), threshold.unwrap().to_owned());
                println!("DONE!!!!")
            }
        }
        Some(("bind", sub_matches)) => {
            let mut source = sub_matches.get_one::<String>("source").map(|s| s.as_str());
            let mut destination = sub_matches.get_one::<String>("destination").map(|s| s.as_str());
            
            if source.is_some() {
                let path = PathBuf::from(source.unwrap());
                let result = bind(&path);
                println!("DONE BINDING")
            } else {
                let input = io::stdin()
                    .lock()
                    .lines()
                    .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");
                println!("BINDING YOUR FILE! std {}", input)
            }

        }
        _ => unreachable!(),
    }
}
