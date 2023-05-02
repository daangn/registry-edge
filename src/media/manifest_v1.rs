use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::str::FromStr;

use crate::digest;
use crate::errors::RegistryError;

#[derive(Debug, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SchemaVersion {
    X = 1,
}

/// See https://docs.docker.com/registry/spec/manifest-v2-1/#manifest-field-descriptions
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestV1 {
    /// SchemaVersion is the image manifest schema that this image follows.
    /// This schema uses version 1.
    pub schema_version: SchemaVersion,

    /// name is the name of the image’s repository
    pub name: String,

    /// tag is the tag of the image
    pub tag: String,

    /// architecture is the host architecture on which this image is intended to run. This is for information purposes and not currently used by the engine
    pub architecture: String,

    /// fsLayers is a list of filesystem layer blob sums contained in this image.
    pub fs_layers: Vec<FSLayer>,

    /// history is a list of unstructured historical data for v1 compatibility. It contains ID of the image layer and ID of the layer’s parent layers.
    pub history: Vec<History>,
}

impl ManifestV1 {
    pub const MIME_TYPE: &'static str = "application/vnd.docker.distribution.manifest.v1+json";
}

pub enum ManifestV1Error {
    ParsingError(serde_json::Error),
}

impl From<serde_json::Error> for ManifestV1Error {
    fn from(err: serde_json::Error) -> Self {
        Self::ParsingError(err)
    }
}

impl From<ManifestV1Error> for RegistryError {
    fn from(err: ManifestV1Error) -> Self {
        match err {
            ManifestV1Error::ParsingError(err) => Self::ManifestInvalid {
                detail: err.to_string(),
            },
        }
    }
}

impl FromStr for ManifestV1 {
    type Err = ManifestV1Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let manifest: Self = serde_json::from_str(s)?;
        Ok(manifest)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FSLayer {
    /// blobSum is the digest of the referenced filesystem image layer. A digest must be a sha256 hash.
    pub blob_sum: digest::ContentDigest,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    /// V1Compatibility is the raw V1 compatibility information. This will contain the JSON object describing the V1 of this image.
    pub v1_compatibility: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_manifest_v1() {
        let json = include_str!("../../tests/data/manifest_v1.json");
        let manifest = ManifestV1::from_str(json);
        assert!(manifest.is_ok());
    }
}
