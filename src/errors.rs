use std::fmt;

// See https://docs.docker.com/registry/spec/api/#errors-2
#[derive(Debug)]
#[allow(dead_code)] // temporarily allow dead code as some of them are not yet being constructed
pub enum RegistryError {
    /// This error may be returned when a blob is unknown to the registry in
    /// a specified repository. This can be returned with a standard get or
    /// if a manifest references an unknown layer during upload.
    BlobUnknown,

    /// The blob upload encountered an error and can no longer proceed.
    BlobUploadInvalid,

    /// If a blob upload has been cancelled or was never started, this error code may
    /// be returned.
    BlobUploadUnknown,

    /// When a blob is uploaded, the registry will check that the content matches
    /// the digest provided by the client. The error may include a detail structure
    /// with the key “digest”, including the invalid digest string. This error may
    /// also be returned when a manifest includes an invalid layer digest.
    DigestInvalid { detail: String },

    /// This error may be returned when a manifest blob is unknown to the registry.
    ManifestBlobUnknown,

    /// During upload, manifests undergo several checks ensuring validity. If those
    /// checks fail, this error may be returned, unless a more specific error is included.
    /// The detail will contain information the failed validation.
    ManifestInvalid { detail: String },

    /// This error is returned when the manifest, identified by name and tag is unknown to
    /// the repository.
    ManifestUnknown,

    /// During manifest upload, if the manifest fails signature verification, this error
    /// will be returned.
    ManifestUnverified,

    /// Invalid repository name encountered either during manifest validation or any API
    /// operation.
    NameInvalid,

    /// This is returned if the name used during an operation is unknown to the registry.
    NameUnknown,

    /// Returned when the `n` parameter (number of results to return) is not an integer,
    /// or `n` is negative.
    PaginationNumberInvalid,

    /// When a layer is uploaded, the provided range is checked against the uploaded chunk.
    /// This error is returned if the range is out of order.
    RangeInvalid,

    /// When a layer is uploaded, the provided size will be checked against the uploaded
    /// content. If they do not match, this error will be returned.
    SizeInvalid { uploaded_size: usize, expected_size: usize },

    /// During a manifest upload, if the tag in the manifest does not match the uri tag,
    /// this error will be returned.
    TagInvalid,

    /// The access controller was unable to authenticate the client. Often this will be
    /// accompanied by a Www-Authenticate HTTP response header indicating how to authenticate.
    Unauthorized,

    /// The access controller denied access for the operation on a resource.
    Denied,

    /// The operation was unsupported due to a missing implementation or invalid set of parameters.
    Unsupported,
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlobUnknown => write!(f, "blob unknown to registry"),
            Self::BlobUploadInvalid => write!(f, "blob upload is invalid"),
            Self::BlobUploadUnknown => write!(f, "blob upload is unknown to registry"),
            Self::DigestInvalid { detail } => write!(f, "provided digest did not match uploaded content: {}", detail),
            Self::ManifestBlobUnknown => write!(f, "manifest blob is unknown to registry"),
            Self::ManifestInvalid { detail } => write!(f, "manifest is invalid: {}", detail),
            Self::ManifestUnknown => write!(f, "manifest unknown"),
            Self::ManifestUnverified => write!(f, "manifest failed signature verification"),
            Self::NameInvalid => write!(f, "invalid repository name"),
            Self::NameUnknown => write!(f, "repository name not known to registry"),
            Self::PaginationNumberInvalid => write!(f, "invalid number of results requested"),
            Self::RangeInvalid => write!(f, "invalid content range"),
            Self::SizeInvalid {
                uploaded_size,
                expected_size,
            } => write!(
                f,
                "provided length ({}) did not match content length ({})",
                expected_size, uploaded_size
            ),
            Self::TagInvalid => write!(f, "manifest tag did not match URI"),
            Self::Unauthorized => write!(f, "authentication required"),
            Self::Denied => write!(f, "requested access to the resource is denied"),
            Self::Unsupported => write!(f, "operation is not supported"),
        }
    }
}
