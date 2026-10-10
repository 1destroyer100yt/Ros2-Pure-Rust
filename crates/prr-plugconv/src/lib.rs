//! # `prr-plugconv`: C++ plugin converter
//!
//! Converts a C++ `pluginlib` plugin package into a Rust crate skeleton. It translates the
//! patterns it recognises, turns everything else into a `todo!()` next to the original C++,
//! and writes a report listing what is left to port. See PLAN.md §4.2.
//!
//! ## Where it sits
//!
//! - Uses: nothing at runtime.
//! - Used by: people porting C++ plugins.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - `ros/pluginlib`
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 7 of `PLAN.md`.

#![forbid(unsafe_code)]
