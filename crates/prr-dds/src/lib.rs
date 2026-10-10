//! # `prr-dds`: DDS implementation
//!
//! The DDS layer that talks to Fast DDS and Cyclone DDS: participants, discovery (SPDP
//! for participants, SEDP for endpoints), best-effort and reliable writers and readers,
//! fragmentation for large messages, QoS matching, and the UDP transports.
//!
//! ## Where it sits
//!
//! - Uses: `prr-rtps`, `prr-cdr`, `prr-mw`.
//! - Used by: `prr` (through the `prr-mw` middleware trait).
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - OMG DDSI-RTPS 2.5
//! - OMG DDS 1.4 (QoS policies and their compatibility rules)
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 3 of `PLAN.md`.

#![forbid(unsafe_code)]
