//! Integration tests verifying turbolint reads and respects eslint.config.js.
//!
//! These tests create temporary directories with config files and JS sources,
//! then invoke the `turbolint` binary and check its output and exit code.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn turbolint_bin() -> PathBuf {
    // The binary is built in the workspace target directory
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // crates/
    path.pop(); // workspace root
    path.push("target/debug/turbolint");
    path
}

fn node_available() -> bool {
    Command::new("node")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("turbolint_integ_{name}"));
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
// Tests
// ---------------------------------------------------------------------------

#[test]
fn config_enables_only_one_rule() {
    if !node_available() {
        eprintln!("Skipping: Node.js not available");
        return;
    }

    let dir = TempDir::new("enables_one_rule");
    dir.write_file(
        "eslint.config.js",
        r#"module.exports = [{ rules: { "no-debugger": "error" } }];"#,
    );
    // This file triggers no-debugger but also has `var` (no-var should NOT fire)
    dir.write_file("test.js", "var x = 1;\ndebugger;\n");

    let result = dir.run(&["test.js"]);
    assert_eq!(result.exit_code, 1, "should exit 1 for errors");
    assert!(
        result.stdout.contains("no-debugger"),
        "should report no-debugger: {}",
        result.stdout
    );
    assert!(
        !result.stdout.contains("no-var"),
        "should NOT report no-var when not configured: {}",
        result.stdout
    );
}

#[test]
fn config_turns_rule_off() {
    if !node_available() {
        eprintln!("Skipping: Node.js not available");
        return;
    }

    let dir = TempDir::new("turns_rule_off");
    dir.write_file(
        "eslint.config.js",
        r#"module.exports = [{ rules: { "no-debugger": "off" } }];"#,
    );
    dir.write_file("test.js", "debugger;\n");

    let result = dir.run(&["test.js"]);
    assert_eq!(result.exit_code, 0, "should exit 0 when rule is off");
    assert!(
        !result.stdout.contains("no-debugger"),
        "should not report disabled rule: {}",
        result.stdout
    );
}

#[test]
fn config_sets_rule_to_warn() {
    if !node_available() {
        eprintln!("Skipping: Node.js not available");
        return;
    }

    let dir = TempDir::new("sets_warn");
    dir.write_file(
        "eslint.config.js",
        r#"module.exports = [{ rules: { "no-debugger": "warn" } }];"#,
    );
    dir.write_file("test.js", "debugger;\n");

    let result = dir.run(&["test.js"]);
    // Warnings produce exit 0 (only errors produce exit 1)
    assert_eq!(result.exit_code, 0, "warnings should exit 0");
    assert!(
        result.stdout.contains("warning"),
        "should show warning severity: {}",
        result.stdout
    );
    assert!(
        result.stdout.contains("no-debugger"),
        "should still report the rule: {}",
        result.stdout
    );
}

#[test]
fn config_numeric_severity() {
    if !node_available() {
        eprintln!("Skipping: Node.js not available");
        return;
    }

    let dir = TempDir::new("numeric_severity");
    dir.write_file(
        "eslint.config.js",
        r#"module.exports = [{ rules: { "no-debugger": 2, "no-var": 0 } }];"#,
    );
    dir.write_file("test.js", "var x = 1;\ndebugger;\n");

    let result = dir.run(&["test.js"]);
    assert_eq!(result.exit_code, 1);
    assert!(result.stdout.contains("no-debugger"));
    assert!(!result.stdout.contains("no-var"), "no-var should be off (0)");
}

#[test]
fn config_ignores_files() {
    if !node_available() {
        eprintln!("Skipping: Node.js not available");
        return;
    }

    let dir = TempDir::new("ignores_files");
    dir.write_file(
        "eslint.config.js",
        r#"module.exports = [
            { rules: { "no-debugger": "error" } },
            { ignores: ["vendor/**"] }
        ];"#,
    );
    dir.write_file("vendor/lib.js", "debugger;\n");
    dir.write_file("src/app.js", "debugger;\n");

    // Lint both files
    let result = dir.run(&["vendor/lib.js", "src/app.js"]);
    assert!(
        !result.stdout.contains("vendor/lib.js"),
        "vendor file should be ignored: {}",
        result.stdout
    );
    assert!(
        result.stdout.contains("src/app.js"),
        "src file should be linted: {}",
        result.stdout
    );
}

#[test]
fn config_files_pattern_limits_scope() {
    if !node_available() {
        eprintln!("Skipping: Node.js not available");
        return;
    }

    let dir = TempDir::new("files_pattern");
    // Only apply no-debugger to test files
    dir.write_file(
        "eslint.config.js",
        r#"module.exports = [
            { files: ["test/**"], rules: { "no-debugger": "error" } }
        ];"#,
    );
    dir.write_file("test/spec.js", "debugger;\n");
    dir.write_file("src/app.js", "debugger;\n");

    let result_test = dir.run(&["test/spec.js"]);
    assert!(
        result_test.stdout.contains("no-debugger"),
        "test file should match files pattern: {}",
        result_test.stdout
    );

    let result_src = dir.run(&["src/app.js"]);
    assert!(
        !result_src.stdout.contains("no-debugger"),
        "src file should NOT match files pattern: {}",
        result_src.stdout
    );
}

