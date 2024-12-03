#!/usr/bin/env zsh

DIR="$(dirname $0)"

source "${DIR}/.venv/bin/activate"

maturin develop

pytest 
