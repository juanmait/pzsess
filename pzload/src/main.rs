//! Load project zomboid sessions previously saved by [pzbackup](../pzbackup/index.html)

use clap::Parser;

mod util;

/// Arguments supported by the `pzload` CLI
#[derive(Parser, Debug)]
struct Args {
    /// Session number to load. Negative values are allowed so for example one
    /// can obtain the id of the last save using `-n=-1` or the second to
    /// last id with `-n=-2`.
    #[arg(short, default_value_t = -1)]
    n: i32,
}

fn main() {
    let args = Args::parse();

    println!("pzload: loading session {}", args.n);

    // Remove any previous temporary session.
    // Panic if the operation fails for any reason other than
    // directory not found
    println!("pzload: removing previous temporary backup..");
    if std::path::Path::new(pzlib::constants::TEMP_SESS_BACKUP_FOLDER).is_dir() {
        std::fs::remove_dir_all(pzlib::constants::TEMP_SESS_BACKUP_FOLDER).unwrap();
    }

    // Before proceeding to restore previous sessions we first backup the
    // current one in case something goes wrong with the restoration.
    println!("pzload: creating new temporary backup..");
    if std::path::Path::new(pzlib::constants::OFFICIAL_SESSIONS_FOLDER).is_dir() {
        std::fs::rename(
            pzlib::constants::OFFICIAL_SESSIONS_FOLDER,
            pzlib::constants::TEMP_SESS_BACKUP_FOLDER,
        )
        .unwrap();
    }

    println!("pzload: recovering session {}", args.n);
    for (absolute_src, absolute_dest) in pzlib::rdr::read_dir_recursive(util::get_session_path(
        args.n,
        pzlib::constants::PZLOAD_SESSIONS_FOLDER,
    ))
    .unwrap()
    .map(|r| r.unwrap())
    .map(|e| e.path())
    .map(|absolute_from| {
        let relative_dest = absolute_from
            .strip_prefix(pzlib::constants::PZLOAD_SESSIONS_FOLDER)
            .unwrap()
            .components()
            .skip(1)
            .collect::<std::path::PathBuf>();
        (absolute_from, relative_dest)
    })
    .map(|(absolute_from, relative_dest)| {
        let absolute_to =
            std::path::Path::new(pzlib::constants::OFFICIAL_SESSIONS_FOLDER).join(relative_dest);
        (absolute_from, absolute_to)
    }) {
        let dir = absolute_dest.parent().unwrap();
        // ensure that the destination directory for this path exists
        std::fs::create_dir_all(dir).unwrap();
        std::fs::copy(absolute_src, absolute_dest).unwrap();
    }

    println!("pzload: finished ok.");
}
