//#![cfg_attr(
//feature = "cargo-clippy",
//allow(unreadable_literal, decimal_literal_representation)
//)]

extern crate serde;
extern crate serde_yaml;

//extern crate unindent;

use serde::{Deserialize, Serialize};
use serde_yaml::{Value, Mapping};
use std::{fs};
use std::fmt::Debug;
use std::path::Path;


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub source: Source,
    pub target: Target,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Source {
    pub paths: Vec<Mapping>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Target {
    pub path: String,
}


pub fn read_config( config_file : &str ) -> Config {

    let config_path: &Path = Path::new(config_file );
    let data = fs::read_to_string(config_path).expect("Unable to read file");
    let config: Config = serde_yaml::from_str(&data).unwrap();

    config
}

//pub trait OpenConfig {
//
//    fn get_target_path( &self ) -> &str;
//}

impl Config {

    pub fn new( config_file : &str ) -> Config {
        let config_path: &Path = Path::new(config_file );
        let data = fs::read_to_string(config_path).expect("Unable to read file");
        let config: Config = serde_yaml::from_str(&data).unwrap();

        config
    }

    pub fn get_target_path( &self ) -> &str {
        let x = &self.target.path;

        x
    }

}
