# Ros2-Pure-Rust

ROS 2 rebuilt from the ground up in 100% Rust: wire-compatible with stock ROS 2
(Fast DDS, Cyclone DDS, later Zenoh), with no C or C++ dependencies.

Status: Phase 0 (project setup) done; no ROS functionality yet. See [PLAN.md](PLAN.md) for the
full roadmap, [AGENTS.md](AGENTS.md) for how work is done in this repo, and
[PROGRESS.md](PROGRESS.md) for where things stand.

## Checks

```sh
cargo xtask ci       # format, lint, tests, docs, purity gate (same as CI)
cargo xtask purity   # just the "no C/C++" gate
```
