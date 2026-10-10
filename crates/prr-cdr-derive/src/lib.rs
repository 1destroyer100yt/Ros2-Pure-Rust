//! # `prr-cdr-derive`: derive macros for `prr-cdr`
//!
//! Provides `#[derive(CdrEncode, CdrDecode)]` so message structs get their CDR encoder and
//! decoder generated at compile time instead of written by hand.
//!
//! ## Where it sits
//!
//! - Uses: `syn`/`quote`-style token handling (added with the first macro).
//! - Used by: `prr-cdr` users, `prr-codegen` output.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - Same as `prr-cdr`
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 1 of `PLAN.md`.

#![forbid(unsafe_code)]
