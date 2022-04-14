# Rust Service

Example microservice in Rust and Actix V4

##### Run
```
bash start.sh
```

##### Run Database
```
git clone https://github.com/electronicsleep/mysql-docker-test.git && cd mysql-docker-test
bash run
```

##### MySQL connection string
```
export datasource_conn_string=mysql://infradb:password@localhost:3306/infradb
```

#### Tests
```
bash src/tests/test-post.sh
```

https://www.rust-lang.org

https://actix.rs

https://docs.rs/mysql/21.0.1/mysql/
