use crate::*;

/// TCPRoute provides a way to route TCP requests. When combined with a Gateway
/// listener, it can be used to forward connections on the port specified by the
/// listener to a set of backends specified by the TCPRoute.
#[derive(
    Clone, Debug, kube::CustomResource, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1alpha2",
    kind = "TCPRoute",
    struct = "TcpRoute",
    status = "TcpRouteStatus",
    namespaced
)]
pub struct TcpRouteSpec {
    /// Common route information.
    #[serde(flatten)]
    pub inner: CommonRouteSpec,

    /// Rules are a list of TCP matchers and actions.
    pub rules: Vec<TcpRouteRule>,
}

/// TCPRouteStatus defines the observed state of TCPRoute
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct TcpRouteStatus {
    /// Common route status.
    #[serde(flatten)]
    pub inner: RouteStatus,
}

/// TCPRouteRule is the configuration for a given rule.
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct TcpRouteRule {
    /// BackendRefs defines the backend(s) where matching requests should be
    /// sent. If unspecified or invalid (refers to a non-existent resource or a
    /// Service with no endpoints), the underlying implementation MUST actively
    /// reject connection attempts to this backend. Connection rejections must
    /// respect weight; if an invalid backend is requested to have 80% of
    /// connections, then 80% of connections must be rejected instead.
    ///
    /// Support: Core for Kubernetes Service
    /// Support: Custom for any other resource
    ///
    /// Support for weight: Extended
    pub backend_refs: Vec<BackendRef>,
}
