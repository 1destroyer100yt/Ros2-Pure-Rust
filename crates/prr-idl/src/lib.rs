//! # `prr-idl`: interface definition parser
//!
//! Parses ROS 2 interface files (`.msg`, `.srv`, `.action`) and the subset of OMG IDL that
//! `rosidl` generates. It expands services and actions into the helper types ROS 2 creates
//! (for example `Fibonacci_SendGoal`), builds each type's `TypeDescription`, and computes the
//! RIHS01 type hash that ROS 2 (Iron and later) advertises during discovery.
//!
//! ## Where it sits
//!
//! - Uses: nothing.
//! - Used by: `prr-codegen`, `prr-cli` (dynamic message support).
//!
//! The full stack diagram is in `PLAN.md` §2. Terms such as CDR, DDS, RTPS and QoS
//! are defined in the [glossary](https://github.com/1destroyer100yt/Ros2-Pure-Rust/blob/master/docs/GLOSSARY.md).
//!
//! ## Sources
//!
//! - `ros2/rosidl` (`rosidl_adapter`, `rosidl_parser`, type hash generation)
//! - OMG IDL 4.2 (the subset rosidl emits)
//!
//! ## Status
//!
//! Skeleton only. Implementation is scheduled for Phase 1 of `PLAN.md`.

#![forbid(unsafe_code)]
