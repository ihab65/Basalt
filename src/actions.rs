use std::io::{self, Read, Write};
use std::fs::{File, self};
use std::path::Path;
use clap::{arg, Args};

const SAVE_FILE_PATH: &str = ".basalt/.path";

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

    match Path::new(SAVE_FILE_PATH).exists() {
        true => {
            let mut existing_content = String::new();
            let mut file = File::open(SAVE_FILE_PATH).unwrap();
            file.read_to_string(&mut existing_content).unwrap();

            if existing_content.is_empty() {
                file.write_all(path.as_bytes()).unwrap()
            }
        },
        false => {
            fs::create_dir(".basalt").unwrap();
            fs::File::create(".basalt/.path").unwrap();
            fs::write(SAVE_FILE_PATH, path).unwrap();
        }
    }
}

pub fn load_path() -> io::Result<String> {
    let mut file = File::open(SAVE_FILE_PATH)?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}


