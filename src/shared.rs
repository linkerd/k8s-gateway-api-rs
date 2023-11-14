use k8s_openapi::apimachinery::pkg::apis::meta::v1 as metav1;

use crate::BackendObjectReference;

/// ParentReference identifies an API object (usually a Gateway) that can be considered
/// a parent of this resource (usually a route). There are two kinds of parent resources
/// with "Core" support:
///
/// * Gateway (Gateway conformance profile)
/// * Service (Mesh conformance profile, experimental, ClusterIP Services only)
///
/// This API may be extended in the future to support additional kinds of parent
/// resources.
///
/// The API object must be valid in the cluster; the Group and Kind must
/// be registered in the cluster for this reference to be valid.
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct ParentReference {
    /// Group is the group of the referent.
    /// When unspecified, "gateway.networking.k8s.io" is inferred.
    /// To set the core API group (such as for a "Service" kind referent),
    /// Group must be explicitly set to "" (empty string).
    ///
    /// Support: Core
    pub group: Option<Group>,

    /// Kind is kind of the referent.
    ///
    /// There are two kinds of parent resources with "Core" support:
    ///
    /// * Gateway (Gateway conformance profile)
    /// * Service (Mesh conformance profile, experimental, ClusterIP Services only)
    ///
    /// Support for other resources is Implementation-Specific.
    pub kind: Option<Kind>,

    /// Namespace is the namespace of the referent. When unspecified, this refers
    /// to the local namespace of the Route.
    ///
    /// Note that there are specific rules for ParentRefs which cross namespace
    /// boundaries. Cross-namespace references are only valid if they are explicitly
    /// allowed by something in the namespace they are referring to. For example:
    /// Gateway has the AllowedRoutes field, and ReferenceGrant provides a
    /// generic way to enable any other kind of cross-namespace reference.
    ///
    /// ParentRefs from a Route to a Service in the same namespace are "producer"
    /// routes, which apply default routing rules to inbound connections from
    /// any namespace to the Service.
    ///
    /// ParentRefs from a Route to a Service in a different namespace are
    /// "consumer" routes, and these routing rules are only applied to outbound
    /// connections originating from the same namespace as the Route, for which
    /// the intended destination of the connections are a Service targeted as a
    /// ParentRef of the Route.
    ///
    /// Support: Core
    pub namespace: Option<Namespace>,

    /// Name is the name of the referent.
    ///
    /// Support: Core
    pub name: ObjectName,

    /// SectionName is the name of a section within the target resource. In the
    /// following resources, SectionName is interpreted as the following:
    ///
    /// * Gateway: Listener Name. When both Port (experimental) and SectionName
    /// are specified, the name and port of the selected listener must match
    /// both specified values.
    /// * Service: Port Name. When both Port (experimental) and SectionName
    /// are specified, the name and port of the selected listener must match
    /// both specified values. Note that attaching Routes to Services as Parents
    /// is part of experimental Mesh support and is not supported for any other
    /// purpose.
    ///
    /// Implementations MAY choose to support attaching Routes to other resources.
    /// If that is the case, they MUST clearly document how SectionName is
    /// interpreted.
    ///
    /// When unspecified (empty string), this will reference the entire resource.
    /// For the purpose of status, an attachment is considered successful if at
    /// least one section in the parent resource accepts it. For example, Gateway
    /// listeners can restrict which Routes can attach to them by Route kind,
    /// namespace, or hostname. If 1 of 2 Gateway listeners accept attachment from
    /// the referencing Route, the Route MUST be considered successfully
    /// attached. If no Gateway listeners accept attachment from this Route, the
    /// Route MUST be considered detached from the Gateway.
    ///
    /// Support: Core
    pub section_name: Option<SectionName>,

    /// Port is the network port this Route targets. It can be interpreted
    /// differently based on the type of parent resource.
    ///
    /// When the parent resource is a Gateway, this targets all listeners
    /// listening on the specified port that also support this kind of Route(and
    /// select this Route). It's not recommended to set `Port` unless the
    /// networking behaviors specified in a Route must apply to a specific port
    /// as opposed to a listener(s) whose port(s) may be changed. When both Port
    /// and SectionName are specified, the name and port of the selected listener
    /// must match both specified values.
    ///
    /// When the parent resource is a Service, this targets a specific port in the
    /// Service spec. When both Port (experimental) and SectionName are specified,
    /// the name and port of the selected port must match both specified values.
    ///
    /// Implementations MAY choose to support other parent resources.
    /// Implementations supporting other types of parent resources MUST clearly
    /// document how/if Port is interpreted.
    ///
    /// For the purpose of status, an attachment is considered successful as
    /// long as the parent resource accepts it partially. For example, Gateway
    /// listeners can restrict which Routes can attach to them by Route kind,
    /// namespace, or hostname. If 1 of 2 Gateway listeners accept attachment
    /// from the referencing Route, the Route MUST be considered successfully
    /// attached. If no Gateway listeners accept attachment from this Route,
    /// the Route MUST be considered detached from the Gateway.
    ///
    /// Support: Extended
    pub port: Option<PortNumber>,
}

