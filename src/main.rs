#![allow(dead_code)]

use std::io;
use std::path::{PathBuf};

use clap::{Parser, Subcommand};

use crate::composer_json::ComposerJson;
use crate::modify_composer_json::ModifyComposerJson;
use crate::parse_handler::ParseFile;

mod composer_json;
mod modify_composer_json;
mod parse_handler;
mod fs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    //command: Option<ParseCommands>,
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(subcommand)]
    Parse(ParseCommands)
}

#[derive(Subcommand, Debug)]
enum ParseCommands {
    /// Parse a composer.json file
    ComposerJson {
        /// Name of the composer.json file to parse
        file: String,

        /// Print the parsed ComposerJson struct to stdout
        #[arg(short)]
        print: Option<bool>,
    },

    /// Parse a modify-composer.json file
    Modify {
        /// Name of the modify-composer.json file to parse
        file: String,

        /// Print the parsed ModifyComposerJson struct to stdout
        #[arg(short)]
        print: Option<bool>,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(c) => {
            match handle(c) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                }
            };
        }
        None => {}
    }
}

fn handle(cmds: &Commands) -> io::Result<()> {
    match cmds {
        Commands::Parse (parse_commands) => handle_parse_commands(parse_commands)
    }?;

    Ok(())
}

fn handle_parse_commands(cmds: &ParseCommands) -> io::Result<()> {
    match cmds {
        ParseCommands::ComposerJson { file, print } => ComposerJson::parse_file_type().handle_parse(file, print),
        ParseCommands::Modify { file, print } => ModifyComposerJson::parse_file_type().handle_parse(file, print)
    }

    Ok(())
}