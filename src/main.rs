use dropshot::{endpoint, HttpError, HttpResponseOk, RequestContext};
use std::sync::Arc;

struct ServerState {}

impl ServerState {
    fn test_mut(&mut self) {}
}

struct ServerContext {
    field: ServerState,
}

impl ServerContext {
    fn test_mut(&mut self) {}
}

#[endpoint {
    method = GET,
    path = "/test"
}]
async fn test_endpoint(
    rqctx: Arc<RequestContext<ServerContext>>,
) -> Result<HttpResponseOk<()>, HttpError> {
    let server_context = rqctx.context();

    // Either of these will cause a panic on nightly-2021-11-24
    //server_context.test_mut();
    server_context.field.test_mut();

    Ok(HttpResponseOk(()))
}

fn main() {}
