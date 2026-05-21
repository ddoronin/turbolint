use std::fs;
use std::process;

use clap::Parser;
use rslint_core::line_index::LineIndex;
use rslint_core::{Diagnostic, Linter, Severity};
use rslint_rules::all_rules;

#[derive(Parser)]
#[command(name = "rslint", about = "A Rust reimplementation of ESLint")]
struct Cli {
    /// Files to lint
    files: Vec<String>,
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

fn main() {
    let cli = Cli::parse();

    if cli.files.is_empty() {
        eprintln!("No files specified.");
        process::exit(1);
    }

    let linter = Linter::new(all_rules());
    let mut error_count: usize = 0;
    let mut warning_count: usize = 0;

    for path in &cli.files {
        let source = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error reading {path}: {e}");
                error_count += 1;
                continue;
            }
        };

        let result = linter.lint(&source);
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
        println!();
    }

    process::exit(if error_count > 0 { 1 } else { 0 });
}
