use std::{collections::HashMap, convert::TryInto, fmt::Display};

use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

/// Serialized Blockchain representation.
#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub data: HashMap<Digest, Data>,
}

/// Serialized Block representation.
#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub prev: Option<Digest>,
    pub payload: Digest,
    pub digest: Digest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Data {
    pub payload: String,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            data: HashMap::new(),
        }
    }

    pub fn anchor(&mut self, payload: &str) {
        let prev = self.blocks.last().map(|last| last.digest);
        let data = Data {
            payload: payload.into(),
        };
        let payload_digest = data.get_digest();
        let mut block = Block {
            prev,
            payload: payload_digest,
            digest: Digest::default(),
        };
        block.digest = block.get_digest();
        self.blocks.push(block);
        self.data.insert(payload_digest, data);
    }

    pub fn validate(&self) -> Result<(), Error> {
        let mut prev_hash = None;
        for block in &self.blocks {
            if block.prev != prev_hash {
                return Err(Error::InvalidParentDigest);
            }
            if block.digest != block.get_digest() {
                return Err(Error::InvalidDigest);
            }
            prev_hash = Some(block.digest);
        }
        Ok(())
    }
}

impl Block {
    /// Computes hash based on block fields.
    fn get_digest(&self) -> Digest {
        #[derive(Debug, Serialize)]
        struct HashInput {
            prev: Option<Digest>,
            payload: Digest,
        }

        let input = HashInput {
            prev: self.prev,
            payload: self.payload,
        };
        let bytes = bincode::serialize(&input).unwrap();
        Digest(blake3::hash(&bytes).into())
    }
}

impl Data {
    fn get_digest(&self) -> Digest {
        Digest(blake3::hash(&self.payload.as_bytes()).into())
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidDigest,
    InvalidParentDigest,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Error::InvalidDigest => write!(f, "invalid digest"),
            &Error::InvalidParentDigest => write!(f, "invalid parent digest"),
        }
    }
}

impl std::error::Error for Error {}

// #[test]
// fn get_hash_test() {
//     let block = Block {
//         prev: None,
//         payload: Digest::default(),
//         digest: Digest::default(),
//     };
//     assert_eq!(
//         block.get_digest(),
//         Digest([
//             117, 154, 127, 33, 90, 227, 203, 89, 28, 80, 35, 144, 68, 16, 100, 195, 44, 203, 115,
//             5, 144, 224, 214, 157, 13, 98, 56, 45, 28, 239, 201, 88
//         ])
//     );
// }

// #[test]
// fn serialize_test() {
//     let mut bc = Blockchain::new();
//     bc.anchor("hello");
//     bc.anchor("world");
//     bc.anchor("!");
//     assert_eq!(
//         serde_json::to_value(&bc).unwrap(),
//         serde_json::json!({
//             "blocks":[
//                 {"prev": null, "payload": "hello", "digest": "PNCuVuF/YvR3tjChYLHz62b2CNn/uTkX4TzpK2K31mM="},
//                 {"prev": "PNCuVuF/YvR3tjChYLHz62b2CNn/uTkX4TzpK2K31mM=", "payload": "world", "digest": "TvbjV6Oy0Ldi7b3E1Ay+JmwwO3LL9bjKyLvDQHOTSnI="},
//                 {"prev": "TvbjV6Oy0Ldi7b3E1Ay+JmwwO3LL9bjKyLvDQHOTSnI=", "payload": "!", "digest": "lbqFTR8Qq7RE07K7VyxwN9WLuQm5mcFwWmndVM4g+Q8="},
//             ]
//         })
//     );
// }

#[test]
fn deserialize_test_fail() {
    let bc = serde_json::from_value::<Blockchain>(serde_json::json!({
        "blocks": [
            {"prev": null, "payload": "hello", "digest": "0000000000000000000000000000000000000000000="},
        ]
    })).unwrap();
    assert!(bc.validate().is_err());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Digest(pub [u8; 32]);

impl Serialize for Digest {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&base64::encode(&self.0))
    }
}

impl<'de> Deserialize<'de> for Digest {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let str = <String>::deserialize(deserializer)?;
        let vec = base64::decode(str).map_err(de::Error::custom)?;
        let digest = vec
            .try_into()
            .map_err(|_| de::Error::custom("invalid digest length"))?;
        Ok(Digest(digest))
    }
}

impl Default for Digest {
    fn default() -> Self {
        Self([0; 32])
    }
}
