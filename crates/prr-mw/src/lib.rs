//! # `prr-mw`: middleware abstraction
//!
//! The trait every transport backend implements (DDS, Zenoh, loopback, shared memory),
//! plus the types they share: global IDs, QoS profiles, and graph events. It plays the role
//! of ROS 2's `rmw` layer as a Rust trait, not the C `rmw` ABI.
//!
//! ## Where it sits
//!
//! - Uses: nothing.
//! - Used by: `prr`, and every backend crate.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - `ros2/rmw` (for the concepts it must cover, not its ABI)
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 4 of `PLAN.md`.

#![forbid(unsafe_code)]
