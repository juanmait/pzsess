use std::fmt::Display;

/// Build an [Iterator] over the directories (excluding files) inside the sessions folder.
fn get_dirs_iter<P>(path: P) -> impl Iterator<Item = std::fs::DirEntry>
where
    P: AsRef<std::path::Path>,
{
    std::fs::read_dir(path)
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
fn get_sorted_list<P>(path: P) -> Vec<u128>
where
    P: AsRef<std::path::Path>,
{
    let dir_iter = get_dirs_iter(path);
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

/// Get the id of a save (timestamp) that correspond to the given `n`
/// session number.
/// 
/// This is mainly to support negative values for `n` so for example one
/// can obtain the id of the last save using `n = -1` or the second to
/// last id with `n = -2`.
fn get_n_timestamp<P>(n: i32, path: P) -> u128
where
    P: AsRef<std::path::Path>,
{
    let list = get_sorted_list(path);
    let list_len = list.len();
    let index = get_n_timestamp_index(n, list_len);
    let timestamp = list.get(index).expect(
        format!(
            "Index not found. No session was found in the given position {n} (computed index {index})"
        )
        .as_str()
    );
    println!(
        "Recovering session {} with timestamp {} (total sessions {}).",
        index + 1,
        timestamp,
        list_len
    );
    *timestamp
}

pub fn get_session_path<P>(n: i32, path: P) -> std::path::PathBuf
where
    P: AsRef<std::path::Path> + Display,
{
    let tm = get_n_timestamp(n, &path);
    let path_string = format!("{}/{}", &path, tm);
    std::path::Path::new(&path_string).to_owned()
}
