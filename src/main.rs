extern crate walkdir;
extern crate chrono;


use std::fs;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::path::Components;
use std::env;
use chrono::{DateTime, Local};
use std::process::exit;

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

/**
    Take all the subfolder of the <source_path> directory and duplicate them
    in the <target_path>/<package_name>/ directory.
*/
fn create_folder_structure(source_path: &str, target_path : &str, project_name : &str, package_name : &str ) {
    let src_path : &Path = Path::new(source_path);
    let trg_path : PathBuf = Path::new(target_path).join(project_name).join(package_name);

    let result = fs::create_dir_all(&trg_path);
    match  result  {
        Ok(_v) => println!("Created directory : [{}]", trg_path.to_str().unwrap()) ,
        Err(_e) => println!("Impossible to create the folder: [{}]", trg_path.to_str().unwrap()),
    }

    dbg!(&trg_path);

    // Determine the depth of the <source_path> folder, including "root".
    let depth = src_path.components().count();

    // Go over all the sub folders.
    for entry in WalkDir::new(src_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir()) {

        let new_sub_path = extract_sub_path(&entry.path(), depth);

        if ! new_sub_path.to_str().unwrap().is_empty() {
            let final_path = trg_path.join(new_sub_path);
            //let new_path_name = final_path.to_str().unwrap();
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
    If the file is xml, md, txt or properties, the ${DOKA_UT_ENV} inside the file is replaced with <target_path>.
*/
fn copy_files(source_path: &str, target_path : &str, project_name : &str, package_name : &str ) {
    let src_path : &Path = Path::new(source_path);
    let trg_path : PathBuf = Path::new(target_path).join(project_name).join(package_name);

    // Determine the depth of the <source_path> folder, including "root".
    let depth = src_path.components().count();

    // Go over all the files.
    for entry in WalkDir::new(src_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir()) {

        let new_sub_path = extract_sub_path(entry.path(), depth);

        if ! new_sub_path.to_str().unwrap().is_empty() {
            let final_path = trg_path.join(new_sub_path);
            let new_path_name = final_path.to_str().unwrap();

            let _ = fs::copy(entry.path(), &final_path);

            println!("Copy file : [{}]", new_path_name);
        }
    }
}

mod config;


/**
cargo run -- -c "C:\Users\denis\wks-tools\simple-backup\env\config\conf.yml"
*/
fn main() {

    let mut config_file  = &String::new();
    let new_config_file;

    // Read the parameters
    let args: Vec<String> = env::args().collect();

    dbg!(&args);

    if args.len() < 2 {
        println!("{}", show_help());
        exit(45);
    } else {

        //for i in 1..=2 {
            let i = 1;
            let option_index = i * 2 - 1;
            let value_index = i * 2;

            let option : &String = &args[option_index];

            dbg!(&option);

            match option.as_ref() {
                "-c" => config_file = &args[value_index],
                _ => println!("Wrong argument"),
            }

            dbg!(&config_file);

        //}

        // For now, we consider that the Windows style separator is replaced.
        new_config_file = config_file.replace("\\", "/");

        dbg!(&new_config_file);

        if new_config_file.is_empty() {
            println!("-c <config_file> is required");
            exit(30);
        }
    }

    //use std::path::Components;

    use crate::config::*;

    let config = Config::new(&new_config_file);
    let target_dir = config.get_target_path();
    let projects = config.get_source_path();

    //println!( ">>>>>>>>>>>> {:?}", &config);

    let now: DateTime<Local> = Local::now();
    let package = now.format("%Y.%m.%d %H.%M.%S %a").to_string();

    for p in projects {
        let project_name = p.0;
        let source_dir = p.1;

        // Copy the folder structure
        create_folder_structure(&source_dir, &target_dir, project_name, &package);

        copy_files(&source_dir, &target_dir, project_name, &package);
    }
}

/**
    Return the help text.
*/
fn show_help() -> &'static str {

        "
    deploy-env -e <env>  -p <project-code> -u <doka-env>

        -e  <env>               Folder of the UT env templates, Ex : \"/home/doka-file-tests/env\"
        -p  <project-code>      Project code, Ex : \"file-api-tests\"
        -u  <doka-env>       Doka UT Environment root folder, Ex :  \"E:/doka-tests/\"

    deploy-env -h

        -h  Show this help file.
"

}