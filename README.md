# rslint

A port of [ESLint](https://eslint.org/) to Rust, aiming to be a fast drop-in replacement.

rslint reimplements ESLint's rules natively in Rust, producing the same output format. The goal is full compatibility with ESLint's behavior — same rule names, same diagnostics, same exit codes.

> **Note:** rslint is in early development. It does not yet read `eslint.config.js` or support ESLint's full feature set. All implemented rules run unconditionally on every file. See [Current limitations](#current-limitations) for details.

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
git clone https://github.com/ddoronin/rslint.git
cd rslint
cargo build --release
```

The binary will be at `target/release/rslint`.

## Usage

Pass one or more JavaScript files to lint:

```sh
rslint file.js
rslint src/index.js src/utils.js
```

### Example output

```
src/index.js
  1:0  error  Unexpected use of 'debugger'  no-debugger

✖ 1 problem (1 error, 0 warnings)
```

### Exit codes

- `0` — no errors found
- `1` — one or more errors found

## Migrating from ESLint

Once rslint reaches feature parity, the migration will be straightforward — replace `eslint` with `rslint` in your scripts:

```diff
 // package.json
 {
   "scripts": {
-    "lint": "eslint src/",
+    "lint": "rslint src/",
   }
 }
```

Today, rslint can be used alongside ESLint to get faster feedback on the rules it already supports.

## Current limitations

- **No config file support** — `eslint.config.js` is not read yet. All implemented rules run on every file.
- **No directory or glob arguments** — you must pass individual file paths.
- **Partial rule coverage** — not all ESLint rules are ported. See below for the current list.
- **No autofix** — `--fix` is not yet supported.
- **No plugins** — only built-in rules are available.

## Supported rules

- `no-debugger` — disallow the use of `debugger`
