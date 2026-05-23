# turbolint

A port of [ESLint](https://eslint.org/) to Rust, aiming to be a fast drop-in replacement.

turbolint reimplements ESLint's rules natively in Rust, producing the same output format. The goal is full compatibility with ESLint's behavior — same rule names, same diagnostics, same exit codes.

> **Note:** turbolint is in early development. It supports native `.turbolintrc` config (JSON and TOML), `--fix` for autofixing, and `--rule` for running specific rules. See [Current limitations](#current-limitations) for details.

## Installation

### npm

```sh
npm install turbolint
```

### Cargo

```sh
cargo install turbolint
```

### Build from source

```sh
git clone https://github.com/ddoronin/turbolint.git
cd turbolint
cargo build --release
```

The binary will be at `target/release/turbolint`.

## Usage

Pass files, directories, or glob patterns:

```sh
turbolint file.js
turbolint src/index.js src/utils.js
turbolint src/                        # recursively lint all .js/.mjs/.cjs files
turbolint "src/**/*.js"               # glob pattern (quote to prevent shell expansion)
turbolint src/ test.js                # mix directories and files
```

### Autofix

Use `--fix` to automatically fix problems where possible:

```sh
turbolint --fix src/
```

Rules with autofix support (10 rules): `eqeqeq`, `no-debugger`, `no-extra-semi`, `no-floating-decimal`, `no-undef-init`, `no-unneeded-ternary`, `no-useless-escape`, `no-useless-rename`, `no-useless-return`, `no-var`.

### Run specific rules

Use `--rule` to run only specific rules (can be repeated):

```sh
turbolint --rule no-var src/
turbolint --rule no-var --rule eqeqeq src/
```

When `--rule` is set, only the named rules run. Severity is taken from config if present, otherwise defaults.

### JSON output

Use `--format json` (or `-f json`) for machine-readable ESLint-compatible JSON output:

```sh
turbolint -f json src/
```

Output matches ESLint's JSON format:

```json
[
  {
    "filePath": "/abs/path/file.js",
    "messages": [
      {
        "ruleId": "no-unused-vars",
        "severity": 2,
        "message": "'x' is defined but never used",
        "line": 3,
        "column": 7,
        "endLine": 3,
        "endColumn": 8,
        "fix": { "range": [14, 24], "text": "" }
      }
    ],
    "errorCount": 1,
    "warningCount": 0,
    "fixableErrorCount": 0,
    "fixableWarningCount": 0
  }
]
```

### Stdin

Read source from stdin with `--stdin`. Use `--stdin-filename` for language detection and config resolution:

```sh
echo "var x = 1;" | turbolint --stdin --stdin-filename src/file.js
echo "var x = 1;" | turbolint --stdin -f json
```

When combined with `--fix`, the fixed source is written to stdout (no files are modified):

```sh
echo "var x = 1;" | turbolint --stdin --fix
# outputs: let x = 1;
```

### Quiet mode

Use `--quiet` to suppress warnings and only report errors:

```sh
turbolint --quiet src/
```

### Max warnings

Use `--max-warnings` to fail if the warning count exceeds a threshold:

```sh
turbolint --max-warnings 10 src/
```

### AI coding tools integration

turbolint is designed to work well with AI coding harnesses (Claude Code, Cursor, Aider, Cline, etc.):

```sh
# Lint an unsaved buffer and get structured JSON output
echo "$BUFFER" | turbolint --stdin --stdin-filename file.js -f json

# Autofix an unsaved buffer without writing to disk
echo "$BUFFER" | turbolint --stdin --fix

# Only report errors, skip warnings
turbolint --quiet -f json src/

# Run only specific rules for faster feedback
turbolint --rule no-var --rule eqeqeq -f json src/
```

### Example output

```
src/index.js
  1:0  error  Unexpected use of 'debugger'  no-debugger

✖ 1 problem (1 error, 0 warnings)
```

### Exit codes

- `0` — no errors found
- `1` — one or more errors found, or `--max-warnings` threshold exceeded

## Config

turbolint uses its own native config format — no Node.js required.

### Initialize config

Generate a default `.turbolintrc` with all rules:

```sh
turbolint --init
```

This creates a `.turbolintrc` file with every rule set to its default severity and sensible ignore patterns.

### Config format

turbolint searches for config files in the current directory and parent directories, in this order:

1. `.turbolintrc.toml`
2. `.turbolintrc.json`
3. `.turbolintrc` (parsed as JSON)

**JSON** (`.turbolintrc` or `.turbolintrc.json`):

```json
{
  "rules": {
    "no-var": "error",
    "eqeqeq": "warn",
    "no-debugger": "off"
  },
  "ignores": ["dist/**", "node_modules/**"]
}
```

**TOML** (`.turbolintrc.toml`):

```toml
ignores = ["dist/**", "node_modules/**"]

[rules]
no-var = "error"
eqeqeq = "warn"
no-debugger = "off"
```

Rule severity values: `"error"` (or `2`), `"warn"` (or `1`), `"off"` (or `0`).

If no config file is found, all rules run at their default severity.

## Migrating from ESLint

Replace `eslint` with `turbolint` in your scripts:

```diff
 // package.json
 {
   "scripts": {
-    "lint": "eslint src/",
+    "lint": "turbolint src/",
   }
 }
```

To migrate your ESLint config, create a `.turbolintrc.json` with the rules you need. turbolint no longer reads `eslint.config.js` — use `turbolint --init` to generate a starting config, then adjust as needed.

Today, turbolint can be used alongside ESLint to get faster feedback on the rules it already supports.

## Current limitations

- **Partial rule coverage** — 292 of ESLint's 294 rules are ported. See below for the current list.
- **No plugins** — only built-in rules are available.
- **Limited autofix** — `--fix` is supported with 10 rules having fixes so far.

## Supported rules (292 / 294)

<details>
<summary>Click to expand full rule list</summary>

`accessor-pairs`,
`array-bracket-newline`,
`array-bracket-spacing`,
`array-callback-return`,
`array-element-newline`,
`arrow-body-style`,
`arrow-parens`,
`arrow-spacing`,
`block-scoped-var`,
`block-spacing`,
`brace-style`,
`callback-return`,
`camelcase`,
`capitalized-comments`,
`class-methods-use-this`,
`comma-dangle`,
`comma-spacing`,
`comma-style`,
`complexity`,
`computed-property-spacing`,
`consistent-return`,
`consistent-this`,
`constructor-super`,
`curly`,
`default-case`,
`default-case-last`,
`default-param-last`,
`dot-location`,
`dot-notation`,
`eol-last`,
`eqeqeq`,
`for-direction`,
`func-call-spacing`,
`func-name-matching`,
`func-names`,
`func-style`,
`function-call-argument-newline`,
`function-paren-newline`,
`generator-star-spacing`,
`getter-return`,
`global-require`,
`grouped-accessor-pairs`,
`guard-for-in`,
`handle-callback-err`,
`id-blacklist`,
`id-denylist`,
`id-length`,
`id-match`,
`implicit-arrow-linebreak`,
`indent`,
`indent-legacy`,
`init-declarations`,
`jsx-quotes`,
`key-spacing`,
`keyword-spacing`,
`line-comment-position`,
`linebreak-style`,
`lines-around-comment`,
`lines-around-directive`,
`lines-between-class-members`,
`logical-assignment-operators`,
`max-classes-per-file`,
`max-depth`,
`max-len`,
`max-lines`,
`max-lines-per-function`,
`max-nested-callbacks`,
`max-params`,
`max-statements`,
`max-statements-per-line`,
`multiline-comment-style`,
`multiline-ternary`,
`new-cap`,
`newline-after-var`,
`newline-before-return`,
`newline-per-chained-call`,
`no-alert`,
`no-array-constructor`,
`no-async-promise-executor`,
`no-await-in-loop`,
`no-bitwise`,
`no-buffer-constructor`,
`no-caller`,
`no-case-declarations`,
`no-catch-shadow`,
`no-class-assign`,
`no-compare-neg-zero`,
`no-cond-assign`,
`no-confusing-arrow`,
`no-console`,
`no-const-assign`,
`no-constant-binary-expression`,
`no-constant-condition`,
`no-constructor-return`,
`no-continue`,
`no-control-regex`,
`no-debugger`,
`no-delete-var`,
`no-div-regex`,
`no-dupe-args`,
`no-dupe-class-members`,
`no-dupe-else-if`,
`no-dupe-keys`,
`no-duplicate-case`,
`no-duplicate-imports`,
`no-else-return`,
`no-empty`,
`no-empty-character-class`,
`no-empty-function`,
`no-empty-pattern`,
`no-empty-static-block`,
`no-eq-null`,
`no-eval`,
`no-ex-assign`,
`no-extend-native`,
`no-extra-bind`,
`no-extra-boolean-cast`,
`no-extra-label`,
`no-extra-parens`,
`no-extra-semi`,
`no-fallthrough`,
`no-floating-decimal`,
`no-func-assign`,
`no-global-assign`,
`no-implicit-coercion`,
`no-implicit-globals`,
`no-implied-eval`,
`no-import-assign`,
`no-inline-comments`,
`no-inner-declarations`,
`no-invalid-regexp`,
`no-invalid-this`,
`no-irregular-whitespace`,
`no-iterator`,
`no-label-var`,
`no-labels`,
`no-lone-blocks`,
`no-lonely-if`,
`no-loop-func`,
`no-loss-of-precision`,
`no-magic-numbers`,
`no-misleading-character-class`,
`no-mixed-operators`,
`no-mixed-requires`,
`no-mixed-spaces-and-tabs`,
`no-multi-assign`,
`no-multi-spaces`,
`no-multi-str`,
`no-multiple-empty-lines`,
`no-native-reassign`,
`no-negated-condition`,
`no-negated-in-lhs`,
`no-nested-ternary`,
`no-new`,
`no-new-func`,
`no-new-native-nonconstructor`,
`no-new-object`,
`no-new-require`,
`no-new-symbol`,
`no-new-wrappers`,
`no-nonoctal-decimal-escape`,
`no-obj-calls`,
`no-object-constructor`,
`no-octal`,
`no-octal-escape`,
`no-param-reassign`,
`no-path-concat`,
`no-plusplus`,
`no-process-env`,
`no-process-exit`,
`no-promise-executor-return`,
`no-proto`,
`no-prototype-builtins`,
`no-redeclare`,
`no-regex-spaces`,
`no-restricted-exports`,
`no-restricted-globals`,
`no-restricted-imports`,
`no-restricted-modules`,
`no-restricted-properties`,
`no-restricted-syntax`,
`no-return-assign`,
`no-return-await`,
`no-script-url`,
`no-self-assign`,
`no-self-compare`,
`no-sequences`,
`no-setter-return`,
`no-shadow`,
`no-shadow-restricted-names`,
`no-spaced-func`,
`no-sparse-arrays`,
`no-sync`,
`no-tabs`,
`no-template-curly-in-string`,
`no-ternary`,
`no-this-before-super`,
`no-throw-literal`,
`no-trailing-spaces`,
`no-undef`,
`no-undef-init`,
`no-undefined`,
`no-underscore-dangle`,
`no-unexpected-multiline`,
`no-unmodified-loop-condition`,
`no-unneeded-ternary`,
`no-unreachable`,
`no-unreachable-loop`,
`no-unsafe-finally`,
`no-unsafe-negation`,
`no-unsafe-optional-chaining`,
`no-unused-expressions`,
`no-unused-labels`,
`no-unused-private-class-members`,
`no-unused-vars`,
`no-use-before-define`,
`no-useless-assignment`,
`no-useless-backreference`,
`no-useless-call`,
`no-useless-catch`,
`no-useless-computed-key`,
`no-useless-concat`,
`no-useless-constructor`,
`no-useless-escape`,
`no-useless-rename`,
`no-useless-return`,
`no-var`,
`no-void`,
`no-warning-comments`,
`no-whitespace-before-property`,
`no-with`,
`nonblock-statement-body-position`,
`object-curly-newline`,
`object-curly-spacing`,
`object-property-newline`,
`object-shorthand`,
`one-var`,
`one-var-declaration-per-line`,
`operator-assignment`,
`operator-linebreak`,
`padded-blocks`,
`padding-line-between-statements`,
`prefer-arrow-callback`,
`prefer-const`,
`prefer-destructuring`,
`prefer-exponentiation-operator`,
`prefer-named-capture-group`,
`prefer-numeric-literals`,
`prefer-object-has-own`,
`prefer-object-spread`,
`prefer-promise-reject-errors`,
`prefer-regex-literals`,
`prefer-rest-params`,
`prefer-spread`,
`prefer-template`,
`preserve-caught-error`,
`quote-props`,
`quotes`,
`radix`,
`require-atomic-updates`,
`require-await`,
`require-unicode-regexp`,
`require-yield`,
`rest-spread-spacing`,
`semi`,
`semi-spacing`,
`semi-style`,
`sort-imports`,
`sort-keys`,
`sort-vars`,
`space-before-blocks`,
`space-before-function-paren`,
`space-in-parens`,
`space-infix-ops`,
`space-unary-ops`,
`spaced-comment`,
`strict`,
`switch-colon-spacing`,
`symbol-description`,
`template-curly-spacing`,
`template-tag-spacing`,
`unicode-bom`,
`use-isnan`,
`valid-typeof`,
`vars-on-top`,
`wrap-iife`,
`wrap-regex`,
`yield-star-spacing`,
`yoda`

</details>
