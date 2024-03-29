mod controllers;
mod digest;
mod errors;
mod media;
mod utils;

use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::set_panic_hook();

    console_log!(
        "\n{} {}\n{:#?}Body: {:?}\n",
        req.method().to_string(),
        req.path(),
        req.headers(),
        req.inner().body().map(|b| b.to_string()),
    );

    // Image tag convention: `[hostname]/[repository_name]/[image_name]`
    // - the hostname is where the worker deployed.
    // - the "repository" is a logically isolated unit of an image repository.
    // - the "image" is an identifier.

    // See https://docs.docker.com/registry/spec/api/#detail
    Router::new()
        // index
        .get_async("/v2/", controllers::v2::index::get_base)
        .get_async(
            "/v2/:repository_name/:image_name/tags/list",
            controllers::v2::index::get_tags,
        )
        .get_async("/v2/_catalog", controllers::v2::index::get_catalog)
        // manifests
        .get_async(
            "/v2/:repository_name/:image_name/manifests/:reference",
            controllers::v2::manifest::get,
        )
        .put_async(
            "/v2/:repository_name/:image_name/manifests/:reference",
            controllers::v2::manifest::put,
        )
        .delete_async(
            "/v2/:repository_name/:image_name/manifests/:reference",
            controllers::v2::manifest::delete,
        )
        // blobs
        .get_async(
            "/v2/:repository_name/:image_name/blobs/:digest",
            controllers::v2::blob::get,
        )
        .delete_async(
            "/v2/:repository_name/:image_name/blobs/:digest",
            controllers::v2::blob::delete,
        )
        // uploads
        .post_async(
            "/v2/:repository_name/:image_name/blobs/uploads",
            controllers::v2::blob_upload::initiate,
        )
        .get_async(
            "/v2/:repository_name/:image_name/blobs/uploads/:uuid",
            controllers::v2::blob_upload::get,
        )
        .patch_async(
            "/v2/:repository_name/:image_name/blobs/uploads/:uuid",
            controllers::v2::blob_upload::append_chunk,
        )
        .put_async(
            "/v2/:repository_name/:image_name/blobs/uploads/:uuid",
            controllers::v2::blob_upload::complete,
        )
        .delete_async(
            "/v2/:repository_name/:image_name/blobs/uploads/:uuid",
            controllers::v2::blob_upload::delete,
        )
        .run(req, env)
        .await
}
