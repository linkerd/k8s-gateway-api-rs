use crate::*;

/// The TLSRoute resource is similar to TCPRoute, but can be configured to match
/// against TLS-specific metadata. This allows more flexibility in matching
/// streams for a given TLS listener.
///
/// If you need to forward traffic to a single target for a TLS listener, you
/// could choose to use a TCPRoute with a TLS listener.
#[derive(
    Clone, Debug, kube::CustomResource, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1alpha2",
    kind = "TLSRoute",
    root = "TlsRoute",
    status = "TlsRouteStatus",
    namespaced
)]
pub struct TlsRouteSpec {
    /// Common route information.
    #[serde(flatten)]
    pub inner: CommonRouteSpec,

    /// Hostnames defines a set of SNI names that should match against the SNI
    /// attribute of TLS ClientHello message in TLS handshake. This matches the
    /// RFC 1123 definition of a hostname with 2 notable exceptions:
    ///
    /// 1. IPs are not allowed in SNI names per RFC 6066.
    /// 2. A hostname may be prefixed with a wildcard label (`*.`). The wildcard
    ///    label must appear by itself as the first label.
    ///
    /// If a hostname is specified by both the Listener and TLSRoute, there must
    /// be at least one intersecting hostname for the TLSRoute to be attached to
    /// the Listener. For example:
    ///
    /// * A Listener with `test.example.com` as the hostname matches TLSRoutes
    ///   that have either not specified any hostnames, or have specified at
    ///   least one of `test.example.com` or `*.example.com`.
    /// * A Listener with `*.example.com` as the hostname matches TLSRoutes
    ///   that have either not specified any hostnames or have specified at
    ///   least one hostname that matches the Listener hostname. For example,
    ///   `test.example.com` and `*.example.com` would both match. On the other
    ///   hand, `example.com` and `test.example.net` would not match.
    ///
    /// If both the Listener and TLSRoute have specified hostnames, any TLSRoute
    /// hostnames that do not match the Listener hostname MUST be ignored. For
    /// example, if a Listener specified `*.example.com`, and the TLSRoute
    /// specified `test.example.com` and `test.example.net`, `test.example.net`
    /// must not be considered for a match.
    ///
    /// If both the Listener and TLSRoute have specified hostnames, and none
    /// match with the criteria above, then the TLSRoute is not accepted. The
    /// implementation must raise an 'Accepted' Condition with a status of
    /// `False` in the corresponding RouteParentStatus.
    ///
    /// Support: Core
    pub hostnames: Option<Vec<Hostname>>,

    /// Rules are a list of TLS matchers and actions.
    pub rules: Vec<TlsRouteRule>,
}

/// TLSRouteStatus defines the observed state of TLSRoute.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct TlsRouteStatus {
    /// The routes status.
    #[serde(flatten)]
    pub inner: RouteStatus,
}

/// TLSRouteRule is the configuration for a given rule.
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct TlsRouteRule {
    /// BackendRefs defines the backend(s) where matching requests should be
    /// sent. If unspecified or invalid (refers to a non-existent resource or a
    /// Service with no endpoints), the rule performs no forwarding; if no
    /// filters are specified that would result in a response being sent, the
    /// underlying implementation must actively reject request attempts to this
    /// backend, by rejecting the connection or returning a 500 status code.
    /// Request rejections must respect weight; if an invalid backend is
    /// requested to have 80% of requests, then 80% of requests must be rejected
    /// instead.
    ///
    /// Support: Core for Kubernetes Service
    /// Support: Custom for any other resource
    ///
    /// Support for weight: Extended
    pub backend_refs: Vec<BackendRef>,
}
