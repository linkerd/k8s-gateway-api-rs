//! Unofficial Rust bindings for the [Kubernetes Gateway API][gh].
//!
//! [gh]: https://github.com/kubernetes-sigs/gateway-api

#![deny(warnings, rust_2018_idioms)]
#![forbid(unsafe_code)]

// TODO(ver): We should deny missing_docs, but this doesn't play with
// CustomResource derivations.

/// The v1alpha2 API version.
pub mod v1alpha2;
