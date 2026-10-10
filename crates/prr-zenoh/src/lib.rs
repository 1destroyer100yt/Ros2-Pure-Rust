//! # `prr-zenoh`: Zenoh middleware backend
//!
//! A middleware backend that speaks the same Zenoh conventions as `rmw_zenoh_cpp`, so
//! nodes can join a Zenoh-based ROS 2 system.
//!
//! ## Where it sits
//!
//! - Uses: `prr-mw`.
//! - Used by: `prr`.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - `ros2/rmw_zenoh` (design document and source)
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 8 of `PLAN.md`.

#![forbid(unsafe_code)]
