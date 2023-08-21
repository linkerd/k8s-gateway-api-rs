//! Unofficial Rust bindings for the [Kubernetes Gateway API][gh].
//!
//! [gh]: https://github.com/kubernetes-sigs/gateway-api

#![deny(warnings, rust_2018_idioms)]
#![forbid(unsafe_code)]

// TODO(ver): We should deny missing_docs, but this doesn't play with
// CustomResource derivations.

mod gateway;
mod gatewayclass;
mod httproute;
mod object_reference;
mod referencegrant;
mod shared;

pub use self::{
    gateway::*, gatewayclass::*, httproute::*, object_reference::*, referencegrant::*, shared::*,
};

#[cfg(feature = "experimental")]
mod exp {
    mod grpcroute;
    mod policy;
    mod tcproute;
    mod tlsroute;
    mod udproute;

    pub use self::{grpcroute::*, policy::*, tcproute::*, tlsroute::*, udproute::*};
}

#[cfg(feature = "experimental")]
pub use self::exp::*;
