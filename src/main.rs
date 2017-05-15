#![feature(plugin, custom_derive)]
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate toml;
extern crate rustc_serialize;
extern crate docopt;
extern crate curl;

pub mod cloudshark;
pub mod config;

use std::env;
use std::path::{Path, PathBuf};
use cloudshark::api;
use docopt::Docopt;

const USAGE: &'static str = "
pcap2cshark - Importing and exporting pcaps to CloudShark.

Usage:
  pcap2cshark --upload <file>...
  pcap2cshark (-h | --help)
  pcap2cshark --version

Options:
  -h --help     Show this screen.
  --version     Show version.
  --upload      Send a pcap to cloudshark.
  --download    Get a pcap from cloudshark.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_upload: bool,
    flag_download: bool,
    arg_file: Vec<String>,
    arg_cid: Vec<String>
}

fn get_app_dir() -> PathBuf {
    let dir: PathBuf = match env::home_dir() {
        Some(path) => PathBuf::from(path),
        None => PathBuf::from(""),
    };
    dir
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.flag_upload {
        for file in args.arg_file {
            let file_path = Path::new(&file);
            if !file_path.exists() {
                println!("File {} not found.", file);
                continue;
            }
            if file_path.extension().is_none() {
                println!("File {} isn't a pcap.", file);
                continue;
            }
            println!("File {} found.", file);
            let config_folder: PathBuf = get_app_dir();
            let config = config::get_config(config_folder);
            api::upload(file.clone(), config);
        }
    }

    if args.flag_download {
        println!("not implemented");
    }
}
