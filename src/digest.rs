#[derive(Debug)]
pub enum SupportedAlgorithm {
    SHA256,
}

#[derive(Debug)]
pub struct ContentDigest {
    algorithm: SupportedAlgorithm,
    hex: String,
}
