//! # `prr-tf2`: coordinate frame transforms
//!
//! A transform buffer with time interpolation, plus broadcasters and listeners for the
//! `/tf` and `/tf_static` topics, compatible with ROS 2 `tf2`.
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
//! - `ros2/geometry2`
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 6 of `PLAN.md`.

#![forbid(unsafe_code)]
