use std::collections::HashMap;
use std::hash::Hash;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::composer_json::{AllowPlugins, PlatformConstraint};
use crate::{ParseFile,ParseFileType};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyComposerJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify: Option<ModifyConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<AddConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<RemoveConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<ReplaceConfig>,
}

impl ParseFile for ModifyComposerJson {
    fn parse_file_type() -> ParseFileType {
        ParseFileType::ModifyComposerJson
    }
}

// region <<- [ ModifyConfig ] ->>

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require: Option<Require>,

    #[serde(rename = "require-dev")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_dev: Option<Require>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<HashMap<String, String>>,
}

// endregion [ ModifyConfig ]

// region <<- [ AddConfig ] ->>

#[derive(Debug, Serialize, Deserialize)]
pub struct AddConfig {}

// endregion [ AddConfig ]

// region <<- [ RemoveConfig ] ->>

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoveConfig {}

// endregion [ RemoveConfig ]

// region <<- [ ReplaceConfig ] ->>

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplaceConfig {}

// endregion [ ReplaceConfig ]

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifierConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<HashMap<crate::composer_json::PlatformPackage, PlatformConstraint>>,

    #[serde(rename = "allow-plugins")]
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_plugins: Option<AllowPlugins>,
}

pub type Require = HashMap<PackagePattern, VersionConstraint>;

pub type VersionConstraint = String;

#[derive(Debug)]
pub struct PackagePattern {
    pattern: String,
    regex: Regex,
}

impl Hash for PackagePattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pattern.hash(state);
    }
}

impl Serialize for PackagePattern {
    fn serialize<S>(&self, serializer: S) -> Result<<S as serde::Serializer>::Ok, <S as serde::Serializer>::Error> where
        S: serde::Serializer {
        serializer.serialize_str(&self.pattern)
    }
}

#[test]
fn package_pattern_serialize() {
    let p = PackagePattern::new("foo/bar").unwrap();
    let s = serde_json::to_string(&p).unwrap();

    assert_eq!(s.to_string(), "\"foo/bar\"");
}

impl<'de> Deserialize<'de> for PackagePattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error> where
        D: serde::Deserializer<'de> {
        let pattern = String::deserialize(deserializer)?;

        PackagePattern::new(&pattern).map_err(serde::de::Error::custom)
    }
}

impl Eq for PackagePattern {}

impl PartialEq for PackagePattern {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
    }
}

impl Into<String> for PackagePattern {
    fn into(self) -> String {
        self.pattern
    }
}

#[cfg(test)]
mod tests {
    use crate::modify_composer_json::PackagePattern;
    use regex::Regex;

    macro_rules! package_pattern_to_string_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let p = PackagePattern::new(input).unwrap();
                let s: String = p.into();
            
                assert_eq!(s, expected);
            }
        )*
        }
    }
    
    package_pattern_to_string_tests! {
        package_pattern_to_string_empty: ("", ""),
        package_pattern_to_string_normal: ("foo/bar", "foo/bar"),
    }

    fn test_package_pattern_to_string(input: &str, expected: &str) {
        let p = PackagePattern::new(input).unwrap();
        let s: String = p.into();

        assert_eq!(s, expected);
    }

    #[test]
    fn package_pattern_to_string_tests() {
        let cases = vec!["", "foo/bar"];

        for case in cases.into_iter() {
            test_package_pattern_to_string(case, case)
        }
    }

    macro_rules! package_pattern_to_regex_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;

                test_package_pattern_to_regex(input, expected);
            }
        )*
        }
    }

    fn test_package_pattern_to_regex(input: &str, expected: &str) {
        let p = PackagePattern::new(input).unwrap();
        let actual: Regex = p.into();
    
        assert_eq!(expected.to_string(), actual.to_string());
    }
    
    package_pattern_to_regex_tests! {
        package_pattern_to_regex_empty: ("", "^$"),
        package_pattern_to_regex_normal: ("foo/bar", "^foo/bar$"),
        package_pattern_to_regex_wildcard: ("foo/*", "^foo/.*$"),
    }
}

impl Into<Regex> for PackagePattern {
    fn into(self) -> Regex {
        self.regex.clone()
    }
}

impl TryFrom<String> for PackagePattern {
    type Error = regex::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        PackagePattern::new(&value)
    }
}

impl TryFrom<&str> for PackagePattern {
    type Error = regex::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        PackagePattern::new(&value)
    }
}

impl PackagePattern {
    pub(crate) fn matches(&self, package: Self) -> bool {
        self.regex.is_match(&package.pattern)
    }

    pub(crate) fn new(pattern: &str) -> Result<PackagePattern, regex::Error> {
        let p = format!("^{}$", pattern.replace("*", ".*"));
        let regex = Regex::new(&p)?;
    
        Ok(PackagePattern { pattern: pattern.into(), regex })
    }
}
