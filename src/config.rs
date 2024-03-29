//#![cfg_attr(
//feature = "cargo-clippy",
//allow(unreadable_literal, decimal_literal_representation)
//)]

extern crate serde;
extern crate serde_yaml;

//extern crate unindent;

use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping};
use std::{fs};
use std::fmt::Debug;
use std::path::Path;


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub source: Source,
    pub target: Target,

    #[serde(default)]
    pub max_size : Option<u64>, // size in MB

    #[serde(default)]
    pub compressed : bool,

    #[serde(default)]
    pub tools : Option<Tools>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Source {
    pub paths: Vec<Mapping>,
    #[serde(default)]
    pub exclude: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Target {
    pub path: String,
    #[serde(rename = "purge-after")]
    pub purge_after : u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Tools {
    #[serde(default)]
    pub zip: String,
}


impl Config {

    pub fn new( config_file : &str ) -> Config {
        let config_path: &Path = Path::new(config_file );
        let data = fs::read_to_string(config_path).expect("Unable to read file");
        let config: Config = serde_yaml::from_str(&data).unwrap();

        config
    }

    /**
    Return the list of the project to backup,
    It's a map such as { project_name, project_path }
    */
    pub fn get_source_path( &self ) -> Vec<(&str,&str)> {

        let mut projects : Vec<(&str,&str)> = vec![];

        let list_projects = &self.source.paths;

        for project in list_projects {

            for kv in  project {
                let project_name = kv.0;
                let project_path = kv.1;
                // TODO handle the unwraps...
                projects.push((project_name.as_str().unwrap(),project_path.as_str().unwrap() ) );
            }
        }
        projects
    }


    /**
    */
    pub fn get_target_path( &self ) -> &str {
        let x = &self.target.path;

        x
    }

    pub fn get_zip_tool( &self ) -> &str {

        match &self.tools {
            None => {
                println!("Missing zip tool");
                ""
            },
            Some(t) => &t.zip,
        }

    }

}
