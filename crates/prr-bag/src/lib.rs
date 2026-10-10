//! # `prr-bag`: bag recording and playback
//!
//! Records topics to MCAP files and plays them back, readable and writable by `ros2 bag`.
//! Compression uses pure-Rust codecs only.
//!
//! ## Where it sits
//!
//! - Uses: `prr`.
//! - Used by: `prr-cli`.
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - `ros2/rosbag2`
//! - MCAP format specification
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 6 of `PLAN.md`.

#![forbid(unsafe_code)]
