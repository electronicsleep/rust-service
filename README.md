# Rust Service

Example microservice in Rust and Actix

Run
```
cargo run
```

Docker
```
docker build -t rust_service .
docker run -t -p 8080:8080 --name rust_servie -it rust_service
```

```
export datasource_conn_string=mysql://infradb:password@localhost:3306/infradb
```

https://www.rust-lang.org

https://actix.rs
