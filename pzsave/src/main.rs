//! pzsave
//! ======
//!
//! TODO
//! -----
//! -   `pzsave -o` `--override-last` command to override the last saved session (delete last one and create a new one).
//!

fn generate_timestamp_string() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}

fn main() {
    let timestamp = generate_timestamp_string();
    println!("Saving PZ session into /{timestamp}/ ...");
    let rdr = pzlib::rdr::read_dir_recursive(pzlib::constants::OFFICIAL_SESSIONS_FOLDER)
        .expect("Error: Unable to read from the official saves folder.");

    for direntry_result in rdr {
        let from_path = direntry_result.unwrap().path();
        let from_str = from_path.to_str().unwrap();
        let to_part_str = &from_str[pzlib::constants::OFFICIAL_SESSIONS_FOLDER.len()..];
        let to_part_path: std::path::PathBuf = to_part_str.into();

        let to_dir_path = to_part_path.parent().unwrap();
        let to_dir_str = to_dir_path.to_str().unwrap();
        let file_name = to_part_path.file_name().unwrap().to_str().unwrap();

        let to_dir_all = [
            &pzlib::constants::PZLOAD_SESSIONS_FOLDER[..],
            "/",
            &timestamp[..],
            to_dir_str,
        ];

        let to_dir_all_string = to_dir_all.concat();
        let final_dest = [to_dir_all_string.as_str(), file_name].join("/");

        std::fs::create_dir_all(&to_dir_all_string).unwrap();
        std::fs::copy(from_path, final_dest).unwrap();
    }

    println!("PZ session saved.");
}
