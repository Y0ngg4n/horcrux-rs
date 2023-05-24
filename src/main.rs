use std::{io::{self, BufRead}, fs::File, path::PathBuf};

use clap::{Arg, ArgAction, Command, builder::FalseyValueParser, value_parser};

pub mod commands;

fn main() {
    let matches = Command::new("hx")
        .version("0.1") //Todo make this env variable
        .about("Utility to split a file into n number of encrypted secrets - no password needed.")
        .long_about("Make your command line a little more magical! Horcrux splits a file into encrypted shards and requires a set threshold to recover them.")
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
                        .action(ArgAction::Set)
                )
                .arg(
                    Arg::new("threshold")
                        .required(true)
                        .long("threshold")
                        .short('t')
                        .help("Number of horcrux shards required to recover the secret")
                        .action(ArgAction::Set)
                )
                .arg(
                    Arg::new("outdir")
                        .required(false)
                        .short('o')
                        .long("outdir")
                        .default_value(".")
                        .help("Directory to save the horcruxes to, a new directory will be created if specified does not exist")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .subcommand(
            Command::new("bind")
                .long_flag("bind")
                .about("Recovers the secret from the")
                .arg(
                    Arg::new("directory")
                        .required(false)
                        .help("location of directory that contains the horcruxes")
                        .short('d')
                        .long("directory")
                        .num_args(1..)
                        .action(ArgAction::Append)
                )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("split", sub_matches)) => {
            let mut file = sub_matches.get_one::<String>("file").map(|s| s.as_str());
            let shards = sub_matches.get_one::<String>("shards").map(|s| s.as_str());
            let threshold = sub_matches
                .get_one::<String>("threshold")
                .map(|s| s.as_str());
            let outdir = sub_matches.get_one::<String>("outdir").map(|s| s.as_str());

            //If file arg not found then check std in.
            if file.is_some() {
                let path = PathBuf::from(file.unwrap());
                if path.is_file() {
                    println!("Found file!")
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
            let mut directory = sub_matches.get_one::<String>("directory").map(|s| s.as_str());
            //TODO if piped then check this
            if directory.is_some() {

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
