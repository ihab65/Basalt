#[allow(deprecated)]

use std::io::{self, Read, Write};
use std::env;
use std::fs::{File, self};
use std::os::unix::prelude::OsStrExt;
use std::path::{Path, PathBuf};
use clap::{arg, Args};

#[derive(Args)]
pub struct PathToSet {
    #[arg(short, long)]
    path: String,
}

impl AsRef<Path> for PathToSet {
    fn as_ref(&self) -> &Path {
        Path::new(&self.path)
    }
}

pub fn set_path(PathToSet { path }: &PathToSet) {
    let save_file_location = env::home_dir().unwrap().join(".basalt");

    // println!("{:?}", save_file_location.join(".path"));
    match save_file_location.join(".path").exists() {
        true => {
            let mut existing_content = String::new();
            let mut file = File::open(save_file_location.join(".path")).unwrap();
            file.read_to_string(&mut existing_content).unwrap();

            if existing_content.is_empty() {
                file.write_all(path.as_bytes()).unwrap()
            };

            println!("save file already exists");
            println!("Default path set to : {}", existing_content);
        },
        false => {
            let path_to_save = PathBuf::new().join(path);
            if  path_to_save.exists() {  
                fs::create_dir(save_file_location.clone()).unwrap();
                fs::File::create(save_file_location.join(".path")).unwrap(); 
                fs::write(save_file_location.join(".path"),
                    path_to_save
                        .canonicalize().unwrap()
                        .as_path()
                        .as_os_str()
                        .as_bytes()
                ).unwrap();

                println!("created save file");
                println!("Default path set to : {}", path_to_save.canonicalize().unwrap().display());
            } else {
                eprintln!("ERROR: specified path doesn't exist")
            }
        }
    }
}

pub fn load_path() -> io::Result<String>{
    let save_file_location = env::home_dir().unwrap().join(".basalt/.path");
    let mut file = File::open(save_file_location)?;

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    Ok(content)
}


