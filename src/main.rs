use std::{io::{self, BufRead}, path::PathBuf, borrow::BorrowMut};

use clap::{Arg, ArgAction, Command, error::{ErrorKind, ContextKind, ContextValue, Error}, ColorChoice, Parser, CommandFactory};
use cli::{Cli, Commands};
use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use utils::{shards_in_range, is_qualified_path, is_qualified_file};

use crate::{commands::split::split, utils::handle_std_in};
use crate::commands::bind::bind;
pub mod commands;
pub mod crypto;
pub mod utils;
pub mod cli;


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

    let cli = Cli::parse();


    match &cli.command {
        Commands::Split(args) => {
            let mut cmd = Cli::command();
            let file = &args.file;
            let shards = &args.shards;
            let threshold = &args.threshold;
            let destination = &args.destination;
            if threshold > shards {
                
                let err = cmd.error(ErrorKind::ArgumentConflict, "threshold cannot be larger than shards");
                err.exit();
            }

            if file.is_some() {
                let split = split(
                    &file.as_ref().unwrap(), 
                    args.destination.as_ref().unwrap(), 
                    args.shards,
                    args.threshold
                );
                match split {
                    Ok(_) => println!("Done created horcruxes"),
                    Err(e) => {
                        let err = cmd.error(ErrorKind::ArgumentConflict, "threshold cannot be larger than shards");
                        err.exit();
                    }
                }
            } else {
                let piped_file = handle_std_in().expect("cannot pipe in file.");
                let split = split(
                    &piped_file, 
                    &destination.as_ref().unwrap(), 
                    shards.to_owned(),
                    threshold.to_owned()
                );
                match split {
                    Ok(_) => println!("Done created horcruxes"),
                    Err(e) => eprintln!("{e}")
                }
            }
            // let file = matches.get_one::<PathBuf>("file");
            // let shards: &u8 = sub_matches.get_one("shards").unwrap();
            // let threshold: &u8 = sub_matches.get_one("threshold").unwrap();
            // let destination = sub_matches.get_one::<PathBuf>("destination").unwrap();
        }
        Commands::Bind(args) => {
            let source = &args.source;
            let destination = &args.destination;
            if source.is_some() {
                let result = bind(&source.as_ref().unwrap(), &destination.as_ref().unwrap());
                match result {
                    Ok(_) => println!("Done created horcruxes"),
                    Err(e) => eprintln!("{e}")
                }
            } else {
                let file = handle_std_in();
                let result = bind(&file.unwrap(), &destination.as_ref().unwrap());
                match result {
                    Ok(_) => println!("Done created horcruxes"),
                    Err(e) => eprintln!("{e}")
                }
            }
        }
    }
}


