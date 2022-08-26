use std::fmt;

// See https://docs.docker.com/registry/spec/api/#errors-2
#[derive(Debug)]
pub enum RegistryError {
    Unsupported,
    InvalidDigest { detail: String },
    InvalidManifest,
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported => write!(f, "unsupported operation"),
            Self::InvalidDigest { detail } => write!(f, "invalid digest, {}", detail),
            Self::InvalidManifest => write!(f, "invalid manifest"),
        }
    }
}
