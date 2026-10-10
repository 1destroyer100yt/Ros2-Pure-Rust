//! # `prr-cli`: the `prr` command-line tool
//!
//! The `prr` command: inspect and drive a running ROS 2 system (topics, nodes, services,
//! actions, parameters, bags, launch) and diagnose connection problems with `prr doctor`.
//!
//! ## Where it sits
//!
//! - Uses: `prr`, `prr-bag`, `prr-launch`.
//! - Used by: people.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - `ros2/ros2cli` (commands to mirror)
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 6 of `PLAN.md`.

#![forbid(unsafe_code)]

use std::process::ExitCode;

/// What the tool prints for `--help` and when run with no arguments.
const USAGE: &str = "\
prr: command-line tool for pure-Rust ROS 2

Usage:
  prr --version    Print the version
  prr --help       Print this message

No ROS commands exist yet. They arrive in Phase 6 of PLAN.md
(topic, node, service, action, param, bag, launch, doctor).";

fn main() -> ExitCode {
    let first_argument = std::env::args().nth(1);
    match first_argument.as_deref() {
        Some("--version" | "-V") => {
            println!("prr {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        None | Some("--help" | "-h") => {
            println!("{USAGE}");
            ExitCode::SUCCESS
        }
        Some(unknown) => {
            eprintln!("error: unknown command `{unknown}`");
            eprintln!("  why:  no ROS commands are implemented yet");
            eprintln!("  fix:  run `prr --help` to see what is available");
            ExitCode::from(2)
        }
    }
}
