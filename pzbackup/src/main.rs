//! It creates recoverable backups of the entire saves folder.
//! Subsequent runs of `pzbackup` won't override previous saves but instead
//! create a new backup.
//!
//! Previous backups can be recovered using the [pzload](../pzload/index.html) crate.

use pzlib::constants::{OFFICIAL_SAVES_FOLDER, PZBACKUP_SAVES_FOLDER};
use pzlib::itfs::{EntryToPath, PathReRoot, ReadDirRecursive};

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

    let rdr = ReadDirRecursive::new(OFFICIAL_SAVES_FOLDER)
        .expect("pzbackup: Error: Unable to read from the official saves folder.");

    // the new root must include the generated timestamp
    let replace_by = [PZBACKUP_SAVES_FOLDER, "/", timestamp.to_string().as_str()].concat();

    let iter = PathReRoot {
        inner_iter: EntryToPath(
            // can't continue without the actual DirEntry item.
            rdr.map(|r| r.expect("pzbackup: Error: Failed to unwrap entry. Backup stopped.")),
        ),
        strip_prefix: OFFICIAL_SAVES_FOLDER,
        replace_by: replace_by.as_str(),
    }
    // as long as we use OFFICIAL_SAVES_FOLDER as `strip_prefix` while iterating
    // over it this failure should never happen...
    .map(|(original, result)| (original, result.expect("Invalid prefix or file path")));

    for (original, updated) in iter {
        // ensure that the destination folder exists
        match updated.parent() {
            Some(parent) => std::fs::create_dir_all(parent).unwrap(),
            // this case should never happen (?
            None => std::fs::create_dir_all(replace_by.as_str()).unwrap(),
        };

        // perform the backup for this file
        std::fs::copy(original, updated).unwrap();
    }

    println!("pzbackup: finished ok");
}
