use worker::*;

/// Initiate a resumable blob upload.
///
/// If successful, an upload location will be provided to complete the upload.
///
/// Optionally, if the digest parameter is present, the request body will be used to complete the upload in a single request.
///
/// See https://docs.docker.com/registry/spec/api/#post-initiate-blob-upload
pub async fn initiate(mut _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}

/// Retrieve status of upload identified by uuid.
///
/// The primary purpose of this endpoint is to resolve the current status of a resumable upload.
///
/// See https://docs.docker.com/registry/spec/api/#get-blob
pub async fn get(mut _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}

/// Upload a chunk of data for the specified upload.
///
/// See https://docs.docker.com/registry/spec/api/#patch-blob-upload
pub async fn append_chunk(mut _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}

/// Complete the upload specified by uuid, optionally appending the body as the final chunk.
///
/// See https://docs.docker.com/registry/spec/api/#put-blob-upload
pub async fn complete(mut _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}

/// Cancel outstanding upload processes, releasing associated resources.
///
/// If this is not called, the unfinished uploads will eventually timeout.
///
/// See https://docs.docker.com/registry/spec/api/#delete-blob-upload
pub async fn delete(mut _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    unimplemented!();
}


