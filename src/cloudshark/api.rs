use std::io::Read;
use std::str;
use curl::easy::{Easy, Form};
use serde_json::{Value, Error};
use serde_json;
use config::Config;

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(default)]
    id: String,
    #[serde(default)]
    filename: String,
    #[serde(default)]
    exceptions: Vec<String>,
    #[serde(default = "success_code")]
    status: i32
}

fn success_code() -> i32 {
    200
}

pub fn upload(file: String, config: Config) {
    println!("Uploading {} to CloudShark...", file);

    let mut data = "this is the body".as_bytes();

    let mut dst = Vec::new();
    let mut easy = Easy::new();
    let mut form = Form::new();
    form.part("file").file(&file).add();
    easy.url(
        &format!("https://www.cloudshark.org/api/v1/{}/upload",
                 config.cloudshark_api)
    ).unwrap();
    easy.httppost(form).unwrap();
    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        let json: Response = serde_json::from_str(
            &str::from_utf8(&data).unwrap()
        ).unwrap();

        if json.status != 200 {
            println!(
                "Error {}: {}",
                json.status,
                json.exceptions.join(", ")
            );
        } else {
            println!("{}: {}", json.filename, json.id);
            println!(
                "capture link: https://www.cloudshark.org/captures/{}",
                json.id
            );
        }

        dst.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    match transfer.perform() {
        Ok(_) => {},
        Err(v) => { println!("{}", v); }
    };
}
