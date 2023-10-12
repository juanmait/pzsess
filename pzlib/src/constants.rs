/// The path to the folder in which the official game sessions are saved.
pub const OFFICIAL_SESSIONS_FOLDER: &'static str = concat!(env!("HOME"), "/Zomboid/Saves");
/// The path to the folder from which pzload will look for previously saved sessions.
pub const PZLOAD_SESSIONS_FOLDER: &'static str = concat!(env!("HOME"), "/Zomboid/BSaves");
/// The path to the folder for temporary backups of the official game sessions.
pub const TEMP_SESS_BACKUP_FOLDER: &'static str = concat!(env!("HOME"), "/Zomboid/Saves_tmp");
