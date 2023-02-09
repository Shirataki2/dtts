#!/bin/bash
set -euv
cargo install cargo-edit
cargo install cargo-watch
cargo install sqlx-cli --no-default-features --features native-tls,postgres
