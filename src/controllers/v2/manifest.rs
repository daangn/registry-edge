use worker::*;

/// See https://docs.docker.com/registry/spec/api/#get-manifest
pub async fn get(mut _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}

/// See https://docs.docker.com/registry/spec/api/#put-manifest
pub async fn put(mut _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}

/// See https://docs.docker.com/registry/spec/api/#delete-manifest
pub async fn delete(mut _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}
