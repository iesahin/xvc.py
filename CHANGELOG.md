## 0.7.1-alpha.5 (unreleased)

- Fix `Xvc(...)` not finding an already initialized repository. The bindings
  never looked one up, so every command outside the session that ran `init()`
  reported "This command requires Xvc repository". Note that Xvc can load one
  repository per process: constructing a second `Xvc` after one is loaded
  reuses nothing and reports the same error.
- Fix `help=True` and unparsable commands taking the interpreter down with
  them: the command line was parsed with `clap`'s `parse_from`, which prints
  and exits the process. They now return the message as the result, which is
  what the bindings already documented.
- Fix output lines being concatenated without separators. Each message is now
  terminated the way the `xvc` binary prints it, so commands emitting one
  message per item (`pipeline().step().list(names_only=True)`) are readable.
  Results that were single messages, `root()` among them, now end with a
  newline.
- Fix `file().untrack(restore_versions=...)` passing `--recheck-method` and
  `pipeline().step().output(image=...)` passing `--output-images`; neither flag
  exists. They now pass `--restore-versions` and `--output-image`.
- Accept `no_recheck` and `no_summary` as well as their hyphenated spellings in
  `file().bring()`, `file().copy()`, `file().mv()` and `file().list()`. The
  hyphenated ones cannot be written as Python keyword arguments at all, so
  those options could not be used.
- Move the integration tests to `xvc-test/python` in
  [xvc-mono](https://github.com/iesahin/xvc-mono), where they run against the
  Xvc commit that repository pins instead of whatever `iesahin/xvc` `main`
  holds at the time. New tests cover `check_ignore`, `run_xvc`, `version`, the
  `Xvc(...)` options, `file send`/`bring`/`share`, `storage new generic` and
  the cloud storage constructors, and `pipeline` export/import/dag.
- Update the Xvc dependencies to `0.7.1-alpha.5` at `f005b1e`, matching the
  version this package carries
- Update PyO3 to 0.29. It no longer supports Python 3.7, so the wheels are
  `abi3-py38` and `requires-python` is `>=3.8`
- Update the rest of the Rust dependencies (`cargo update`)
- Bump Xvc dependency pin to 0.7.1-alpha.5
- Add `storage().new_dropbox()` binding for the Dropbox storage backend (`xvc storage new dropbox`)

## 0.7.1

- Upgrade Xvc API to 0.7.1
- Refactor Rust bindings to reduce code duplication
- Add missing pipeline step remove command
- Update pipeline dependency flags to match Xvc 0.7.1

## 0.6.13

- See https://github.com/iesahin/xvc/blob/main/CHANGELOG.md#v0613-2024-12-30 for changes in Xvc

## 0.6.10

- Added sqlite dependency option to pipelines
- Fixed params pytest
