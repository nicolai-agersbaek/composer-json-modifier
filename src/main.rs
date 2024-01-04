#![allow(dead_code)]

use std::fs;
use std::io;
use std::fmt;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use serde_json::to_string_pretty;
use serde::{Deserialize, Serialize};

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

pub(crate) trait ParseFile {
    fn parse_file_type() -> ParseFileType;
}

enum ParseFileType {
    ComposerJson,
    ModifyComposerJson
}

impl fmt::Display for ParseFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseFileType::ComposerJson => f.write_str("composer.json"),
            ParseFileType::ModifyComposerJson => f.write_str("modify-composer.json")
        }
    }
}

impl ParseFileType {
    fn handle_parse(&self, file_name: &str, print: &Option<bool>) -> () {
        match self {
            ParseFileType::ComposerJson => self.handle_parse2::<ComposerJson>(file_name, print),
            ParseFileType::ModifyComposerJson => self.handle_parse2::<ModifyComposerJson>(file_name, print)
        }
    }
    
    fn handle_parse2<S>(&self, file_name: &str, print: &Option<bool>) -> () 
        where S: for<'a> Deserialize<'a>+Serialize
    {
        match self.parse::<S>(&file_name) {
            Ok(parsed) => {
                println!("successfully parsed {} file: {}", self, file_name);
                self.print_parsed_json::<S>(parsed, file_name, print)
            }
            Err(e) => eprintln!("error parsing {}: {}", file_name, e),
        }
    }

    fn parse<S>(&self, file_name: &str) -> io::Result<S>
        where S: for<'a> Deserialize<'a>+Serialize
    {
        let file_path = get_file_path(file_name)?;
        let file_contents = fs::read_to_string(file_path)?;
        let result: S = serde_json::from_str(&file_contents)?;
    
        Ok(result)
    }
    
    fn print_parsed_json<S>(&self, parsed: S, file_name: &str, print: &Option<bool>) -> () 
            where S: for<'a> Deserialize<'a>+Serialize
        {
        if print.unwrap_or(false) {
            let result = to_string_pretty(&parsed);
    
            match result {
                Ok(pretty) => { println!("\n{}:\n{}", file_name, pretty); }
                Err(e) => { eprintln!("error prettifying JSON: {}", e) }
            }
        }
    }
}
