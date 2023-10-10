pub mod rdr {

    //! Iterator similar to the standard [fs::ReadDir] but recursive.
    //!
    //! **WARNING** Iteration over directories with many files and/or subdirectories can lead to panics
    //! due to **stack overflows** when is done in dev mode instead of release mode (`--release`).

    use std::{fs, io, path};

    /// Iterator similar to the standard [fs::ReadDir] but recursive.
    ///
    /// ## Iteration Example:
    /// ```
    /// use readdir_recursive::ReadDirRecursive;
    ///
    /// let rdr = ReadDirRecursive::new(".").unwrap();
    ///
    /// for entry_result in rdr {
    ///     let entry = entry_result.unwrap();
    ///     println!("Found file: '{:?}'", entry.path());
    /// }
    /// ```
    pub struct ReadDirRecursive {
        /// This field hods the [fs::ReadDir] instance that is currently being iterated.
        ///
        /// At the beginning, it holds the [fs::ReadDir] iterator of the root directory
        /// (given as param) but later, when all entries in the root where consumed (the
        /// iterator reached the end) it will be replaced by a new instances of [fs::ReadDir]
        /// as the main iteration continues visiting subdirectories of the root.
        pub read_dir: fs::ReadDir,
        /// Sub Directories are not visited immediately when found. Instead they're
        /// pushed onto a vector of pending directories/[entries][fs::DirEntry] (this field)
        /// and the iteration of the current directory continues with the next entry.
        /// Once that iteration is done, [ReadDirRecursive] will `pop` one entry from this stack,
        /// create an instance of [fs::ReadDir] and resume the iteration.
        pub pending_dirs: Vec<fs::DirEntry>,
    }

    impl ReadDirRecursive {
        /// Create a new instance of [ReadDirRecursive] for the given path.
        ///
        /// ```
        /// use readdir_recursive::ReadDirRecursive;
        ///
        /// let rdr = ReadDirRecursive::new("/some/path");
        /// ```
        pub fn new<P: AsRef<path::Path>>(path: P) -> io::Result<Self> {
            Ok(ReadDirRecursive {
                pending_dirs: vec![],
                read_dir: fs::read_dir(path)?,
            })
        }
    }

    // Implement Iterator for ReadDirRecursive
    impl Iterator for ReadDirRecursive {
        // our Item is the same as the wrapped iter
        type Item = io::Result<fs::DirEntry>;

        fn next(&mut self) -> Option<Self::Item> {
            match self.read_dir.next() {
                // entry found
                Some(Ok(entry)) => match entry.metadata() {
                    Ok(meta) => {
                        // if the entry is a directory we need to save it for later inspection
                        // and move to the next entry in the iterator..
                        if meta.is_dir() {
                            // push the directory onto the stack
                            self.pending_dirs.push(entry);

                            // move to the next entry
                            return self.next();
                        }

                        Some(Ok(entry))
                    }
                    // Error trying to obtain the entry's metadata.
                    // Since we won't know if the entry was a file or a directory we just return the
                    // error instead of the entry.
                    Err(e) => Some(Err(e)),
                },
                // Entry found but is an error. No special treatment, we just return the error as is
                Some(Err(err)) => Some(Err(err)),
                // The current `read_dir` iterator reached the end (there are no more
                // files/entries in this directory).
                None => {
                    // deal with the next pending directory (if any)
                    if let Some(dir_entry) = self.pending_dirs.pop() {
                        match fs::read_dir(dir_entry.path()) {
                            Ok(read_dir) => {
                                // throw away the consumed iterator and put the new one in his place
                                self.read_dir = read_dir;
                                return self.next();
                            }
                            Err(e) => return Some(Err(e)),
                        }
                    }
                    None
                }
            }
        }
    }

    /// Create an instance of [ReadDirRecursive] for the given path.
    /// The same can be achieved by passing the path to the `new` method of [ReadDirRecursive].
    ///
    /// Example:
    /// ```
    /// use readdir_recursive::ReadDirRecursive;
    ///
    /// let rdr = ReadDirRecursive::new("/some/path");
    /// ```
    pub fn read_dir_recursive<P: AsRef<path::Path>>(path: P) -> io::Result<ReadDirRecursive> {
        ReadDirRecursive::new(path)
    }
}
