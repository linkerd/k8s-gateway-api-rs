use crate::*;

/// HTTPRoute provides a way to route HTTP requests. This includes the
/// capability to match requests by hostname, path, header, or query param.
/// Filters can be used to specify additional processing steps. Backends specify
/// where matching requests should be routed.
#[derive(
    Clone,
    Debug,
    Default,
    kube::CustomResource,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "HTTPRoute",
    struct = "HttpRoute",
    status = "HttpRouteStatus",
    namespaced
)]
pub struct HttpRouteSpec {
    /// Common route information.
    #[serde(flatten)]
    pub inner: CommonRouteSpec,

    /// Hostnames defines a set of hostnames that should match against the HTTP Host
    /// header to select a HTTPRoute used to process the request. Implementations
    /// MUST ignore any port value specified in the HTTP Host header while
    /// performing a match and (absent of any applicable header modification
    /// configuration) MUST forward this header unmodified to the backend.
    ///
    /// Valid values for Hostnames are determined by RFC 1123 definition of a
    /// hostname with 2 notable exceptions:
    ///
    /// 1. IPs are not allowed.
    /// 2. A hostname may be prefixed with a wildcard label (`*.`). The wildcard
    ///    label must appear by itself as the first label.
    ///
    /// If a hostname is specified by both the Listener and HTTPRoute, there
    /// must be at least one intersecting hostname for the HTTPRoute to be
    /// attached to the Listener. For example:
    ///
    /// * A Listener with `test.example.com` as the hostname matches HTTPRoutes
    ///   that have either not specified any hostnames, or have specified at
    ///   least one of `test.example.com` or `*.example.com`.
    /// * A Listener with `*.example.com` as the hostname matches HTTPRoutes
    ///   that have either not specified any hostnames or have specified at least
    ///   one hostname that matches the Listener hostname. For example,
    ///   `*.example.com`, `test.example.com`, and `foo.test.example.com` would
    ///   all match. On the other hand, `example.com` and `test.example.net` would
    ///   not match.
    ///
    /// Hostnames that are prefixed with a wildcard label (`*.`) are interpreted
    /// as a suffix match. That means that a match for `*.example.com` would match
    /// both `test.example.com`, and `foo.test.example.com`, but not `example.com`.
    ///
    /// If both the Listener and HTTPRoute have specified hostnames, any
    /// HTTPRoute hostnames that do not match the Listener hostname MUST be
    /// ignored. For example, if a Listener specified `*.example.com`, and the
    /// HTTPRoute specified `test.example.com` and `test.example.net`,
    /// `test.example.net` must not be considered for a match.
    ///
    /// If both the Listener and HTTPRoute have specified hostnames, and none
    /// match with the criteria above, then the HTTPRoute is not accepted. The
    /// implementation must raise an 'Accepted' Condition with a status of
    /// `False` in the corresponding RouteParentStatus.
    ///
    /// In the event that multiple HTTPRoutes specify intersecting hostnames (e.g.
    /// overlapping wildcard matching and exact matching hostnames), precedence must
    /// be given to rules from the HTTPRoute with the largest number of:
    ///
    /// * Characters in a matching non-wildcard hostname.
    /// * Characters in a matching hostname.
    ///
    /// If ties exist across multiple Routes, the matching precedence rules for
    /// HTTPRouteMatches takes over.
    ///
    /// Support: Core
    pub hostnames: Option<Vec<Hostname>>,

    /// Rules are a list of HTTP matchers, filters and actions.
    pub rules: Option<Vec<HttpRouteRule>>,
}

