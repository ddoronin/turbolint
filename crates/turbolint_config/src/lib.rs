use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

/// A single config object in the flat config array.
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct ConfigObject {
    /// Glob patterns for files this config applies to.
    #[serde(default)]
    pub files: Vec<StringOrStrings>,

    /// Glob patterns for files to ignore.
    #[serde(default)]
    pub ignores: Vec<StringOrStrings>,

    /// Rule configurations: rule name -> severity or [severity, ...options].
    #[serde(default)]
    pub rules: HashMap<String, RuleSetting>,
}

/// A rule setting can be a number, a string, or an array starting with severity.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RuleSetting {
    Severity(SeverityValue),
    WithOptions(Vec<serde_json::Value>),
}

/// Severity can be a string ("off"/"warn"/"error") or a number (0/1/2).
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SeverityValue {
    Number(u8),
    String(String),
}

/// File patterns can be a single string or an array of strings.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum StringOrStrings {
    Single(String),
    Multiple(Vec<String>),
}

/// Normalized severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigSeverity {
    Off,
    Warn,
    Error,
}

/// Resolved rule configuration for a specific file.
#[derive(Debug, Clone)]
pub struct ResolvedRuleConfig {
    pub severity: ConfigSeverity,
}

/// The loaded configuration.
#[derive(Debug, Clone)]
pub struct Config {
    pub objects: Vec<ConfigObject>,
    /// Directory containing the config file (used to relativize file paths for pattern matching).
    pub config_dir: PathBuf,
}

impl Config {
    /// Resolve the effective rule configuration for a given file path.
    /// Walks the config array in order; later entries override earlier ones.
    pub fn resolve_rules_for_file(&self, file_path: &str) -> HashMap<String, ResolvedRuleConfig> {
        let mut rules: HashMap<String, ResolvedRuleConfig> = HashMap::new();

        // Make the path relative to the config directory for pattern matching
        let rel_path = Path::new(file_path)
            .strip_prefix(&self.config_dir)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| file_path.to_string());

        for obj in &self.objects {
            // Check if this config object applies to the file
            if !config_matches_file(obj, &rel_path) {
                continue;
            }

            // Check if the file is ignored by this config object
            if is_ignored_by(obj, &rel_path) {
                continue;
            }

            // Merge rules
            for (name, setting) in &obj.rules {
                let severity = parse_severity(setting);
                rules.insert(name.clone(), ResolvedRuleConfig { severity });
            }
        }

        rules
    }
}

/// Check if a config object's `files` patterns match the given path.
/// If no `files` patterns are specified, the config applies to all files.
fn config_matches_file(obj: &ConfigObject, file_path: &str) -> bool {
    if obj.files.is_empty() {
        return true;
    }

    for pattern_set in &obj.files {
        let patterns = match pattern_set {
            StringOrStrings::Single(s) => vec![s.as_str()],
            StringOrStrings::Multiple(v) => v.iter().map(|s| s.as_str()).collect(),
        };
        for pattern in patterns {
            if glob_matches(pattern, file_path) {
                return true;
            }
        }
    }
    false
}

/// Check if a file is ignored by this config object.
fn is_ignored_by(obj: &ConfigObject, file_path: &str) -> bool {
    for pattern_set in &obj.ignores {
        let patterns = match pattern_set {
            StringOrStrings::Single(s) => vec![s.as_str()],
            StringOrStrings::Multiple(v) => v.iter().map(|s| s.as_str()).collect(),
        };
        for pattern in patterns {
            if glob_matches(pattern, file_path) {
                return true;
            }
        }
    }
    false
}

fn glob_matches(pattern: &str, path: &str) -> bool {
    // The `glob` crate doesn't support brace expansion ({a,b}).
    // Expand braces into multiple patterns and match any.
    for expanded in expand_braces(pattern) {
        if glob::Pattern::new(&expanded)
            .map(|p| p.matches(path))
            .unwrap_or(false)
        {
            return true;
        }
    }
    false
}

