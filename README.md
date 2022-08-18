# k8s-gateway-api

(Unofficial) Rust bindings for the [Kubernetes Gateway API][site].

Based on [gateway-api-v0.5.0-rc1].

[![Crates.io][crate-badge]][crate-url]
[![Documentation][docs-badge]][docs-url]
[![License](https://img.shields.io/crates/l/k8s-gateway-api)](LICENSE)

## Status

This crate is experimental.

It defines all of the *v1beta1* Gateway API types with documentation, as well as
the *v1alpha2* types when the `experimental` feature is enabled.

### TODO

* Express validation constraints
* Rustify/Linkify documentation

[gateway-api-v0.5.0-rc1]: https://github.com/kubernetes-sigs/gateway-api/tree/4f86f0bd65173b04dadb558f63fbbd53330736d2
[site]: https://gateway-api.sigs.k8s.io/
[crate-badge]: https://img.shields.io/crates/v/k8s-gateway-api.svg
[crate-url]: https://crates.io/crates/k8s-gateway-api
[docs-badge]: https://docs.rs/k8s-gateway-api/badge.svg
[docs-url]: https://docs.rs/k8s-gateway-api