/// HTTPRouteRule defines semantics for matching an HTTP request based on
/// conditions (matches), processing it (filters), and forwarding the request to
/// an API object (backendRefs).
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct HttpRouteRule {
    /// Matches define conditions used for matching the rule against incoming
    /// HTTP requests. Each match is independent, i.e. this rule will be matched
    /// if **any** one of the matches is satisfied.
    ///
    /// For example, take the following matches configuration:
    ///
    /// ```yaml
    /// matches:
    /// - path:
    ///     value: "/foo"
    ///   headers:
    ///   - name: "version"
    ///     value: "v2"
    /// - path:
    ///     value: "/v2/foo"
    /// ```
    ///
    /// For a request to match against this rule, a request must satisfy
    /// EITHER of the two conditions:
    ///
    /// - path prefixed with `/foo` AND contains the header `version: v2`
    /// - path prefix of `/v2/foo`
    ///
    /// See the documentation for HTTPRouteMatch on how to specify multiple
    /// match conditions that should be ANDed together.
    ///
    /// If no matches are specified, the default is a prefix
    /// path match on "/", which has the effect of matching every
    /// HTTP request.
    ///
    /// Proxy or Load Balancer routing configuration generated from HTTPRoutes
    /// MUST prioritize matches based on the following criteria, continuing on
    /// ties. Across all rules specified on applicable Routes, precedence must be
    /// given to the match having:
    ///
    /// * "Exact" path match.
    /// * "Prefix" path match with largest number of characters.
    /// * Method match.
    /// * Largest number of header matches.
    /// * Largest number of query param matches.
    ///
    /// Note: The precedence of RegularExpression path matches are implementation-specific.
    ///
    /// If ties still exist across multiple Routes, matching precedence MUST be
    /// determined in order of the following criteria, continuing on ties:
    ///
    /// * The oldest Route based on creation timestamp.
    /// * The Route appearing first in alphabetical order by
    ///   "{namespace}/{name}".
    ///
    /// If ties still exist within an HTTPRoute, matching precedence MUST be granted
    /// to the FIRST matching rule (in list order) with a match meeting the above
    /// criteria.
    ///
    /// When no rules matching a request have been successfully attached to the
    /// parent a request is coming from, a HTTP 404 status code MUST be returned.
    pub matches: Option<Vec<HttpRouteMatch>>,

    /// Filters define the filters that are applied to requests that match
    /// this rule.
    ///
    /// The effects of ordering of multiple behaviors are currently unspecified.
    /// This can change in the future based on feedback during the alpha stage.
    ///
    /// Conformance-levels at this level are defined based on the type of filter:
    ///
    /// - ALL core filters MUST be supported by all implementations.
    /// - Implementers are encouraged to support extended filters.
    /// - Implementation-specific custom filters have no API guarantees across
    ///   implementations.
    ///
    /// Specifying the same filter multiple times is not supported unless explicitly
    /// indicated in the filter.
    ///
    /// All filters are expected to be compatible with each other except for the
    /// URLRewrite and RequestRedirect filters, which may not be combined. If an
    /// implementation can not support other combinations of filters, they must clearly
    /// document that limitation. In cases where incompatible or unsupported
    /// filters are specified and cause the `Accepted` condition to be set to status
    /// `False`, implementations may use the `IncompatibleFilters` reason to specify
    /// this configuration error.
    ///
    /// Support: Core
    pub filters: Option<Vec<HttpRouteFilter>>,

    /// BackendRefs defines the backend(s) where matching requests should be
    /// sent.
    ///
    /// Failure behavior here depends on how many BackendRefs are specified and
    /// how many are invalid.
    ///
    /// If *all* entries in BackendRefs are invalid, and there are also no filters
    /// specified in this route rule, *all* traffic which matches this rule MUST
    /// receive a 500 status code.
    ///
    /// See the HTTPBackendRef definition for the rules about what makes a single
    /// HTTPBackendRef invalid.
    ///
    /// When a HTTPBackendRef is invalid, 500 status codes MUST be returned for
    /// requests that would have otherwise been routed to an invalid backend. If
    /// multiple backends are specified, and some are invalid, the proportion of
    /// requests that would otherwise have been routed to an invalid backend
    /// MUST receive a 500 status code.
    ///
    /// For example, if two backends are specified with equal weights, and one is
    /// invalid, 50 percent of traffic must receive a 500. Implementations may
    /// choose how that 50 percent is determined.
    ///
    /// Support: Core for Kubernetes Service
    ///
    /// Support: Extended for Kubernetes ServiceImport
    ///
    /// Support: Implementation-specific for any other resource
    ///
    /// Support for weight: Core
    pub backend_refs: Option<Vec<HttpBackendRef>>,

    // Timeouts defines the timeouts that can be configured for an HTTP request.
    //
    // Support: Extended
    pub timeouts: Option<HttpRouteTimeouts>,
}