/// CommonRouteSpec defines the common attributes that all Routes MUST include
/// within their spec.
#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct CommonRouteSpec {
    /// ParentRefs references the resources (usually Gateways) that a Route wants
    /// to be attached to. Note that the referenced parent resource needs to
    /// allow this for the attachment to be complete. For Gateways, that means
    /// the Gateway needs to allow attachment from Routes of this kind and
    /// namespace. For Services, that means the Service must either be in the same
    /// namespace for a "producer" route, or the mesh implementation must support
    /// and allow "consumer" routes for the referenced Service. ReferenceGrant is
    /// not applicable for governing ParentRefs to Services - it is not possible to
    /// create a "producer" route for a Service in a different namespace from the
    /// Route.
    ///
    /// There are two kinds of parent resources with "Core" support:
    ///
    /// * Gateway (Gateway conformance profile)
    /// * Service (Mesh conformance profile, experimental, ClusterIP Services only)
    /// This API may be extended in the future to support additional kinds of parent
    /// resources.
    ///
    /// ParentRefs must be _distinct_. This means either that:
    ///
    /// * They select different objects.  If this is the case, then parentRef
    ///   entries are distinct. In terms of fields, this means that the
    ///   multi-part key defined by `group`, `kind`, `namespace`, and `name` must
    ///   be unique across all parentRef entries in the Route.
    /// * They do not select different objects, but for each optional field used,
    ///   each ParentRef that selects the same object must set the same set of
    ///   optional fields to different values. If one ParentRef sets a
    ///   combination of optional fields, all must set the same combination.
    ///
    /// Some examples:
    ///
    /// * If one ParentRef sets `sectionName`, all ParentRefs referencing the
    ///   same object must also set `sectionName`.
    /// * If one ParentRef sets `port`, all ParentRefs referencing the same
    ///   object must also set `port`.
    /// * If one ParentRef sets `sectionName` and `port`, all ParentRefs
    ///   referencing the same object must also set `sectionName` and `port`.
    ///
    /// It is possible to separately reference multiple distinct objects that may
    /// be collapsed by an implementation. For example, some implementations may
    /// choose to merge compatible Gateway Listeners together. If that is the
    /// case, the list of routes attached to those resources should also be
    /// merged.
    ///
    /// Note that for ParentRefs that cross namespace boundaries, there are specific
    /// rules. Cross-namespace references are only valid if they are explicitly
    /// allowed by something in the namespace they are referring to. For example,
    /// Gateway has the AllowedRoutes field, and ReferenceGrant provides a
    /// generic way to enable other kinds of cross-namespace reference.
    ///
    /// ParentRefs from a Route to a Service in the same namespace are "producer"
    /// routes, which apply default routing rules to inbound connections from
    /// any namespace to the Service.
    ///
    /// ParentRefs from a Route to a Service in a different namespace are
    /// "consumer" routes, and these routing rules are only applied to outbound
    /// connections originating from the same namespace as the Route, for which
    /// the intended destination of the connections are a Service targeted as a
    /// ParentRef of the Route.
    pub parent_refs: Option<Vec<ParentReference>>,
}

/// PortNumber defines a network port.
pub type PortNumber = u16;

