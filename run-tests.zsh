#!/usr/bin/env zsh

DIR="$(dirname $0)"

source "${DIR}/.venv/bin/activate"

maturin develop

pytest --forked # tests/test_xvc_file.py::test_file_track_symlink
