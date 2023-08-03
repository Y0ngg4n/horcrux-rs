use std::cmp::min;
use std::fmt::Write;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use clap::{error::ErrorKind, Parser, CommandFactory};
use cli::{Cli, Commands};
use indicatif::{ProgressBar, ProgressStyle, ProgressState};

use crate::{commands::split::split, utils::handle_std_in};
use crate::commands::bind::bind;
pub mod commands;
pub mod crypto;
pub mod utils;
pub mod cli;


fn main() {
    
    let cli = Cli::parse();
    //todo handle empty stdin, progress bar,
    match &cli.command {
        Commands::Split(args) => {
            let mut cmd = Cli::command();
            let file = &args.file;
            let shards = &args.shards;
            let threshold = &args.threshold;
            let destination = &args.destination;
            if threshold > shards {
                let err = cmd.error(ErrorKind::ArgumentConflict, "Threshold cannot be larger than shards.");
                err.exit();
            }
            
            let source: &PathBuf;
            // let piped_file = handle_std_in().unwrap();
            let piped_file = PathBuf::new();
            
            if file.is_some() {
                source = file.as_ref().unwrap();
            } else {
                if !piped_file.metadata().unwrap().len() == 0 {
                    let err = cmd.error(ErrorKind::MissingRequiredArgument, "Cannot find a secret to split.");
                    err.exit();
                }
                source = &piped_file;
            }

            let split_result = split(
                &source, 
                destination.as_ref().unwrap(), 
                args.shards,
                args.threshold
            );
            match split_result {
                Ok(_) => println!("Done created horcruxes"),
                Err(err) => {
                    let err = cmd.error(ErrorKind::ArgumentConflict, err);
                    err.exit();
                }
            }
        }
        Commands::Bind(args) => {
            let mut cmd = Cli::command();
            let source = &args.source;
            let destination = &args.destination;
            if source.is_some() {
                let result = bind(&source.as_ref().unwrap(), &destination.as_ref().unwrap());
                match result {
                    Ok(_) => println!("Recovered the secret!"),
                    Err(err) => {
                        let err = cmd.error(ErrorKind::ArgumentConflict, err);
                        err.exit();
                    }
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