/// BackendRef defines how a Route should forward a request to a Kubernetes
/// resource.
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
///
/// Note that when the BackendTLSPolicy object is enabled by the implementation,
/// there are some extra rules about validity to consider here. See the fields
/// where this struct is used for more information about the exact behavior.
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
pub struct BackendRef {
    /// Weight specifies the proportion of requests forwarded to the referenced
    /// backend. This is computed as weight/(sum of all weights in this
    /// BackendRefs list). For non-zero values, there may be some epsilon from
    /// the exact proportion defined here depending on the precision an
    /// implementation supports. Weight is not a percentage and the sum of
    /// weights does not need to equal 100.
    ///
    /// If only one backend is specified and it has a weight greater than 0, 100%
    /// of the traffic is forwarded to that backend. If weight is set to 0, no
    /// traffic should be forwarded for this entry. If unspecified, weight
    /// defaults to 1.
    ///
    /// Support for this field varies based on the context where used.
    pub weight: Option<u16>,

    /// BackendObjectReference references a Kubernetes object.
    #[serde(flatten)]
    pub inner: BackendObjectReference,
}

/// RouteConditionType is a type of condition for a route.
pub type RouteConditionType = String;

/// RouteConditionReason is a reason for a route condition.
pub type RouteConditionReason = String;

/// RouteParentStatus describes the status of a route with respect to an
/// associated Parent.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RouteParentStatus {
    /// ParentRef corresponds with a ParentRef in the spec that this
    /// RouteParentStatus struct describes the status of.
    pub parent_ref: ParentReference,

    /// ControllerName is a domain/path string that indicates the name of the
    /// controller that wrote this status. This corresponds with the
    /// controllerName field on GatewayClass.
    ///
    /// Example: "example.net/gateway-controller".
    ///
    /// The format of this field is DOMAIN "/" PATH, where DOMAIN and PATH are
    /// valid Kubernetes [names][names].
    ///
    /// Controllers MUST populate this field when writing status. Controllers should ensure that
    /// entries to status populated with their ControllerName are cleaned up when they are no
    /// longer necessary.
    ///
    /// [names]: https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names
    pub controller_name: GatewayController,

    /// Conditions describes the status of the route with respect to the Gateway.
    /// Note that the route's availability is also subject to the Gateway's own
    /// status conditions and listener status.
    ///
    /// If the Route's ParentRef specifies an existing Gateway that supports
    /// Routes of this kind AND that Gateway's controller has sufficient access,
    /// then that Gateway's controller MUST set the "Accepted" condition on the
    /// Route, to indicate whether the route has been accepted or rejected by the
    /// Gateway, and why.
    ///
    /// A Route MUST be considered "Accepted" if at least one of the Route's
    /// rules is implemented by the Gateway.
    ///
    /// There are a number of cases where the "Accepted" condition may not be set
    /// due to lack of controller visibility, that includes when:
    ///
    /// * The Route refers to a non-existent parent.
    /// * The Route is of a type that the controller does not support.
    /// * The Route is in a namespace the controller does not have access to.
    pub conditions: Vec<metav1::Condition>,
}

/// RouteStatus defines the common attributes that all Routes MUST include
/// within their status.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct RouteStatus {
    /// Parents is a list of parent resources (usually Gateways) that are
    /// associated with the route, and the status of the route with respect to
    /// each parent. When this route attaches to a parent, the controller that
    /// manages the parent must add an entry to this list when the controller
    /// first sees the route and should update the entry as appropriate when the
    /// route or gateway is modified.
    ///
    /// Note that parent references that cannot be resolved by an implementation
    /// of this API will not be added to this list. Implementations of this API
    /// can only populate Route status for the Gateways/parent resources they
    /// are responsible for.
    ///
    /// A maximum of 32 Gateways will be represented in this list. An empty list
    /// means the route has not been attached to any Gateway.
    pub parents: Vec<RouteParentStatus>,
}

/// Hostname is the fully qualified domain name of a network host. This matches
/// the RFC 1123 definition of a hostname with 2 notable exceptions:
///
/// 1. IPs are not allowed.
/// 2. A hostname may be prefixed with a wildcard label (`*.`). The wildcard
///    label must appear by itself as the first label.
///
/// Hostname can be "precise" which is a domain name without the terminating dot
/// of a network host (e.g. "foo.example.com") or "wildcard", which is a domain
/// name prefixed with a single wildcard label (e.g. `*.example.com`).
///
/// Note that as per RFC1035 and RFC1123, a *label* must consist of lower case
/// alphanumeric characters or '-', and must start and end with an alphanumeric
/// character. No other punctuation is allowed.
pub type Hostname = String;

