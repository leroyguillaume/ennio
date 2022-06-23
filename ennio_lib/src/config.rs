use jsonschema::JSONSchema;
use log::{error, info};
use serde_json;
use serde_yaml;
use std::{
    fmt::{self, Display, Formatter},
    fs,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Config {}

impl Config {
    pub fn load(filepath: &str) -> Result<Self, LoadingError> {
        info!("Loading configuration from {}", filepath);
        let file_content = fs::read_to_string(filepath).map_err(|err| {
            let err = LoadingError::Reading(err.to_string());
            error!("Unable to load configuration: {}", err);
            err
        })?;
        let json = serde_yaml::from_str::<serde_json::Value>(&file_content).map_err(|err| {
            let err = LoadingError::Parsing(err.to_string());
            error!("Unable to load configuration: {}", err);
            err
        })?;
        let schema_json = String::from_utf8_lossy(include_bytes!("../resources/ennio.schema.json"));
        let schema = serde_json::from_str::<serde_json::Value>(&schema_json).unwrap();
        let schema = JSONSchema::compile(&schema).unwrap();
        schema.validate(&json).map_err(|errs| {
            let msgs: Vec<String> = errs.map(|err| err.to_string()).collect();
            let err = LoadingError::Validating(msgs);
            error!("Unable to load configuration: {}", err);
            err
        })?;
        Ok(Config {})
    }
}

#[derive(Debug)]
pub enum LoadingError {
    Reading(String),
    Parsing(String),
    Validating(Vec<String>),
}

impl Display for LoadingError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::Reading(err) => write!(f, "{}", err),
            Self::Parsing(err) => write!(f, "{}", err),
            Self::Validating(msgs) => write!(f, "{}", msgs.join(", ")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod config {
        use super::*;

        mod load {
            use super::*;

            #[test]
            fn should_return_reading_err() {
                match Config::load("./null") {
                    Ok(_) => panic!("should fail"),
                    Err(LoadingError::Reading(_)) => {}
                    Err(err) => panic!("{}", err),
                }
            }

            #[test]
            fn should_return_parsing_err() {
                match Config::load("./Cargo.toml") {
                    Ok(_) => panic!("should fail"),
                    Err(LoadingError::Parsing(_)) => {}
                    Err(err) => panic!("{}", err),
                }
            }

            #[test]
            fn should_return_config() {
                let expected = Config {};
                let cfg = Config::load("./test/ennio.yml").unwrap();
                assert_eq!(cfg, expected);
            }
        }
    }

    mod loading_error {
        use super::*;

        mod display {
            use super::*;

            macro_rules! test {
                ($name:ident, $value:expr) => {
                    #[test]
                    fn $name() {
                        let msg = "error";
                        let err = $value(String::from(msg));
                        assert_eq!(err.to_string(), msg);
                    }
                };
            }

            test!(parsing, LoadingError::Parsing);
            test!(reading, LoadingError::Reading);

            #[test]
            fn validating() {
                let msgs = vec![String::from("error1"), String::from("error2")];
                let err = LoadingError::Validating(msgs.clone());
                assert_eq!(err.to_string(), msgs.join(", "));
            }
        }
    }
}
