//! TODOS:
//! -   print message at the end about how many "saves" they are
//! -   `pzload -n -2` command to choose to load older saves (other than the last one) with numbers like `0`, `-1`

use std::{fs, path};

/// The path to the folder in which the official game sessions are saved.
const OFFICIAL_SESSIONS_FOLDER: &'static str = concat!(env!("HOME"), "/Zomboid/Saves");
/// The path to the folder for temporary backups of the official game sessions.
const TEMP_SESS_BACKUP_FOLDER: &'static str = concat!(env!("HOME"), "/Zomboid/Saves_tmp");
/// The path to the folder from which pzload will look for previously saved sessions.
const PZLOAD_SESSIONS_FOLDER: &'static str = concat!(env!("HOME"), "/Zomboid/BSaves");

/// Build an [Iterator] over the directories (excluding files) inside the sessions folder.
fn get_dirs_iter() -> impl Iterator<Item = std::fs::DirEntry> {
    std::fs::read_dir(PZLOAD_SESSIONS_FOLDER)
        .expect("Error trying to read_dir sessions folder.")
        .map(|r| r.expect("Error unwrapping DirEntry while iterating over sessions folder."))
        .filter(|d| {
            d.metadata()
                .expect(
                    "Error unwrapping metadata of DirEntry while iterating over sessions folder.",
                )
                .is_dir()
        })
}

/// Map from [std::fs::DirEntry] to [std::path::PathBuf]
fn into_paths_iter(
    iter: impl Iterator<Item = std::fs::DirEntry>,
) -> impl Iterator<Item = std::path::PathBuf> {
    iter.map(|e| e.path())
}

/// Map an iterator over items of type [std::path::PathBuf] into an iterator of items of type [u128].
/// Used to parse session folders (named with a timestamp in milliseconds since EPOCH) into the analog
/// [u128] number so later we can easily sort the collection of sessions from oldest to newest.
fn into_u128_timestamp(
    iter: impl Iterator<Item = std::path::PathBuf>,
) -> impl Iterator<Item = u128> {
    iter.map(|e| {
        e.file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .parse::<u128>()
            .unwrap()
    })
}

/// Collect an iterator over [u128]s into a vector ([Vec<u128>]).
fn iter_to_vec(iter: impl Iterator<Item = u128>) -> Vec<u128> {
    iter.collect()
}

/// Generate a sorted list of saved sessions from oldest to newest.
/// At this point the returned _saved sessions_ is a sorted vector of [u128]s
/// (timestamp in milliseconds since EPOCH in which the session was saved)
/// ordered from _oldest_ to _newest_.
fn get_sorted_list() -> Vec<u128> {
    let dir_iter = get_dirs_iter();
    let path_iter = into_paths_iter(dir_iter);
    let times_iter = into_u128_timestamp(path_iter);
    let mut vec = iter_to_vec(times_iter);
    vec.sort();
    vec
}

/// Find the real index of a save that correspond to the given `n` number given as parameter.
/// This is mainly to support negative values for the `n` parameter so for example one
/// can obtain the last save using `n = -1` or the second to last save with
/// `n = -2` or the first one with `n = 0` etc.
/// If `n` is a positive number this function will return the absolute value of
/// it casted to a `usize`.
fn get_n_timestamp_index(n: i32, list_len: usize) -> usize {
    let n_absolute: usize = n
        .abs()
        .try_into()
        .expect("Error at trying to cast the given `i32` into `usize`.");

    if n.is_negative() {
        return list_len - n_absolute;
    }

    n_absolute
}

/// Get the id (timestamp) of a save using the given `n` number.
/// This is mainly to support negative values for `n` so for example one
/// can obtain the id of the last save using `n = -1` or the second to
/// last id with `n = -2`.
fn get_n_timestamp(n: i32) -> u128 {
    let list = get_sorted_list();
    let list_len = list.len();
    let index = get_n_timestamp_index(n, list_len);
    let timestamp = list.get(index).expect(
        format!(
            "Index not found. No session was found in the given position {n} (computed index {index})"
        )
        .as_str()
    );
    println!(
        "\nRecovering save with timestamp {timestamp} at position {index} of a total of {list_len} entries)\n\n",
    );
    *timestamp
}

fn get_session_path(n: i32) -> std::path::PathBuf {
    let tm = get_n_timestamp(n);
    let path_string = format!("{}/{}", PZLOAD_SESSIONS_FOLDER.to_owned(), tm);
    std::path::Path::new(&path_string).to_owned()
}

fn main() {
    // Remove any previous temporary session.
    // We check for existence first to prevent [std::fs::remove_dir_all]
    // to fail if it doesn't...
    if std::path::Path::new(TEMP_SESS_BACKUP_FOLDER).is_dir() {
        std::fs::remove_dir_all(TEMP_SESS_BACKUP_FOLDER).unwrap();
    }

    // Before proceeding to restore previous sessions we first backup the
    // current one in case something goes wrong with the restoration.
    if std::path::Path::new(OFFICIAL_SESSIONS_FOLDER).is_dir() {
        fs::rename(OFFICIAL_SESSIONS_FOLDER, TEMP_SESS_BACKUP_FOLDER).unwrap();
    }

    for (absolute_from, absolute_to) in pzlib::rdr::read_dir_recursive(get_session_path(-1))
        .unwrap()
        .map(|r| r.unwrap())
        .map(|e| e.path())
        .map(|absolute_from| {
            let relative_dest = absolute_from
                .strip_prefix(PZLOAD_SESSIONS_FOLDER)
                .unwrap()
                .components()
                .skip(1)
                .collect::<path::PathBuf>();
            (absolute_from, relative_dest)
        })
        .map(|(absolute_from, relative_dest)| {
            let absolute_to = std::path::Path::new(OFFICIAL_SESSIONS_FOLDER).join(relative_dest);
            (absolute_from, absolute_to)
        })
    {
        let dir = absolute_to.parent().unwrap();
        // ensure that the destination directory for this path exists
        fs::create_dir_all(dir).unwrap();
        fs::copy(absolute_from, absolute_to).unwrap();
    }

    println!("Done.");
}
