use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::path::Path;
use std::process;

use clap::{Parser, ValueEnum};
use colored::Colorize;
use rayon::prelude::*;
use serde::Serialize;
use turbolint_config::{Config, ConfigSeverity};
use turbolint_core::line_index::LineIndex;
use turbolint_core::{Diagnostic, Language, Linter, Rule, Severity};
use turbolint_rules::all_rules;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Stylish,
    Json,
}

#[derive(Parser)]
#[command(name = "turbolint", about = "A Rust reimplementation of ESLint")]
struct Cli {
    /// Files, directories, or glob patterns to lint
    files: Vec<String>,

    /// Automatically fix problems
    #[arg(long)]
    fix: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Stylish)]
    format: OutputFormat,

    /// Read source from stdin
    #[arg(long)]
    stdin: bool,

    /// Filename for stdin (used for language detection and config resolution)
    #[arg(long, default_value = "<stdin>")]
    stdin_filename: String,

    /// Only report errors (suppress warnings)
    #[arg(long)]
    quiet: bool,

    /// Exit with code 1 if warning count exceeds this limit
    #[arg(long)]
    max_warnings: Option<i32>,

    /// Run only the specified rule(s). Can be repeated.
    #[arg(long)]
    rule: Vec<String>,

    /// Generate a default .turbolintrc configuration file
    #[arg(long)]
    init: bool,
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

// ── JSON output types (ESLint-compatible) ─────────────────────────────

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonFileResult {
    file_path: String,
    messages: Vec<JsonMessage>,
    error_count: usize,
    warning_count: usize,
    fixable_error_count: usize,
    fixable_warning_count: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct JsonMessage {
    rule_id: &'static str,
    severity: Severity,
    message: String,
    line: usize,
    column: usize,
    end_line: usize,
    end_column: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    fix: Option<JsonFix>,
}

#[derive(Serialize)]
struct JsonFix {
    range: [u32; 2],
    text: String,
}

fn to_json_result(
    path: &str,
    diagnostics: &[Diagnostic],
    line_index: &LineIndex,
    is_fix_mode: bool,
) -> JsonFileResult {
    let mut error_count = 0usize;
    let mut warning_count = 0usize;
    let mut fixable_error_count = 0usize;
    let mut fixable_warning_count = 0usize;

    let messages: Vec<JsonMessage> = diagnostics
        .iter()
        .map(|d| {
            match d.severity {
                Severity::Error => error_count += 1,
                Severity::Warning => warning_count += 1,
            }
            if !is_fix_mode && d.fix.is_some() {
                match d.severity {
                    Severity::Error => fixable_error_count += 1,
                    Severity::Warning => fixable_warning_count += 1,
                }
            }

            let (line, column) = line_index.line_col(d.span.start);
            let (end_line, end_column) = line_index.line_col(d.span.end);

            let fix = d.fix.as_ref().map(|f| JsonFix {
                range: [f.range.start, f.range.end],
                text: f.text.clone(),
            });

            JsonMessage {
                rule_id: d.rule_id,
                severity: d.severity,
                message: format_message(&d.message).to_string(),
                line,
                column,
                end_line,
                end_column,
                fix,
            }
        })
        .collect();

    JsonFileResult {
        file_path: std::path::absolute(path)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| path.to_string()),
        messages,
        error_count,
        warning_count,
        fixable_error_count,
        fixable_warning_count,
    }
}

// ── Linter construction ───────────────────────────────────────────────

