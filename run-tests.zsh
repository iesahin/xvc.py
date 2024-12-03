#!/usr/bin/env zsh

DIR="$(dirname $0)"

source "${DIR}/.venv/bin/activate"

maturin develop

pytest tests/test_xvc_root.py
