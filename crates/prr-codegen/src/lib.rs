//! # `prr-codegen`: interface code generator
//!
//! Turns parsed interface definitions from `prr-idl` into Rust source: one struct per
//! message, request/response pairs for services, and the full set of action types, each with
//! its `TypeSupport` implementation (ROS name, DDS name, type hash, CDR codec).
//!
//! Used from `build.rs` scripts so packages generate their message types with plain `cargo
//! build`, without colcon or ament.
//!
//! ## Where it sits
//!
//! - Uses: `prr-idl`.
//! - Used by: `prr-interfaces`, user packages' `build.rs`.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - `ros2/rosidl` naming conventions (`pkg::msg::dds_::Type_`)
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 1 of `PLAN.md`.

#![forbid(unsafe_code)]
