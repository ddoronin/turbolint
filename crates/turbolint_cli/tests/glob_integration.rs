//! Integration tests for file discovery: glob patterns, directories, and default ignores.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn turbolint_bin() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // crates/
    path.pop(); // workspace root
    path.push("target/debug/turbolint");
    path
}

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("turbolint_glob_{name}"));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    fn write_file(&self, name: &str, content: &str) {
        let file_path = self.path.join(name);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(file_path, content).unwrap();
    }

    fn run(&self, args: &[&str]) -> RunResult {
        let bin = turbolint_bin();
        let output = Command::new(&bin)
            .args(args)
            .current_dir(&self.path)
            .output()
            .unwrap_or_else(|e| panic!("Failed to run {}: {e}", bin.display()));
        RunResult {
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
        }
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[allow(dead_code)]
struct RunResult {
    stdout: String,
    stderr: String,
    exit_code: i32,
}

// ---------------------------------------------------------------------------
// Glob pattern tests
// ---------------------------------------------------------------------------

#[test]
fn glob_star_matches_js_files() {
    let dir = TempDir::new("glob_star");
    dir.write_file("a.js", "debugger;\n");
    dir.write_file("b.js", "debugger;\n");
    dir.write_file("c.txt", "debugger;\n"); // not JS, should not match

    let result = dir.run(&["*.js"]);
    assert_eq!(result.exit_code, 1);
    assert!(result.stdout.contains("a.js"), "should lint a.js: {}", result.stdout);
    assert!(result.stdout.contains("b.js"), "should lint b.js: {}", result.stdout);
    assert!(!result.stdout.contains("c.txt"), "should not lint c.txt: {}", result.stdout);
}

#[test]
fn glob_recursive_star_star() {
    let dir = TempDir::new("glob_recursive");
    dir.write_file("src/app.js", "debugger;\n");
    dir.write_file("src/lib/util.js", "debugger;\n");

    let result = dir.run(&["src/**/*.js"]);
    assert_eq!(result.exit_code, 1);
    assert!(result.stdout.contains("src/app.js"), "should find src/app.js: {}", result.stdout);
    assert!(
        result.stdout.contains("src/lib/util.js"),
        "should find nested file: {}",
        result.stdout
    );
}

#[test]
fn glob_question_mark_wildcard() {
    let dir = TempDir::new("glob_question");
    dir.write_file("a1.js", "debugger;\n");
    dir.write_file("a2.js", "debugger;\n");
    dir.write_file("abc.js", "debugger;\n"); // should NOT match a?.js

    let result = dir.run(&["a?.js"]);
    assert_eq!(result.exit_code, 1);
    assert!(result.stdout.contains("a1.js"), "should match a1.js: {}", result.stdout);
    assert!(result.stdout.contains("a2.js"), "should match a2.js: {}", result.stdout);
    assert!(!result.stdout.contains("abc.js"), "should not match abc.js: {}", result.stdout);
}

#[test]
fn glob_bracket_character_class() {
    let dir = TempDir::new("glob_bracket");
    dir.write_file("test1.js", "debugger;\n");
    dir.write_file("test2.js", "debugger;\n");
    dir.write_file("test3.js", "debugger;\n"); // should NOT match [12]

    let result = dir.run(&["test[12].js"]);
    assert_eq!(result.exit_code, 1);
    assert!(result.stdout.contains("test1.js"), "should match test1.js: {}", result.stdout);
    assert!(result.stdout.contains("test2.js"), "should match test2.js: {}", result.stdout);
    assert!(
        !result.stdout.contains("test3.js"),
        "should not match test3.js: {}",
        result.stdout
    );
}

#[test]
fn glob_no_matches_exits_with_error() {
    let dir = TempDir::new("glob_no_match");
    // No JS files at all
    dir.write_file("readme.txt", "hello\n");

    let result = dir.run(&["*.js"]);
    assert_ne!(result.exit_code, 0, "should fail when no files match");
    assert!(
        result.stderr.contains("No matching files"),
        "should print no-match message: {}",
        result.stderr
    );
}

// ---------------------------------------------------------------------------
// Directory argument tests
// ---------------------------------------------------------------------------

#[test]
fn directory_discovers_js_files_recursively() {
    let dir = TempDir::new("dir_discover");
    dir.write_file("src/index.js", "debugger;\n");
    dir.write_file("src/utils/helper.js", "debugger;\n");
    dir.write_file("src/styles/main.css", "body {}\n"); // not JS

    let result = dir.run(&["src"]);
    assert_eq!(result.exit_code, 1);
    assert!(
        result.stdout.contains("src/index.js"),
        "should find index.js: {}",
        result.stdout
    );
    assert!(
        result.stdout.contains("src/utils/helper.js"),
        "should find nested JS: {}",
        result.stdout
    );
    assert!(
        !result.stdout.contains("main.css"),
        "should not lint CSS files: {}",
        result.stdout
    );
}

#[test]
fn directory_discovers_mjs_and_cjs() {
    let dir = TempDir::new("dir_extensions");
    dir.write_file("lib/a.mjs", "debugger;\n");
    dir.write_file("lib/b.cjs", "debugger;\n");
    dir.write_file("lib/c.js", "debugger;\n");

    let result = dir.run(&["lib"]);
    assert_eq!(result.exit_code, 1);
    assert!(result.stdout.contains("a.mjs"), "should find .mjs: {}", result.stdout);
    assert!(result.stdout.contains("b.cjs"), "should find .cjs: {}", result.stdout);
    assert!(result.stdout.contains("c.js"), "should find .js: {}", result.stdout);
}

#[test]
fn directory_ignores_node_modules() {
    let dir = TempDir::new("dir_node_modules");
    dir.write_file("src/app.js", "debugger;\n");
    dir.write_file("node_modules/pkg/index.js", "debugger;\n");

    let result = dir.run(&["."]);
    assert_eq!(result.exit_code, 1);
    assert!(
        result.stdout.contains("src/app.js"),
        "should lint src/app.js: {}",
        result.stdout
    );
    assert!(
        !result.stdout.contains("node_modules"),
        "should skip node_modules: {}",
        result.stdout
    );
}

#[test]
fn directory_ignores_nested_node_modules() {
    let dir = TempDir::new("dir_nested_nm");
    dir.write_file("app.js", "debugger;\n");
    dir.write_file("packages/foo/node_modules/bar/index.js", "debugger;\n");

    let result = dir.run(&["."]);
    assert_eq!(result.exit_code, 1);
    assert!(result.stdout.contains("app.js"), "should lint app.js: {}", result.stdout);
    assert!(
        !result.stdout.contains("node_modules"),
        "should skip nested node_modules: {}",
        result.stdout
    );
}

// ---------------------------------------------------------------------------
// Mixed arguments
// ---------------------------------------------------------------------------

#[test]
fn mixed_file_directory_and_glob() {
    let dir = TempDir::new("mixed_args");
    dir.write_file("single.js", "debugger;\n");
    dir.write_file("src/a.js", "debugger;\n");
    dir.write_file("tests/t1.js", "debugger;\n");
    dir.write_file("tests/t2.js", "debugger;\n");

    let result = dir.run(&["single.js", "src", "tests/*.js"]);
    assert_eq!(result.exit_code, 1);
    assert!(result.stdout.contains("single.js"), "explicit file: {}", result.stdout);
    assert!(result.stdout.contains("src/a.js"), "dir discovery: {}", result.stdout);
    assert!(result.stdout.contains("tests/t1.js"), "glob match: {}", result.stdout);
    assert!(result.stdout.contains("tests/t2.js"), "glob match: {}", result.stdout);
}

#[test]
fn deduplicates_files_from_overlapping_args() {
    let dir = TempDir::new("dedup");
    dir.write_file("src/app.js", "debugger;\n");

    // Pass the same file via explicit path and directory — should only report once
    let result = dir.run(&["src/app.js", "src"]);
    assert_eq!(result.exit_code, 1);
    let count = result.stdout.matches("src/app.js").count();
    assert_eq!(count, 1, "file should appear exactly once: {}", result.stdout);
}

// ---------------------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------------------

#[test]
fn explicit_file_not_found() {
    let dir = TempDir::new("file_not_found");

    let result = dir.run(&["nonexistent.js"]);
    assert_ne!(result.exit_code, 0, "should fail for missing file");
    assert!(
        result.stderr.contains("Error reading") || result.stderr.contains("nonexistent"),
        "should report read error: {}",
        result.stderr
    );
}

#[test]
fn empty_directory_no_js_files() {
    let dir = TempDir::new("empty_dir");
    dir.write_file("lib/data.txt", "hello\n");

    let result = dir.run(&["lib"]);
    assert_ne!(result.exit_code, 0, "should fail when dir has no JS files");
    assert!(
        result.stderr.contains("No matching files"),
        "should print no-match message: {}",
        result.stderr
    );
}

#[test]
fn no_arguments_exits_with_error() {
    let dir = TempDir::new("no_args");

    let result = dir.run(&[]);
    assert_ne!(result.exit_code, 0, "should fail with no arguments");
    assert!(
        result.stderr.contains("No files specified"),
        "should print no-files message: {}",
        result.stderr
    );
}
