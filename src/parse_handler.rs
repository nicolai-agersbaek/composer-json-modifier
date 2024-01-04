use std::fmt;
use std::io;

use serde_json::to_string_pretty;
use serde::{Deserialize, Serialize};

use crate::composer_json::ComposerJson;
use crate::modify_composer_json::ModifyComposerJson;
use crate::fs::get_file_contents;

pub(crate) trait ParseFile {
    fn parse_file_type() -> ParseFileType;
}

const COMPOSER_JSON_FILE_NAME : &str = "composer.json";
const MODIFY_COMPOSER_JSON_FILE_NAME : &str = "modify-composer.json";

pub(crate) enum ParseFileType {
    ComposerJson,
    ModifyComposerJson
}

impl fmt::Display for ParseFileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseFileType::ComposerJson => f.write_str(COMPOSER_JSON_FILE_NAME),
            ParseFileType::ModifyComposerJson => f.write_str(MODIFY_COMPOSER_JSON_FILE_NAME)
        }
    }
}

impl ParseFileType {
    pub(crate) fn handle_parse(&self, file_name: &str, print: &bool) -> () {
        match self {
            ParseFileType::ComposerJson => self._handle_parse::<ComposerJson>(file_name, print),
            ParseFileType::ModifyComposerJson => self._handle_parse::<ModifyComposerJson>(file_name, print)
        }
    }
    
    fn _handle_parse<S>(&self, file_name: &str, print: &bool) -> () 
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

    /*
    pub(crate) fn handle_parse_and_return<S>(&self, file_name: &str) -> io::Result<S> 
        where S: for<'a> Deserialize<'a>+Serialize
    {
        return match self {
            ParseFileType::ComposerJson => self._handle_parse_and_return::<ComposerJson>(file_name),
            ParseFileType::ModifyComposerJson => self._handle_parse_and_return::<ModifyComposerJson>(file_name)
        }
    }
    */
    
    pub(crate) fn _handle_parse_and_return<S>(&self, file_name: &str) -> io::Result<S> 
        where S: for<'a> Deserialize<'a>+Serialize
    {
        return match self.parse::<S>(&file_name) {
            Ok(parsed) => {
                Ok(parsed)
            },
            Err(e) => {
                Err(
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("error parsing {}: {}", file_name, e),
                    )
                )
            },
        }
    }

    pub(crate) fn parse<S>(&self, file_name: &str) -> io::Result<S>
        where S: for<'a> Deserialize<'a>+Serialize
    {
        let file_contents = get_file_contents(file_name)?;
        let result: S = serde_json::from_str(&file_contents)?;
    
        Ok(result)
    }
    
    fn print_parsed_json<S>(&self, parsed: S, file_name: &str, print: &bool) -> () 
            where S: for<'a> Deserialize<'a>+Serialize
        {
        if *print {
            let result = to_string_pretty(&parsed);
    
            match result {
                Ok(pretty) => { println!("\n{}:\n{}", file_name, pretty); }
                Err(e) => { eprintln!("error prettifying JSON: {}", e) }
            }
        }
    }
}
