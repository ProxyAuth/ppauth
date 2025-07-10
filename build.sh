#!/usr/bin/env bash
set -e

versions=("3.7.17" "3.8.18" "3.9.18" "3.10.13" "3.11.9" "3.12.3" "3.13.3")

for v in "${versions[@]}"; do
    echo "Python $v"

    PYTHON="$HOME/.pyenv/versions/$v/bin/python"
    MATURIN="$HOME/.pyenv/versions/$v/bin/maturin"

    if [ ! -x "$PYTHON" ]; then
        echo "❌ Python $v not found at $PYTHON"
        continue
    fi

    echo "Using Python: $PYTHON"

    "$PYTHON" -m pip install --upgrade pip maturin

    if [ ! -x "$MATURIN" ]; then
        echo "Error: maturin not found at $MATURIN"
        continue
    fi

    if [[ "$v" == "3.13."* ]]; then
        echo "Python 3.13 detected"
        PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 "$MATURIN" build --release -o target/wheels/ --interpreter "$PYTHON"
    else
        "$MATURIN" build --release -o target/wheels/ --interpreter "$PYTHON"
    fi
done
