use crate::app::App;
use crate::structs::Key;
use std::env;
use std::path::PathBuf;
use std::process::Command;

pub fn open_configure(app: &mut App, key: &mut Key) {
    let mut yaml_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    yaml_path.push("..");
    yaml_path.push("spoify");
    yaml_path.push("configure");

    let yaml_file = yaml_path.clone();

    #[cfg(target_os = "windows")]
    let spawn_command = Command::new("cmd")
        .args(["/C", &format!("explorer {}", yaml_file.display())])
        .spawn();

    #[cfg(not(target_os = "windows"))]
    let spawn_command = Command::new("open").arg(yaml_path).spawn();
    #[cfg(not(target_os = "windows"))]
    let _temp = yaml_file;

    match spawn_command {
        Ok(_) => println!("Press {} to refresh", key.refresh_key),
        Err(e) => eprintln!("Failed to spawn terminal: {}", e),
    }
}
