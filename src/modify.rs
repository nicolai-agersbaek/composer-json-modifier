use std::io;

use crate::composer_json::ComposerJson;
use crate::modify_composer_json::{ModifyComposerJson,Require};
use crate::parse_handler::ParseFileType;

pub(crate) fn handle_modify(composer_json_file_name: &str, modify_file_name: &str, _print: &bool, dry_run: &bool) -> Result<(), io::Error> {
    //let mut c = ParseFileType::ComposerJson.parse(composer_json_file_name)?;
    //let m = ParseFileType::ModifyComposerJson.parse(modify_file_name)?;
    let c = ParseFileType::ComposerJson._handle_parse_and_return::<ComposerJson>(composer_json_file_name)?;
    let m = ParseFileType::ModifyComposerJson._handle_parse_and_return::<ModifyComposerJson>(modify_file_name)?;

    let d = remove(c, &m, &dry_run)?;

    Ok(())
}

fn remove(c: ComposerJson, m: &ModifyComposerJson, dry_run: &bool) -> Result<ComposerJson, io::Error> {
    return match &m.remove {
        Some(remove) => {
            return match &remove.require {
                Some(require) => remove_require(c, &require, dry_run),
                None => Ok(c)
            }
        },
        None => Ok(c)
    }
}

fn remove_require(c: ComposerJson, require: &Require, dry_run: &bool) -> Result<ComposerJson, io::Error> {
    for (k, v) in require.iter() {
        println!("[R] (k, v) = (\"{}\", \"{}\")", k, v);
    }

    for (k, v) in c.package_links.require.iter() {
        println!("[O] (k, v) = (\"{}\", \"{}\")", k, v);
    }

    Ok(c)
}