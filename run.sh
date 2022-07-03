#!/bin/bash
set -e
#export datasource_conn_string=mysql://infradb:password@localhost:3306/infradb
export DATABASE_URL=mysql://infradb:password@localhost:3306/infradb
cargo fmt
cargo build --release
./target/release/rust_service
