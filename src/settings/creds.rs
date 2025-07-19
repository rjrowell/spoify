use serde::Deserialize;
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use crate::app::App;

#[derive(Deserialize, Debug)]
struct Creds(HashMap<String, String>);

pub fn read_creds(app: &mut App) -> HashMap<String, String> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(".."); // Move up to the root of the Git repository
    path.push("spoify");
    path.push("configure");
    path.push("creds.yml");

    println!("DEBUG: Looking for creds file at: {:?}", path);
    println!("DEBUG: Path exists: {}", path.exists());
    println!("DEBUG: app.file_name = {:?}", app.file_name);
    println!("DEBUG: CARGO_MANIFEST_DIR = {:?}", env!("CARGO_MANIFEST_DIR"));

    let file = File::open(&path).expect("Unable to open creds file");
    let reader = BufReader::new(file);
    let Creds(creds) = serde_yaml::from_reader(reader).expect("Unable to parse creds from YAML");
    creds
}

pub fn set_creds(app: &mut App) {
    let creds = read_creds(app);

    if let Some(value_str) = creds.get("Client ID") {
        app.client_id = value_str.as_str().to_string();
    }

    if let Some(value_str) = creds.get("Client Secret") {
        app.client_secret = value_str.as_str().to_string();
    }
}
