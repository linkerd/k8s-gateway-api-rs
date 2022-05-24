use super::*;

/// PolicyTargetReference identifies an API object to apply policy to. This
/// should be used as part of Policy resources that can target Gateway API
/// resources. For more information on how this policy attachment model works,
/// and a sample Policy resource, refer to the policy attachment documentation
/// for Gateway API.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct PolicyTargetReference {
    /// Group is the group of the target resource.
    pub group: Group,

    /// Kind is kind of the target resource.
    pub kind: Kind,

    /// Name is the name of the target resource.
    pub name: ObjectName,

    /// Namespace is the namespace of the referent. When unspecified, the local
    /// namespace is inferred. Even when policy targets a resource in a
    /// different namespace, it MUST only apply to traffic originating from the
    /// same namespace as the policy.
    pub namespace: Option<Namespace>,
}
