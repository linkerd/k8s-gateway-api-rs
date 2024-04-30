use crate::*;

/// Spec defines the desired state of GrpcRoute.
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
    version = "v1alpha2",
    kind = "GRPCRoute",
    root = "GrpcRoute",
    status = "GrpcRouteStatus",
    namespaced
)]
pub struct GrpcRouteSpec {
    /// Common route information.
    #[serde(flatten)]
    pub inner: CommonRouteSpec,
    /// Hostnames defines a set of hostnames to match against the GRPC
    /// Host header to select a GRPCRoute to process the request. This matches
    /// the RFC 1123 definition of a hostname with 2 notable exceptions:
    ///
    /// 1. IPs are not allowed.
    /// 2. A hostname may be prefixed with a wildcard label (`*.`). The wildcard
    ///    label MUST appear by itself as the first label.
    ///
    /// If a hostname is specified by both the Listener and GRPCRoute, there
    /// MUST be at least one intersecting hostname for the GRPCRoute to be
    /// attached to the Listener. For example:
    ///
    /// * A Listener with `test.example.com` as the hostname matches GRPCRoutes
    ///   that have either not specified any hostnames, or have specified at
    ///   least one of `test.example.com` or `*.example.com`.
    /// * A Listener with `*.example.com` as the hostname matches GRPCRoutes
    ///   that have either not specified any hostnames or have specified at least
    ///   one hostname that matches the Listener hostname. For example,
    ///   `test.example.com` and `*.example.com` would both match. On the other
    ///   hand, `example.com` and `test.example.net` would not match.
    ///
    /// Hostnames that are prefixed with a wildcard label (`*.`) are interpreted
    /// as a suffix match. That means that a match for `*.example.com` would match
    /// both `test.example.com`, and `foo.test.example.com`, but not `example.com`.
    ///
    /// If both the Listener and GRPCRoute have specified hostnames, any
    /// GRPCRoute hostnames that do not match the Listener hostname MUST be
    /// ignored. For example, if a Listener specified `*.example.com`, and the
    /// GRPCRoute specified `test.example.com` and `test.example.net`,
    /// `test.example.net` MUST NOT be considered for a match.
    ///
    /// If both the Listener and GRPCRoute have specified hostnames, and none
    /// match with the criteria above, then the GRPCRoute MUST NOT be accepted by
    /// the implementation. The implementation MUST raise an 'Accepted' Condition
    /// with a status of `False` in the corresponding RouteParentStatus.
    ///
    /// If a Route (A) of type HTTPRoute or GRPCRoute is attached to a
    /// Listener and that listener already has another Route (B) of the other
    /// type attached and the intersection of the hostnames of A and B is
    /// non-empty, then the implementation MUST accept exactly one of these two
    /// routes, determined by the following criteria, in order:
    ///
    /// * The oldest Route based on creation timestamp.
    /// * The Route appearing first in alphabetical order by
    ///   "{namespace}/{name}".
    ///
    /// The rejected Route MUST raise an 'Accepted' condition with a status of
    /// 'False' in the corresponding RouteParentStatus.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<Vec<String>>,
    /// Rules are a list of Grpc matchers, filters and actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<GrpcRouteRule>>,
}

/// Status defines the current state of GrpcRoute.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GrpcRouteStatus {
    /// Common route status information.
    #[serde(flatten)]
    pub inner: RouteStatus,
}

impl From<GrpcRouteStatus> for HttpRouteStatus {
    fn from(route: GrpcRouteStatus) -> Self {
        Self { inner: route.inner }
    }
}

