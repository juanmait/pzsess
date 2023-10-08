use std::{env, fs, path::PathBuf, time};

const SAVED_GAMES_FOLDER: &'static str = concat!(env!("HOME"), "/Zomboid/Saves");
const BACKUP_GAMES_FOLDER: &'static str = concat!(env!("HOME"), "/Zomboid/BSaves");

fn now() -> String {
    time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}

fn main() {
    let timestamp = now();
    println!("Writing backup to: /{timestamp}/ ...");
    let rdr = pzsave::rdr::read_dir_recursive(SAVED_GAMES_FOLDER).unwrap();

    for direntry_result in rdr {
        let from_path = direntry_result.unwrap().path();
        let from_str = from_path.to_str().unwrap();
        let to_part_str = &from_str[SAVED_GAMES_FOLDER.len()..];
        let to_part_path: PathBuf = to_part_str.into();

        let to_dir_path = to_part_path.parent().unwrap();
        let to_dir_str = to_dir_path.to_str().unwrap();
        let file_name = to_part_path.file_name().unwrap().to_str().unwrap();

        let to_dir_all = [&BACKUP_GAMES_FOLDER[..], "/", &timestamp[..], to_dir_str];

        let to_dir_all_string = to_dir_all.concat();
        let final_dest = [to_dir_all_string.as_str(), file_name].join("/");

        fs::create_dir_all(&to_dir_all_string).unwrap();
        fs::copy(from_path, final_dest).unwrap();
    }

    println!("Backup created!");
}
