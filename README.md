# Project Zomboid session backup/recover utilities

This repo contains some utilities to backup and then recover PZ sessions.

-   [pzsave](./pzsave): save current session.
-   [pzload](./pzload): load previously saved session.

## Generate and open RustDocs for the workspace

```bash
rm -rf ./target/doc && cargo doc --no-deps --open
```