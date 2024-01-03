use k8s_gateway_api::{
    BackendObjectReference, BackendRef, HttpBackendRef, HttpHeaderMatch, HttpRoute, HttpRouteMatch,
    HttpRouteRule, HttpRouteSpec,
};
use kube::{api::PostParams, core::ObjectMeta};

#[tokio::test(flavor = "current_thread")]
async fn round_trip() {
    tracing::trace!("Initializing client");
    let client = kube::Client::try_default()
        .await
        .expect("failed to initialize k8s client");
    let api = kube::Api::<HttpRoute>::default_namespaced(client);

    let route = HttpRoute {
        metadata: ObjectMeta {
            name: Some("bar-route".to_string()),
            namespace: None,
            labels: Some([("gateway".to_string(), "external-https-prod".to_string())].into()),
            ..Default::default()
        },
        spec: HttpRouteSpec {
            hostnames: Some(vec!["bar.example.com".to_string()]),
            rules: Some(vec![
                HttpRouteRule {
                    backend_refs: Some(vec![
                        HttpBackendRef {
                            backend_ref: Some(BackendRef {
                                inner: mk_inner_ref("bar-v1", None, Some(8080)),
                                weight: Some(90),
                            }),
                            filters: None,
                        },
                        HttpBackendRef {
                            backend_ref: Some(BackendRef {
                                inner: mk_inner_ref("bar-v2", None, Some(8080)),
                                weight: Some(10),
                            }),
                            filters: None,
                        },
                    ]),
                    filters: None,
                    matches: None,
                },
                HttpRouteRule {
                    matches: Some(vec![HttpRouteMatch {
                        headers: Some(vec![HttpHeaderMatch::Exact {
                            name: "env".to_string(),
                            value: "canary".to_string(),
                        }]),
                        ..HttpRouteMatch::default()
                    }]),
                    backend_refs: Some(vec![HttpBackendRef {
                        backend_ref: Some(BackendRef {
                            inner: mk_inner_ref("bar-v2", None, Some(8080)),
                            weight: None,
                        }),
                        filters: None,
                    }]),
                    filters: None,
                },
            ]),
            ..HttpRouteSpec::default()
        },
        status: None,
    };
    let post_params = PostParams {
        field_manager: Some("gateway-api-test".to_string()),
        ..Default::default()
    };
    api.create(&post_params, &route)
        .await
        .expect("failed to create resource");

    api.get("bar-route").await.expect("failed to get resource");

    api.delete("bar-route", &Default::default())
        .await
        .expect("failed to delete resource");
}

fn mk_inner_ref(
    name: &str,
    namespace: Option<String>,
    port: Option<u16>,
) -> BackendObjectReference {
    BackendObjectReference {
        group: None, // core group inferred
        kind: None,  // defaults to Service when not specified
        name: name.to_string(),
        namespace, // defaults to local namespace when unspecified
        port,
    }
}
