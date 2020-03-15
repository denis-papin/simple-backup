extern crate walkdir;
extern crate chrono;

use std::fs;
use std::path::Path;
use std::path::PathBuf;
use walkdir::WalkDir;
use std::path::Components;
use std::env;
use chrono::{DateTime, Local};
use std::process::{exit};
use std::fs::File;
use subprocess::Exec;

use crate::config::*;


struct Processing<'a> {
    source_path: &'a str,
    target_path : &'a str,
    project_name : &'a str,
    package_name : &'a str,
    config : &'a Config,
}

impl Processing<'_> {
    /**
// TODO handle the case where there is only files in the source path
*/
    fn copy_zipped_folders( &self )  {

        use std::fs::File;

        let src_path : &Path = Path::new(&self.source_path);
        let trg_path : PathBuf = Path::new(&self.target_path).join(&self.package_name).join(&self.project_name);

        // let current_dir = env::current_dir().unwrap();

        let depth = src_path.components().count();

        dbg!(src_path );

        for entry in fs::read_dir(src_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            let metadata = fs::metadata(&path).unwrap();
            //let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();

            // dbg!("-----");
            dbg!("-----", &path);


            let new_sub_path = extract_sub_path(&path, depth);

            if metadata.is_dir() {
                use std::fs;


                dbg!(&path);



//          "C:/Program Files/7-Zip/7z" a -p"xxxx"
//                          "D:/work-doc-backups/2018.04.28 13.38/Visa&Factures 2016.zip"
//                          "C:/Users/InSoft Design/Dropbox/Visa&Factures 2016"

                if ! new_sub_path.to_str().unwrap().is_empty() {
                    let final_path = trg_path.join(new_sub_path);

                    let exit_status = Exec::cmd(&self.config.get_zip_tool())
                        .arg("a")
                        .arg("-p\"x\"")
                        .arg(final_path.to_str().unwrap().to_owned() + ".7z")
                        .arg(&path)
                        .join();

                    dbg!(exit_status);

                }

            } else {
                println!("THIS IS A FILE {:?}", &path );

                if ! new_sub_path.to_str().unwrap().is_empty() {
                    let final_path = trg_path.join("root_files");

                    let exit_status = Exec::cmd(&self.config.get_zip_tool())
                        .arg("a")
                        .arg("-p\"x\"")
                        .arg(final_path.to_str().unwrap().to_owned() + ".7z")
                        .arg(&path)
                        .join();
                }

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

/**
    Take all the subfolder of the <source_path> directory and duplicate them
    in the <target_path>/<package_name>/ directory.
*/
fn create_folder_structure(source_path: &str, target_path : &str, project_name : &str, package_name : &str ) {
    let src_path : &Path = Path::new(source_path);
    let trg_path : PathBuf = Path::new(target_path).join(package_name).join(project_name);

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
fn copy_files(source_path: &str, target_path : &str, project_name : &str, package_name : &str ) {
    let src_path : &Path = Path::new(source_path);
    let trg_path : PathBuf = Path::new(target_path).join(package_name).join(project_name);

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


///**
//// TODO handle the case where there is only files in the source path
//*/
//fn copy_zipped_folders( process : &Processing )  {
//
//    use std::fs::File;
//
//
//    let src_path : &Path = Path::new(&process.source_path);
//    // let current_dir = env::current_dir().unwrap();
//
//    let depth = src_path.components().count();
//
//    dbg!(src_path );
//
//    for entry in fs::read_dir(src_path).unwrap() {
//        let entry = entry.unwrap();
//        let path = entry.path();
//
//        let metadata = fs::metadata(&path).unwrap();
//        let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();
//
//        // dbg!("-----");
//        dbg!("-----", &path);
//        // dbg!(&metadata);
//
//        if metadata.is_dir() {
//            use std::fs;
////            use tar::Builder;
//
//            dbg!(&path);
//
//            let new_sub_path = extract_sub_path(&path, depth);
//
////          "C:/Program Files/7-Zip/7z" a -p"xxxx"
////          "D:/work-doc-backups/2018.04.28 13.38/Visa&Factures 2016.zip"
////          "C:/Users/InSoft Design/Dropbox/Visa&Factures 2016"
//
//            let exit_status = Exec::cmd(&process.config.get_zip_tool())
//                .arg("a")
//                .arg("-p\"x\"")
//                .arg(new_sub_path.to_str().unwrap().to_owned() + ".7z")
//                .arg(&path)
//                .join();
//
//
////          let tar_gz = File::create(new_sub_path.to_str().unwrap().to_owned() + ".tar.gz").unwrap();
////          let enc = GzEncoder::new(tar_gz, Compression::default());
////          let mut tar = tar::Builder::new(enc);
//
////            dbg!(&new_sub_path);
////
////
////            let ret = tar.append_dir_all(".", &path);
//
//            dbg!(exit_status);
//        }
//    }
//
//}



mod config;


/**
cargo run -- -c "C:\Users\denis\wks-tools\simple-backup\env\config\conf.yml"
*/
fn main() {

    //let mut cf = String::default();
    let mut config_file = String::default();

    // Read the parameters
    let args: Vec<String> = env::args().collect();

    // dbg!(&args);

    if args.len() < 2 {
        println!("{}", show_help());
        exit(45);
    } else {

        //for i in 1..=2 {
            let i = 1;
            let option_index = i * 2 - 1;
            let value_index = i * 2;

            let option : &String = &args[option_index];

            //dbg!(&option);

            match option.as_ref() {
                "-c" => config_file = String::from(&args[value_index] ),
                _ => println!("Wrong argument"),
            }

            dbg!(&config_file);

        //}

        // For now, we consider that the Windows style separator is replaced.
        config_file = config_file.replace("\\", "/");

        dbg!(&config_file);

        if config_file.is_empty() {
            println!("-c <config_file> is required");
            exit(30);
        }
    }

    //use std::path::Components;



    // Read the configuration file.
    let config = Config::new(&config_file);
    let target_dir = config.get_target_path();
    let projects = config.get_source_path();

    println!( ">>>>>>>>>>>> {:?}", &config);

    let now: DateTime<Local> = Local::now();
    let package = now.format("%Y.%m.%d %H.%M.%S %a").to_string();

    for p in projects {
        let project_name = p.0;
        let source_dir = p.1;

        let process = Processing {
            source_path : &source_dir,
            target_path : &target_dir,
            project_name,
            package_name : &package,
            config : &config,
        };

        &process.copy_zipped_folders();

        //copy_zipped_folders(&process);
        // copy_zipped_folders(&source_dir, &target_dir, project_name, &package, &config);

        /*
        // Copy the folder structure
        create_folder_structure(&source_dir, &target_dir, project_name, &package);

        copy_files(&source_dir, &target_dir, project_name, &package);
        */
    }
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