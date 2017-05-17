use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub cloudshark_api: String,
}

pub fn get_config(folder: PathBuf) -> Config {
    let mut config_folder = folder.as_path()
        .join(".config");

    if !config_folder.exists() {
        fs::create_dir(&config_folder);
    }
    
    config_folder = config_folder
        .join("pcap2cshark");

    if !config_folder.exists() {
        fs::create_dir(&config_folder);
    }
    
    let config_file = config_folder.join("pcapconfig.toml");

    if !config_file.exists() {
        create_config(config_file.to_path_buf());
    }
    
    config_file.to_path_buf();
    let mut file = File::open(&config_file).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    let config: Config = toml::from_str(&contents).unwrap();

    config
}

fn create_config(folder: PathBuf) {
    let mut input = String::new();

    println!("Provide your api key: ");
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read line");
    
    let mut file = File::create(folder).unwrap();
    file.write_all(
        &format!("cloudshark_api = '{}'", input.trim()).as_bytes()
    );
    file.sync_all();
}
