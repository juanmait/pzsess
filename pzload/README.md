# pzload

CLI utility to load PZ sessions previously saved with [pzbackup](../pzbackup/).

**WARNING**: The current PZ session will be overridden/replaced by the loaded session.

```bash
# recover the last saved session
❯ pzload

# recover the second to last session
❯ pzload -n=-2

# print help
❯ pzload --help
```
