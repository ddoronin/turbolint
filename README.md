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
- **Partial rule coverage** — 292 of ESLint's 294 rules are ported. See below for the current list.
- **No autofix** — `--fix` is not yet supported.
- **No plugins** — only built-in rules are available.

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