/// Expand brace patterns like `*.{ts,tsx}` into `["*.ts", "*.tsx"]`.
/// Handles one level of braces. If no braces, returns the pattern as-is.
fn expand_braces(pattern: &str) -> Vec<String> {
    if let Some(open) = pattern.find('{') {
        if let Some(close) = pattern[open..].find('}') {
            let close = open + close;
            let prefix = &pattern[..open];
            let suffix = &pattern[close + 1..];
            let alternatives = &pattern[open + 1..close];
            let mut result = Vec::new();
            for alt in alternatives.split(',') {
                // Recursively expand in case of nested braces in suffix
                for expanded in expand_braces(&format!("{prefix}{alt}{suffix}")) {
                    result.push(expanded);
                }
            }
            return result;
        }
    }
    vec![pattern.to_string()]
}

fn parse_severity(setting: &RuleSetting) -> ConfigSeverity {
    match setting {
        RuleSetting::Severity(v) => severity_value_to_config(v),
        RuleSetting::WithOptions(arr) => {
            if let Some(first) = arr.first() {
                if let Some(n) = first.as_u64() {
                    return match n {
                        0 => ConfigSeverity::Off,
                        1 => ConfigSeverity::Warn,
                        _ => ConfigSeverity::Error,
                    };
                }
                if let Some(s) = first.as_str() {
                    return match s {
                        "off" => ConfigSeverity::Off,
                        "warn" => ConfigSeverity::Warn,
                        "error" => ConfigSeverity::Error,
                        _ => ConfigSeverity::Error,
                    };
                }
            }
            ConfigSeverity::Error
        }
    }
}

fn severity_value_to_config(v: &SeverityValue) -> ConfigSeverity {
    match v {
        SeverityValue::Number(0) => ConfigSeverity::Off,
        SeverityValue::Number(1) => ConfigSeverity::Warn,
        SeverityValue::Number(_) => ConfigSeverity::Error,
        SeverityValue::String(s) => match s.as_str() {
            "off" => ConfigSeverity::Off,
            "warn" => ConfigSeverity::Warn,
            "error" => ConfigSeverity::Error,
            _ => ConfigSeverity::Error,
        },
    }
}

/// The JS snippet we inject into Node to evaluate the config and serialize it.
const EVAL_SCRIPT: &str = r#"
(async () => {
    const path = require('path');
    const configPath = process.argv[1];
    let config;
    try {
        config = require(configPath);
    } catch (e) {
        if (e.code === 'ERR_REQUIRE_ESM' || e.code === 'ERR_REQUIRE_ASYNC_MODULE') {
            const url = require('url');
            config = await import(url.pathToFileURL(configPath).href);
        } else {
            throw e;
        }
    }
    // Handle default export (ESM-style)
    if (config && config.default) config = config.default;
    // Handle promise
    if (config && typeof config.then === 'function') config = await config;
    // Ensure it's an array
    if (!Array.isArray(config)) config = [config];
    // Extract only the fields we care about
    const result = config.map(obj => ({
        files: obj.files || [],
        ignores: obj.ignores || [],
        rules: obj.rules || {},
    }));
    process.stdout.write(JSON.stringify(result));
})().catch(e => {
    process.stderr.write(e.message);
    process.exit(1);
});
"#;

/// The config file names we search for, in priority order.
const CONFIG_FILES: &[&str] = &[
    "eslint.config.js",
    "eslint.config.mjs",
    "eslint.config.cjs",
];

