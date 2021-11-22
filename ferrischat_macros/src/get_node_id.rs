#[macro_export]
/// Expands to a block that either fetches the current node ID and returns it or
/// returns a HTTP 500 from the function.
macro_rules! get_node_id {
    () => {{
        use ferrischat_common::types::InternalServerErrorJson;
        use ferrischat_redis::NODE_ID;
        match NODE_ID.get() {
                Some(id) => *id,
                None => {
                    return HttpResponse::InternalServerError().json(InternalServerErrorJson {
                        reason: "Redis has not been set up yet".to_string(),
                        is_bug: true,
                        link: Some(
                    "https://github.com/FerrisChat/Server/issues/new?assignees=tazz4843&labels=bug&\
                    template=api_bug_report.yml&title=%5B500%5D%3A+redis+not+set+up"
                        .to_string()),
                    })
                }
            }
    }};
}