/// HTTPRouteTimeouts defines timeouts that can be configured for an HTTPRoute.
/// Timeout values are represented with Gateway API Duration formatting.
/// Specifying a zero value such as "0s" is interpreted as no timeout.
#[derive(
    Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct HttpRouteTimeouts {
    /// Request specifies the maximum duration for a gateway to respond to an HTTP request.
    /// If the gateway has not been able to respond before this deadline is met, the gateway
    /// MUST return a timeout error.
    ///
    /// For example, setting the `rules.timeouts.request` field to the value `10s` in an
    /// `HTTPRoute` will cause a timeout if a client request is taking longer than 10 seconds
    /// to complete.
    ///
    /// This timeout is intended to cover as close to the whole request-response transaction
    /// as possible although an implementation MAY choose to start the timeout after the entire
    /// request stream has been received instead of immediately after the transaction is
    /// initiated by the client.
    ///
    /// When this field is unspecified, request timeout behavior is implementation-specific.
    ///
    /// Support: Extended
    pub request: Option<crate::duration::K8sDuration>,

    /// BackendRequest specifies a timeout for an individual request from the gateway
    /// to a backend. This covers the time from when the request first starts being
    /// sent from the gateway to when the full response has been received from the backend.
    ///
    /// An entire client HTTP transaction with a gateway, covered by the Request timeout,
    /// may result in more than one call from the gateway to the destination backend,
    /// for example, if automatic retries are supported.
    ///
    /// Because the Request timeout encompasses the BackendRequest timeout, the value of
    /// BackendRequest must be <= the value of Request timeout.
    ///
    /// Support: Extended
    pub backend_request: Option<crate::duration::K8sDuration>,
}

/// HTTPRouteMatch defines the predicate used to match requests to a given
/// action. Multiple match types are ANDed together, i.e. the match will
/// evaluate to true only if all conditions are satisfied.
///
/// For example, the match below will match a HTTP request only if its path
/// starts with `/foo` AND it contains the `version: v1` header:
///
/// ```yaml
/// match:
///   path:
///     value: "/foo"
///   headers:
///   - name: "version"
///     value "v1"
/// ```
#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct HttpRouteMatch {
    /// Path specifies a HTTP request path matcher. If this field is not
    /// specified, a default prefix match on the "/" path is provided.
    pub path: Option<HttpPathMatch>,

    /// Headers specifies HTTP request header matchers. Multiple match values
    /// are ANDed together, meaning, a request must match all the specified
    /// headers to select the route.
    pub headers: Option<Vec<HttpHeaderMatch>>,

    /// QueryParams specifies HTTP query parameter matchers. Multiple match
    /// values are ANDed together, meaning, a request must match all the
    /// specified query parameters to select the route.
    ///
    /// Support: Extended
    pub query_params: Option<Vec<HttpQueryParamMatch>>,

    /// Method specifies HTTP method matcher.
    /// When specified, this route will be matched only if the request has the
    /// specified method.
    ///
    /// Support: Extended
    pub method: Option<HttpMethod>,
}

/// HTTPPathMatch describes how to select a HTTP route by matching the HTTP request path.
///
/// The `type` specifies the semantics of how HTTP paths should be compared.
/// Valid PathMatchType values are:
///
/// * "Exact"
/// * "PathPrefix"
/// * "RegularExpression"
///
/// PathPrefix and Exact paths must be syntactically valid:
///
/// - Must begin with the `/` character
/// - Must not contain consecutive `/` characters (e.g. `/foo///`, `//`)
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum HttpPathMatch {
    Exact { value: String },
    PathPrefix { value: String },
    RegularExpression { value: String },
}

/// HTTPHeaderName is the name of an HTTP header.
///
/// Valid values include:
///
/// * "Authorization"
/// * "Set-Cookie"
///
/// Invalid values include:
///
/// * ":method" - ":" is an invalid character. This means that HTTP/2 pseudo
///   headers are not currently supported by this type.
/// * "/invalid" - "/" is an invalid character
pub type HttpHeaderName = String;

