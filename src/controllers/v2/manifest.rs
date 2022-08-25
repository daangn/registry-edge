use worker::*;

/// See https://docs.docker.com/registry/spec/api/#get-manifest
pub async fn get(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}

/// See https://docs.docker.com/registry/spec/api/#put-manifest
pub async fn put(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}

/// See https://docs.docker.com/registry/spec/api/#delete-manifest
pub async fn delete(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}
