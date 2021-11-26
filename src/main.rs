extern crate walkdir;
extern crate chrono;

use std::{fs};
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::path::Components;
use std::env;
use chrono::{DateTime, Local};
use std::process::{exit};
use std::fs::{remove_dir_all};
use subprocess::Exec;

use crate::config::*;
use std::ops::Sub;
use std::time::{Duration, SystemTime};

mod config;

const PASS : &'static str = "il faut viser la lune";

struct Processing<'a> {
    source_path: &'a str,
    target_path : &'a str,
    project_name : &'a str,
    package_name : &'a str,
    config : &'a Config,
}

impl Processing<'_> {

    fn is_forbidden( &self, path_name : &str) -> bool {
        let mut has_forbidden : bool = false;
        match  &self.config.source.exclude {
            Some(v) => {
                for forbidden in v {
                    has_forbidden = has_forbidden | path_name.contains(forbidden);
                }
                has_forbidden
            },
            None => false,
        }
    }

    /**
        // TODO handle the case where there is only files in the source path
    */
    fn copy_zipped_folders( &self )  {

        let src_path : &Path = Path::new(&self.source_path);
        let trg_path : PathBuf = Path::new(&self.target_path).join(&self.package_name).join(&self.project_name);

        let depth = src_path.components().count();

        //dbg!(src_path );

        for entry in fs::read_dir(src_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            let metadata = fs::metadata(&path).unwrap();
            let new_sub_path = extract_sub_path(&path, depth);

            let has_forbidden : bool = self.is_forbidden(new_sub_path.to_str().unwrap());

            if (! new_sub_path.to_str().unwrap().is_empty())  && (! has_forbidden)
            {

                if metadata.is_dir() {

                    //  "C:/Program Files/7-Zip/7z" a -p"xxxx"
                    //         "D:/work-doc-backups/2018.04.28 13.38/Visa&Factures 2016.zip"
                    //         "C:/Users/InSoft Design/Dropbox/Visa&Factures 2016"


                    let final_path = trg_path.join(new_sub_path);

                    println!("Create the Zip archive : {:?}", &final_path.to_str());

                    let _exit_status = Exec::cmd(&self.config.get_zip_tool())
                        .arg("a")
                        .arg("-p".to_owned() + PASS)
                        .arg(final_path.to_str().unwrap().to_owned() + ".zip")
                        .arg(&path)
                        .join();

                } else {
                    println!("Add a root file to the archive : {:?}", &path.as_os_str());

                    if !new_sub_path.to_str().unwrap().is_empty() {
                        let final_path = trg_path.join("root_files");

                        let _exit_status = Exec::cmd(&self.config.get_zip_tool())
                            .arg("a")
                            .arg("-p".to_owned() + PASS)
                            .arg(final_path.to_str().unwrap().to_owned() + ".zip")
                            .arg(&path)
                            .join();
                    }
                }
            } // if ! contains(...)

        }

    }


    /**
    Take all the subfolder of the <source_path> directory and duplicate them
    in the <target_path>/<package_name>/ directory.
*/
    fn create_folder_structure( &self ) {
        let src_path : &Path = Path::new(&self.source_path);
        let trg_path : PathBuf = Path::new(&self.target_path).join(&self.package_name).join(&self.project_name);

        let result = fs::create_dir_all(&trg_path);
        match  result  {
            Ok(_v) => println!("Created directory : [{}]", trg_path.to_str().unwrap()) ,
            Err(_e) => println!("Impossible to create the folder: [{}]", trg_path.to_str().unwrap()),
        }

        // Determine the depth of the <source_path> folder, including "root".
        let depth = src_path.components().count();

        // Go over all the sub folders.
        for entry in WalkDir::new(src_path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_dir()) {

            let new_sub_path = extract_sub_path(&entry.path(), depth);

            let has_forbidden : bool = self.is_forbidden(new_sub_path.to_str().unwrap());

            if (! new_sub_path.to_str().unwrap().is_empty())  && (! has_forbidden)
            {
                let final_path = trg_path.join(new_sub_path);

                let result = fs::create_dir_all(&final_path);

                match  result  {
                    Ok(_v) => println!("Created directory : [{}]", final_path.to_str().unwrap()) ,
                    Err(_e) => println!("Impossible to create the folder: [{}]", final_path.to_str().unwrap()),
                }
            }
        }

    }


    /**
    Take all the files of the <source_path> directory (recurse) and duplicate them
    in the <target_path>/<package_name>/ directory.
    */
    fn copy_files( &self ) {
        let src_path : &Path = Path::new(&self.source_path);
        let trg_path : PathBuf = Path::new(&self.target_path).join(&self.package_name).join(&self.project_name);

        // Determine the depth of the <source_path> folder, including "root".
        let depth = src_path.components().count();

        // Go over all the files.
        for entry in WalkDir::new(src_path)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir()) {

            let new_sub_path = extract_sub_path(entry.path(), depth);
            let has_forbidden : bool = self.is_forbidden(new_sub_path.to_str().unwrap());

            let too_big = match self.config.max_size {
                None => {false}
                Some(limit) => {
                    let metadata = fs::metadata(entry.path()).unwrap();
                    match &metadata.is_dir() {
                        true => {false}
                        false => {
                            let fsize = metadata.len();
                            //println!("File {:?} has size {}", entry.path(), fsize);
                            fsize >= limit * 1_000_000
                        }
                    }
                }
            };

            if too_big {
                println!("************* File is too big");
            }

            if (! new_sub_path.to_str().unwrap().is_empty())  && (! has_forbidden) && (!too_big)
            {
                let final_path = trg_path.join(new_sub_path);
                let new_path_name = final_path.to_str().unwrap();

                let _ = fs::copy(entry.path(), &final_path);

                println!("Copy file : [{}]", new_path_name);
            }
        }
    }


}

