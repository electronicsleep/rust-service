#!/bin/bash
set -e

export MYSQL_USER=infradb
export MYSQL_PASSWORD=password
export MYSQL_HOST=localhost
export MYSQL_PORT=3306
export MYSQL_DBNAME=infradb

cargo fmt
cargo build --release
./target/release/rust_service