/// GrpcRouteRule defines the semantics for matching a gRPC request based on
/// conditions (matches), processing it (filters), and forwarding the request to
/// an API object (backendRefs).
#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
pub struct GrpcRouteRule {
    /// Filters define the filters that are applied to requests that match
    /// this rule.
    ///
    /// The effects of ordering of multiple behaviors are currently unspecified.
    /// This can change in the future based on feedback during the alpha stage.
    ///
    /// Conformance-levels at this level are defined based on the type of filter:
    ///
    /// - ALL core filters MUST be supported by all implementations that support
    ///   GRPCRoute.
    /// - Implementers are encouraged to support extended filters.
    /// - Implementation-specific custom filters have no API guarantees across
    ///   implementations.
    ///
    /// Specifying the same filter multiple times is not supported unless explicitly
    /// indicated in the filter.
    ///
    /// If an implementation can not support a combination of filters, it must clearly
    /// document that limitation. In cases where incompatible or unsupported
    /// filters are specified and cause the `Accepted` condition to be set to status
    /// `False`, implementations may use the `IncompatibleFilters` reason to specify
    /// this configuration error.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<GrpcRouteFilter>>,
    /// Matches define conditions used for matching the rule against incoming
    /// gRPC requests. Each match is independent, i.e. this rule will be matched
    /// if **any** one of the matches is satisfied.
    ///
    /// For example, take the following `matches` configuration:
    ///
    /// ```yaml
    /// matches:
    ///   - method:
    ///       service: foo.bar
    ///     headers:
    ///       values:
    ///         version: 2
    ///   - method:
    ///       service: foo.bar.v2
    /// ```
    ///
    /// For a request to match against this rule, it MUST satisfy
    /// EITHER of the two conditions:
    ///
    /// - service of `foo.bar` AND contains the header `version: 2`
    /// - service of `foo.bar.v2`
    ///
    /// See the documentation for GRPCRouteMatch on how to specify multiple
    /// match conditions to be ANDed together.
    ///
    /// If no matches are specified, the implementation MUST match every gRPC request.
    ///
    /// Proxy or Load Balancer routing configuration generated from GRPCRoutes
    /// MUST prioritize rules based on the following criteria, continuing on
    /// ties. Merging MUST not be done between GRPCRoutes and HTTPRoutes.
    /// Precedence MUST be given to the rule with the largest number of:
    ///
    /// * Characters in a matching non-wildcard hostname.
    /// * Characters in a matching hostname.
    /// * Characters in a matching service.
    /// * Characters in a matching method.
    /// * Header matches.
    ///
    /// If ties still exist across multiple Routes, matching precedence MUST be
    /// determined in order of the following criteria, continuing on ties:
    ///
    /// * The oldest Route based on creation timestamp.
    /// * The Route appearing first in alphabetical order by
    ///   "{namespace}/{name}".
    ///
    /// If ties still exist within the Route that has been given precedence,
    /// matching precedence MUST be granted to the first matching rule meeting
    /// the above criteria.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<GrpcRouteMatch>>,
    /// BackendRefs defines the backend(s) where matching requests should be
    /// sent.
    ///
    /// Failure behavior here depends on how many BackendRefs are specified and
    /// how many are invalid.
    ///
    /// If *all* entries in BackendRefs are invalid, and there are also no filters
    /// specified in this route rule, *all* traffic which matches this rule MUST
    /// receive an `UNAVAILABLE` status.
    ///
    /// See the GRPCBackendRef definition for the rules about what makes a single
    /// GRPCBackendRef invalid.
    ///
    /// When a GRPCBackendRef is invalid, `UNAVAILABLE` statuses MUST be returned for
    /// requests that would have otherwise been routed to an invalid backend. If
    /// multiple backends are specified, and some are invalid, the proportion of
    /// requests that would otherwise have been routed to an invalid backend
    /// MUST receive an `UNAVAILABLE` status.
    ///
    /// For example, if two backends are specified with equal weights, and one is
    /// invalid, 50 percent of traffic MUST receive an `UNAVAILABLE` status.
    /// Implementations may choose how that 50 percent is determined.
    ///
    /// Support: Core for Kubernetes Service
    ///
    /// Support: Implementation-specific for any other resource
    ///
    /// Support for weight: Core
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        rename = "backendRefs"
    )]
    pub backend_refs: Option<Vec<GrpcRouteBackendRef>>,
}

/// GrpcRouteMatch defines the predicate used to match requests to a given
/// action. Multiple match types are ANDed together, i.e. the match will
/// evaluate to true only if all conditions are satisfied.
///
///
/// For example, the match below will match a gRPC request only if its service
/// is `foo` AND it contains the `version: v1` header:
///
///
/// ```yaml
/// matches:
///   - method:
///     type: Exact
///     service: "foo"
///     headers:
///   - name: "version"
///     value "v1"
///
///
/// ```
#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
pub struct GrpcRouteMatch {
    /// Method specifies a gRPC request service/method matcher. If this field is
    /// not specified, all services and methods will match.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_method_match"
    )]
    pub method: Option<GrpcMethodMatch>,
    /// Headers specifies gRPC request header matchers. Multiple match values are
    /// ANDed together, meaning, a request MUST match all the specified headers
    /// to select the route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<GrpcHeaderMatch>>,
}