/// Find the config file starting from `dir` and searching up to the filesystem root.
pub fn find_config_file(dir: &Path) -> Option<PathBuf> {
    let mut current = dir.to_path_buf();
    loop {
        for name in CONFIG_FILES {
            let candidate = current.join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
        if !current.pop() {
            return None;
        }
    }
}

/// Check if Node.js is available on the system.
pub fn node_available() -> bool {
    Command::new("node")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Load the config from a JS config file by evaluating it with Node.js.
pub fn load_config_file(config_path: &Path) -> Result<Config, String> {
    let abs_path = if config_path.is_absolute() {
        config_path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| format!("Failed to get cwd: {e}"))?
            .join(config_path)
    };

    let output = Command::new("node")
        .arg("-e")
        .arg(EVAL_SCRIPT)
        .arg(&abs_path)
        .output()
        .map_err(|e| format!("Failed to run node: {e}. Is Node.js installed?"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "Failed to load config file {}: {stderr}",
            config_path.display()
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let objects: Vec<ConfigObject> =
        serde_json::from_str(&stdout).map_err(|e| format!("Failed to parse config JSON: {e}"))?;

    let config_dir = abs_path.parent().unwrap_or(Path::new(".")).to_path_buf();
    Ok(Config { objects, config_dir })
}

/// Try to load config from the current working directory.
/// Returns `Ok(None)` if no config file is found or if Node.js is not installed.
pub fn load_config(start_dir: &Path) -> Result<Option<Config>, String> {
    match find_config_file(start_dir) {
        Some(path) => {
            if !node_available() {
                eprintln!(
                    "Warning: Found {} but Node.js is not installed. \
                     All rules will run at default severity.",
                    path.display()
                );
                return Ok(None);
            }
            load_config_file(&path).map(Some)
        }
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_severity_off_string() {
        let setting = RuleSetting::Severity(SeverityValue::String("off".to_string()));
        assert_eq!(parse_severity(&setting), ConfigSeverity::Off);
    }

    #[test]
    fn parse_severity_warn_number() {
        let setting = RuleSetting::Severity(SeverityValue::Number(1));
        assert_eq!(parse_severity(&setting), ConfigSeverity::Warn);
    }

    #[test]
    fn parse_severity_error_number() {
        let setting = RuleSetting::Severity(SeverityValue::Number(2));
        assert_eq!(parse_severity(&setting), ConfigSeverity::Error);
    }

    #[test]
    fn parse_severity_array_form() {
        let setting = RuleSetting::WithOptions(vec![serde_json::json!("warn")]);
        assert_eq!(parse_severity(&setting), ConfigSeverity::Warn);
    }

    #[test]
    fn resolve_rules_no_files_pattern() {
        let config = Config {
            config_dir: PathBuf::new(),
            objects: vec![ConfigObject {
                files: vec![],
                ignores: vec![],
                rules: HashMap::from([(
                    "no-var".to_string(),
                    RuleSetting::Severity(SeverityValue::String("error".to_string())),
                )]),
            }],
        };
        let resolved = config.resolve_rules_for_file("test.js");
        assert_eq!(resolved["no-var"].severity, ConfigSeverity::Error);
    }

    #[test]
    fn resolve_rules_with_files_match() {
        let config = Config {
            config_dir: PathBuf::new(),
            objects: vec![ConfigObject {
                files: vec![StringOrStrings::Single("**/*.js".to_string())],
                ignores: vec![],
                rules: HashMap::from([(
                    "eqeqeq".to_string(),
                    RuleSetting::Severity(SeverityValue::String("warn".to_string())),
                )]),
            }],
        };
        let resolved = config.resolve_rules_for_file("src/foo.js");
        assert_eq!(resolved["eqeqeq"].severity, ConfigSeverity::Warn);
    }

    #[test]
    fn resolve_rules_with_files_no_match() {
        let config = Config {
            config_dir: PathBuf::new(),
            objects: vec![ConfigObject {
                files: vec![StringOrStrings::Single("**/*.ts".to_string())],
                ignores: vec![],
                rules: HashMap::from([(
                    "eqeqeq".to_string(),
                    RuleSetting::Severity(SeverityValue::String("warn".to_string())),
                )]),
            }],
        };
        let resolved = config.resolve_rules_for_file("src/foo.js");
        assert!(resolved.is_empty());
    }

    #[test]
    fn resolve_rules_ignores() {
        let config = Config {
            config_dir: PathBuf::new(),
            objects: vec![ConfigObject {
                files: vec![],
                ignores: vec![StringOrStrings::Single("vendor/**".to_string())],
                rules: HashMap::from([(
                    "no-var".to_string(),
                    RuleSetting::Severity(SeverityValue::Number(2)),
                )]),
            }],
        };
        let resolved = config.resolve_rules_for_file("vendor/lib.js");
        assert!(resolved.is_empty());
    }

    #[test]
    fn resolve_rules_later_overrides_earlier() {
        let config = Config {
            config_dir: PathBuf::new(),
            objects: vec![
                ConfigObject {
                    files: vec![],
                    ignores: vec![],
                    rules: HashMap::from([(
                        "no-var".to_string(),
                        RuleSetting::Severity(SeverityValue::String("error".to_string())),
                    )]),
                },
                ConfigObject {
                    files: vec![],
                    ignores: vec![],
                    rules: HashMap::from([(
                        "no-var".to_string(),
                        RuleSetting::Severity(SeverityValue::String("off".to_string())),
                    )]),
                },
            ],
        };
        let resolved = config.resolve_rules_for_file("test.js");
        assert_eq!(resolved["no-var"].severity, ConfigSeverity::Off);
    }

    #[test]
    fn deserialize_config_json() {
        let json = r#"[
            {
                "files": ["**/*.js"],
                "rules": {
                    "no-var": "warn",
                    "eqeqeq": 2,
                    "no-debugger": ["error"]
                }
            },
            {
                "ignores": ["dist/**"]
            }
        ]"#;
        let objects: Vec<ConfigObject> = serde_json::from_str(json).unwrap();
        assert_eq!(objects.len(), 2);
        assert_eq!(objects[0].rules.len(), 3);
        assert_eq!(objects[1].ignores.len(), 1);
    }

    #[test]
    fn parse_severity_zero_is_off() {
        let setting = RuleSetting::Severity(SeverityValue::Number(0));
        assert_eq!(parse_severity(&setting), ConfigSeverity::Off);
    }

    #[test]
    fn parse_severity_array_numeric() {
        let setting = RuleSetting::WithOptions(vec![serde_json::json!(0)]);
        assert_eq!(parse_severity(&setting), ConfigSeverity::Off);
    }

    #[test]
    fn parse_severity_unknown_string_defaults_to_error() {
        let setting = RuleSetting::Severity(SeverityValue::String("invalid".to_string()));
        assert_eq!(parse_severity(&setting), ConfigSeverity::Error);
    }

    #[test]
    fn resolve_multiple_rules_in_one_object() {
        let config = Config {
            config_dir: PathBuf::new(),
            objects: vec![ConfigObject {
                files: vec![],
                ignores: vec![],
                rules: HashMap::from([
                    (
                        "no-var".to_string(),
                        RuleSetting::Severity(SeverityValue::String("error".to_string())),
                    ),
                    (
                        "eqeqeq".to_string(),
                        RuleSetting::Severity(SeverityValue::String("warn".to_string())),
                    ),
                    (
                        "no-debugger".to_string(),
                        RuleSetting::Severity(SeverityValue::String("off".to_string())),
                    ),
                ]),
            }],
        };
        let resolved = config.resolve_rules_for_file("test.js");
        assert_eq!(resolved.len(), 3);
        assert_eq!(resolved["no-var"].severity, ConfigSeverity::Error);
        assert_eq!(resolved["eqeqeq"].severity, ConfigSeverity::Warn);
        assert_eq!(resolved["no-debugger"].severity, ConfigSeverity::Off);
    }

    #[test]
    fn resolve_rules_ignores_do_not_affect_non_matching_files() {
        let config = Config {
            config_dir: PathBuf::new(),
            objects: vec![ConfigObject {
                files: vec![],
                ignores: vec![StringOrStrings::Single("vendor/**".to_string())],
                rules: HashMap::from([(
                    "no-var".to_string(),
                    RuleSetting::Severity(SeverityValue::Number(2)),
                )]),
            }],
        };
        let resolved = config.resolve_rules_for_file("src/app.js");
        assert_eq!(resolved["no-var"].severity, ConfigSeverity::Error);
    }

    #[test]
    fn resolve_rules_files_multiple_patterns() {
        let config = Config {
            config_dir: PathBuf::new(),
            objects: vec![ConfigObject {
                files: vec![StringOrStrings::Multiple(vec![
                    "**/*.js".to_string(),
                    "**/*.mjs".to_string(),
                ])],
                ignores: vec![],
                rules: HashMap::from([(
                    "eqeqeq".to_string(),
                    RuleSetting::Severity(SeverityValue::String("warn".to_string())),
                )]),
            }],
        };
        assert_eq!(
            config.resolve_rules_for_file("src/foo.mjs")["eqeqeq"].severity,
            ConfigSeverity::Warn
        );
        assert_eq!(
            config.resolve_rules_for_file("src/foo.js")["eqeqeq"].severity,
            ConfigSeverity::Warn
        );
        assert!(config.resolve_rules_for_file("src/foo.ts").is_empty());
    }

    #[test]
    fn resolve_empty_config_gives_no_rules() {
        let config = Config { config_dir: PathBuf::new(), objects: vec![] };
        let resolved = config.resolve_rules_for_file("test.js");
        assert!(resolved.is_empty());
    }

    #[test]
    fn resolve_ignore_only_object_gives_no_rules() {
        let config = Config {
            config_dir: PathBuf::new(),
            objects: vec![ConfigObject {
                files: vec![],
                ignores: vec![StringOrStrings::Single("dist/**".to_string())],
                rules: HashMap::new(),
            }],
        };
        let resolved = config.resolve_rules_for_file("src/app.js");
        assert!(resolved.is_empty());
    }

    #[test]
    fn resolve_partial_override_merges_rules() {
        let config = Config {
            config_dir: PathBuf::new(),
            objects: vec![
                ConfigObject {
                    files: vec![],
                    ignores: vec![],
                    rules: HashMap::from([
                        (
                            "no-var".to_string(),
                            RuleSetting::Severity(SeverityValue::String("error".to_string())),
                        ),
                        (
                            "eqeqeq".to_string(),
                            RuleSetting::Severity(SeverityValue::String("error".to_string())),
                        ),
                    ]),
                },
                ConfigObject {
                    files: vec![],
                    ignores: vec![],
                    rules: HashMap::from([(
                        "no-var".to_string(),
                        RuleSetting::Severity(SeverityValue::String("warn".to_string())),
                    )]),
                },
            ],
        };
        let resolved = config.resolve_rules_for_file("test.js");
        // no-var overridden to warn, eqeqeq stays error
        assert_eq!(resolved["no-var"].severity, ConfigSeverity::Warn);
        assert_eq!(resolved["eqeqeq"].severity, ConfigSeverity::Error);
    }

    #[test]
    fn resolve_file_specific_override() {
        let config = Config {
            config_dir: PathBuf::new(),
            objects: vec![
                ConfigObject {
                    files: vec![],
                    ignores: vec![],
                    rules: HashMap::from([(
                        "no-var".to_string(),
                        RuleSetting::Severity(SeverityValue::String("error".to_string())),
                    )]),
                },
                ConfigObject {
                    files: vec![StringOrStrings::Single("test/**".to_string())],
                    ignores: vec![],
                    rules: HashMap::from([(
                        "no-var".to_string(),
                        RuleSetting::Severity(SeverityValue::String("off".to_string())),
                    )]),
                },
            ],
        };
        // Test files get no-var turned off
        assert_eq!(
            config.resolve_rules_for_file("test/foo.js")["no-var"].severity,
            ConfigSeverity::Off
        );
        // Source files keep no-var as error
        assert_eq!(
            config.resolve_rules_for_file("src/foo.js")["no-var"].severity,
            ConfigSeverity::Error
        );
    }

    #[test]
    fn find_config_file_in_dir() {
        let dir = std::env::temp_dir().join("turbolint_test_find_config");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join("eslint.config.js");
        std::fs::write(&config_path, "module.exports = [];").unwrap();

        let found = find_config_file(&dir);
        assert_eq!(found, Some(config_path));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn find_config_file_walks_up() {
        let parent = std::env::temp_dir().join("turbolint_test_walk_up");
        let child = parent.join("src");
        let _ = std::fs::remove_dir_all(&parent);
        std::fs::create_dir_all(&child).unwrap();
        let config_path = parent.join("eslint.config.js");
        std::fs::write(&config_path, "module.exports = [];").unwrap();

        let found = find_config_file(&child);
        assert_eq!(found, Some(config_path));

        std::fs::remove_dir_all(&parent).unwrap();
    }

    #[test]
    fn find_config_file_prefers_js_over_mjs() {
        let dir = std::env::temp_dir().join("turbolint_test_priority");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("eslint.config.js"), "").unwrap();
        std::fs::write(dir.join("eslint.config.mjs"), "").unwrap();

        let found = find_config_file(&dir);
        assert_eq!(found, Some(dir.join("eslint.config.js")));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_config_file_with_node() {
        if !node_available() {
            return; // Skip if Node.js not installed
        }
        let dir = std::env::temp_dir().join("turbolint_test_load");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join("eslint.config.cjs");
        std::fs::write(
            &config_path,
            r#"module.exports = [
                { rules: { "no-var": "error", "eqeqeq": "warn" } },
                { ignores: ["dist/**"] }
            ];"#,
        )
        .unwrap();

        let config = load_config_file(&config_path).unwrap();
        assert_eq!(config.objects.len(), 2);
        let resolved = config.resolve_rules_for_file("src/app.js");
        assert_eq!(resolved["no-var"].severity, ConfigSeverity::Error);
        assert_eq!(resolved["eqeqeq"].severity, ConfigSeverity::Warn);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_config_file_single_object() {
        if !node_available() {
            return;
        }
        let dir = std::env::temp_dir().join("turbolint_test_single_obj");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join("eslint.config.cjs");
        // Non-array config (single object) should be wrapped into array
        std::fs::write(
            &config_path,
            r#"module.exports = { rules: { "no-debugger": "error" } };"#,
        )
        .unwrap();

        let config = load_config_file(&config_path).unwrap();
        assert_eq!(config.objects.len(), 1);
        let resolved = config.resolve_rules_for_file("test.js");
        assert_eq!(resolved["no-debugger"].severity, ConfigSeverity::Error);

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_config_file_numeric_severity() {
        if !node_available() {
            return;
        }
        let dir = std::env::temp_dir().join("turbolint_test_numeric");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join("eslint.config.cjs");
        std::fs::write(
            &config_path,
            r#"module.exports = [{ rules: { "no-var": 2, "eqeqeq": 1, "no-debugger": 0 } }];"#,
        )
        .unwrap();

        let config = load_config_file(&config_path).unwrap();
        let resolved = config.resolve_rules_for_file("test.js");
        assert_eq!(resolved["no-var"].severity, ConfigSeverity::Error);
        assert_eq!(resolved["eqeqeq"].severity, ConfigSeverity::Warn);
        assert_eq!(resolved["no-debugger"].severity, ConfigSeverity::Off);

        std::fs::remove_dir_all(&dir).unwrap();
    }
}
