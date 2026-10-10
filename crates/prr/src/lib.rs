//! # `prr`: the ROS 2 client library
//!
//! The crate applications use: `Context`, `Node`, publishers, subscriptions, timers,
//! services, parameters, executors, logging, and time. It is the equivalent of `rclcpp` and
//! `rclpy`, written directly on top of the `prr-mw` middleware trait.
//!
//! ## Where it sits
//!
//! - Uses: `prr-mw`, a backend such as `prr-dds`, `prr-interfaces`.
//! - Used by: applications and higher-level crates.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - `ros2/rcl` and `ros2/rclcpp` (behaviour to match)
//! - design.ros2.org (naming, QoS, node/participant mapping)
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 4 of `PLAN.md`.

#![forbid(unsafe_code)]
