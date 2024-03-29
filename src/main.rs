#![allow(dead_code)]

use std::io;
use std::path::{PathBuf};

use clap::{Parser, Subcommand};

use crate::composer_json::ComposerJson;
use crate::modify_composer_json::ModifyComposerJson;
use crate::parse_handler::ParseFile;
use crate::modify::handle_modify;

mod composer_json;
mod modify_composer_json;
mod parse_handler;
mod fs;
mod modify;

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
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Parse a composer.json or other JSON config file, e.g. modify-composer.json
    #[command(subcommand)]
    Parse(ParseCommands),

    /// Modify a composer.json file
    #[command(subcommand)]
    Modify(ModifyCommands)
}

#[derive(Subcommand, Debug)]
enum ParseCommands {
    /// Parse a composer.json file
    ComposerJson {
        /// Name of the composer.json file to parse
        #[arg(value_name="composer-json")]
        file: String,

        /// Print the parsed ComposerJson struct to stdout
        #[arg(short, long, default_value="false")]
        print: bool,
    },

    /// Parse a modify-composer.json file
    Modify {
        /// Name of the modify-composer.json file to parse
        #[arg(value_name="composer-json")]
        file: String,

        /// Print the parsed ModifyComposerJson struct to stdout
        #[arg(short, long, default_value="false")]
        print: bool,
    },
}

#[derive(Subcommand, Debug)]
enum ModifyCommands {
    /// Modify a composer.json file
    Run {
        /// Path to the composer.json file to modify
        #[arg(value_name="composer-json")]
        composer_json: String,

        /// Path to the modify-composer.json configuration file
        #[arg(value_name="modify")]
        modify: String,

        /// Print the modified ComposerJson struct to stdout
        #[arg(short, long, default_value="false")]
        print: bool,

        /// Whether to apply resulting changes to target file
        #[arg(short, long, default_value="false")]
        dry_run: bool,
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
        Commands::Parse (commands) => handle_parse_commands(commands),
        Commands::Modify (commands) => handle_modify_commands(commands)
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

fn handle_modify_commands(cmds: &ModifyCommands) -> io::Result<()> {
    match cmds {
        ModifyCommands::Run { composer_json, modify, print, dry_run } => {
            if *dry_run {
                println!("Modifying {} using {} (in dry-run mode)", composer_json, modify)
            } else {
                println!("Modifying {} using {}", composer_json, modify)
            }

            match handle_modify(composer_json, modify, print, dry_run) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("error parsing {}: {}", composer_json, e);

                    return Ok(());
                },
            };

            if *print {
                let pretty = "<placeholder>";

                println!("\n{}:\n{}", composer_json, pretty);
            }
        }
    }

    Ok(())
}