//! # `prr-action`: ROS 2 actions
//!
//! Action servers and clients: long-running goals with feedback, cancellation, and
//! results, built from services and topics exactly as ROS 2 composes them.
//!
//! ## Where it sits
//!
//! - Uses: `prr`.
//! - Used by: applications.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - `ros2/rcl` (`rcl_action`) and `ros2/rclcpp` (`rclcpp_action`)
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 5 of `PLAN.md`.

#![forbid(unsafe_code)]
