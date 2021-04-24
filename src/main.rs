use std::{env, fs};
use chrono::{DateTime, Utc};
use std::path::PathBuf;
use std::error::Error;

// fn main() -> Result<(), Err> {
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let dir_path = &args[1];
    let dir_path = PathBuf::from(dir_path);
    // println!("dir_path: {:?}", dir_path);
    let paths = fs::read_dir(dir_path.clone()).unwrap();
    // return
    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            continue
        }
        // println!("Name: {}", .display());
        let now: DateTime<Utc> = Utc::now();
        let date_full = now.format("%Y_%m_%d_%H_%M_%S");
        let date = now.format("%Y_%m_%d");
        let new_dir_path = dir_path.join(date.to_string());
        let _ = fs::create_dir(new_dir_path.clone());
        let ext = path.extension().unwrap().to_str().unwrap();
        let path2 = path.clone().with_extension("");
        let new_file_name = path2.file_name().unwrap().to_str().unwrap();
        let together = format!("{}{}.{}", new_file_name, date_full, ext);
        let new_path = new_dir_path.join(together);
        // println!("from: {:?}, to: {:?}", path, new_path);
        fs::rename(path, new_path)?;
    }
    Ok(())
}