fn deserialize_method_match<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<GrpcMethodMatch>, D::Error> {
    <Option<GrpcMethodMatch> as serde::Deserialize>::deserialize(deserializer).map(|value| {
        match value.as_ref() {
            Some(rule) if rule.is_empty() => None,
            _ => value,
        }
    })
}

#[allow(unused_qualifications)]
pub type GrpcHeaderMatch = crate::httproute::HttpHeaderMatch;

/// Method specifies a gRPC request service/method matcher. If this field is
/// not specified, all services and methods will match.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, schemars::JsonSchema)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum GrpcMethodMatch {
    #[serde(rename_all = "camelCase")]
    Exact {
        /// Value of the method to match against. If left empty or omitted, will
        /// match all services.
        ///
        /// At least one of Service and Method MUST be a non-empty string.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        method: Option<String>,
        /// Value of the service to match against. If left empty or omitted, will
        /// match any service.
        ///
        /// At least one of Service and Method MUST be a non-empty string.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        service: Option<String>,
    },

    #[serde(rename_all = "camelCase")]
    RegularExpression {
        /// Value of the method to match against. If left empty or omitted, will
        /// match all services.
        ///
        /// At least one of Service and Method MUST be a non-empty string.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        method: Option<String>,
        /// Value of the service to match against. If left empty or omitted, will
        /// match any service.
        ///
        /// At least one of Service and Method MUST be a non-empty string.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        service: Option<String>,
    },
}

impl GrpcMethodMatch {
    fn is_empty(&self) -> bool {
        let (method, service) = match self {
            Self::Exact { method, service } => (method, service),
            Self::RegularExpression { method, service } => (method, service),
        };

        method.as_deref().map(str::is_empty).unwrap_or(true)
            && service.as_deref().map(str::is_empty).unwrap_or(true)
    }
}

fn empty_option_strings_are_none(value: Option<String>) -> Option<String> {
    match value.as_ref() {
        Some(string) if string.is_empty() => None,
        _ => value,
    }
}

impl<'de> serde::Deserialize<'de> for GrpcMethodMatch {
    // NOTE: This custom deserialization exists to ensure the deserialization
    //       behavior matches the behavior prescribed by the gateway api docs
    //       for how the "type" field on `GRPCRouteMatch` is expected to work.
    //
    //       ref: https://gateway-api.sigs.k8s.io/reference/spec/#gateway.networking.k8s.io%2fv1alpha2.GRPCMethodMatch
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Type,
            Method,
            Service,
        }

        struct GrpcMethodMatchVisitor;

        impl<'de> serde::de::Visitor<'de> for GrpcMethodMatchVisitor {
            type Value = GrpcMethodMatch;

            fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                formatter.write_str("GrpcMethodMatch")
            }

            fn visit_map<V>(self, mut map: V) -> Result<GrpcMethodMatch, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let (mut r#type, mut method, mut service) = (None, None, None);

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if r#type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type = map
                                .next_value::<Option<String>>()
                                .map(empty_option_strings_are_none)?;
                        }
                        Field::Method => {
                            if method.is_some() {
                                return Err(serde::de::Error::duplicate_field("method"));
                            }
                            method = map
                                .next_value::<Option<String>>()
                                .map(empty_option_strings_are_none)?;
                        }
                        Field::Service => {
                            if service.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            service = map
                                .next_value::<Option<String>>()
                                .map(empty_option_strings_are_none)?;
                        }
                    }
                }

                match r#type.as_deref() {
                    None | Some("Exact") => Ok(GrpcMethodMatch::Exact { method, service }),
                    Some("RegularExpression") => {
                        Ok(GrpcMethodMatch::RegularExpression { method, service })
                    }
                    Some(value) => Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Str(value),
                        &r#"one of: {"Exact", "RegularExpression"}"#,
                    )),
                }
            }
        }

        const FIELDS: &[&str] = &["type", "method", "service"];
        deserializer.deserialize_struct("GrpcMethodMatch", FIELDS, GrpcMethodMatchVisitor)
    }
}

/// GrpcRouteFilter defines processing steps that must be completed during the
/// request or response lifecycle. GrpcRouteFilters are meant as an extension
/// point to express processing that may be done in Gateway implementations. Some
/// examples include request or response modification, implementing
/// authentication strategies, rate-limiting, and traffic shaping. API
/// guarantee/conformance is defined based on the type of the filter.
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(tag = "type", rename_all = "PascalCase")]
pub enum GrpcRouteFilter {
    /// ExtensionRef is an optional, implementation-specific extension to the
    /// "filter" behavior.  For example, resource "myroutefilter" in group
    /// "networking.example.net"). ExtensionRef MUST NOT be used for core and
    /// extended filters.
    ///
    /// Support: Implementation-specific
    ///
    /// This filter can be used multiple times within the same rule.
    #[serde(rename_all = "camelCase")]
    ExtensionRef { extension_ref: LocalObjectReference },

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
}

