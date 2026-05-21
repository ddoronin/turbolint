# rslint

A fast ESLint reimplementation built with Rust.

## Installation

### npm

```sh
npm install rslint
```

### Cargo

```sh
cargo install rslint
```

### Build from source

```sh
git clone https://github.com/user/rslint.git
cd rslint
cargo build --release
```

The binary will be at `target/release/rslint`.

## Usage

```sh
rslint file.js
rslint src/**/*.js
```

### Example output

```
src/index.js
  1:0  error  Unexpected use of 'debugger'  no-debugger

✖ 1 problem (1 error, 0 warnings)
```

## Supported rules

- `no-debugger` — disallow the use of `debugger`
