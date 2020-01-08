# Changelog

All notable changes to this project will be documented in this file.

The format is based on Keep a Changelog and this project adheres to Semantic Versioning.

## [Unreleased]

- `LoopError` has an additional variant `IoSpawn` that indicates that spawning
  another task with the handler has failed.

- Connecting to neovim via tcp or a unix-socket (unix only) is now supported again

- `CallError` has a new variant `WrongType` to indicate that a message from
  neovim contained a value of the wrong type. Previously, the lib would panic
  in this case, now the user has the choice to handle it (or, more probably,
  log it properly and quit).

## 0.1.0 - 2020-02-01
- Initial release
