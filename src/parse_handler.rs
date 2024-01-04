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

pub(crate) enum ParseFileType {
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
    pub(crate) fn handle_parse(&self, file_name: &str, print: &Option<bool>) -> () {
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
        let file_contents = get_file_contents(file_name)?;
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