/// HTTPHeaderMatch describes how to select a HTTP route by matching HTTP
/// request headers.
///
/// `name` is the name of the HTTP Header to be matched. Name matching MUST be
/// case insensitive. (See <https://tools.ietf.org/html/rfc7230#section-3.2>).
///
/// If multiple entries specify equivalent header names, only the first
/// entry with an equivalent name MUST be considered for a match. Subsequent
/// entries with an equivalent header name MUST be ignored. Due to the
/// case-insensitivity of header names, "foo" and "Foo" are considered
/// equivalent.
///
/// When a header is repeated in an HTTP request, it is
/// implementation-specific behavior as to how this is represented.
/// Generally, proxies should follow the guidance from the RFC:
/// <https://www.rfc-editor.org/rfc/rfc7230.html#section-3.2.2> regarding
/// processing a repeated header, with special handling for "Set-Cookie".
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum HttpHeaderMatch {
    #[serde(rename_all = "camelCase")]
    Exact { name: HttpHeaderName, value: String },

    #[serde(rename_all = "camelCase")]
    RegularExpression {
        name: HttpHeaderName,

        /// Since RegularExpression HeaderMatchType has custom conformance,
        /// implementations can support POSIX, PCRE or any other dialects of regular
        /// expressions. Please read the implementation's documentation to determine
        /// the supported dialect.
        value: String,
    },
}

/// HTTPQueryParamMatch describes how to select a HTTP route by matching HTTP
/// query parameters.
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum HttpQueryParamMatch {
    #[serde(rename_all = "camelCase")]
    Exact { name: String, value: String },

    #[serde(rename_all = "camelCase")]
    RegularExpression { name: String, value: String },
}

/// HTTPMethod describes how to select a HTTP route by matching the HTTP
/// method as defined by
/// [RFC 7231](https://datatracker.ietf.org/doc/html/rfc7231#section-4) and
/// [RFC 5789](https://datatracker.ietf.org/doc/html/rfc5789#section-2).
/// The value is expected in upper case.
pub type HttpMethod = String;

/// HTTPRouteFilter defines processing steps that must be completed during the
/// request or response lifecycle. HTTPRouteFilters are meant as an extension
/// point to express processing that may be done in Gateway implementations. Some
/// examples include request or response modification, implementing
/// authentication strategies, rate-limiting, and traffic shaping. API
/// guarantee/conformance is defined based on the type of the filter.
///
/// Type identifies the type of filter to apply. As with other API fields,
/// types are classified into three conformance levels:
///
/// - Core: Filter types and their corresponding configuration defined by
///   "Support: Core" in this package, e.g. "RequestHeaderModifier". All
///   implementations must support core filters.
///
/// - Extended: Filter types and their corresponding configuration defined by
///   "Support: Extended" in this package, e.g. "RequestMirror". Implementers
///   are encouraged to support extended filters.
///
/// - Implementation-specific: Filters that are defined and supported by
///   specific vendors.
///   In the future, filters showing convergence in behavior across multiple
///   implementations will be considered for inclusion in extended or core
///   conformance levels. Filter-specific configuration for such filters
///   is specified using the ExtensionRef field. `Type` should be set to
///   "ExtensionRef" for custom filters.
///
/// Implementers are encouraged to define custom implementation types to
/// extend the core API with implementation-specific behavior.
///
/// If a reference to a custom filter type cannot be resolved, the filter
/// MUST NOT be skipped. Instead, requests that would have been processed by
/// that filter MUST receive a HTTP error response.
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum HttpRouteFilter {
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
    /// This filter can be used multiple times within the same rule. Note that
    /// not all implementations will be able to support mirroring to multiple
    /// backends.
    ///
    /// Support: Extended
    #[serde(rename_all = "camelCase")]
    RequestMirror {
        request_mirror: HttpRequestMirrorFilter,
    },

    /// RequestRedirect defines a schema for a filter that responds to the
    /// request with an HTTP redirection.
    ///
    /// Support: Core
    #[serde(rename_all = "camelCase")]
    RequestRedirect {
        request_redirect: HttpRequestRedirectFilter,
    },

    /// URLRewrite defines a schema for a filter that modifies a request during forwarding.
    ///
    /// Support: Extended
    #[serde(rename_all = "camelCase")]
    URLRewrite { url_rewrite: HttpUrlRewriteFilter },

    /// ExtensionRef is an optional, implementation-specific extension to the
    /// "filter" behavior.  For example, resource "myroutefilter" in group
    /// "networking.example.net"). ExtensionRef MUST NOT be used for core and
    /// extended filters.
    ///
    /// This filter can be used multiple times within the same rule.
    ///
    /// Support: Implementation-specific
    #[serde(rename_all = "camelCase")]
    ExtensionRef { extension_ref: LocalObjectReference },
}

