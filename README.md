# Rust Template for Playdate


## Install

1. Playdate SDK

https://play.date/dev/

2. rustup

https://rustup.rs/

## Setup

1. Install nightly toolchain

```sh
$ rustup install nightly
$ rustup default nightly
```

2. Install Rust target

```sh
$ rustup target add thumbv7em-none-eabihf
```


## Build

```sh
$ rake generate:simulator:debug
$ rake build
```

or

```sh
$ rake generate:device:debug
$ rake build
```
