#!/usr/bin/env bash

set -e

versions=("3.7.17" "3.8.18" "3.9.18" "3.10.13" "3.11.9" "3.12.3")

for v in "${versions[@]}"; do
    echo "Python $v"

    export PYO3_PYTHON="$HOME/.pyenv/versions/$v/bin/python"

    if [ ! -x "$PYO3_PYTHON" ]; then
        echo "Python $v not found at $PYO3_PYTHON"
        continue
    fi

    "$PYO3_PYTHON" -m pip install --upgrade pip maturin

    "$PYO3_PYTHON" -m maturin build --release -o dist
done

