//! # `prr-loopback`: in-process test middleware
//!
//! A middleware backend that delivers messages inside one process on a virtual clock.
//! Whole multi-node systems can be tested deterministically with no network and no timing
//! flakiness.
//!
//! ## Where it sits
//!
//! - Uses: `prr-mw`.
//! - Used by: tests across the workspace.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - None: this backend is our own design
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 4 of `PLAN.md`.

#![forbid(unsafe_code)]