#[test]
fn config_later_object_overrides_earlier() {
    if !node_available() {
        eprintln!("Skipping: Node.js not available");
        return;
    }

    let dir = TempDir::new("override");
    dir.write_file(
        "eslint.config.js",
        r#"module.exports = [
            { rules: { "no-debugger": "error" } },
            { rules: { "no-debugger": "off" } }
        ];"#,
    );
    dir.write_file("test.js", "debugger;\n");

    let result = dir.run(&["test.js"]);
    assert_eq!(
        result.exit_code, 0,
        "second config turns rule off, should exit 0"
    );
    assert!(!result.stdout.contains("no-debugger"));
}

#[test]
fn no_config_runs_all_rules_at_default() {
    // No eslint.config.js in the dir — all rules should run at default
    let dir = TempDir::new("no_config");
    dir.write_file("test.js", "var x = 1;\ndebugger;\n");

    let result = dir.run(&["test.js"]);
    assert_eq!(result.exit_code, 1);
    assert!(
        result.stdout.contains("no-debugger"),
        "no-debugger should fire at default: {}",
        result.stdout
    );
    assert!(
        result.stdout.contains("no-var"),
        "no-var should fire at default: {}",
        result.stdout
    );
}

#[test]
fn config_fix_respects_config() {
    if !node_available() {
        eprintln!("Skipping: Node.js not available");
        return;
    }

    let dir = TempDir::new("fix_respects_config");
    // Only enable eqeqeq, not no-var
    dir.write_file(
        "eslint.config.js",
        r#"module.exports = [{ rules: { "eqeqeq": "error" } }];"#,
    );
    dir.write_file("test.js", "var x = 1;\nif (x == 2) {}\n");

    let result = dir.run(&["--fix", "test.js"]);
    let fixed = fs::read_to_string(dir.path.join("test.js")).unwrap();
    // eqeqeq should fix == to ===
    assert!(
        fixed.contains("==="),
        "eqeqeq fix should apply: {fixed}"
    );
    // no-var should NOT fix var to let (it's not configured)
    assert!(
        fixed.starts_with("var"),
        "no-var should not fix when not configured: {fixed}"
    );
    assert_eq!(result.exit_code, 0, "all fixable errors should be resolved");
}

#[test]
fn config_multiple_rules_mixed_severity() {
    if !node_available() {
        eprintln!("Skipping: Node.js not available");
        return;
    }

    let dir = TempDir::new("mixed_severity");
    dir.write_file(
        "eslint.config.js",
        r#"module.exports = [{
            rules: {
                "no-debugger": "error",
                "no-var": "warn",
                "eqeqeq": "off"
            }
        }];"#,
    );
    dir.write_file("test.js", "var x = 1;\ndebugger;\nif (x == 2) {}\n");

    let result = dir.run(&["test.js"]);
    assert_eq!(result.exit_code, 1, "has errors so exit 1");
    assert!(result.stdout.contains("no-debugger"));
    assert!(result.stdout.contains("no-var"));
    assert!(!result.stdout.contains("eqeqeq"), "eqeqeq is off");
    // Check severity display
    assert!(result.stdout.contains("error"), "no-debugger is error");
    assert!(result.stdout.contains("warning"), "no-var is warning");
}

#[test]
fn config_cjs_extension() {
    if !node_available() {
        eprintln!("Skipping: Node.js not available");
        return;
    }

    let dir = TempDir::new("cjs_ext");
    // Use .cjs extension
    dir.write_file(
        "eslint.config.cjs",
        r#"module.exports = [{ rules: { "no-debugger": "error" } }];"#,
    );
    dir.write_file("test.js", "debugger;\n");

    let result = dir.run(&["test.js"]);
    assert_eq!(result.exit_code, 1);
    assert!(
        result.stdout.contains("no-debugger"),
        "should load .cjs config: {}",
        result.stdout
    );
}

#[test]
fn config_empty_rules_runs_defaults() {
    if !node_available() {
        eprintln!("Skipping: Node.js not available");
        return;
    }

    let dir = TempDir::new("empty_rules");
    // Config exists but has no rules — should run all rules at default
    dir.write_file(
        "eslint.config.js",
        r#"module.exports = [{}];"#,
    );
    dir.write_file("test.js", "debugger;\nvar x = 1;\n");

    let result = dir.run(&["test.js"]);
    assert!(
        result.stdout.contains("no-debugger"),
        "empty config should run defaults: {}",
        result.stdout
    );
    assert!(
        result.stdout.contains("no-var"),
        "empty config should run defaults: {}",
        result.stdout
    );
}
