#!/bin/bash
set -e
export datasource_db=infradb
export datasource_user=infradb
export datasource_password=password
export datasource_conn_string=mysql://$datasource_user:$datasource_password@localhost:3306/$datasource_db

cargo fmt
cargo build --release
./target/release/rust_service
