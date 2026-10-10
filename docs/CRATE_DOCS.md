# Crate documentation template

Every crate's `src/lib.rs` (or `src/main.rs`) starts with this block. Fill in each section;
delete none. See PLAN.md §0.5 for the reasoning.

```rust
//! # `prr-example`: one line saying what this crate is
//!
//! One to three short paragraphs: what problem it solves, the main types, and anything a
//! reader must know before using it (for example "sans-IO: never touches sockets").
//!
//! ## Where it sits
//!
//! - Uses: the crates below it.
//! - Used by: the crates above it.
//!
//! A small ASCII diagram is welcome when the relationship is not obvious.
//!
//! ## Example
//!
//! A short doctest showing the main use. Added with the first public API; it must compile
//! and run.
//!
//! ## Sources
//!
//! - Spec name, version and section, or upstream repo + file + commit.
//!
//! ## Status
//!
//! What works, what doesn't yet, and which PLAN.md phase finishes it.
```

Modules start with a `//!` paragraph saying what lives in them and why they are separate.
