//! # `prr-rtps`: RTPS wire protocol
//!
//! Encodes and decodes DDSI-RTPS messages: the header, every submessage (DATA, HEARTBEAT,
//! ACKNACK, GAP, ...), and the parameter lists that discovery uses. It is *sans-IO*: it only
//! turns bytes into typed values and back. It never opens a socket or reads a clock, which
//! makes every behaviour testable with plain byte vectors.
//!
//! ## Where it sits
//!
//! - Uses: `prr-cdr`.
//! - Used by: `prr-dds`.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - OMG DDSI-RTPS 2.5 (formal/22-04-01)
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 2 of `PLAN.md`.

#![no_std]
#![forbid(unsafe_code)]
