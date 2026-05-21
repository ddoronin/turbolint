# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**turbolint** is a Rust reimplementation of ESLint. The project is in its earliest stage — no Rust code has been written yet. The `eslint/` directory contains a vendored copy of ESLint v10.4.0 source (JavaScript) as the reference implementation.

## Build & Test Commands

No build system is set up yet. When Rust code is added, expect standard Cargo commands:
- `cargo build` — build the project
- `cargo test` — run all tests
- `cargo test <test_name>` — run a single test
- `cargo clippy` — lint Rust code
- `cargo fmt` — format Rust code

## Reference ESLint Architecture (in `eslint/`)

The vendored ESLint source serves as the specification for what turbolint needs to implement. Key components:

- **CLI** (`eslint/lib/cli.js`) — Command-line interface, option parsing, output formatting
- **ESLint class** (`eslint/lib/eslint/`) — High-level API: file discovery, config loading, orchestrating linting, caching, parallel workers
- **Linter** (`eslint/lib/linter/`) — Core linting engine: AST traversal, rule execution, disable directives, autofix (up to 10 passes), source code fixing
- **Config system** (`eslint/lib/config/`) — Flat config (`eslint.config.js`), config arrays, schema validation
- **Languages** (`eslint/lib/languages/js/`) — JavaScript language plugin: parsing (via espree), source code representation, visitor keys
- **Rules** (`eslint/lib/rules/`) — 294 built-in lint rules
- **Rule Tester** (`eslint/lib/rule-tester/`) — Test harness for validating rules

### Key design patterns in ESLint to preserve:
- Rules are visitor-based: they subscribe to AST node types and receive node + context
- Config is composable via flat config arrays with cascading/merging
- Autofixing applies fixes iteratively until stable (max 10 passes)
- The linter is language-agnostic; language plugins provide parsing and traversal