/// PreciseHostname is the fully qualified domain name of a network host. This
/// matches the RFC 1123 definition of a hostname with 1 notable exception that
/// numeric IP addresses are not allowed.
///
/// Note that as per RFC1035 and RFC1123, a *label* must consist of lower case
/// alphanumeric characters or '-', and must start and end with an alphanumeric
/// character. No other punctuation is allowed.
pub type PreciseHostname = String;

/// Group refers to a Kubernetes Group. It must either be an empty string or a
/// RFC 1123 subdomain.
///
/// This validation is based off of the corresponding Kubernetes validation:
/// <https://github.com/kubernetes/apimachinery/blob/02cfb53916346d085a6c6c7c66f882e3c6b0eca6/pkg/util/validation/validation.go#L208>
///
/// Valid values include:
///
/// * "" - empty string implies core Kubernetes API group
/// * "networking.k8s.io"
/// * "foo.example.com"
///
/// Invalid values include:
///
/// * "example.com/bar" - "/" is an invalid character
pub type Group = String;

/// Kind refers to a Kubernetes Kind.
///
/// Valid values include:
///
/// * "Service"
/// * "HTTPRoute"
///
/// Invalid values include:
///
/// * "invalid/kind" - "/" is an invalid character
pub type Kind = String;

/// ObjectName refers to the name of a Kubernetes object.
///
/// Object names can have a variety of forms, including RFC1123 subdomains, RFC
/// 1123 labels, or RFC 1035 labels.
pub type ObjectName = String;

/// Namespace refers to a Kubernetes namespace. It must be a RFC 1123 label.
///
/// This validation is based off of the corresponding Kubernetes validation:
/// <https://github.com/kubernetes/apimachinery/blob/02cfb53916346d085a6c6c7c66f882e3c6b0eca6/pkg/util/validation/validation.go#L187>
///
/// This is used for Namespace name validation here:
/// <https://github.com/kubernetes/apimachinery/blob/02cfb53916346d085a6c6c7c66f882e3c6b0eca6/pkg/api/validation/generic.go#L63>
///
/// Valid values include:
///
/// * "example"
///
/// Invalid values include:
///
/// * "example.com" - "." is an invalid character
pub type Namespace = String;

/// SectionName is the name of a section in a Kubernetes resource.
///
/// This validation is based off of the corresponding Kubernetes validation:
/// <https://github.com/kubernetes/apimachinery/blob/02cfb53916346d085a6c6c7c66f882e3c6b0eca6/pkg/util/validation/validation.go#L208>
///
/// Valid values include:
///
/// * "example.com"
/// * "foo.example.com"
///
/// Invalid values include:
///
/// * "example.com/bar" - "/" is an invalid character
pub type SectionName = String;

/// GatewayController is the name of a Gateway API controller. It must be a
/// domain prefixed path.
///
/// Valid values include:
///
/// * "example.com/bar"
///
/// Invalid values include:
///
/// * "example.com" - must include path
/// * "foo.example.com" - must include path
pub type GatewayController = String;

/// AnnotationKey is the key of an annotation in Gateway API. This is used for
/// validation of maps such as TLS options. This matches the Kubernetes
/// "qualified name" validation that is used for annotations and other common
/// values.
///
/// Valid values include:
///
/// * example
/// * example.com
/// * example.com/path
/// * example.com/path.html
///
/// Invalid values include:
///
/// * example~ - "~" is an invalid character
/// * example.com. - can not start or end with "."
pub type AnnotationKey = String;

/// AnnotationValue is the value of an annotation in Gateway API. This is used
/// for validation of maps such as TLS options. This roughly matches Kubernetes
/// annotation validation, although the length validation in that case is based
/// on the entire size of the annotations struct.
pub type AnnotationValue = String;

/// AddressType defines how a network address is represented as a text string.
/// This may take two possible forms:
///
/// * A predefined CamelCase string identifier (currently limited to `IPAddress` or `Hostname`)
/// * A domain-prefixed string identifier (like `acme.io/CustomAddressType`)
///
/// Values `IPAddress` and `Hostname` have Extended support.
///
/// The `NamedAddress` value has been deprecated in favor of implementation
/// specific domain-prefixed strings.
///
/// All other values, including domain-prefixed values have Implementation-specific support,
/// which are used in implementation-specific behaviors. Support for additional
/// predefined CamelCase identifiers may be added in future releases.
pub type AddressType = String;
