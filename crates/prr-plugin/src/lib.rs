//! # `prr-plugin`: plugin system
//!
//! Loads node components by name at runtime, like ROS 2's `pluginlib`. Plugins can be
//! compiled in (the default), loaded from shared libraries through a versioned C-ABI table,
//! or run as sandboxed WebAssembly modules.
//!
//! The dynamic loader will need `unsafe` code; when it lands, the justification goes here
//! (PLAN.md §0.3).
//!
//! ## Where it sits
//!
//! - Uses: `prr`.
//! - Used by: applications, `prr-plugconv` output.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - `ros/pluginlib` (manifest format and naming)
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 7 of `PLAN.md`.
