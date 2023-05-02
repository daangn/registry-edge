use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt, str::FromStr};

use crate::errors::RegistryError;

#[derive(Debug, Eq, PartialEq)]
pub enum SupportedAlgorithm {
    Sha256,
}

impl fmt::Display for SupportedAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Sha256 => write!(f, "sha256"),
        }
    }
}

impl FromStr for SupportedAlgorithm {
    type Err = RegistryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sha256" => Ok(Self::Sha256),
            _ => Err(RegistryError::DigestInvalid {
                detail: "only sha256 digest currently supported".to_string(),
            }),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ContentDigest {
    pub alg: SupportedAlgorithm,
    pub hash: String,
}

impl fmt::Display for ContentDigest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.alg, self.hash)
    }
}

impl FromStr for ContentDigest {
    type Err = RegistryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<alg>[A-Za-z0-9_+.-]+):(?P<hash>[A-Fa-f0-9]+)$").unwrap();
        }
        RE.captures(s)
            .and_then(|cap| match (cap.name("alg"), cap.name("hash")) {
                (Some(alg), Some(hash)) => match SupportedAlgorithm::from_str(alg.as_str()) {
                    Ok(alg) => Some(Self {
                        alg,
                        hash: hash.as_str().to_string(),
                    }),
                    _ => None,
                },
                _ => None,
            })
            .ok_or_else(|| RegistryError::DigestInvalid {
                detail: "format shoud be `{alg}:{hash}`".to_string(),
            })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_digest() {
        let digest = "sha256:6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b".parse::<ContentDigest>();
        assert!(digest.is_ok());

        let digest = digest.unwrap();
        assert_eq!(digest.alg, SupportedAlgorithm::Sha256);
        assert_eq!(
            digest.hash,
            "6c3c624b58dbbcd3c0dd82b4c53f04194d1247c6eebdaab7c610cf7d66709b3b".to_string()
        )
    }

    #[test]
    fn invalid_unsupported_algorithm() {
        let digest =
            "sha512:ee26b0dd4af7e749aa1a8ee3c10ae9923f618980772e473f8819a5d4940e0db27ac185f8a0e1d5f84f88bc887fd67b143732c304cc5fa9ad8e6f57f50028a8ff"
                .parse::<ContentDigest>();
        assert!(digest.is_err());
    }
}

impl serde::ser::Serialize for ContentDigest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl<'de> serde::de::Deserialize<'de> for ContentDigest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ContentDigest;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "\"alg:hash\" string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                ContentDigest::from_str(value).map_err(E::custom)
            }
        }

        deserializer.deserialize_identifier(Visitor)
    }
}
