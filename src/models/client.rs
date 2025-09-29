use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SoundcloudIdentifier {
    Id(i64),
    Urn(String),
}

impl fmt::Display for SoundcloudIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SoundcloudIdentifier::Id(id) => write!(f, "{}", id),
            SoundcloudIdentifier::Urn(urn) => write!(f, "{}", urn),
        }
    }
}
