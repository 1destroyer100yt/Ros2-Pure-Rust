//! # `prr-lifecycle`: managed (lifecycle) nodes
//!
//! Nodes with the standard lifecycle state machine (unconfigured, inactive, active,
//! finalized), its transition services, and publishers that only send while active.
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
//! - `ros2/rcl` (`rcl_lifecycle`) and the managed nodes design article on design.ros2.org
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 5 of `PLAN.md`.

#![forbid(unsafe_code)]
