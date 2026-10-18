//! `type-handl` — Handl stdlib port.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub const TYPE_HANDL_HNL: &str = include_str!("type-handl.hnl");
