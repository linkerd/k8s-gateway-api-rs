use super::*;

/// LocalObjectReference identifies an API object within the namespace of the
/// referrer.
/// The API object must be valid in the cluster; the Group and Kind must
/// be registered in the cluster for this reference to be valid.
///
/// References to objects with invalid Group and Kind are not valid, and must
/// be rejected by the implementation, with appropriate Conditions set
/// on the containing object.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct LocalObjectReference {
    /// Group is the group of the referent. For example, "networking.k8s.io".
    /// When unspecified (empty string), core API group is inferred.
    pub group: Group,

    /// Kind is kind of the referent. For example "HTTPRoute" or "Service".
    pub kind: Kind,

    /// Name is the name of the referent.
    pub name: ObjectName,
}

/// SecretObjectReference identifies an API object including its namespace,
/// defaulting to Secret.
///
/// The API object must be valid in the cluster; the Group and Kind must
/// be registered in the cluster for this reference to be valid.
///
/// References to objects with invalid Group and Kind are not valid, and must
/// be rejected by the implementation, with appropriate Conditions set
/// on the containing object.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct SecretObjectReference {
    /// Group is the group of the referent. For example, "networking.k8s.io".
    /// When unspecified (empty string), core API group is inferred.
    pub group: Option<Group>,

    /// Kind is kind of the referent. For example "HTTPRoute" or "Service".
    pub kind: Option<Kind>,

    /// Name is the name of the referent.
    pub name: ObjectName,

    /// Namespace is the namespace of the backend. When unspecified, the local
    /// namespace is inferred.
    ///
    /// Note that when a namespace is specified, a ReferencePolicy object
    /// is required in the referent namespace to allow that namespace's
    /// owner to accept the reference. See the ReferencePolicy documentation
    /// for details.
    ///
    /// Support: Core
    pub namespace: Option<Namespace>,
}

/// BackendObjectReference defines how an ObjectReference that is
/// specific to BackendRef. It includes a few additional fields and features
/// than a regular ObjectReference.
///
/// Note that when a namespace is specified, a ReferencePolicy object
/// is required in the referent namespace to allow that namespace's
/// owner to accept the reference. See the ReferencePolicy documentation
/// for details.
///
/// The API object must be valid in the cluster; the Group and Kind must
/// be registered in the cluster for this reference to be valid.
///
/// References to objects with invalid Group and Kind are not valid, and must
/// be rejected by the implementation, with appropriate Conditions set
/// on the containing object.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct BackendObjectReference {
    /// Group is the group of the referent. For example, "networking.k8s.io".
    /// When unspecified (empty string), core API group is inferred.
    pub group: Option<Group>,

    /// Kind is kind of the referent. For example "HTTPRoute" or "Service".
    /// Defaults to "Service" when not specified.
    pub kind: Option<Kind>,

    /// Name is the name of the referent.
    pub name: ObjectName,

    /// Namespace is the namespace of the backend. When unspecified, the local
    /// namespace is inferred.
    ///
    /// Note that when a namespace is specified, a ReferencePolicy object
    /// is required in the referent namespace to allow that namespace's
    /// owner to accept the reference. See the ReferencePolicy documentation
    /// for details.
    ///
    /// Support: Core
    pub namespace: Option<Namespace>,

    /// Port specifies the destination port number to use for this resource.
    /// Port is required when the referent is a Kubernetes Service. For other
    /// resources, destination port might be derived from the referent resource
    /// or this field.
    pub port: Option<PortNumber>,
}
