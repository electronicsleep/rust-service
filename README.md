# Rust Service

Example microservice in Rust and Actix V4

https://crates.io/crates/actix-web

##### Cargo build and run
```
bash run.sh
```

##### Run Database
```
git clone https://github.com/electronicsleep/mysql-docker-test.git && \
cd mysql-docker-test && ./run.sh
```

##### MySQL connection string
```
export datasource_conn_string=mysql://infradb:password@localhost:3306/infradb
```

#### Tests
```
bash src/test/curl-tests.sh
```

https://www.rust-lang.org

https://actix.rs

https://docs.rs/mysql/21.0.1/mysql/

https://crates.io/crates/r2d2_mysql

https://crates.io/crates/r2d2
