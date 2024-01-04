#![allow(dead_code)]

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use serde_json::to_string_pretty;

use crate::composer_json::ComposerJson;
use crate::modify_composer_json::ModifyComposerJson;

mod composer_json;
mod modify_composer_json;

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

trait PathAsserts {
    fn assert_exists(&self) -> Result<&Path, io::Error>;
    fn assert_is_file(&self) -> Result<&Path, io::Error>;
}

impl PathAsserts for Path {
    fn assert_exists(&self) -> Result<&Path, io::Error> {
        if !self.exists() {
            return Err(
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("File not found: {}", self.display()),
                )
            );
        }

        Ok(self)
    }

    fn assert_is_file(&self) -> Result<&Path, io::Error> {
        if !self.is_file() {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("Path is not a file: {}", self.display()),
                )
            );
        }

        Ok(self)
    }
}

fn get_file_path(s: &str) -> Result<&Path, io::Error> {
    Path::new(s).assert_exists()?.assert_is_file()
}

fn handle(cmds: &Commands) -> io::Result<()> {
    match cmds {
        Commands::Parse (parse_commands) => handle_parse(parse_commands)
    }?;

    Ok(())
}

fn handle_parse(cmds: &ParseCommands) -> io::Result<()> {
    match cmds {
        ParseCommands::ComposerJson { file, print } => handle_parse_composer_json(file, print),
        ParseCommands::Modify { file, print } => handle_parse_modify(file, print)
    }

    Ok(())
}

fn handle_parse_composer_json(file: &str, print: &Option<bool>) -> () {
    match parse_composer_json_file(&file) {
        Ok(c) => {
            println!("successfully parsed composer.json file: {}", file);
            print_composer_json(c, print)
        }
        Err(e) => eprintln!("error in processing : {}", e),
    }
}

fn parse_composer_json_file(file_name: &str) -> io::Result<ComposerJson> {
    let file_path = get_file_path(file_name)?;
    let file_contents = fs::read_to_string(file_path)?;
    let composer_json: ComposerJson = serde_json::from_str(&file_contents)?;

    Ok(composer_json)
}

fn print_composer_json(c: ComposerJson, print: &Option<bool>) -> () {
    if print.unwrap_or(false) {
        let result = to_string_pretty(&c);

        match result {
            Ok(pretty) => { println!("\ncomposer.json:\n{}", pretty); }
            Err(e) => { eprintln!("error in processing : {}", e) }
        }
    }
}

fn handle_parse_modify(file: &str, print: &Option<bool>) -> () {
    match parse_modify_composer_json_file(&file) {
        Ok(c) => {
            println!("successfully parsed modify-composer.json file: {}", file);
            print_modify_composer_json(c, print)
        }
        Err(e) => eprintln!("error in processing : {}", e),
    }
}

fn parse_modify_composer_json_file(file_name: &str) -> io::Result<ModifyComposerJson> {
    let file_path = get_file_path(file_name)?;
    let file_contents = fs::read_to_string(file_path)?;
    let modify_composer_json: ModifyComposerJson = serde_json::from_str(&file_contents)?;

    Ok(modify_composer_json)
}

fn print_modify_composer_json(c: ModifyComposerJson, print: &Option<bool>) -> () {
    if print.unwrap_or(false) {
        let result = to_string_pretty(&c);

        match result {
            Ok(pretty) => { println!("\nmodify-composer.json:\n{}", pretty); }
            Err(e) => { eprintln!("error in processing : {}", e) }
        }
    }
}
