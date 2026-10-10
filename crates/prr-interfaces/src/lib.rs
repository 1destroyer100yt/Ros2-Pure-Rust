//! # `prr-interfaces`: standard ROS 2 message types
//!
//! Generated Rust types for the standard interface packages (`std_msgs`, `geometry_msgs`,
//! `sensor_msgs`, `rcl_interfaces`, and the rest listed in PLAN.md Phase 1), built from the
//! vendored definition files in `interfaces/`.
//!
//! ## Where it sits
//!
//! - Uses: `prr-codegen` (at build time), `prr-cdr`.
//! - Used by: `prr` and every application.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - `ros2/common_interfaces`, `ros2/rcl_interfaces`, `ros2/example_interfaces`
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 1 of `PLAN.md`.

#![forbid(unsafe_code)]
