use crate::Kind;
use serde::{Deserialize, Serialize};

#[derive(Hash, Kind, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Checksum(pub(crate) [u8; 32]);
