//! TODOS:
//! -   imprimir mensaje el final que diga la cantidad de "saves" encontrados
//! -   permitir obtener una partida especificando números como `0`, `-1`

use std::{fs, path};

const SAVED_GAMES_FOLDER: &'static str = concat!(env!("HOME"), "/Zomboid/Saves");
const SAVES_TMP: &'static str = concat!(env!("HOME"), "/Zomboid/Saves_tmp");
const BACKUP_GAMES_FOLDER: &'static str = concat!(env!("HOME"), "/Zomboid/BSaves");

fn get_dirs_iter() -> impl Iterator<Item = std::fs::DirEntry> {
    std::fs::read_dir(BACKUP_GAMES_FOLDER)
        .unwrap()
        .map(|r| r.unwrap())
        .filter(|d| d.metadata().unwrap().is_dir())
}

/// Map from [std::fs::DirEntry] to [std::path::PathBuf]
fn into_paths_iter(
    iter: impl Iterator<Item = std::fs::DirEntry>,
) -> impl Iterator<Item = std::path::PathBuf> {
    iter.map(|e| e.path())
}

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

fn iter_to_vec(iter: impl Iterator<Item = u128>) -> Vec<u128> {
    iter.collect()
}

fn get_sorted_list() -> Vec<u128> {
    let dir_iter = get_dirs_iter();
    let path_iter = into_paths_iter(dir_iter);
    let times_iter = into_u128_timestamp(path_iter);
    let mut vec = iter_to_vec(times_iter);
    vec.sort();
    vec
}

fn get_newest_timestamp() -> u128 {
    get_sorted_list().pop().unwrap()
}

fn get_newest_path() -> std::path::PathBuf {
    let tm = get_newest_timestamp();
    let path_string = format!("{}/{}", BACKUP_GAMES_FOLDER.to_owned(), tm);
    std::path::Path::new(&path_string).to_owned()
}

fn main() {
    if std::path::Path::new(SAVES_TMP).is_dir() {
        std::fs::remove_dir_all(SAVES_TMP).unwrap();
    }

    fs::rename(SAVED_GAMES_FOLDER, SAVES_TMP).unwrap();

    for (absolute_from, absolute_to) in pzload::rdr::read_dir_recursive(get_newest_path())
        .unwrap()
        .map(|r| r.unwrap())
        .map(|e| e.path())
        .map(|absolute_from| {
            let relative_dest = absolute_from
                .strip_prefix(BACKUP_GAMES_FOLDER)
                .unwrap()
                .components()
                .skip(1)
                .collect::<path::PathBuf>();
            (absolute_from, relative_dest)
        })
        .map(|(absolute_from, relative_dest)| {
            let absolute_to = std::path::Path::new(SAVED_GAMES_FOLDER).join(relative_dest);
            (absolute_from, absolute_to)
        })
    {
        let dir = absolute_to.parent().unwrap();
        fs::create_dir_all(dir).unwrap();
        fs::copy(absolute_from, absolute_to).unwrap();
    }

    println!("Done.");
}
