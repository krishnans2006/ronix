//! ronix — Serialize Rust structs to Nix expressions and convert RON to Nix.
//!
//! Provides a custom serde [`Serializer`] that converts any `Serialize` type
//! into a Nix expression string. Also provides direct RON string to Nix
//! conversion via [`ron_to_nix`] and [`ron_to_nix_module`].
//!
//! # Examples
//!
//! ```
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct Config {
//!     poll_interval_ms: u64,
//!     name: String,
//!     enabled: bool,
//! }
//!
//! let config = Config {
//!     poll_interval_ms: 2000,
//!     name: "test".into(),
//!     enabled: true,
//! };
//!
//! let nix = ronix::to_nix(&config).unwrap();
//! assert!(nix.contains("poll_interval_ms = 2000;"));
//! assert!(nix.contains("name = \"test\";"));
//! assert!(nix.contains("enabled = true;"));
//! ```

mod nix_ser;

#[cfg(feature = "cli")]
pub mod cli;

pub use nix_ser::{escape_nix_string, ron_to_nix, ron_to_nix_module, to_nix, to_nix_module, Error};