/**
    Removes the first <depth> folders from the <one_path> path.
    Ex : ( "/a/b/c/d", 2 ) -> ( "c/d" )
*/
fn extract_sub_path(one_path: &Path, depth: usize) -> &Path {
    let mut comps : Components = one_path.components();

    // Skip all the folders of the <env>.
    for _ in 1..depth+1 {
        comps.next();
    }

    let new_sub_path = comps.as_path();
    new_sub_path
}

const SECONDS_IN_DAY : u64 = 24 * 60 * 60;

/**
Loop the target package to delete the folders older than x days
*/
fn purge_package(    target_path : &str,
                     config : &Config)  {

    for entry in fs::read_dir(Path::new(&target_path)).unwrap() {

        let entry = entry.unwrap();
        let path = entry.path();

        let metadata = fs::metadata(&path).unwrap();

        let to_be_delete = SystemTime::now().sub(Duration::from_secs((config.target.purge_after as u64) * SECONDS_IN_DAY))
                            .ge(&metadata.created().unwrap());

        if to_be_delete {
            println!("Trying to remove {:?}", &path);
            remove_dir_all(&path).expect("Failed to remove target");
            println!("Done");
        }
    }


}

/**
cargo run -- -c "C:\Users\denis\wks-tools\cloud-backup\env\config\conf.yml"
*/
fn main() {

    let mut config_file = String::default();

    // Read the parameters
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{}", show_help());
        exit(45);
    } else {

        let option : &String = &args[1];
        match option.as_ref() {
            "-c" => config_file = String::from(&args[2] ),
            _ => println!("Wrong argument"),
        }

        // For now, we consider that the Windows style separator is replaced.

        dbg!(&config_file);

        if config_file.is_empty() {
            println!("-c <config_file> is required");
            exit(30);
        }
    }

    // Read the configuration file.
    let config = Config::new(&config_file);
    println!("{:?}", &config);

    let target_dir = config.get_target_path();
    let projects = config.get_source_path();

    let now: DateTime<Local> = Local::now();
    let package = now.format("%Y.%m.%d %H.%M.%S %a").to_string();

    for p in projects {
        let project_name = p.0;
        let source_dir = p.1;

        println!( "*** Prepare to backup project  {} : {} ***", &project_name, &source_dir);

        let process = Processing {
            source_path : &source_dir,
            target_path : &target_dir,
            project_name,
            package_name : &package,
            config : &config,
        };

        if config.compressed {
            let _ = &process.copy_zipped_folders();
        } else {
            // Copy the folder structure
            let _ = &process.create_folder_structure();
            let _ = &process.copy_files();
        }

    }

    println!( "*** Start the purge process in  {} ***", &target_dir);
    purge_package( &target_dir,  &config);

}

/**
    Return the help text.
*/
fn show_help() -> &'static str {

        "

    Simple Backup v0.9.0

    simple-backup -c <yaml-config-file>

        -c <yaml-config-file>   Yaml file to configure the current backup options Ex : \"/home/doka-file-tests/env\"

    simple-backup -h

        -h  Show this help file.
"

}