use std::{io::{self, BufRead}, fs::File, path::PathBuf};

use clap::{Arg, ArgAction, Command, builder::FalseyValueParser, value_parser};

use crate::commands::split::split;
use crate::commands::bind::bind;
pub mod commands;

fn main() {
    let matches = Command::new("hx")
        .version("0.1") //Todo make this env variable
        .about("Utility to split a file into n number of encrypted secrets - no password needed.")
        .long_about("Horcrust adds magic to your command line, Use it to splits a file into a desired number encrypted shards. Set a required threshold in order to recover them - no password necessary.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Author")
        .subcommand(
            Command::new("split")
                .long_flag("split")
                .about("Split a secret into encrypted file shards")
                .arg(
                    Arg::new("file")
                        .required(false)
                        .short('f')
                        .long("file")
                        .action(ArgAction::Append)
                )
                .arg(
                    Arg::new("shards")
                        .required(true)
                        .short('s')
                        .long("shards")
                        .help("Desired number of shards to split the secret into")
                        .value_parser(value_parser!(u8))
                        .action(ArgAction::Set)
                )
                .arg(
                    Arg::new("threshold")
                        .required(true)
                        .short('t')
                        .long("threshold")
                        .help("Number of horcrux shards required to recover the secret")
                        .value_parser(value_parser!(u8))
                        .action(ArgAction::Set)
                )
                .arg(
                    Arg::new("destination")
                        .required(false)
                        .short('d')
                        .long("destination")
                        .default_value(".")
                        .help("Directory to save the horcruxes to, a new directory will be created if specified does not exist.")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .subcommand(
            Command::new("bind")
                .long_flag("bind")
                .about("Recovers the secret from the")
                .arg(
                    Arg::new("source")
                        .required(false)
                        .help("Source location that contains the horcruxes")
                        .short('s')
                        .long("source")
                        .num_args(1..)
                        .action(ArgAction::Append)
                )
                .arg(
                    Arg::new("destination")
                        .required(false)
                        .short('d')
                        .long("destination")
                        .default_value(".")
                        .help("Directory to place the recovered file.")
                        .action(ArgAction::Set)
                        .num_args(1..)
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("split", sub_matches)) => {
            let mut file = sub_matches.get_one::<String>("file").map(|s| s.as_str());
            let shards: Option<&u8> = sub_matches.get_one("shards");
            let threshold: Option<&u8> = sub_matches.get_one("threshold");
            let destination = sub_matches.get_one::<String>("destination").map(|s| s.as_str());

            //If file arg not found then check std in.
            if file.is_some() {
                let path = PathBuf::from(file.unwrap());
                let x = shards.unwrap().to_owned();
                if path.is_file() {
                    println!("Found file!");
                    let result = split(&path.to_str().unwrap(), destination.unwrap(), x, threshold.unwrap().to_owned());
                    println!("DONE!!!!")
                } else {
                    println!("Not a file!")
                }
            } else {
                let input_file = io::stdin()
                .lock()
                .lines()
                .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");
            println!("SPLITTING YOUR FILE! std {}", input_file)
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
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
