use std::fs;
use std::path::Path;
use std::process;

use clap::Parser;
use colored::Colorize;
use rayon::prelude::*;
use turbolint_config::{Config, ConfigSeverity};
use turbolint_core::line_index::LineIndex;
use turbolint_core::{Diagnostic, Language, Linter, Rule, Severity};
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

const JS_EXTENSIONS: &[&str] = &["js", "mjs", "cjs", "ts", "mts", "cts", "tsx"];
const DEFAULT_IGNORES: &[&str] = &["node_modules"];

fn is_glob_pattern(s: &str) -> bool {
    s.contains('*') || s.contains('?') || s.contains('[')
}

fn is_default_ignored(path: &Path) -> bool {
    path.components().any(|c| {
        DEFAULT_IGNORES
            .iter()
            .any(|ig| c.as_os_str() == std::ffi::OsStr::new(ig))
    })
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
                        if entry.is_file() && !is_default_ignored(&entry) {
                            paths.push(entry.display().to_string());
                        }
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

fn print_results(path: &str, diagnostics: &[Diagnostic], line_index: &LineIndex, source: &str) {
    let source_lines: Vec<&str> = source.lines().collect();

    let positions: Vec<(usize, usize)> = diagnostics
        .iter()
        .map(|d| line_index.line_col(d.span.start))
        .collect();

    for (d, (line, col)) in diagnostics.iter().zip(positions.iter()) {
        // Header: error[rule-id]: message
        let (severity_label, severity_color): (&str, fn(&str) -> colored::ColoredString) = match d.severity {
            Severity::Error => ("error", |s: &str| s.red()),
            Severity::Warning => ("warning", |s: &str| s.yellow()),
        };
        println!(
            "{}{}{}{} {}",
            severity_color(severity_label).bold(),
            "[".bold(),
            severity_color(d.rule_id).bold(),
            "]".bold(),
            format_message(&d.message).bold(),
        );

        // Location: --> path:line:col
        let line_idx = line.saturating_sub(1);
        let start = line_idx.saturating_sub(2);
        let end = (line_idx + 3).min(source_lines.len());
        let gutter_width = end.to_string().len();

        println!(
            " {} {}:{}:{}",
            format!("{:>gw$}-->", "", gw = gutter_width).cyan().bold(),
            path,
            line,
            col,
        );

        // Top border
        println!(" {} {}", format!("{:>gw$}", "", gw = gutter_width).cyan().bold(), "|".cyan().bold());

        for i in start..end {
            let ln = i + 1;
            if i == line_idx {
                println!(
                    " {} {} {}",
                    format!("{ln:>gw$}", gw = gutter_width).cyan().bold(),
                    "|".cyan().bold(),
                    source_lines[i],
                );

                // Underline with ^
                let col_0 = col.saturating_sub(1);
                let span_len = if d.span.end > d.span.start {
                    let len = (d.span.end - d.span.start) as usize;
                    let remaining = source_lines[i].len().saturating_sub(col_0);
                    len.min(remaining).max(1)
                } else {
                    1
                };
                let padding = " ".repeat(col_0);
                let underline = "^".repeat(span_len);
                println!(
                    " {} {} {}{}",
                    format!("{:>gw$}", "", gw = gutter_width).cyan().bold(),
                    "|".cyan().bold(),
                    padding,
                    severity_color(&underline).bold(),
                );
            } else {
                println!(
                    " {} {} {}",
                    format!("{ln:>gw$}", gw = gutter_width).cyan().bold(),
                    "|".cyan().bold(),
                    source_lines[i].dimmed(),
                );
            }
        }

        // Bottom border
        println!(" {} {}", format!("{:>gw$}", "", gw = gutter_width).cyan().bold(), "|".cyan().bold());
        println!();
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

    let mut resolved = config.resolve_rules_for_file(file_path);

    // Map @typescript-eslint/* rules to core equivalents.
    // When tseslint.configs.recommended disables a core rule and enables its
    // TS-aware replacement (e.g. no-unused-vars → @typescript-eslint/no-unused-vars),
    // use the TS rule's severity for the core rule.
    let ts_prefix = "@typescript-eslint/";
    let ts_overrides: Vec<(String, turbolint_config::ResolvedRuleConfig)> = resolved
        .iter()
        .filter_map(|(name, rc)| {
            let core_name = name.strip_prefix(ts_prefix)?;
            // Only apply if the core rule is off or absent
            let core_off = resolved
                .get(core_name)
                .map_or(true, |c| c.severity == ConfigSeverity::Off);
            if core_off && rc.severity != ConfigSeverity::Off {
                Some((core_name.to_string(), rc.clone()))
            } else {
                None
            }
        })
        .collect();
    for (core_name, rc) in ts_overrides {
        resolved.insert(core_name, rc);
    }

    let config_has_any_rules = config.objects.iter().any(|o| !o.rules.is_empty());

    // If config has no rule entries at all (e.g. only ignores/files), run all rules at defaults
    if !config_has_any_rules {
        return Linter::new(rules);
    }

    // If config defines rules but none resolved for this file (e.g. file doesn't match
    // any `files` pattern), only run rules that resolved. If resolved is empty, no rules run.
    let mut filtered_rules: Vec<Box<dyn Rule>> = Vec::new();
    let mut severities: Vec<Severity> = Vec::new();

    for rule in rules {
        let name = rule.name();
        if let Some(rc) = resolved.get(name) {
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
        // Rules not mentioned in resolved config are disabled (ESLint behavior:
        // only explicitly configured rules run)
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

    // Determine the config search directory from the input arguments.
    // Use the first CLI argument: if it's a directory use it directly,
    // otherwise use its parent. This mirrors ESLint's behaviour of
    // searching for eslint.config.js from the target directory upward.
    let config_search_dir = {
        let first = Path::new(&cli.files[0]);
        let abs = if first.is_absolute() {
            first.to_path_buf()
        } else {
            std::env::current_dir().unwrap_or_default().join(first)
        };
        if abs.is_dir() {
            abs
        } else {
            abs.parent().unwrap_or(&abs).to_path_buf()
        }
    };

    // Load config (if present)
    let config = match turbolint_config::load_config(&config_search_dir) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error loading config: {e}");
            process::exit(2);
        }
    };

    struct FileResult {
        path: String,
        source: String,
        diagnostics: Vec<Diagnostic>,
        line_index: LineIndex,
        error: bool, // true if this entry represents a read/write error (counted but no diagnostics)
    }

    let results: Vec<FileResult> = resolved
        .par_iter()
        .filter(|path| {
            if let Some(ref cfg) = config {
                !is_globally_ignored(cfg, path)
            } else {
                true
            }
        })
        .filter_map(|path| {
            let source = match fs::read_to_string(path) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error reading {path}: {e}");
                    return Some(FileResult {
                        path: path.clone(),
                        source: String::new(),
                        diagnostics: Vec::new(),
                        line_index: LineIndex::new(""),
                        error: true,
                    });
                }
            };

            let lang = Path::new(path)
                .extension()
                .and_then(|e| e.to_str())
                .and_then(Language::from_extension)
                .unwrap_or(Language::JavaScript);

            let linter = build_linter_for_file(&config, path);

            if cli.fix {
                let result = linter.lint_and_fix_lang(&source, lang);
                if result.fixed {
                    if let Err(e) = fs::write(path, &result.output) {
                        eprintln!("Error writing {path}: {e}");
                        return Some(FileResult {
                            path: path.clone(),
                            source: String::new(),
                            diagnostics: Vec::new(),
                            line_index: LineIndex::new(""),
                            error: true,
                        });
                    }
                }
                if result.diagnostics.is_empty() {
                    return None;
                }
                Some(FileResult {
                    path: path.clone(),
                    source: result.output,
                    diagnostics: result.diagnostics,
                    line_index: result.line_index,
                    error: false,
                })
            } else {
                let result = linter.lint_lang(&source, lang);
                if result.diagnostics.is_empty() {
                    return None;
                }
                Some(FileResult {
                    path: path.clone(),
                    source,
                    diagnostics: result.diagnostics,
                    line_index: result.line_index,
                    error: false,
                })
            }
        })
        .collect();

    let mut error_count: usize = 0;
    let mut warning_count: usize = 0;
    let mut fixable_count: usize = 0;

    // Sort results by path for deterministic output order
    let mut results = results;
    results.sort_by(|a, b| a.path.cmp(&b.path));

    for result in &results {
        if result.error {
            error_count += 1;
            continue;
        }
        for d in &result.diagnostics {
            match d.severity {
                Severity::Error => error_count += 1,
                Severity::Warning => warning_count += 1,
            }
            if !cli.fix && d.fix.is_some() {
                fixable_count += 1;
            }
        }
        print_results(&result.path, &result.diagnostics, &result.line_index, &result.source);
    }

    let total = error_count + warning_count;
    if total > 0 {
        println!();
        let summary = format!(
            "\u{2716} {total} {} ({error_count} {}, {warning_count} {})",
            pluralize("problem", total),
            pluralize("error", error_count),
            pluralize("warning", warning_count),
        );
        if error_count > 0 {
            println!("{}", summary.red().bold());
        } else {
            println!("{}", summary.yellow().bold());
        }
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
    let rel_path = Path::new(file_path)
        .strip_prefix(&config.config_dir)
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| file_path.to_string());

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
                        .map(|p| p.matches(&rel_path))
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
