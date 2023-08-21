use crate::*;
use k8s_openapi::api::core::v1::Service;

#[derive(
    Clone, Debug, kube::CustomResource, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1alpha2",
    kind = "GRPCRoute",
    struct = "GRPCRoute",
    status = "GRPCRouteStatus",
    namespaced
)]
pub struct GRPCRouteSpec {
    /// Common route information.
    #[serde(flatten)]
    pub inner: CommonRouteSpec,

    /// Hostnames defines a set of hostnames to match against the GRPC Host
    /// header to select a GRPCRoute to process the request. This matches
    /// the RFC 1123 definition of a hostname with 2 notable exceptions:
    ///
    /// 1. IPs are not allowed.
    /// 2. A hostname may be prefixed with a wildcard label (*.). The wildcard
    /// label MUST appear by itself as the first label.
    ///
    /// If a hostname is specified by both the Listener and GRPCRoute,
    /// there MUST be at least one intersecting hostname for the GRPCRoute
    /// to be attached to the Listener. For example:
    ///
    /// - A Listener with test.example.com as the hostname matches GRPCRoutes
    /// that have either not specified any hostnames, or have specified at
    /// least one of test.example.com or *.example.com.
    /// - A Listener with *.example.com as the hostname matches GRPCRoutes
    /// that have either not specified any hostnames or have specified at
    /// least one hostname that matches the Listener hostname. For example,
    /// test.example.com and *.example.com would both match. On the other
    /// hand, example.com and test.example.net would not match.
    ///
    /// Hostnames that are prefixed with a wildcard label (*.) are interpreted
    /// as a suffix match. That means that a match for *.example.com would
    /// match both test.example.com, and foo.test.example.com, but not example.com.
    ///
    /// If both the Listener and GRPCRoute have specified hostnames, any
    /// GRPCRoute hostnames that do not match the Listener hostname MUST
    /// be ignored. For example, if a Listener specified *.example.com,
    /// and the GRPCRoute specified test.example.com and test.example.net,
    /// test.example.net MUST NOT be considered for a match.
    ///
    /// If both the Listener and GRPCRoute have specified hostnames, and
    /// none match with the criteria above, then the GRPCRoute MUST NOT
    /// be accepted by the implementation. The implementation MUST raise
    /// an ‘Accepted’ Condition with a status of False in the corresponding
    /// RouteParentStatus.
    ///
    /// If a Route (A) of type HTTPRoute or GRPCRoute is attached to a
    /// Listener and that listener already has another Route (B) of the
    /// other type attached and the intersection of the hostnames of A
    /// and B is non-empty, then the implementation MUST accept exactly
    /// one of these two routes, determined by the following criteria,
    /// in order:
    ///
    /// - The oldest Route based on creation timestamp.
    /// - The Route appearing first in alphabetical order by
    /// “{namespace}/{name}”.
    ///
    /// The rejected Route MUST raise an ‘Accepted’ condition with a
    /// status of ‘False’ in the corresponding RouteParentStatus.
    ///
    /// Support: Core
    pub hostnames: Option<Vec<Hostname>>,

    /// Rules are a list of HTTP matchers, filters and actions.
    pub rules: Option<Vec<GRPCRouteRule>>,
}

