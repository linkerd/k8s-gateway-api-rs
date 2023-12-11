use k8s_gateway_api::{
    AllowedRoutes, Gateway, GatewaySpec, GatewayTlsConfig, Listener, RouteGroupKind,
    SecretObjectReference,
};
use kube::{api::PostParams, core::ObjectMeta};

#[tokio::test(flavor = "current_thread")]
async fn round_trip() {
    tracing::trace!("Initializing client");
    let client = kube::Client::try_default()
        .await
        .expect("failed to initialize k8s client");
    let api = kube::Api::<Gateway>::default_namespaced(client);

    let gateway = Gateway {
        metadata: ObjectMeta {
            name: Some("prod-web".to_string()),
            ..Default::default()
        },
        spec: GatewaySpec {
            gateway_class_name: "acme-lb".to_string(),
            infrastructure: None,
            listeners: vec![Listener {
                protocol: "HTTPS".to_string(),
                port: 443,
                allowed_routes: Some(AllowedRoutes {
                    namespaces: None,
                    kinds: Some(vec![RouteGroupKind {
                        group: Some("gateway.networking.k8s.io".to_string()),
                        kind: "HTTPRoute".to_string(),
                    }]),
                }),
                tls: Some(GatewayTlsConfig {
                    certificate_refs: Some(vec![SecretObjectReference {
                        name: "admin-controlled-cert".to_string(),
                        ..SecretObjectReference::default()
                    }]),
                    ..GatewayTlsConfig::default()
                }),
                name: "https".to_string(),
                hostname: None,
            }],
            addresses: None,
        },
        status: Default::default(),
    };
    let post_params = PostParams {
        field_manager: Some("gateway-api-test".to_string()),
        ..Default::default()
    };
    api.create(&post_params, &gateway)
        .await
        .expect("failed to create resource");

    api.get("prod-web").await.expect("failed to get resource");

    api.delete("prod-web", &Default::default())
        .await
        .expect("failed to delete resource");
}
