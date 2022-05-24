# k8s-gateway-api

(Unofficial) Rust bindings for the [Kubernetes Gateway API][ref].

Based on <https://github.com/kubernetes-sigs/gateway-api/releases/tag/v0.4.3>.

[![Crates.io][crate-badge]][crate-url]
[![Documentation][docs-badge]][docs-url]
[![License][lic-badge]](LICENSE)

[crate-badge]: https://img.shields.io/crates/v/k8s-gateway-api.svg
[crate-url]: https://crates.io/crates/k8s-gateway-api
[docs-badge]: https://docs.rs/k8s-gateway-api/badge.svg
[docs-url]: https://docs.rs/k8s-gateway-api
[docs-url]: https://img.shields.io/crates/l/k8s-gateway-api
[lic-badge]: https://img.shields.io/crates/l/k8s-gateway-api

## Status

This crate is experimental.

It defines all of the v1alpha2 Gateway API types with documentation.

### TODO

* Express validation constraints
* Rustify/Linkify documentation
* Support Linkerd-specific extensions (via feature flag).

[ref]: https://gateway-api.sigs.k8s.io/