impl From<GrpcRouteFilter> for HttpRouteFilter {
    fn from(filter: GrpcRouteFilter) -> Self {
        match filter {
            GrpcRouteFilter::ExtensionRef { extension_ref } => Self::ExtensionRef { extension_ref },
            GrpcRouteFilter::RequestMirror { request_mirror } => {
                Self::RequestMirror { request_mirror }
            }
            GrpcRouteFilter::RequestHeaderModifier {
                request_header_modifier,
            } => Self::RequestHeaderModifier {
                request_header_modifier,
            },
            GrpcRouteFilter::ResponseHeaderModifier {
                response_header_modifier,
            } => Self::ResponseHeaderModifier {
                response_header_modifier,
            },
        }
    }
}

/// GrpcBackendRef defines how a GrpcRoute forwards a gRPC request.
///
/// Note that when a namespace different from the local namespace is specified, a
/// ReferenceGrant object is required in the referent namespace to allow that
/// namespace's owner to accept the reference. See the ReferenceGrant
/// documentation for details.
///
/// <gateway:experimental:description>
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
///
/// </gateway:experimental:description>
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
pub struct GrpcRouteBackendRef {
    /// BackendObjectReference references a Kubernetes object.
    #[serde(flatten)]
    pub inner: BackendObjectReference,
    /// Filters defined at this level MUST be executed if and only if the
    /// request is being forwarded to the backend defined here.
    ///
    /// Support: Implementation-specific (For broader support of filters, use the
    /// Filters field in GrpcRouteRule.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<GrpcRouteFilter>>,
    /// Weight specifies the proportion of requests forwarded to the referenced
    /// backend. This is computed as weight/(sum of all weights in this
    /// BackendRefs list). For non-zero values, there may be some epsilon from
    /// the exact proportion defined here depending on the precision an
    /// implementation supports. Weight is not a percentage and the sum of
    /// weights does not need to equal 100.
    ///
    /// If only one backend is specified, and it has a weight greater than 0, 100%
    /// of the traffic is forwarded to that backend. If weight is set to 0, no
    /// traffic should be forwarded for this entry. If unspecified, weight
    /// defaults to 1.
    ///
    /// Support for this field varies based on the context where used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<u16>,
}

impl From<GrpcRouteBackendRef> for HttpBackendRef {
    fn from(backend: GrpcRouteBackendRef) -> Self {
        let filters = backend
            .filters
            .map(|filters| filters.into_iter().map(Into::into).collect());

        Self {
            filters,
            backend_ref: Some(BackendRef {
                inner: backend.inner,
                weight: backend.weight,
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grpc_route_deserialization() {
        // Test deserialization against upstream example
        // ref: https://gateway-api.sigs.k8s.io/api-types/grpcroute/#backendrefs-optional
        let data = r#"{
          "apiVersion": "gateway.networking.k8s.io/v1alpha2",
          "kind": "GRPCRoute",
          "metadata": {
            "name": "grpc-app-1"
          },
          "spec": {
            "parentRefs": [
              {
                "name": "my-gateway"
              }
            ],
            "hostnames": [
              "example.com"
            ],
            "rules": [
              {
                "matches": [
                  {
                    "method": {
                      "service": "com.example.User",
                      "method": "Login"
                    }
                  },
                  {
                    "method": {
                      "service": "com.example.User",
                      "method": "Logout",
                      "type": "Exact"
                    }
                  },
                  {
                    "method": {
                      "service": "com.example.User",
                      "method": "UpdateProfile",
                      "type": "RegularExpression"
                    }
                  }
                ],
                "backendRefs": [
                  {
                    "name": "my-service1",
                    "port": 50051
                  }
                ]
              },
              {
                "matches": [
                  {
                    "headers": [
                      {
                        "type": "Exact",
                        "name": "magic",
                        "value": "foo"
                      }
                    ],
                    "method": {
                      "service": "com.example.Things",
                      "method": "DoThing"
                    }
                  }
                ],
                "backendRefs": [
                  {
                    "name": "my-service2",
                    "port": 50051
                  }
                ]
              }
            ]
          }
        }"#;
        let route = serde_json::from_str::<GrpcRoute>(data);
        assert!(route.is_ok());
    }
}
