use std::path::PathBuf;
use clap::{Parser, Subcommand, Args, ArgAction};
use crate::utils::{is_qualified_path, shards_in_range, is_qualified_file};

pub static BANNER: &str = " ██░ ██  ▒█████   ██▀███   ▄████▄   ██▀███   █    ██   ██████ ▄▄▄█████▓
▓██░ ██▒▒██▒  ██▒▓██ ▒ ██▒▒██▀ ▀█  ▓██ ▒ ██▒ ██  ▓██▒▒██    ▒ ▓  ██▒ ▓▒
▒██▀▀██░▒██░  ██▒▓██ ░▄█ ▒▒▓█    ▄ ▓██ ░▄█ ▒▓██  ▒██░░ ▓██▄   ▒ ▓██░ ▒░
░▓█ ░██ ▒██   ██░▒██▀▀█▄  ▒▓▓▄ ▄██▒▒██▀▀█▄  ▓▓█  ░██░  ▒   ██▒░ ▓██▓ ░ 
░▓█▒░██▓░ ████▓▒░░██▓ ▒██▒▒ ▓███▀ ░░██▓ ▒██▒▒▒█████▓ ▒██████▒▒  ▒██▒ ░ 
 ▒ ░░▒░▒░ ▒░▒░▒░ ░ ▒▓ ░▒▓░░ ░▒ ▒  ░░ ▒▓ ░▒▓░░▒▓▒ ▒ ▒ ▒ ▒▓▒ ▒ ░  ▒ ░░   
 ▒ ░▒░ ░  ░ ▒ ▒░   ░▒ ░ ▒░  ░  ▒     ░▒ ░ ▒░░░▒░ ░ ░ ░ ░▒  ░ ░    ░    
 ░  ░░ ░░ ░ ░ ▒    ░░   ░ ░          ░░   ░  ░░░ ░ ░ ░  ░  ░    ░      
 ░  ░  ░    ░ ░     ░     ░ ░         ░        ░           ░           
                          ░                                            ";


#[derive(Parser)]
#[command(
    name="horcrust",
    bin_name="horcrust", 
    version = option_env!("CARGO_PKG_VERSION").unwrap_or("dev"),
    about = "Split a file into encrypted shards, no password required - secrecy preserved.", 
    long_about = "Horcrust is an encryption program that splits a file into encrypted shards. Users can set a threshold, defining the number required for secret recovery, eliminating the need for passwords. Horcrust supports piped input. For example, you can use it like this: cat secret.txt | hx split -s 3 -t 2.",
    subcommand_required = true,
    arg_required_else_help = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(about = "Split a file into desired shards and threshold.")]
    Split(SplitArguments),
    #[command(about = "Recover the secret.")]
    Bind(BindArguments)
}

#[derive(Args)]
pub struct SplitArguments {
    #[arg(
        required = false, 
        index = 1, 
        help = "The secret to split",
        value_parser = is_qualified_file,
        action = ArgAction::Set,
    )]
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
        required = true, 
        index = 1,
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
        help = "Where to place the recovered secret, a new directory will be created if the provided one doesn't exist.",
        value_parser = is_qualified_path,
        default_value = ".",
        action = ArgAction::Set,
    )]
    pub destination: Option<PathBuf>
}

