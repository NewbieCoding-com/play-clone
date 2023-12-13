#!/usr/bin/env bash
set -eux

cargo clean
cargo build
export PYO3_CONFIG_FILE=$(pwd)/server/python/build/pyo3-build-config-file.txt
cargo build  --release  --no-default-features --features=prod
