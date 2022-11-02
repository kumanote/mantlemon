# mantlemon

> mantlemon is a asset mantle node daemon node monitoring tool.

# Features

- multiple node monitoring
  - you can monitor your validator node via your sentry nodes
- using only grpc endpoints of Asset Mantle node daemon(you need to open grpc port internally to this tool in advance.) 
- check node syncing status
- check new proposal
- check if validator node missed sign for block
- check validator status
- check slashes
- alert to [Airbrake](https://airbrake.io/) (or [Errbit](https://github.com/errbit/errbit))
  - you can customize [logger](https://github.com/kumanote/logger-rs) to change how and where to report the alerting log to.

# How to install

## Prerequisite

- [Rust with Cargo](http://rust-lang.org)
  - There is no specific `MSRV(Minimum Supported Rust Version)` 
  - Only tested with the latest stable version Rust compiler (older/nightly builds may work...)

## Install

```bash
# download
$ git clone git@github.com:kumanote/mantlemon.git
# install
$ cd mantlemon
$ cargo build --release
# then you will find an executable in the following path
$ ls -ls ./target/release/mantlemon
```

# Docker build (optional)

```bash
# download
$ git clone git@github.com:kumanote/mantlemon.git
# build
$ docker build -t mantlemon:latest .
```

# Run

Please set up config files before running the tool.
See [config.toml.example](config.toml.example) for configuration file example.

```bash
$ mantlemon -c /path/to/config.toml
```
