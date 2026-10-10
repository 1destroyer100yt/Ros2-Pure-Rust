//! # `prr-launch`: launch files
//!
//! Starts and supervises groups of nodes from XML or YAML launch files (the same formats
//! ROS 2 uses) or from a Rust API. Python launch files are not supported because they need a
//! Python interpreter; a converter for simple ones is planned.
//!
//! ## Where it sits
//!
//! - Uses: `prr`.
//! - Used by: `prr-cli`.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - `ros2/launch` and `ros2/launch_ros`
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 6 of `PLAN.md`.

#![forbid(unsafe_code)]
