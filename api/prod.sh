#!/bin/bash
sqlx database create
sqlx migrate run
cargo run --release --bin serve
