# x-server-stats

A simple web server(and library) to display server stats over HTTP and Websockets/SSE or stream it to other systems. x-server(in x-server-stats) is not to be confused with X Window System.

### Getting started

##### Installing the crate
```bash
$ cargo install x-server-stats
```

##### Setting up environment variables

You need to export below environment variables to run the server.
The environment variables prefixed with `PG` are for PotsgreSQL database. Currently, only PostgreSQL is supported for storing stats.

```bash
export SERVER_ADDR=localhost:8082
export PG__USER=PG_USERNAME
export PG__PASSWORD=PG_PASSWORD
export PG__HOST=PG_HOST_URL
export PG__PORT=5432
export PG__DBNAME=PG_DB_NAME
export PG__POOL__MAX_SIZE=20
export BASE_ADDR=http://localhost:8082
```

##### Building(and running) the application

Before running the application, you need to build it. You can do this by running the following command:

```bash
$ cargo build --release 
```

Run the binary with the following command:

```bash
$ ./target/release/x-server-stats
```

##### Running the application directly

You can run the application directly by running the following command:

```bash
$ cargo run --release
```


