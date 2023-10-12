//! pzload
//! ======
//!
//! TODO
//! ----
//!
//! -   Print message at the end about how many "saves" there are.

use clap::Parser;

mod util;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, default_value_t = -1)]
    n: i32,
}

fn main() {
    let args = Args::parse();

    // Remove any previous temporary session.
    // We check for existence first to prevent [std::fs::remove_dir_all]
    // to fail if it doesn't...
    if std::path::Path::new(pzlib::constants::TEMP_SESS_BACKUP_FOLDER).is_dir() {
        println!("Removing previous temporary backup..");
        std::fs::remove_dir_all(pzlib::constants::TEMP_SESS_BACKUP_FOLDER).unwrap();
    }

    // Before proceeding to restore previous sessions we first backup the
    // current one in case something goes wrong with the restoration.
    if std::path::Path::new(pzlib::constants::OFFICIAL_SESSIONS_FOLDER).is_dir() {
        println!("Creating backup of the current session ..");
        std::fs::rename(
            pzlib::constants::OFFICIAL_SESSIONS_FOLDER,
            pzlib::constants::TEMP_SESS_BACKUP_FOLDER,
        )
        .unwrap();
    }

    println!("Recovering last saved session ..");
    for (absolute_from, absolute_to) in pzlib::rdr::read_dir_recursive(util::get_session_path(
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
        let dir = absolute_to.parent().unwrap();
        // ensure that the destination directory for this path exists
        std::fs::create_dir_all(dir).unwrap();
        std::fs::copy(absolute_from, absolute_to).unwrap();
    }

    println!("Done.");
}
