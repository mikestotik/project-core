# Project Core

## Configuration

By default, the app uses `config.yml` in the root project directory
for the project configuration.

To set up environment-specific configurations,
use an environment config file like `config.{env-name}.yml`,
which will override the default config parameters.

Set the `APP_ENV` variable in the `.env` file in the root project directory for the current environment, like:

```dotenv
APP_ENV=dev
```

or

```dotenv
APP_ENV=prod
```

## Build

To build the project, run:

```bash
cargo build --release
```

## Run

To run the project, execute:

```bash
cargo run

# With Hot Reload
cargo watch -x run
```

## Docker

To start the Docker container, run:

```shell
docker compose up -d
```

To stop the Docker container, run:

```shell
docker compose stop
```


## Migrations

To generate, run:

```shell
sea migrate generate "create user table"
```

To run migration:

```shell
sea migrate up
```
OR
```shell
sea migrate up -s seaorm_by_example
```