/// GRPCRouteRule defines the semantics for matching a gRPC request based
/// on conditions (matches), processing it (filters), and forwarding the
/// request to an API object (backendRefs).
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct GRPCRouteRule {
    /// Matches define conditions used for matching the rule against incoming
    /// gRPC requests. Each match is independent, i.e. this rule will be
    /// matched if any one of the matches is satisfied.
    ///
    /// For example, take the following matches configuration:
    ///
    /// ``` yaml
    /// matches:
    /// - method:
    /// service: foo.bar
    /// headers:
    /// values:
    /// version: 2
    /// - method:
    /// service: foo.bar.v2
    /// ```
    /// For a request to match against this rule, it MUST satisfy EITHER
    /// of the two conditions:
    ///
    /// - service of foo.bar AND contains the header version: 2
    /// - service of foo.bar.v2
    /// See the documentation for GRPCRouteMatch on how to specify multiple
    /// match conditions to be ANDed together.
    ///
    /// If no matches are specified, the implementation MUST match every
    /// gRPC request.
    ///
    /// Proxy or Load Balancer routing configuration generated from
    /// GRPCRoutes MUST prioritize rules based on the following criteria,
    /// continuing on ties. Merging MUST not be done between GRPCRoutes
    /// and HTTPRoutes. Precedence MUST be given to the rule with the
    /// largest number of:
    ///
    /// - Characters in a matching non-wildcard hostname.
    /// - Characters in a matching hostname.
    /// - Characters in a matching service.
    /// - Characters in a matching method.
    /// - Header matches.
    /// If ties still exist across multiple Routes, matching precedence
    /// MUST be determined in order of the following criteria, continuing
    /// on ties:
    ///
    /// - The oldest Route based on creation timestamp.
    /// - The Route appearing first in alphabetical order by
    /// “{namespace}/{name}”.
    /// If ties still exist within the Route that has been given precedence,
    /// matching precedence MUST be granted to the first matching rule
    /// meeting the above criteria.
    pub matches: Option<Vec<GRPCRouteMatch>>,

    /// Filters define the filters that are applied to requests that match
    /// this rule.
    ///
    /// The effects of ordering of multiple behaviors are currently unspecified.
    /// This can change in the future based on feedback during the alpha stage.
    ///
    /// Conformance-levels at this level are defined based on the type of filter:
    ///
    /// - ALL core filters MUST be supported by all implementations that support
    /// GRPCRoute.
    /// - Implementers are encouraged to support extended filters.
    /// - Implementation-specific custom filters have no API guarantees across
    /// implementations.
    /// Specifying the same filter multiple times is not supported unless explicitly
    /// indicated in the filter.
    ///
    /// If an implementation can not support a combination of filters, it must
    /// clearly document that limitation. In cases where incompatible or unsupported
    /// filters are specified and cause the Accepted condition to be set to status
    /// False, implementations may use the IncompatibleFilters reason to specify
    /// this configuration error.
    ///
    /// Support: Core
    pub filters: Option<Vec<GRPCRouteFilter>>,

    /// BackendRefs defines the backend(s) where matching requests should
    /// be sent.
    ///
    /// Failure behavior here depends on how many BackendRefs are specified
    /// and how many are invalid.
    ///
    /// If all entries in BackendRefs are invalid, and there are also no
    /// filters specified in this route rule, all traffic which matches
    /// this rule MUST receive an UNAVAILABLE status.
    ///
    /// See the GRPCBackendRef definition for the rules about what makes
    /// a single GRPCBackendRef invalid.
    ///
    /// When a GRPCBackendRef is invalid, UNAVAILABLE statuses MUST be
    /// returned for requests that would have otherwise been routed to
    /// an invalid backend. If multiple backends are specified, and some
    /// are invalid, the proportion of requests that would otherwise have
    /// been routed to an invalid backend MUST receive an UNAVAILABLE status.
    ///
    /// For example, if two backends are specified with equal weights,
    /// and one is invalid, 50 percent of traffic MUST receive an
    /// UNAVAILABLE status. Implementations may choose how that 50 percent
    /// is determined.
    ///
    /// Support: Core for Kubernetes Service
    ///
    /// Support: Implementation-specific for any other resource
    ///
    /// Support for weight: Core
    pub backendRefs: Option<Vec<GRPCBackendRef>>,
}

/// GRPCRouteMatch defines the predicate used to match requests to a given action.
/// Multiple match types are ANDed together, i.e. the match will evaluate to
/// true only if all conditions are satisfied.
///
/// For example, the match below will match a gRPC request only if its service
/// is foo AND it contains the version: v1 header:
///
/// ``` yaml
/// matches:
/// - method:
/// type: Exact
/// service: "foo"
/// headers:
/// - name: "version"
/// value "v1"
/// ```
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct GRPCRouteMatch {
    /// Method specifies a gRPC request service/method matcher. If this field
    /// is not specified, all services and methods will match.
    pub method: Option<GRPCMethodMatch>,

    /// Headers specifies gRPC request header matchers. Multiple match values
    /// are ANDed together, meaning, a request MUST match all the specified
    /// headers to select the route.
    pub headers: Option<Vec<GRPCHeaderMatch>>,
}

