/// Path to the official's saves folder.
pub const OFFICIAL_SAVES_FOLDER: &'static str = concat!(env!("HOME"), "/Zomboid/Saves");
/// Path to the folder where [pzbackup](../../pzbackup/index.html) will store  [pzload](../../pzload/index.html) will look for previously saved sessions.
pub const PZBACKUP_SAVES_FOLDER: &'static str = concat!(env!("HOME"), "/Zomboid/BSaves");
/// Path for temporary backups of the current game save. Used by [pzload](../../pzload/index.html)
/// to store the current game save before loading other saved sessions. This is to be able to manually
/// recover saves that could have been mistakenly overridden by [pzload](../../pzload/index.html).
pub const TEMP_SESS_BACKUP_FOLDER: &'static str = concat!(env!("HOME"), "/Zomboid/Saves_tmp");
