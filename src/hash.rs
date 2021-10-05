use serde::Serialize;

type Hash = [u8; 32];

#[derive(Debug, Serialize)]
pub(crate) struct BlockData<'a> {
    pub parent_hash: Option<Hash>,
    pub payload: &'a str,
}

impl BlockData<'_> {
    pub(crate) fn hash(&self) -> Hash {
        blake3::hash(&bincode::serialize(self).unwrap()).into()
    }
}
