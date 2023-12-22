//! # rango-sdk
//!
//! Unofficial Rango SDK for Rust
//!
//!
//! ## Usage
//!
//! 1. Initialize a client:
//! ```rust
//! let rango = Client::new(
//!        "put-a-device-id-for-your-client",
//!        "YOUR_API_KEY",
//!        None,
//!    );
//! ```
//!
//! 2. Use the methods (e.g. Getting meta):
//! ```rust
//! let result = rango.meta.chains().await;
//! ```
//!
pub mod check;
pub mod client;
mod error;
mod meta;
pub mod quote;