/// HTTPRequestHeaderFilter defines configuration for the RequestHeaderModifier
/// filter.
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
pub struct HttpRequestHeaderFilter {
    /// Set overwrites the request with the given header (name, value)
    /// before the action.
    ///
    /// Input:
    ///   GET /foo HTTP/1.1
    ///   my-header: foo
    ///
    /// Config:
    ///   set:
    ///   - name: "my-header"
    ///     value: "bar"
    ///
    /// Output:
    ///   GET /foo HTTP/1.1
    ///   my-header: bar
    pub set: Option<Vec<HttpHeader>>,

    /// Add adds the given header(s) (name, value) to the request
    /// before the action. It appends to any existing values associated
    /// with the header name.
    ///
    /// Input:
    ///   GET /foo HTTP/1.1
    ///   my-header: foo
    ///
    /// Config:
    ///   add:
    ///   - name: "my-header"
    ///     value: "bar"
    ///
    /// Output:
    ///   GET /foo HTTP/1.1
    ///   my-header: foo
    ///   my-header: bar
    pub add: Option<Vec<HttpHeader>>,

    /// Remove the given header(s) from the HTTP request before the action. The
    /// value of Remove is a list of HTTP header names. Note that the header
    /// names are case-insensitive (see
    /// <https://datatracker.ietf.org/doc/html/rfc2616#section-4.2>).
    ///
    /// Input:
    ///   GET /foo HTTP/1.1
    ///   my-header1: foo
    ///   my-header2: bar
    ///   my-header3: baz
    ///
    /// Config:
    ///   remove: ["my-header1", "my-header3"]
    ///
    /// Output:
    ///   GET /foo HTTP/1.1
    ///   my-header2: bar
    pub remove: Option<Vec<String>>,
}

/// HTTPHeader represents an HTTP Header name and value as defined by RFC 7230.
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
pub struct HttpHeader {
    /// Name is the name of the HTTP Header to be matched. Name matching MUST be
    /// case insensitive. (See <https://tools.ietf.org/html/rfc7230#section-3.2>).
    ///
    /// If multiple entries specify equivalent header names, the first entry with
    /// an equivalent name MUST be considered for a match. Subsequent entries
    /// with an equivalent header name MUST be ignored. Due to the
    /// case-insensitivity of header names, "foo" and "Foo" are considered
    /// equivalent.
    pub name: HttpHeaderName,

    /// Value is the value of HTTP Header to be matched.
    pub value: String,
}

/// HTTPPathModifier defines configuration for path modifiers.
///
// gateway:experimental
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum HttpPathModifier {
    /// ReplaceFullPath specifies the value with which to replace the full path
    /// of a request during a rewrite or redirect.
    #[serde(rename_all = "camelCase")]
    ReplaceFullPath { replace_full_path: String },

    /// ReplacePrefixMatch specifies the value with which to replace the prefix
    /// match of a request during a rewrite or redirect. For example, a request
    /// to "/foo/bar" with a prefix match of "/foo" and a ReplacePrefixMatch
    /// of "/xyz" would be modified to "/xyz/bar".
    ///
    /// Note that this matches the behavior of the PathPrefix match type. This
    /// matches full path elements. A path element refers to the list of labels
    /// in the path split by the `/` separator. When specified, a trailing `/` is
    /// ignored. For example, the paths `/abc`, `/abc/`, and `/abc/def` would all
    /// match the prefix `/abc`, but the path `/abcd` would not.
    ///
    /// ReplacePrefixMatch is only compatible with a `PathPrefix` HTTPRouteMatch.
    /// Using any other HTTPRouteMatch type on the same HTTPRouteRule will result in
    /// the implementation setting the Accepted Condition for the Route to `status: False`.
    ///
    /// Request Path | Prefix Match | Replace Prefix | Modified Path
    /// -------------|--------------|----------------|----------
    /// /foo/bar     | /foo         | /xyz           | /xyz/bar
    /// /foo/bar     | /foo         | /xyz/          | /xyz/bar
    /// /foo/bar     | /foo/        | /xyz           | /xyz/bar
    /// /foo/bar     | /foo/        | /xyz/          | /xyz/bar
    /// /foo         | /foo         | /xyz           | /xyz
    /// /foo/        | /foo         | /xyz           | /xyz/
    /// /foo/bar     | /foo         | <empty string> | /bar
    /// /foo/        | /foo         | <empty string> | /
    /// /foo         | /foo         | <empty string> | /
    /// /foo/        | /foo         | /              | /
    /// /foo         | /foo         | /              | /
    #[serde(rename_all = "camelCase")]
    ReplacePrefixMatch { replace_prefix_match: String },
}

