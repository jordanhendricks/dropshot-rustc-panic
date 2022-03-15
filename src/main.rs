use dropshot::{ApiDescription, HttpError, HttpResponseOk, RequestContext, endpoint};
use std::sync::Arc;


struct ServerState {}

impl ServerState {
    fn test_mut(&mut self) {}
}

struct ServerContext {
    field1: ServerState,
    field2: ServerState,
}

impl ServerContext {}

#[endpoint {
    method = GET,
    path = "/test"
}]
async fn test_endpoint(
    rqctx: Arc<RequestContext<ServerContext>>,
) -> Result<HttpResponseOk<()>, HttpError>
{
    let server_context = rqctx.context();
    server_context.field1.test_mut();
    Ok(HttpResponseOk(()))
}

fn main() {
    let mut api = ApiDescription::new();
    api.register(test_endpoint);
}
