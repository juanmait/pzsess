//! It creates recoverable backups of the entire saves folder.
//! Subsequent runs of `pzbackup` won't override previous saves.
//!
//! Previous backups can be recovered using the [pzload](../pzload/index.html) crate.

/// Creates a string representation of a timestamp
/// in milliseconds since unix EPOCH. See [std::time::UNIX_EPOCH].
fn generate_timestamp_string() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}

fn main() {
    let timestamp = generate_timestamp_string();
    println!("pzbackup: saving session into /{timestamp}/ ...");
    let rdr = pzlib::rdr::read_dir_recursive(pzlib::constants::OFFICIAL_SESSIONS_FOLDER)
        .expect("pzbackup: Error: Unable to read from the official saves folder.");

    for direntry_result in rdr {
        let absolute_src_path = direntry_result.unwrap().path();
        let absolute_src_str = absolute_src_path.to_str().unwrap();
        let relative_src_str =
            &absolute_src_str[pzlib::constants::OFFICIAL_SESSIONS_FOLDER.len()..];
        let relative_src_path: std::path::PathBuf = relative_src_str.into();

        let relative_src_dir_path = relative_src_path.parent().unwrap();
        let relative_src_dir_str = relative_src_dir_path.to_str().unwrap();
        let relative_src_file_name = relative_src_path.file_name().unwrap().to_str().unwrap();

        let to_dir_all = [
            pzlib::constants::PZLOAD_SESSIONS_FOLDER,
            "/",
            timestamp.as_str(),
            relative_src_dir_str,
        ];

        let to_dir_all_string = to_dir_all.concat();
        let absolute_dest = [to_dir_all_string.as_str(), relative_src_file_name].join("/");

        std::fs::create_dir_all(&to_dir_all_string).unwrap();
        std::fs::copy(absolute_src_path, absolute_dest).unwrap();
    }

    println!("pzbackup: finished ok");
}
