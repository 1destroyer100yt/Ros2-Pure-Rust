//! # `prr-testkit`: test support
//!
//! Shared helpers for tests: golden byte vectors captured from real ROS 2, and the harness
//! that runs interoperability scenarios against stock ROS 2 nodes.
//!
//! ## Where it sits
//!
//! - Uses: whatever a test needs.
//! - Used by: tests across the workspace.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - `tests/interop/` in this repository
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 1 of `PLAN.md`.

#![forbid(unsafe_code)]
