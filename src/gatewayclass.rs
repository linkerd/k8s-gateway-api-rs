use crate::*;
use k8s_openapi::apimachinery::pkg::apis::meta::v1 as metav1;

/// GatewayClass describes a class of Gateways available to the user for creating
/// Gateway resources.
///
/// It is recommended that this resource be used as a template for Gateways. This
/// means that a Gateway is based on the state of the GatewayClass at the time it
/// was created and changes to the GatewayClass or associated parameters are not
/// propagated down to existing Gateways. This recommendation is intended to
/// limit the blast radius of changes to GatewayClass or associated parameters.
/// If implementations choose to propagate GatewayClass changes to existing
/// Gateways, that MUST be clearly documented by the implementation.
///
/// Whenever one or more Gateways are using a GatewayClass, implementations SHOULD
/// add the `gateway-exists-finalizer.gateway.networking.k8s.io` finalizer on the
/// associated GatewayClass. This ensures that a GatewayClass associated with a
/// Gateway is not deleted while in use.
///
/// GatewayClass is a Cluster level resource.
#[derive(
    Clone, Debug, kube::CustomResource, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "GatewayClass",
    status = "GatewayClassStatus"
)]
#[serde(rename_all = "camelCase")]
pub struct GatewayClassSpec {
    /// ControllerName is the name of the controller that is managing Gateways
    /// of this class. The value of this field MUST be a domain prefixed path.
    ///
    /// Example: "example.net/gateway-controller".
    ///
    /// This field is not mutable and cannot be empty.
    ///
    /// Support: Core
    pub controller_name: GatewayController,

    /// ParametersRef is a reference to a resource that contains the
    /// configuration parameters corresponding to the GatewayClass. This is
    /// optional if the controller does not require any additional
    /// configuration.
    ///
    /// ParametersRef can reference a standard Kubernetes resource, i.e.
    /// ConfigMap, or an implementation-specific custom resource. The resource
    /// can be cluster-scoped or namespace-scoped.
    ///
    /// If the referent cannot be found, the GatewayClass's "InvalidParameters"
    /// status condition will be true.
    ///
    /// Support: Implementation-specific
    pub paramters_ref: Option<ParametersReference>,

    /// Description helps describe a GatewayClass with more details.
    pub description: Option<String>,
}

/// ParametersReference identifies an API object containing controller-specific
/// configuration resource within the cluster.
#[derive(
    Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema,
)]
pub struct ParametersReference {
    /// Group is the group of the referent.
    pub group: Group,

    /// Kind is the kind of the referent.
    pub kind: Kind,

    /// Name is the name of the referent.
    pub name: String,

    /// Namespace is the namespace of the referent.
    ///
    /// This field is required when referring to a Namespace-scoped resource and
    /// MUST be unset when referring to a Cluster-scoped resource.
    pub namespace: Option<String>,
}

/// GatewayClassConditionType is the type for status conditions on
/// Gateway resources. This type should be used with the
/// GatewayClassStatus.Conditions field.
pub type GatewayClassConditionType = String;

/// GatewayClassConditionReason defines the set of reasons that explain why a
/// particular GatewayClass condition type has been raised.
pub type GatewayClassConditionReason = String;

/// GatewayClassStatus is the current status for the GatewayClass.
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize, schemars::JsonSchema)]
pub struct GatewayClassStatus {
    /// Conditions is the current status from the controller for this
    /// GatewayClass.
    ///
    /// Controllers should prefer to publish conditions using values of
    /// GatewayClassConditionType for the type of each Condition.
    pub conditions: Option<Vec<metav1::Condition>>,

    /// SupportedFeatures is the set of features the GatewayClass support.
    /// It MUST be sorted in ascending alphabetical order.
    pub supported_features: Option<Vec<SupportedFeature>>,
}

/// SupportedFeature is used to describe distinct features that are covered by
/// conformance tests.
pub type SupportedFeature = String;
