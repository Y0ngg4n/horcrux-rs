use clap::{Arg, ArgAction, Command};

pub mod commands;

fn main() {
    let matches = Command::new("hx")
        .version("0.1") //Todo make this env variable
        .about("Make your command line a little more magical! Horcrux Split a file into encrypted parts, set a threshold to recover them.")
        .long_about("Make your command line a little more magical! Horcrux Split a file into encrypted parts, set a threshold to recover them.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Author")
        .subcommand(
            Command::new("split")
                .long_flag("split")
                .about("Splits a file into shards.")
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .conflicts_with("pipe")
                        .action(ArgAction::Append)
                )
                .arg(
                    Arg::new("shards")
                        .required(true)
                        .short('s')
                        .long("shards")
                        .help("desired number of file shards")
                        .action(ArgAction::Set)
                        .num_args(1..1),
                )
                .arg(
                    Arg::new("threshold")
                        .required(true)
                        .long("threshold")
                        .short('t')
                        .help("number of horcruxes required to recover the secret")
                        .action(ArgAction::Set)
                        .num_args(1..1),
                )
                .arg(
                    Arg::new("outdir")
                        .short('o')
                        .long("outdir")
                        .help("directory location of where to set put the horcruxes. a new directory will be created if none exists.")
                        .action(ArgAction::Set)
                        .num_args(1..1),
                )
                .arg(
                    Arg::new("pipe")
                        .required(true)
                        .long("pipe")
                        .short('p')
                        .help("read input from command line")
                        .action(ArgAction::Set)),
        )
        .subcommand(
            Command::new("bind")
                .long_flag("bind")
                .about("Recovers the secret from the")
                .arg(
                    Arg::new("directory")
                        .conflicts_with("pipe")
                        .long("directory")
                        .short('d')
                        .help("location of directory that contains the horcruxes")
                        .action(ArgAction::Append)
                )
                .arg(
                    Arg::new("pipe")
                        .required(true)
                        .long("pipe")
                        .short('p')
                        .conflicts_with("directory")
                        .help("optional argument to read input from command line")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("split", sync_matches)) => {
            if sync_matches.contains_id("search") {
                let packages: Vec<_> = sync_matches
                    .get_many::<String>("search")
                    .expect("contains_id")
                    .map(|s| s.as_str())
                    .collect();
                let values = packages.join(", ");
                println!("Searching for {values}...");
                return;
            }

            let packages: Vec<_> = sync_matches
                .get_many::<String>("package")
                .expect("is present")
                .map(|s| s.as_str())
                .collect();
            let values = packages.join(", ");

            if sync_matches.get_flag("info") {
                println!("Retrieving info for {values}...");
            } else {
                println!("Installing {values}...");
            }
        }
        Some(("bind", query_matches)) => {
            if let Some(packages) = query_matches.get_many::<String>("info") {
                let comma_sep = packages.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                println!("Retrieving info for {comma_sep}...");
            } else if let Some(queries) = query_matches.get_many::<String>("search") {
                let comma_sep = queries.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                println!("Searching Locally for {comma_sep}...");
            } else {
                println!("Displaying all locally installed packages...");
            }
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
