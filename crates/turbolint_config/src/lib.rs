use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Severity can be a string ("off"/"warn"/"error") or a number (0/1/2).
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum RuleSetting {
    String(String),
    Number(u8),
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

/// Native turbolint configuration (`.turbolintrc`, `.turbolintrc.json`, `.turbolintrc.toml`).
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub rules: HashMap<String, RuleSetting>,
    #[serde(default)]
    pub ignores: Vec<String>,
    /// Directory containing the config file (not deserialized).
    #[serde(skip)]
    pub config_dir: PathBuf,
}

impl Config {
    /// Check whether the given file path (relative to config_dir) is ignored.
    pub fn is_ignored(&self, file_path: &str) -> bool {
        let rel_path = Path::new(file_path)
            .strip_prefix(&self.config_dir)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| file_path.to_string());

        for pattern in &self.ignores {
            if glob::Pattern::new(pattern)
                .map(|p| p.matches(&rel_path))
                .unwrap_or(false)
            {
                return true;
            }
        }
        false
    }

    /// Returns true if this config has any rule entries.
    pub fn has_rules(&self) -> bool {
        !self.rules.is_empty()
    }

    /// Get the resolved severity for a specific rule, if configured.
    pub fn rule_severity(&self, rule_name: &str) -> Option<ConfigSeverity> {
        self.rules.get(rule_name).map(parse_severity)
    }
}

fn parse_severity(setting: &RuleSetting) -> ConfigSeverity {
    match setting {
        RuleSetting::String(s) => match s.as_str() {
            "off" => ConfigSeverity::Off,
            "warn" => ConfigSeverity::Warn,
            "error" => ConfigSeverity::Error,
            _ => ConfigSeverity::Error,
        },
        RuleSetting::Number(0) => ConfigSeverity::Off,
        RuleSetting::Number(1) => ConfigSeverity::Warn,
        RuleSetting::Number(_) => ConfigSeverity::Error,
    }
}

/// Config file names in priority order.
const CONFIG_FILES: &[&str] = &[
    ".turbolintrc.toml",
    ".turbolintrc.json",
    ".turbolintrc",
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

/// Load the config from a file (JSON or TOML).
pub fn load_config_file(config_path: &Path) -> Result<Config, String> {
    let abs_path = if config_path.is_absolute() {
        config_path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|e| format!("Failed to get cwd: {e}"))?
            .join(config_path)
    };

    let content = std::fs::read_to_string(&abs_path)
        .map_err(|e| format!("Failed to read {}: {e}", abs_path.display()))?;

    let file_name = abs_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    let mut config: Config = if file_name.ends_with(".toml") {
        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse TOML config {}: {e}", abs_path.display()))?
    } else {
        // JSON (both .turbolintrc and .turbolintrc.json)
        serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse JSON config {}: {e}", abs_path.display()))?
    };

    config.config_dir = abs_path.parent().unwrap_or(Path::new(".")).to_path_buf();
    Ok(config)
}

