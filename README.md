Simple CLI IPC for scripting.

- `climsg` _(binary)_ -> Main CLI tool to interact.
- `climsgd` _(binary)_ -> Daemon binary.
- `climsgd-core` _(library)_ -> Rust lib to interact.

## CLI usage

```sh
climsg listen CHANNEL
```

In another shell, you can run:
```sh
climsg send CHANNEL MESSAGE
```

All listeners will print out lines in STDOUT, one line per message.
