//! # `xtask`: developer tooling for this workspace
//!
//! Run through the cargo alias in `.cargo/config.toml`:
//!
//! ```text
//! cargo xtask purity   # fail if any shipped crate pulls in C/C++ (PLAN.md §0.2)
//! cargo xtask ci       # run every check CI runs, in the same order
//! ```
//!
//! `xtask` is a developer tool, not a shipped crate, so it lives outside `crates/` and is
//! not itself subject to the purity gate.

#![forbid(unsafe_code)]

mod ci;
mod purity;

use std::process::ExitCode;

/// Shown for `--help`, no arguments, or an unknown command.
const USAGE: &str = "\
Usage: cargo xtask <command>

Commands:
  purity   Check that no crate under crates/ depends on C/C++ build tooling
  ci       Run fmt, clippy, tests, docs and the purity gate, like CI does";

fn main() -> ExitCode {
    let command = std::env::args().nth(1);
    let result = match command.as_deref() {
        Some("purity") => purity::run(),
        Some("ci") => ci::run(),
        None | Some("--help" | "-h") => {
            println!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        Some(unknown) => {
            eprintln!("error: unknown command `{unknown}`\n\n{USAGE}");
            return ExitCode::from(2);
        }
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}