/// HTTPRequestRedirect defines a filter that redirects a request. This filter
/// MUST not be used on the same Route rule as a HTTPURLRewrite filter.
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct HttpRequestRedirectFilter {
    /// Scheme is the scheme to be used in the value of the `Location`
    /// header in the response.
    /// When empty, the scheme of the request is used.
    ///
    /// Scheme redirects can affect the port of the redirect, for more information,
    /// refer to the documentation for the port field of this filter.
    ///
    /// Support: Extended
    pub scheme: Option<String>,

    /// Hostname is the hostname to be used in the value of the `Location`
    /// header in the response.
    /// When empty, the hostname in the `Host` header of the request is used.
    ///
    /// Support: Core
    pub hostname: Option<PreciseHostname>,

    /// Path defines parameters used to modify the path of the incoming request.
    /// The modified path is then used to construct the `Location` header. When
    /// empty, the request path is used as-is.
    ///
    /// Support: Extended
    pub path: Option<HttpPathModifier>,

    /// Port is the port to be used in the value of the `Location`
    /// header in the response.
    ///
    /// If no port is specified, the redirect port MUST be derived using the
    /// following rules:
    ///
    /// * If redirect scheme is not-empty, the redirect port MUST be the well-known
    ///   port associated with the redirect scheme. Specifically "http" to port 80
    ///   and "https" to port 443. If the redirect scheme does not have a
    ///   well-known port, the listener port of the Gateway SHOULD be used.
    /// * If redirect scheme is empty, the redirect port MUST be the Gateway
    ///   Listener port.
    ///
    /// Implementations SHOULD NOT add the port number in the 'Location'
    /// header in the following cases:
    ///
    /// * A Location header that will use HTTP (whether that is determined via
    ///   the Listener protocol or the Scheme field) _and_ use port 80.
    /// * A Location header that will use HTTPS (whether that is determined via
    ///   the Listener protocol or the Scheme field) _and_ use port 443.
    ///
    /// Support: Extended
    pub port: Option<PortNumber>,

    /// StatusCode is the HTTP status code to be used in response.
    ///
    /// Support: Core
    pub status_code: Option<u16>,
}

/// HTTPURLRewriteFilter defines a filter that modifies a request during
/// forwarding. At most one of these filters may be used on a Route rule. This
/// may not be used on the same Route rule as a HTTPRequestRedirect filter.
///
/// gateway:experimental
/// Support: Extended
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
pub struct HttpUrlRewriteFilter {
    /// Hostname is the value to be used to replace the Host header value during
    /// forwarding.
    ///
    /// Support: Extended
    pub hostname: Option<PreciseHostname>,

    /// Path defines a path rewrite.
    ///
    /// Support: Extended
    pub path: Option<HttpPathModifier>,
}

/// HTTPRequestMirrorFilter defines configuration for the RequestMirror filter.
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct HttpRequestMirrorFilter {
    /// BackendRef references a resource where mirrored requests are sent.
    ///
    /// Mirrored requests must be sent only to a single destination endpoint
    /// within this BackendRef, irrespective of how many endpoints are present
    /// within this BackendRef.
    ///
    /// If the referent cannot be found, this BackendRef is invalid and must be
    /// dropped from the Gateway. The controller must ensure the "ResolvedRefs"
    /// condition on the Route status is set to `status: False` and not configure
    /// this backend in the underlying implementation.
    ///
    /// If there is a cross-namespace reference to an *existing* object
    /// that is not allowed by a ReferenceGrant, the controller must ensure the
    /// "ResolvedRefs"  condition on the Route is set to `status: False`,
    /// with the "RefNotPermitted" reason and not configure this backend in the
    /// underlying implementation.
    ///
    /// In either error case, the Message of the `ResolvedRefs` Condition
    /// should be used to provide more detail about the problem.
    ///
    /// Support: Extended for Kubernetes Service
    ///
    /// Support: Implementation-specific for any other resource
    pub backend_ref: BackendObjectReference,
}