/// Build a Linter for a specific file given the config, --rule filter, and all available rules.
fn build_linter_for_file(
    config: &Option<Config>,
    rule_filter: &HashSet<String>,
    file_path: &str,
) -> Linter {
    let rules = all_rules();

    // If --rule flag is set, only run those rules (ignore config for rule selection)
    if !rule_filter.is_empty() {
        let filtered: Vec<Box<dyn Rule>> = rules
            .into_iter()
            .filter(|r| rule_filter.contains(r.name()))
            .collect();
        return match config {
            Some(c) => {
                let mut severities = Vec::new();
                let mut kept = Vec::new();
                for rule in filtered {
                    let severity = match c.rule_severity(rule.name()) {
                        Some(ConfigSeverity::Off) => continue,
                        Some(ConfigSeverity::Warn) => Severity::Warning,
                        Some(ConfigSeverity::Error) => Severity::Error,
                        None => rule.default_severity(),
                    };
                    severities.push(severity);
                    kept.push(rule);
                }
                Linter::with_severities(kept, severities)
            }
            None => Linter::new(filtered),
        };
    }

    let config = match config {
        Some(c) => c,
        None => return Linter::new(rules),
    };

    // If config has no rule entries, run all rules at defaults
    if !config.has_rules() {
        return Linter::new(rules);
    }

    // Config defines rules: only run rules mentioned in config (at configured severity)
    let _ = file_path; // not needed for simple config model
    let mut filtered_rules: Vec<Box<dyn Rule>> = Vec::new();
    let mut severities: Vec<Severity> = Vec::new();

    for rule in rules {
        let name = rule.name();
        if let Some(sev) = config.rule_severity(name) {
            if sev == ConfigSeverity::Off {
                continue;
            }
            let severity = match sev {
                ConfigSeverity::Warn => Severity::Warning,
                ConfigSeverity::Error => Severity::Error,
                ConfigSeverity::Off => unreachable!(),
            };
            severities.push(severity);
            filtered_rules.push(rule);
        }
    }

    Linter::with_severities(filtered_rules, severities)
}

