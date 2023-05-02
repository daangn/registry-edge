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

/// See https://docs.docker.com/registry/spec/manifest-v2-2/#manifest-list-field-descriptions
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestList {
    /// This field specifies the image manifest schema version as an integer.
    /// This schema uses the version 2.
    pub schema_version: SchemaVersion,

    /// The MIME type of the manifest list.
    /// This should be set to `application/vnd.docker.distribution.manifest.list.v2+json`.
    pub media_type: String,

    /// The manifests field contains a list of manifests for specific platforms.
    pub manifests: Vec<ManifestListItem>,
}

impl ManifestList {
    pub const MIME_TYPE: &'static str = "application/vnd.docker.distribution.manifest.list.v2+json";
}

pub enum ManifestListError {
    ParsingError(serde_json::Error),
}

impl From<serde_json::Error> for ManifestListError {
    fn from(err: serde_json::Error) -> Self {
        Self::ParsingError(err)
    }
}

impl FromStr for ManifestList {
    type Err = ManifestListError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let manifest_list: Self = serde_json::from_str(s)?;
        Ok(manifest_list)
    }
}

impl From<ManifestListError> for RegistryError {
    fn from(err: ManifestListError) -> Self {
        match err {
            ManifestListError::ParsingError(err) => Self::ManifestInvalid {
                detail: err.to_string(),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestListItem {
    /// The MIME type of the referenced object.
    /// This will generally be `application/vnd.docker.distribution.manifest.v2+json`, but it could also be `application/vnd.docker.distribution.manifest.v1+json` if the manifest list references a legacy schema-1 manifest.
    pub media_type: String,

    /// The size in bytes of the object.
    /// This field exists so that a client will have an expected size for the content before validating.
    /// If the length of the retrieved content does not match the specified length, the content should not be trusted.
    pub size: u64,

    /// The digest of the content, as defined by the [Registry V2 HTTP API Specificiation](https://docs.docker.com/registry/spec/api/#digest-parameter).
    pub digest: digest::ContentDigest,

    /// The platform object describes the platform which the image in the manifest runs on.
    /// A full list of valid operating system and architecture values are listed in the [Go language documentation for `$GOOS` and `$GOARCH`](https://golang.org/doc/install/source#environment)
    pub platform: ImagePlatform,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImagePlatform {
    /// The architecture field specifies the CPU architecture, for example `amd64` or `ppc64le`.
    pub architecture: String,

    /// The os field specifies the operating system, for example `linux` or `windows`.
    pub os: String,

    /// The optional variant field specifies a variant of the CPU, for example `armv6l` to specify a particular CPU variant of the ARM CPU.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,

    /// The optional features field specifies an array of strings, each listing a required CPU feature (for example `sse4` or `aes`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<String>>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_manifest_v1() {
        let json = include_str!("../../tests/data/manifest_list.json");
        let manifest = ManifestList::from_str(json);
        assert!(manifest.is_ok());
    }
}
