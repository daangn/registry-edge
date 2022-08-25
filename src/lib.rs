mod media;
mod errors;
mod digest;
mod controllers;
mod utils;

use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::set_panic_hook();

    // See https://docs.docker.com/registry/spec/api/#detail
    Router::new()
        // index
        .get_async("/v2", controllers::v2::index::get_base)
        .get_async("/v2/:pathname/:name/tags/list", controllers::v2::index::get_tags)
        .get_async("/v2/_catalog", controllers::v2::index::get_catalog)

        // manifests
        .get_async("/v2/:pathname/:name/manifests/:reference", controllers::v2::manifest::get)
        .put_async("/v2/:pathname/:name/manifests/:reference", controllers::v2::manifest::put)
        .delete_async("/v2/:pathname/:name/manifests/:reference", controllers::v2::manifest::delete)

        // blobs
        .get_async("/v2/:pathname/:name/blobs/:digest", controllers::v2::blob::get)
        .delete_async("/v2/:pathname/:name/blobs/:digest", controllers::v2::blob::delete)

        // uploads
        .post_async("/v2/:pathname/:name/blobs/uploads", controllers::v2::blob_upload::initiate)
        .get_async("/v2/:pathname/:name/blobs/uploads/:uuid", controllers::v2::blob_upload::get)
        .patch_async("/v2/:pathname/:name/blobs/uploads/:uuid", controllers::v2::blob_upload::append_chunk)
        .put_async("/v2/:pathname/:name/blobs/uploads/:uuid", controllers::v2::blob_upload::complete)
        .delete_async("/v2/:pathname/:name/blobs/uploads/:uuid", controllers::v2::blob_upload::delete)

        .run(req, env).await
}