/// HTTPBackendRef defines how a HTTPRoute forwards a HTTP request.
///
/// Note that when a namespace different than the local namespace is specified, a
/// ReferenceGrant object is required in the referent namespace to allow that
/// namespace's owner to accept the reference. See the ReferenceGrant
/// documentation for details.
///
/// When the BackendRef points to a Kubernetes Service, implementations SHOULD
/// honor the appProtocol field if it is set for the target Service Port.
///
/// Implementations supporting appProtocol SHOULD recognize the Kubernetes
/// Standard Application Protocols defined in KEP-3726.
///
/// If a Service appProtocol isn't specified, an implementation MAY infer the
/// backend protocol through its own means. Implementations MAY infer the
/// protocol from the Route type referring to the backend Service.
///
/// If a Route is not able to send traffic to the backend using the specified
/// protocol then the backend is considered invalid. Implementations MUST set the
/// "ResolvedRefs" condition to "False" with the "UnsupportedProtocol" reason.
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct HttpBackendRef {
    /// BackendRef is a reference to a backend to forward matched requests to.
    ///
    /// A BackendRef can be invalid for the following reasons. In all cases, the
    /// implementation MUST ensure the `ResolvedRefs` Condition on the Route
    /// is set to `status: False`, with a Reason and Message that indicate
    /// what is the cause of the error.
    ///
    /// A BackendRef is invalid if:
    ///
    /// * It refers to an unknown or unsupported kind of resource. In this
    ///   case, the Reason must be set to `InvalidKind` and Message of the
    ///   Condition must explain which kind of resource is unknown or unsupported.
    ///
    /// * It refers to a resource that does not exist. In this case, the Reason must
    ///   be set to `BackendNotFound` and the Message of the Condition must explain
    ///   which resource does not exist.
    ///
    /// * It refers a resource in another namespace when the reference has not been
    ///   explicitly allowed by a ReferenceGrant (or equivalent concept). In this
    ///   case, the Reason must be set to `RefNotPermitted` and the Message of the
    ///   Condition must explain which cross-namespace reference is not allowed.
    ///
    /// * It refers to a Kubernetes Service that has an incompatible appProtocol
    ///   for the given Route type
    ///
    /// * The BackendTLSPolicy object is installed in the cluster, a BackendTLSPolicy
    ///   is present that refers to the Service, and the implementation is unable
    ///   to meet the requirement. At the time of writing, BackendTLSPolicy is
    ///   experimental, but once it becomes standard, this will become a MUST
    ///   requirement.
    ///
    /// Support: Core for Kubernetes Service
    ///
    /// Support: Implementation-specific for any other resource
    ///
    /// Support for weight: Core
    ///
    /// Support for Kubernetes Service appProtocol: Extended
    ///
    /// Support for BackendTLSPolicy: Experimental and ImplementationSpecific
    #[serde(flatten)]
    pub backend_ref: Option<BackendRef>,

    /// Filters defined at this level should be executed if and only if the
    /// request is being forwarded to the backend defined here.
    ///
    /// Support: Implementation-specific (For broader support of filters, use the
    /// Filters field in HTTPRouteRule.)
    pub filters: Option<Vec<HttpRouteFilter>>,
}

/// HTTPRouteStatus defines the observed state of HTTPRoute.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct HttpRouteStatus {
    /// Common route status information.
    #[serde(flatten)]
    pub inner: RouteStatus,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_http_route() {
        // Test json with a URLRewrote
        let test_json = r#"{
            "apiVersion":"gateway.networking.k8s.io/v1beta1",
            "kind":"HTTPRoute",
            "metadata":{"name":"route_name"},
            "spec":{
                "parentRefs":null,
                "hostnames":null,
                "rules":[{
                    "matches":null,
                    "filters":[{
                        "type":"URLRewrite",
                        "urlRewrite":{
                            "hostname":null,
                            "path":{
                                "type":"ReplacePrefixMatch",
                                "replacePrefixMatch":"/"
                            }
                        }
                    }],
                    "backendRefs":null
                }]}}"#;
        let route: Result<HttpRoute, _> = serde_json::from_str(test_json);
        assert!(route.is_ok());
    }
}
