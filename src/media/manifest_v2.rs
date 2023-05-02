use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

use crate::digest;
use crate::errors::RegistryError;

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SchemaVersion {
    X = 2,
}

/// See https://docs.docker.com/registry/spec/manifest-v2-2/#image-manifest
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestV2 {
    /// This field specifies the image manifest schema version as an integer.
    /// This schema uses version 2.
    pub schema_version: SchemaVersion,

    /// The MIME type of the manifest.
    /// This should be set to `application/vnd.docker.distribution.manifest.v2+json`.
    pub media_type: String,

    /// The config field references a configuration object for a container, by digest.
    /// This configuration item is a JSON blob that the runtime uses to set up the container.
    /// This new schema uses a tweaked version of this configuration to allow image content-addressability on the daemon side.
    pub config: ImageConfig,

    /// The layer list is ordered starting from the base image (opposite order of schema1).
    pub layers: Vec<ImageLayer>,
}

pub enum ManifestV2Error {
    ParsingError(serde_json::Error),
}

impl From<serde_json::Error> for ManifestV2Error {
    fn from(err: serde_json::Error) -> Self {
        Self::ParsingError(err)
    }
}

impl From<ManifestV2Error> for RegistryError {
    fn from(err: ManifestV2Error) -> Self {
        match err {
            ManifestV2Error::ParsingError(err) => Self::ManifestInvalid {
                detail: err.to_string(),
            },
        }
    }
}

impl FromStr for ManifestV2 {
    type Err = ManifestV2Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let manifest: Self = serde_json::from_str(s)?;
        Ok(manifest)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageConfig {
    /// The MIME type of the referenced object.
    /// This should generally be `application/vnd.docker.container.image.v1+json`.
    pub media_type: String,

    /// The size in bytes of the object.
    /// This field exists so that a client will have an expected size for the content before validating.
    /// If the length of the retrieved content does not match the specified length, the content should not be trusted.
    pub size: u64,

    /// The digest of the content.
    pub digest: digest::ContentDigest,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageLayer {
    /// The MIME type of the referenced object. This should generally be `application/vnd.docker.image.rootfs.diff.tar.gzip`.
    /// Layers of type `application/vnd.docker.image.rootfs.foreign.diff.tar.gzip` may be pulled from a remote location but they should never be pushed.
    pub media_type: String,

    /// The size in bytes of the object.
    /// This field exists so that a client will have an expected size for the content before validating.
    /// If the length of the retrieved content does not match the specified length, the content should not be trusted.
    pub size: u64,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_manifest_v2() {
        let json = include_str!("../../tests/data/manifest_v2.json");
        let manifest = json.parse::<ManifestV2>();
        assert!(manifest.is_ok());
    }
}
