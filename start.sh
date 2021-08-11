#!/bin/bash
set -e
export datasource_conn_string=mysql://infradb:password@localhost:3306/infradb
cargo fmt
cargo run
