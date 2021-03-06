#[test]
fn list() {
	use k8s_openapi::api::core::v1 as api;

	crate::Client::with("pod-list", |client| {
		let (request, response_body) = api::Pod::list_namespaced_pod("kube-system", Default::default()).expect("couldn't list pods");
		let response = client.execute(request);
		let pod_list =
			crate::get_single_value(response, response_body, |response, status_code| match response {
				k8s_openapi::ListResponse::Ok(pod_list) => crate::ValueResult::GotValue(pod_list),
				other => panic!("{:?} {}", other, status_code),
			});

		assert_eq!(k8s_openapi::kind(&pod_list), "PodList");

		let apiserver_pod =
			pod_list
			.items.into_iter()
			.filter_map(|pod| {
				let name = pod.metadata.name.as_ref()?;
				if name.starts_with("kube-apiserver-") {
					Some(pod)
				}
				else {
					None
				}
			})
			.next().expect("couldn't find apiserver pod");

		let apiserver_container_spec =
			apiserver_pod
			.spec.expect("couldn't get apiserver pod spec")
			.containers
			.into_iter()
			.next().expect("couldn't get apiserver container spec");
		assert_eq!(apiserver_container_spec.name, "kube-apiserver");

		let apiserver_pod_status = apiserver_pod.status.expect("couldn't get apiserver pod status");
		assert_eq!(apiserver_pod_status.phase, Some("Running".to_string()));
	});
}
