# Project Zomboid session backup/recover utilities

This repo contains some CLI utilities to backup and then recover PZ sessions.

-   [pzsave](./pzsave): save current session.
-   [pzload](./pzload): load previously saved session.

## Generate and open RustDocs for the workspace

```bash
cargo doc --no-deps --open
```

## Build all the binaries

```bash
# run the build in the root of the workspace
cargo build --release
```
