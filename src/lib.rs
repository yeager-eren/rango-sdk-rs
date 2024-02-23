//! Unofficial Rango SDK for Rust
//!
//! Rango Exchange is providing two types of API for interacting with its services:
//! 1. Single step (a.k.a Basic API)
//! 2. Multistep (a.k.a Main API)
//!
//! The Basic API is designed to provide a straightforward integration experience, unless you have specific and unique requirements that necessitate the use of the Main API.
//! You can find more details on [Rango's docs](https://docs.rango.exchange/integration-quick-start/overview).
//!
//! NOTE: this crate only supports for single step api.
//!
//! # Prerequisite
//!
//! Before doing anything, you need to get your API key first.
//! Here you can find out how you can [get a key](https://docs.rango.exchange/integration-quick-start/overview#get-your-api-key).
//!
//! # Usage
//!
//! Create a clinet:
//! ```rust
//! let rango = Client::new(
//!        "put-a-device-id-for-your-client",
//!        "YOUR_API_KEY",
//!        None,
//!    );
//! ```
//!
//! Use available methods to get what you need (e.g. Getting chains):
//! ```rust
//! let result = rango.meta.chains().await;
//! ```
//!

mod check;
/// Initialize a client to send request to server
pub mod client;
mod error;
mod meta;
mod quote;
mod swap;

/// Create request to be passed to different methods.
pub mod request {
    pub use crate::check::{
        balance::BalanceRequest, is_approved::IsApprovedRequest, status::StatusRequest,
    };
    pub use crate::quote::{Asset, QuoteRequest};
    pub use crate::swap::SwapRequest;
}
