use std::{io::{self, BufRead}, path::PathBuf, ops::RangeInclusive, fs::File, thread, time::Duration, cmp::min, fmt::Write};

use clap::{Arg, ArgAction, Command, value_parser, builder::OsStr, Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle, ProgressState};

use crate::commands::split::split;
use crate::commands::bind::bind;
pub mod commands;




#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    //Disable output
    #[arg(short, long, action = clap::ArgAction::Count)]
    silent: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Split {
        /// lists test values
        #[arg(short, long, required = false)]
        file: Option<PathBuf>,
        #[arg(short, long, required = false)]
        shards: u8,
        threshold: u8,
        destination: PathBuf
    },
    Bind {
        source: PathBuf,
        destination: PathBuf
    }
}



fn non_main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Split { file, shards, threshold, destination }) => {
            if(file.is_some()) {
                
            } else {

            }
        }
        Some(Commands::Bind { source, destination }) => {

        }
        _ => unreachable!()
    }
}


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
                        .action(ArgAction::Set)
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
                        .help("Source directory that contains the horcruxes")
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
                        .help("Directory to place the recovered file.")
                        .action(ArgAction::Set)
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("split", sub_matches)) => {
            let file = sub_matches.get_one::<String>("file").map(|s| s.as_str());
            let shards: Option<&u8> = sub_matches.get_one("shards");
            let threshold: Option<&u8> = sub_matches.get_one("threshold");
            let destination = sub_matches.get_one::<String>("destination").map(|s| s.as_str());

            //If file arg not found then check std in.
            if file.is_some() {
                let path = PathBuf::from(file.unwrap());
                let x = shards.unwrap().to_owned();
                if path.is_file() {
                    println!("Found file!");
                    let result = split(&path, destination.unwrap(), x, threshold.unwrap().to_owned());
                    println!("DONE!!!!")
                } else {
                    println!("Not a file!")
                }
            } else {
                let input_file = io::stdin()
                .lock()
                .lines()
                .fold("".to_string(), |acc, line| acc + &line.unwrap() + "\n");
                let term_file = PathBuf::from(input_file);
                let result = split(&term_file, destination.unwrap(), shards.unwrap().to_owned(), threshold.unwrap().to_owned());
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
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
