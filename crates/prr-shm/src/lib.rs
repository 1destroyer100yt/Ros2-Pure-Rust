//! # `prr-shm`: shared-memory transport
//!
//! Zero-copy message passing between processes on the same machine. This is one of the
//! crates allowed `unsafe` code; each use will be justified here when it lands
//! (PLAN.md §0.3).
//!
//! ## Where it sits
//!
//! - Uses: `prr-mw`.
//! - Used by: `prr-dds` and `prr`.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - None yet: design to be written in Phase 9
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 9 of `PLAN.md`.
