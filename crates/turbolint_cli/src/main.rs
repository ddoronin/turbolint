use std::fs;
use std::path::Path;
use std::process;

use clap::Parser;
use turbolint_config::{Config, ConfigSeverity};
use turbolint_core::line_index::LineIndex;
use turbolint_core::{Diagnostic, Linter, Rule, Severity};
use turbolint_rules::all_rules;

#[derive(Parser)]
#[command(name = "turbolint", about = "A Rust reimplementation of ESLint")]
struct Cli {
    /// Files, directories, or glob patterns to lint
    files: Vec<String>,

    /// Automatically fix problems
    #[arg(long)]
    fix: bool,
}

const JS_EXTENSIONS: &[&str] = &["js", "mjs", "cjs"];

fn is_glob_pattern(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[')
}

fn resolve_paths(args: &[String]) -> Vec<String> {
    let mut paths = Vec::new();
    for arg in args {
        if is_glob_pattern(arg) {
            match glob::glob(arg) {
                Ok(entries) => {
                    for entry in entries.flatten() {
                        if entry.is_file() {
                            paths.push(entry.display().to_string());
                        }
                    }
                }
                Err(e) => eprintln!("Invalid glob pattern '{arg}': {e}"),
            }
        } else if Path::new(arg).is_dir() {
            for ext in JS_EXTENSIONS {
                let pattern = format!("{arg}/**/*.{ext}");
                if let Ok(entries) = glob::glob(&pattern) {
                    for entry in entries.flatten() {
                        paths.push(entry.display().to_string());
                    }
                }
            }
        } else {
            paths.push(arg.clone());
        }
    }
    paths.sort();
    paths.dedup();
    paths
}

fn pluralize(word: &str, count: usize) -> String {
    if count == 1 {
        word.to_string()
    } else {
        format!("{word}s")
    }
}

fn format_message(msg: &str) -> &str {
    // ESLint strips trailing period from messages
    msg.strip_suffix('.').unwrap_or(msg)
}

fn print_results(path: &str, diagnostics: &[Diagnostic], line_index: &LineIndex) {
    // Find the widest line number and column for alignment
    let mut max_line_width = 0;
    let mut max_col_width = 0;
    let positions: Vec<(usize, usize)> = diagnostics
        .iter()
        .map(|d| {
            let (line, col) = line_index.line_col(d.span.start);
            let line_str = line.to_string();
            let col_str = col.to_string();
            if line_str.len() > max_line_width {
                max_line_width = line_str.len();
            }
            if col_str.len() > max_col_width {
                max_col_width = col_str.len();
            }
            (line, col)
        })
        .collect();

    println!();
    println!("{path}");

    for (d, (line, col)) in diagnostics.iter().zip(positions.iter()) {
        println!(
            "  {line:>lw$}:{col:<cw$}  {severity:<7}  {msg}  {rule}",
            lw = max_line_width,
            cw = max_col_width,
            severity = d.severity,
            msg = format_message(&d.message),
            rule = d.rule_id,
        );
    }
}

/// Build a Linter for a specific file given the config and all available rules.
/// If config is None, all rules run at default severity.
/// If config is Some, only rules mentioned in the resolved config run (at configured severity).
/// Rules not mentioned in the config but available run at default severity (matching ESLint behavior).
fn build_linter_for_file(
    config: &Option<Config>,
    file_path: &str,
) -> Linter {
    let rules = all_rules();

    let config = match config {
        Some(c) => c,
        None => return Linter::new(rules),
    };

    let resolved = config.resolve_rules_for_file(file_path);

    // If config has no rule entries at all (e.g. only ignores/files), run all rules at defaults
    if resolved.is_empty() && config.objects.iter().all(|o| o.rules.is_empty()) {
        return Linter::new(rules);
    }

    let mut filtered_rules: Vec<Box<dyn Rule>> = Vec::new();
    let mut severities: Vec<Severity> = Vec::new();

    for rule in rules {
        let name = rule.name();
        match resolved.get(name) {
            Some(rc) => {
                if rc.severity == ConfigSeverity::Off {
                    continue; // Rule disabled
                }
                let severity = match rc.severity {
                    ConfigSeverity::Warn => Severity::Warning,
                    ConfigSeverity::Error => Severity::Error,
                    ConfigSeverity::Off => unreachable!(),
                };
                severities.push(severity);
                filtered_rules.push(rule);
            }
            None => {
                // Rule not mentioned in config — if the config defines any rules,
                // treat unmentioned rules as disabled (ESLint behavior: only
                // explicitly configured rules run).
                // But if no rules are configured at all, keep defaults.
                if !resolved.is_empty() {
                    continue;
                }
                severities.push(rule.default_severity());
                filtered_rules.push(rule);
            }
        }
    }

    Linter::with_severities(filtered_rules, severities)
}

fn main() {
    let cli = Cli::parse();

    if cli.files.is_empty() {
        eprintln!("No files specified.");
        process::exit(1);
    }

    let resolved = resolve_paths(&cli.files);
    if resolved.is_empty() {
        eprintln!("No matching files found.");
        process::exit(1);
    }

    // Load config (if present)
    let config = match turbolint_config::load_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading config: {e}");
            process::exit(2);
        }
    };

    let mut error_count: usize = 0;
    let mut warning_count: usize = 0;
    let mut fixable_count: usize = 0;

    for path in &resolved {
        // Check if this file is globally ignored by config
        if let Some(ref cfg) = config {
            if is_globally_ignored(cfg, path) {
                continue;
            }
        }

        let source = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading {path}: {e}");
                error_count += 1;
                continue;
            }
        };

        let linter = build_linter_for_file(&config, path);

        if cli.fix {
            let result = linter.lint_and_fix(&source);
            if result.fixed {
                if let Err(e) = fs::write(path, &result.output) {
                    eprintln!("Error writing {path}: {e}");
                    error_count += 1;
                    continue;
                }
            }
            if result.diagnostics.is_empty() {
                continue;
            }
            for d in &result.diagnostics {
                match d.severity {
                    Severity::Error => error_count += 1,
                    Severity::Warning => warning_count += 1,
                }
            }
            print_results(path, &result.diagnostics, &result.line_index);
        } else {
            let result = linter.lint(&source);
            if result.diagnostics.is_empty() {
                continue;
            }
            for d in &result.diagnostics {
                match d.severity {
                    Severity::Error => error_count += 1,
                    Severity::Warning => warning_count += 1,
                }
                if d.fix.is_some() {
                    fixable_count += 1;
                }
            }
            print_results(path, &result.diagnostics, &result.line_index);
        }
    }

    let total = error_count + warning_count;
    if total > 0 {
        println!();
        println!(
            "\u{2716} {total} {} ({error_count} {}, {warning_count} {})",
            pluralize("problem", total),
            pluralize("error", error_count),
            pluralize("warning", warning_count),
        );
        if !cli.fix && fixable_count > 0 {
            println!(
                "  {} {} potentially fixable with the `--fix` option.",
                fixable_count,
                pluralize("problem", fixable_count),
            );
        }
        println!();
    }

    process::exit(if error_count > 0 { 1 } else { 0 });
}

/// Check if a file is globally ignored (config objects with only `ignores` and no `files`).
fn is_globally_ignored(config: &Config, file_path: &str) -> bool {
    for obj in &config.objects {
        if obj.files.is_empty() && !obj.ignores.is_empty() && obj.rules.is_empty() {
            // This is a global ignore-only config object
            for pattern_set in &obj.ignores {
                let patterns = match pattern_set {
                    turbolint_config::StringOrStrings::Single(s) => vec![s.as_str()],
                    turbolint_config::StringOrStrings::Multiple(v) => {
                        v.iter().map(|s| s.as_str()).collect()
                    }
                };
                for pattern in patterns {
                    if glob::Pattern::new(pattern)
                        .map(|p| p.matches(file_path))
                        .unwrap_or(false)
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
}
