use std::path::PathBuf;

use clap::{Parser, Subcommand, Args, ColorChoice, ArgAction};

use crate::utils::{is_qualified_path, shards_in_range, is_qualified_file};


#[derive(Parser)]
#[command(
    bin_name="hx", 
    version = "0.0.1", 
    color = ColorChoice::Always,
    about = "Split a file(s) into encrypted shards, no password required - secrecy preserved.", 
    long_about = None,
    subcommand_required = true,
    arg_required_else_help = true,
)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adds files to myapp
    Split(SplitArguments),
    Bind(BindArguments)
}

#[derive(Args)]
pub struct SplitArguments {
    #[arg(required = false, index = 1, action = ArgAction::Set, value_parser = is_qualified_file)]
    pub file: Option<PathBuf>,
    #[arg(
        required = true, 
        short = 's', 
        long = "shards",
        help = "Number of shards to split the secret into.",
        value_parser = shards_in_range,
        action = ArgAction::Set,
    )]
    pub shards: u8,
    #[arg(
        required = true, 
        short = 't', 
        long = "threshold",
        help = "Number of shards required to resurrect the original secret.",
        value_parser = shards_in_range,
        action = ArgAction::Set,
    )]
    pub threshold: u8,
    #[arg(
        required = false, 
        short = 'd', 
        long = "destination",
        help = "Where to place the horcruxes, a new directory will be created if specified one does not exist.",
        value_parser = is_qualified_path,
        default_value = ".",
        action = ArgAction::Set,
    )]
    pub destination: Option<PathBuf>,
}

#[derive(Args)]
pub struct BindArguments { 
    #[arg(
        required = false, 
        short = 's', 
        long = "source",
        help = "Source directory that contains the horcruxes.",
        value_parser = is_qualified_path,
        default_value = ".",
        action = ArgAction::Set,
    )]
    pub source: Option<PathBuf>,
    #[arg(
        required = false, 
        short = 'd', 
        long = "destination",
        help = "Where to place the recovered secret, a new directory will be created if specified one does not exist.",
        value_parser = is_qualified_path,
        default_value = ".",
        action = ArgAction::Set,
    )]
    pub destination: Option<PathBuf>
}

