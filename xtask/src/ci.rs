//! `cargo xtask ci`: the same checks the GitHub Actions workflow runs, so "it passes
//! locally" and "it passes in CI" mean the same thing.

use std::process::Command;

/// One check: a label for the output and the cargo arguments that run it.
struct Step {
    label: &'static str,
    cargo_args: &'static [&'static str],
    /// Extra environment variables, e.g. to turn rustdoc warnings into errors.
    env: &'static [(&'static str, &'static str)],
}

/// Ordered cheapest first, so the common mistakes fail fast.
const STEPS: &[Step] = &[
    Step {
        label: "format",
        cargo_args: &["fmt", "--all", "--check"],
        env: &[],
    },
    Step {
        label: "clippy",
        cargo_args: &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
        env: &[],
    },
    Step {
        label: "tests",
        cargo_args: &["test", "--workspace"],
        env: &[],
    },
    Step {
        label: "docs",
        cargo_args: &["doc", "--workspace", "--no-deps"],
        env: &[("RUSTDOCFLAGS", "-D warnings")],
    },
];

/// Runs every step, stopping at the first failure, then the purity gate.
///
/// # Errors
///
/// Returns a message naming the step that failed.
pub fn run() -> Result<(), String> {
    for step in STEPS {
        println!("==> {}: cargo {}", step.label, step.cargo_args.join(" "));
        let status = Command::new(env!("CARGO"))
            .args(step.cargo_args)
            .envs(step.env.iter().copied())
            .status()
            .map_err(|error| {
                format!("error: could not start cargo for `{}`: {error}", step.label)
            })?;
        if !status.success() {
            return Err(format!(
                "error: the `{}` step failed\n  fix:  read the output above, fix it, rerun `cargo xtask ci`",
                step.label
            ));
        }
    }
    println!("==> purity");
    crate::purity::run()?;
    println!("\nall checks passed");
    Ok(())
}
