# x-server-stats

A simple web server(and library) to display server stats over HTTP and Websockets/SSE or stream it to other systems. x-server(in x-server-stats) is not to be confused with X Window System.

### Getting started

##### Installing the crate
```bash
$ cargo install x-server-stats
```

##### Installing Rust

You can install Rust by following the instructions from the rust lang [website](https://www.rust-lang.org/tools/install).
The command to install rust using bash looks like below on the website, it's always advisable to follow the instructions on the official website.
We don't recommend using the command below as it may not be the latest version and other repos can have malicious bash script as well.

```bash
# Just for reference, please always head over to the official website for the instructions.
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

##### Install PostgreSQL 14

You can install PostgreSQL 14 by following the instructions from the PostgreSQL [website](https://www.postgresql.org/download/).
We use PostgreSQL 14 for the database, you can use any other databases as well, but we don't provide any support for other databases as of now(please raise an issue if you have a usecase for some other database).

```bash
# Just for reference, please always head over to the official homebrew/postgres website for the instructions for your OS.
$ brew install postgresql@14
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

##### Running the application with Docker

First copy the `Dockerfile.example` file to `Dockerfile` and fill in the environment variables. Then run the below commands to build and run the Docker image.

```bash
$ docker build -t x-server-stats .
$ docker run -p 8082:8082 x-server-stats
```


### Features

##### Web page for server stats
TODO - Need to work on this. The web page will be built using just HTML, CSS, JS with the whole page under 14KB gzipped and minified(due to initcwnd of TCP on linux servers).

##### Request throttling
x-server-stats uses actix-governor to throttle incoming requests based on IP address.
The config is present at `src/main.rs` and can be changed accordingly.
Throttling is important if you don't want to overwhelm the server with too many requests by internal systems.
We plan to make this configurable in the future, so you don't have to build from source to change the config.