/// GRPCMethodMatch describes how to select a gRPC route by matching the
/// gRPC request service and/or method.
///
/// At least one of Service and Method MUST be a non-empty string.
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum GRPCMethodMatch {
    /// Matches the method or service exactly and with case sensitivity.
    Exact { service: String, method: String },
    /// Matches if the method or service matches the given regular expression
    /// with case sensitivity.
    ///
    /// Since "RegularExpression" has implementation-specific conformance,
    /// implementations can support POSIX, PCRE, RE2 or any other regular
    /// expression dialect. Please read the implementation’s documentation
    /// to determine the supported dialect.
    RegularExpression { service: String, method: String },
}

/// GRPCHeaderMatch describes how to select a gRPC route by matching gRPC
/// request headers.
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum GRPCHeaderMatch {
    Exact { name: GRPCHeaderName, value: String },
    RegularExpression { name: GRPCHeaderName, value: String },
}

/// Name is the name of the gRPC Header to be matched.
///
/// If multiple entries specify equivalent header names, only the first entry
/// with an equivalent name MUST be considered for a match. Subsequent entries
/// with an equivalent header name MUST be ignored. Due to the case-insensitivity
/// of header names, “foo” and “Foo” are considered equivalent.
pub type GRPCHeaderName = String;

/// GRPCRouteFilter defines processing steps that must be completed during the
/// request or response lifecycle. GRPCRouteFilters are meant as an extension
/// point to express processing that may be done in Gateway implementations.
/// Some examples include request or response modification, implementing
/// authentication strategies, rate-limiting, and traffic shaping. API
/// guarantee/conformance is defined based on the type of the filter.
#[derive(
Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum GRPCRouteFilter {
    /// RequestHeaderModifier defines a schema for a filter that modifies request
    /// headers.
    ///
    /// Support: Core
    #[serde(rename_all = "camelCase")]
    RequestHeaderModifier {
        request_header_modifier: HttpRequestHeaderFilter,
    },

    /// ResponseHeaderModifier defines a schema for a filter that modifies
    /// response headers.
    ///
    /// Support: Extended
    #[serde(rename_all = "camelCase")]
    ResponseHeaderModifier {
        response_header_modifier: HttpRequestHeaderFilter,
    },

    /// RequestMirror defines a schema for a filter that mirrors requests.
    /// Requests are sent to the specified destination, but responses from
    /// that destination are ignored.
    ///
    /// Support: Extended
    #[serde(rename_all = "camelCase")]
    RequestMirror {
        request_mirror: HttpRequestMirrorFilter,
    },

    /// ExtensionRef is an optional, implementation-specific extension to the
    /// "filter" behavior.  For example, resource "myroutefilter" in group
    /// "networking.example.net"). ExtensionRef MUST NOT be used for core and
    /// extended filters.
    ///
    /// Support: Implementation-specific
    #[serde(rename_all = "camelCase")]
    ExtensionRef { extension_ref: LocalObjectReference },

}

#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct GRPCBackendRef {
    /// BackendRef is a reference to a backend to forward matched requests to.
    ///
    /// A BackendRef can be invalid for the following reasons. In all cases,
    /// the implementation MUST ensure the ResolvedRefs Condition on the Route
    /// is set to status: False, with a Reason and Message that indicate what
    /// is the cause of the error.
    ///
    /// A BackendRef is invalid if:
    ///
    /// It refers to an unknown or unsupported kind of resource. In this case,
    /// the Reason MUST be set to InvalidKind and Message of the Condition
    /// MUST explain which kind of resource is unknown or unsupported.
    ///
    /// It refers to a resource that does not exist. In this case, the Reason
    /// MUST be set to BackendNotFound and the Message of the Condition MUST
    /// explain which resource does not exist.
    ///
    /// It refers a resource in another namespace when the reference has not
    /// been explicitly allowed by a ReferenceGrant (or equivalent concept).
    /// In this case, the Reason MUST be set to RefNotPermitted and the Message
    /// of the Condition MUST explain which cross-namespace reference is not allowed.
    ///
    /// Support: Core for Kubernetes Service
    ///
    /// Support: Extended for Kubernetes ServiceImport
    ///
    /// Support: Implementation-specific for any other resource
    ///
    /// Support for weight: Core
    #[serde(flatten)]
    pub backend_ref: Option<BackendRef>,

    /// Filters defined at this level MUST be executed if and only if the
    /// request is being forwarded to the backend defined here.
    ///
    /// Support: Implementation-specific (For broader support of filters,
    /// use the Filters field in GRPCRouteRule.)
    pub filters: Option<Vec<GRPCRouteFilter>>,
}
