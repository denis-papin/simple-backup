extern crate walkdir;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::path::Components;
use std::env;
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
    (new_sub_path)
}


/**
    Take all the subfolder of the <env> directory and duplicate them
    in the <doka_env>/<project_code>/ directory.
*/
fn create_folder_structure(env: &str, doka_env : &str, project_code : &str ) {
    let path_env : &Path = Path::new(env);
    let root_test_env : PathBuf = Path::new(doka_env).join(project_code);

    // Determine the depth of the <env> folder, including "root".
    let depth = path_env.components().count();

    // Go over all the sub folders.
    for entry in WalkDir::new(path_env)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir()) {

        let entry2 : walkdir::DirEntry = entry;
        let new_sub_path = extract_sub_path(entry2.path(), depth);

        if ! new_sub_path.to_str().unwrap().is_empty() {
            let final_path = root_test_env.join(new_sub_path);
            let new_path_name = final_path.to_str().unwrap();
            let result = fs::create_dir_all(&final_path);
            match  result  {
                Ok(_v) => println!("Created directory : [{}]", new_path_name) ,
                Err(_e) => println!("Impossible to create the folder"),
            }
        }
    }

}


/**
    Take all the files of the <env> directory (recurse) and duplicate them
    in the <doka_env>/<project_code>/ directory.
    If the file is xml, md, txt or properties, the ${DOKA_UT_ENV} inside the file is replaced with <doka_env>.
*/
fn copy_test_environment_files(env: &str, doka_env : &str, project_code : &str ) {
    let path_env : &Path = Path::new(env);
    let root_test_env : PathBuf = Path::new(doka_env).join(project_code);

    // Determine the depth of the <env> folder, including "root".
    let depth = path_env.components().count();

    // Go over all the files.
    for entry in WalkDir::new(path_env)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir()) {

        let new_sub_path = extract_sub_path(entry.path(), depth);

        if ! new_sub_path.to_str().unwrap().is_empty() {
            let final_path = root_test_env.join(new_sub_path);
            let new_path_name = final_path.to_str().unwrap();

            // Data replacement
            let ext1 = final_path.extension();

            let allow_replacement;
            match ext1 {
                None => {
                    allow_replacement = false;
                    },
                Some(&_) => {
                    let ext2 = ext1.unwrap().to_str();
                    //dbg!(ext2);
                    allow_replacement = match ext2 {
                        Some("xml") | Some("md") | Some("txt") | Some("properties") => true,
                        None | Some(&_) => false,
                    };
                },
            };

            let is_config_file = new_path_name.contains("config");

            // println!("{}", allow_replacement);

            if allow_replacement && is_config_file {
                let data = fs::read_to_string(entry.path()).expect("Unable to read file");
                let data_replaced = data.replace("{{DOKA_UT_ENV}}", doka_env)
                                                .replace("{{DOKA_ENV}}", doka_env)
                                                .replace("{{PROJECT_CODE}}", project_code);
                // println!("{}", data_replaced);

                // Create a text file from a string.
                fs::write(&final_path, data_replaced).expect("Unable to write file");
            } else {
                let _ = fs::copy(entry.path(), &final_path);
            }
            println!("Copy file : [{}]", new_path_name);
        }
    }
}


/**
    Return the help text.
*/
fn show_help() -> &'static str {
    (
"
    deploy-env -e <env>  -p <project-code> -u <doka-env>

        -e  <env>               Folder of the UT env templates, Ex : \"/home/doka-file-tests/env\"
        -p  <project-code>      Project code, Ex : \"file-api-tests\"
        -u  <doka-env>       Doka UT Environment root folder, Ex :  \"E:/doka-tests/\"

    deploy-env -h

        -h  Show this help file.
"
    )
}


/**
See the show_help() routine for the parameter description.
cargo run -- -e  "/Users/denis/Documents/doka-idea/r-script/normalized-ut-env/env"  -p  "super€project" -u  "/Users/denis/Documents/doka-idea/r-script/normalized-ut-env/env_copy"

This program allows to deployment of a UT environment.
*/
fn main() {

    let mut env : &String = &String::from("");
    let mut project_code : &String = &String::from("");
    let mut doka_test : &String = &String::from("");
    let new_doka_test;

    // Read the parameters
    let args: Vec<String> = env::args().collect();

    if args.len() < 7 {
        println!("{}", show_help());
        exit(45);
    } else {

        for i in 1..3+1 {
            let option_index = i * 2 - 1;
            let value_index = i * 2;

            let option : &String = &args[option_index];

            match option.as_ref() {
                "-e" => env = &args[value_index],
                "-p" => project_code = &args[value_index],
                "-u" => doka_test = &args[value_index],
                _ => println!("Wrong argument"),
            }

        }

        // For now, we consider that the Windows style separator is replaced.
        new_doka_test = doka_test.replace("\\", "/");

        if env.is_empty() {
            println!("-e <env> is required");
            exit(30);
        }

        if project_code.is_empty() {
            println!("-p <project-code> is required");
            exit(32);
        }

        if doka_test.is_empty() {
            println!("-u <doka-env> is required");
            exit(34);
        }
    }

    // copy the folder structure
    create_folder_structure(env,  &new_doka_test, project_code );

    // copy the files to the ah hoc folders
        // if the file is a config file .xml  or .properties, replace meta variables.
    copy_test_environment_files(env,  &new_doka_test, project_code);

}
