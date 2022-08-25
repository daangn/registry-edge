use worker::*;

/// Check that the endpoint implements Docker Registry API V2.
///
/// See https://docs.docker.com/registry/spec/api/#get-base
pub async fn get_base(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}

/// Fetch the tags under the repository identified by `name`.
///
/// See https://docs.docker.com/registry/spec/api/#get-tags
pub async fn get_tags(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}

/// Retrieve a sorted, json list of repositories available in the registry.
///
/// See https://docs.docker.com/registry/spec/api/#get-catalog
pub async fn get_catalog(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}