fn generate_default_config() -> String {
    let rules = all_rules();
    let mut lines = Vec::new();
    lines.push("{".to_string());
    lines.push("  \"rules\": {".to_string());

    let mut entries: Vec<(&'static str, &str)> = rules
        .iter()
        .map(|r| {
            let sev = match r.default_severity() {
                Severity::Error => "error",
                Severity::Warning => "warn",
            };
            (r.name(), sev)
        })
        .collect();
    entries.sort_by_key(|(name, _)| *name);

    for (i, (name, sev)) in entries.iter().enumerate() {
        let comma = if i + 1 < entries.len() { "," } else { "" };
        lines.push(format!("    \"{name}\": \"{sev}\"{comma}"));
    }

    lines.push("  },".to_string());
    lines.push("  \"ignores\": [\"node_modules/**\", \"dist/**\"]".to_string());
    lines.push("}".to_string());
    lines.join("\n") + "\n"
}

fn main() {
    let cli = Cli::parse();
    let rule_filter: HashSet<String> = cli.rule.iter().cloned().collect();

    // ── init mode ─────────────────────────────────────────────────────
    if cli.init {
        let config_path = Path::new(".turbolintrc");
        if config_path.exists() {
            eprintln!("Error: .turbolintrc already exists. Remove it first to reinitialize.");
            process::exit(1);
        }
        let content = generate_default_config();
        fs::write(config_path, &content).unwrap_or_else(|e| {
            eprintln!("Error writing .turbolintrc: {e}");
            process::exit(1);
        });
        println!("Created .turbolintrc with {} rules.", all_rules().len());
        process::exit(0);
    }

    // ── stdin mode ────────────────────────────────────────────────────
    if cli.stdin {
        let mut source = String::new();
        std::io::stdin().read_to_string(&mut source).unwrap_or_else(|e| {
            eprintln!("Error reading stdin: {e}");
            process::exit(1);
        });

        let filename = &cli.stdin_filename;

        let lang = Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .and_then(Language::from_extension)
            .unwrap_or(Language::JavaScript);

        // Config resolution: use stdin_filename's parent dir (if real path)
        let config_search_dir = {
            let p = Path::new(filename);
            let abs = if p.is_absolute() {
                p.to_path_buf()
            } else {
                std::env::current_dir().unwrap_or_default().join(p)
            };
            abs.parent().unwrap_or(&abs).to_path_buf()
        };

        let config = match turbolint_config::load_config(&config_search_dir) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error loading config: {e}");
                process::exit(2);
            }
        };

        let linter = build_linter_for_file(&config, &rule_filter, filename);

        if cli.fix {
            let result = linter.lint_and_fix_lang(&source, lang);
            // With --fix + --stdin, write fixed source to stdout
            print!("{}", result.output);
            return;
        }

        let result = linter.lint_lang(&source, lang);
        let mut diagnostics = result.diagnostics;

        if cli.quiet {
            diagnostics.retain(|d| d.severity == Severity::Error);
        }

        match cli.format {
            OutputFormat::Json => {
                let json_result = to_json_result(filename, &diagnostics, &result.line_index, false);
                println!("{}", serde_json::to_string(&[json_result]).unwrap());
            }
            OutputFormat::Stylish => {
                if !diagnostics.is_empty() {
                    print_results(filename, &diagnostics, &result.line_index, &source);
                }
                let error_count = diagnostics.iter().filter(|d| d.severity == Severity::Error).count();
                let warning_count = diagnostics.iter().filter(|d| d.severity == Severity::Warning).count();
                let total = error_count + warning_count;
                if total > 0 {
                    print_stylish_summary(error_count, warning_count, 0, false);
                }
                exit_with_counts(&cli, error_count, warning_count);
            }
        }

        let error_count = diagnostics.iter().filter(|d| d.severity == Severity::Error).count();
        let warning_count = diagnostics.iter().filter(|d| d.severity == Severity::Warning).count();
        exit_with_counts(&cli, error_count, warning_count);
    }

    // ── file mode ─────────────────────────────────────────────────────
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
        error: bool,
    }

    let lint_file = |path: &String| -> Option<FileResult> {
        if let Some(ref cfg) = config {
            if cfg.is_ignored(path) {
                return None;
            }
        }

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

        let linter = build_linter_for_file(&config, &rule_filter, path);

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
    };

    // Skip rayon for single-file to avoid thread pool init overhead
    let results: Vec<FileResult> = if resolved.len() == 1 {
        resolved.iter().filter_map(lint_file).collect()
    } else {
        resolved.par_iter().filter_map(lint_file).collect()
    };

    let mut error_count: usize = 0;
    let mut warning_count: usize = 0;
    let mut fixable_count: usize = 0;

    // Sort results by path for deterministic output order
    let mut results = results;
    results.sort_by(|a, b| a.path.cmp(&b.path));

    // Apply --quiet filter
    if cli.quiet {
        for result in &mut results {
            result.diagnostics.retain(|d| d.severity == Severity::Error);
        }
    }

    match cli.format {
        OutputFormat::Json => {
            let json_results: Vec<JsonFileResult> = results
                .iter()
                .filter(|r| !r.error)
                .map(|r| {
                    let jr = to_json_result(&r.path, &r.diagnostics, &r.line_index, cli.fix);
                    error_count += jr.error_count;
                    warning_count += jr.warning_count;
                    jr
                })
                .collect();
            // Count errors from read/write failures
            error_count += results.iter().filter(|r| r.error).count();
            println!("{}", serde_json::to_string(&json_results).unwrap());
        }
        OutputFormat::Stylish => {
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
                if !result.diagnostics.is_empty() {
                    print_results(&result.path, &result.diagnostics, &result.line_index, &result.source);
                }
            }

            let total = error_count + warning_count;
            if total > 0 {
                print_stylish_summary(error_count, warning_count, fixable_count, cli.fix);
            }
        }
    }

    exit_with_counts(&cli, error_count, warning_count);
}

fn print_stylish_summary(error_count: usize, warning_count: usize, fixable_count: usize, is_fix_mode: bool) {
    let total = error_count + warning_count;
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
    if !is_fix_mode && fixable_count > 0 {
        println!(
            "  {} {} potentially fixable with the `--fix` option.",
            fixable_count,
            pluralize("problem", fixable_count),
        );
    }
    println!();
}

fn exit_with_counts(cli: &Cli, error_count: usize, warning_count: usize) -> ! {
    if error_count > 0 {
        process::exit(1);
    }
    if let Some(max) = cli.max_warnings {
        if max >= 0 && warning_count > max as usize {
            eprintln!(
                "turbolint found too many warnings (maximum: {max}, found: {warning_count})."
            );
            process::exit(1);
        }
    }
    process::exit(0);
}
