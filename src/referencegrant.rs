use crate::*;

/// ReferenceGrant identifies kinds of resources in other namespaces that are
/// trusted to reference the specified kinds of resources in the same namespace
/// as the policy.
///
/// Each ReferenceGrant can be used to represent a unique trust relationship.
/// Additional Reference Policies can be used to add to the set of trusted
/// sources of inbound references for the namespace they are defined within.
///
/// All cross-namespace references in Gateway API (with the exception of
/// cross-namespace Gateway-route attachment) require a ReferenceGrant.
#[derive(
    Clone, Debug, kube::CustomResource, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1beta1",
    kind = "ReferenceGrant",
    struct = "ReferenceGrant",
    namespaced
)]
pub struct ReferenceGrantSpec {
    /// From describes the trusted namespaces and kinds that can reference the
    /// resources described in "To". Each entry in this list must be considered
    /// to be an additional place that references can be valid from, or to put
    /// this another way, entries must be combined using OR.
    ///
    /// Support: Core
    pub from: Vec<ReferenceGrantFrom>,

    /// To describes the resources that may be referenced by the resources
    /// described in "From". Each entry in this list must be considered to be an
    /// additional place that references can be valid to, or to put this another
    /// way, entries must be combined using OR.
    ///
    /// Support: Core
    pub to: Vec<ReferenceGrantFrom>,
}

/// ReferenceGrantFrom describes trusted namespaces and kinds.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ReferenceGrantFrom {
    /// Group is the group of the referent.
    ///
    /// When empty, the Kubernetes core API group is inferred.
    ///
    /// Support: Core
    pub group: Group,

    /// Kind is the kind of the referent. Although implementations may support
    /// additional resources, the following types are part of the "Core" support
    /// level for this field.
    ///
    /// When used to permit a SecretObjectReference:
    ///
    /// - Gateway
    ///
    /// When used to permit a BackendObjectReference:
    ///
    /// - GRPCRoute
    /// - HTTPRoute
    /// - TCPRoute
    /// - TLSRoute
    /// - UDPRoute
    pub kind: Kind,

    /// Namespace is the namespace of the referent.
    ///
    /// Support: Core
    pub namespace: Namespace,
}

/// ReferenceGrantTo describes what Kinds are allowed as targets of the
/// references.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct ReferenceGrantTo {
    /// Group is the group of the referent.
    /// When empty, the Kubernetes core API group is inferred.
    ///
    /// Support: Core
    pub group: Group,

    /// Kind is the kind of the referent. Although implementations may support
    /// additional resources, the following types are part of the "Core" support
    /// level for this field:
    ///
    /// - Secret when used to permit a SecretObjectReference
    /// - Service when used to permit a BackendObjectReference
    pub kind: Kind,

    /// Name is the name of the referent. When unspecified, this policy
    /// refers to all resources of the specified Group and Kind in the local
    /// namespace.
    pub name: Option<ObjectName>,
}
