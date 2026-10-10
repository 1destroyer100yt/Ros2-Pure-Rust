//! # `prr-cdr`: CDR serialization
//!
//! Encodes and decodes ROS 2 messages as CDR, the byte format every DDS-based ROS 2
//! middleware puts on the wire. ROS 2 uses version 1 of extended CDR (XCDR1, often called
//! "plain CDR") behind a 4-byte encapsulation header that says which byte order follows.
//!
//! The crate is `no_std` + `alloc` so the same code runs on microcontrollers (PLAN.md Phase 11).
//!
//! ## Where it sits
//!
//! - Uses: nothing (bottom of the stack).
//! - Used by: generated message types, `prr-rtps`, `prr-dds`.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - OMG DDS-XTypes 1.3, data representation chapter (XCDR1 and XCDR2)
//! - OMG DDSI-RTPS 2.5, serialized payload encapsulation identifiers
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 1 of `PLAN.md`.

#![no_std]
#![forbid(unsafe_code)]
