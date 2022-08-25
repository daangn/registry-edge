use worker::*;

/// Retrieve the blob from the registry identified by `digest`. A `HEAD` request can also be issued to this endpoint to obtain resource information without receiving all data.
///
/// See https://docs.docker.com/registry/spec/api/#get-blob
pub async fn get(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}

/// Delete the blob identified by `name` and `digest`
///
/// See https://docs.docker.com/registry/spec/api/#delete-blob
pub async fn delete(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}
