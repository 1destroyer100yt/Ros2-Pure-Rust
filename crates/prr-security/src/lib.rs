//! # `prr-security`: DDS Security / SROS2
//!
//! Authentication, access control, and encryption compatible with SROS2 keystores, built
//! on pure-Rust cryptography. Experimental until a human security audit (PLAN.md §8).
//!
//! ## Where it sits
//!
//! - Uses: `prr-dds`.
//! - Used by: `prr-dds`.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - OMG DDS Security 1.1
//! - `ros2/sros2`
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 10 of `PLAN.md`.

#![forbid(unsafe_code)]