/// Try to load config starting from `start_dir`, walking up.
/// Returns `Ok(None)` if no config file is found.
pub fn load_config(start_dir: &Path) -> Result<Option<Config>, String> {
    match find_config_file(start_dir) {
        Some(path) => load_config_file(&path).map(Some),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_severity_off_string() {
        let setting = RuleSetting::String("off".to_string());
        assert_eq!(parse_severity(&setting), ConfigSeverity::Off);
    }

    #[test]
    fn parse_severity_warn_number() {
        let setting = RuleSetting::Number(1);
        assert_eq!(parse_severity(&setting), ConfigSeverity::Warn);
    }

    #[test]
    fn parse_severity_error_number() {
        let setting = RuleSetting::Number(2);
        assert_eq!(parse_severity(&setting), ConfigSeverity::Error);
    }

    #[test]
    fn parse_severity_zero_is_off() {
        let setting = RuleSetting::Number(0);
        assert_eq!(parse_severity(&setting), ConfigSeverity::Off);
    }

    #[test]
    fn parse_severity_unknown_string_defaults_to_error() {
        let setting = RuleSetting::String("invalid".to_string());
        assert_eq!(parse_severity(&setting), ConfigSeverity::Error);
    }

    #[test]
    fn deserialize_json_config() {
        let json = r#"{
            "rules": {
                "no-var": "warn",
                "eqeqeq": "error",
                "no-debugger": "off"
            },
            "ignores": ["dist/**"]
        }"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.rules.len(), 3);
        assert_eq!(config.ignores.len(), 1);
    }

    #[test]
    fn deserialize_toml_config() {
        let toml_str = r#"
ignores = ["dist/**", "node_modules/**"]

[rules]
no-var = "error"
eqeqeq = "warn"
no-debugger = "off"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.rules.len(), 3);
        assert_eq!(config.ignores.len(), 2);
    }

    #[test]
    fn is_ignored_matches() {
        let config = Config {
            rules: HashMap::new(),
            ignores: vec!["vendor/**".to_string()],
            config_dir: PathBuf::new(),
        };
        assert!(config.is_ignored("vendor/lib.js"));
        assert!(!config.is_ignored("src/app.js"));
    }

    #[test]
    fn rule_severity_returns_correct() {
        let mut rules = HashMap::new();
        rules.insert("no-var".to_string(), RuleSetting::String("error".to_string()));
        rules.insert("eqeqeq".to_string(), RuleSetting::String("warn".to_string()));
        rules.insert("no-debugger".to_string(), RuleSetting::String("off".to_string()));
        let config = Config {
            rules,
            ignores: vec![],
            config_dir: PathBuf::new(),
        };
        assert_eq!(config.rule_severity("no-var"), Some(ConfigSeverity::Error));
        assert_eq!(config.rule_severity("eqeqeq"), Some(ConfigSeverity::Warn));
        assert_eq!(config.rule_severity("no-debugger"), Some(ConfigSeverity::Off));
        assert_eq!(config.rule_severity("unknown"), None);
    }

    #[test]
    fn has_rules_check() {
        let empty = Config { rules: HashMap::new(), ignores: vec![], config_dir: PathBuf::new() };
        assert!(!empty.has_rules());

        let mut rules = HashMap::new();
        rules.insert("no-var".to_string(), RuleSetting::String("error".to_string()));
        let with_rules = Config { rules, ignores: vec![], config_dir: PathBuf::new() };
        assert!(with_rules.has_rules());
    }

    #[test]
    fn find_config_toml() {
        let dir = std::env::temp_dir().join("turbolint_test_find_toml");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join(".turbolintrc.toml");
        std::fs::write(&config_path, "[rules]\n").unwrap();

        let found = find_config_file(&dir);
        assert_eq!(found, Some(config_path));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn find_config_json() {
        let dir = std::env::temp_dir().join("turbolint_test_find_json");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join(".turbolintrc.json");
        std::fs::write(&config_path, "{}").unwrap();

        let found = find_config_file(&dir);
        assert_eq!(found, Some(config_path));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn find_config_walks_up() {
        let parent = std::env::temp_dir().join("turbolint_test_walk_up_native");
        let child = parent.join("src");
        let _ = std::fs::remove_dir_all(&parent);
        std::fs::create_dir_all(&child).unwrap();
        let config_path = parent.join(".turbolintrc.json");
        std::fs::write(&config_path, "{}").unwrap();

        let found = find_config_file(&child);
        assert_eq!(found, Some(config_path));

        std::fs::remove_dir_all(&parent).unwrap();
    }

    #[test]
    fn find_config_toml_has_priority_over_json() {
        let dir = std::env::temp_dir().join("turbolint_test_priority_native");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(".turbolintrc.toml"), "[rules]\n").unwrap();
        std::fs::write(dir.join(".turbolintrc.json"), "{}").unwrap();

        let found = find_config_file(&dir);
        assert_eq!(found, Some(dir.join(".turbolintrc.toml")));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_json_config_file() {
        let dir = std::env::temp_dir().join("turbolint_test_load_json");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join(".turbolintrc.json");
        std::fs::write(
            &config_path,
            r#"{ "rules": { "no-var": "error", "eqeqeq": "warn" } }"#,
        ).unwrap();

        let config = load_config_file(&config_path).unwrap();
        assert_eq!(config.rule_severity("no-var"), Some(ConfigSeverity::Error));
        assert_eq!(config.rule_severity("eqeqeq"), Some(ConfigSeverity::Warn));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_toml_config_file() {
        let dir = std::env::temp_dir().join("turbolint_test_load_toml");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join(".turbolintrc.toml");
        std::fs::write(
            &config_path,
            "[rules]\nno-var = \"error\"\neqeqeq = \"warn\"\n",
        ).unwrap();

        let config = load_config_file(&config_path).unwrap();
        assert_eq!(config.rule_severity("no-var"), Some(ConfigSeverity::Error));
        assert_eq!(config.rule_severity("eqeqeq"), Some(ConfigSeverity::Warn));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn load_turbolintrc_no_extension_as_json() {
        let dir = std::env::temp_dir().join("turbolint_test_load_noext");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let config_path = dir.join(".turbolintrc");
        std::fs::write(
            &config_path,
            r#"{ "rules": { "no-debugger": "error" } }"#,
        ).unwrap();

        let config = load_config_file(&config_path).unwrap();
        assert_eq!(config.rule_severity("no-debugger"), Some(ConfigSeverity::Error));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn numeric_severity_in_json() {
        let json = r#"{ "rules": { "no-var": 2, "eqeqeq": 1, "no-debugger": 0 } }"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.rule_severity("no-var"), Some(ConfigSeverity::Error));
        assert_eq!(config.rule_severity("eqeqeq"), Some(ConfigSeverity::Warn));
        assert_eq!(config.rule_severity("no-debugger"), Some(ConfigSeverity::Off));
    }
}
