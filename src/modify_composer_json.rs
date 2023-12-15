use std::collections::HashMap;
use std::hash::Hash;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::composer_json::{AllowPlugins, PlatformConstraint};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyComposerJson {
    pub modify: ModifyConfig,
    pub add: AddConfig,
    pub remove: RemoveConfig,
    pub replace: ReplaceConfig,
}

// region <<- [ ModifyConfig ] ->>

#[derive(Debug, Serialize, Deserialize)]
pub struct ModifyConfig {
    pub require: Option<Require>,

    #[serde(rename = "require-dev")]
    pub require_dev: Option<Require>,

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
    pub platform: Option<HashMap<crate::composer_json::PlatformPackage, PlatformConstraint>>,

    #[serde(rename = "allow-plugins")]
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

impl<'de> Deserialize<'de> for PackagePattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error> where
        D: serde::Deserializer<'de> {
        let pattern = String::deserialize(deserializer)?;

        parse_package_pattern(&pattern).map_err(serde::de::Error::custom)
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

impl Into<Regex> for PackagePattern {
    fn into(self) -> Regex {
        self.regex.clone()
    }
}

impl TryFrom<String> for PackagePattern {
    type Error = regex::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        parse_package_pattern(&value)
    }
}

impl TryFrom<&str> for PackagePattern {
    type Error = regex::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_package_pattern(&value)
    }
}

fn parse_package_pattern(pattern: &str) -> Result<PackagePattern, regex::Error> {
    let p = format!("^{}$", pattern.replace("*", ".*"));
    let regex = Regex::new(&p)?;

    Ok(PackagePattern { pattern: pattern.into(), regex })
}

impl PackagePattern {
    pub(crate) fn matches(&self, package: Self) -> bool {
        self.regex.is_match(&package.pattern)
    }
}
