use crate::{
    encoding::{
        varint::{decode_varint, encode_varint},
        BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError,
    },
    types::{EncodingVersion, RollType, Stat},
};

use super::{AnyBlock, DataBlockId};

/// The block for identification data
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IdentificationData {
    /// The identifications
    pub identifications: Vec<Stat>,
    /// Whether or not extended encoding is used or to be used for encoding.
    ///
    /// If extended encoding is used then all values will have their base values and rolls encoded.
    /// Without extended encoding only the rolls are encoded and pre-identified values are ignored.
    pub extended_encoding: bool,
}

impl BlockId for IdentificationData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::IdentificationData
    }
}

impl DataEncoder for IdentificationData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::V1 => {
                // wynntils spec allows for an item to have 255 identifications and 255 pre-identified identifications
                if self
                    .identifications
                    .iter()
                    .filter(|id| id.pre_identified())
                    .count()
                    > 255
                    || self
                        .identifications
                        .iter()
                        .filter(|id| id.pre_identified())
                        .count()
                        > 255
                {
                    return Err(EncodeError::TooManyIdentifications);
                }

                let encoded_id_count: u8 = self
                    .identifications
                    .iter()
                    .filter(|id| !id.pre_identified())
                    .count() as u8;

                out.push(encoded_id_count);
                out.push(u8::from(self.extended_encoding));

                self.encode_individual_idents(out)?;

                Ok(())
            }
        }
    }

    fn should_encode_data(&self, ver: EncodingVersion) -> bool {
        match ver {
            EncodingVersion::V1 => {
                if self.extended_encoding {
                    !self.identifications.is_empty()
                } else {
                    self.identifications
                        .iter()
                        .any(|id: &Stat| !id.pre_identified())
                }
            }
        }
    }
}

impl IdentificationData {
    fn encode_individual_idents(&self, bytes: &mut Vec<u8>) -> Result<(), EncodeError> {
        // encode the static values if extended encoding is used
        if self.extended_encoding {
            let preid_stats: Vec<_> = self
                .identifications
                .iter()
                .filter(|id| id.pre_identified())
                .collect();

            bytes.push(preid_stats.len() as u8);

            for stat in preid_stats {
                // first add the id of the ident
                bytes.push(stat.kind);

                // then add the basevalue
                bytes.append(&mut encode_varint(
                    stat.base.ok_or(EncodeError::NoBasevalueGiven(stat.kind))?,
                ));
            }
        }

        for ident in &self.identifications {
            // only handle non preids since preids are encoded using the earlier system
            if let RollType::Value(roll_val) = ident.roll {
                // add id of the ident
                bytes.push(ident.kind);

                if self.extended_encoding {
                    // push the baseval
                    bytes.append(&mut encode_varint(
                        ident
                            .base
                            .ok_or(EncodeError::NoBasevalueGiven(ident.kind))?,
                    ));
                }

                bytes.push(roll_val);
            }
        }

        Ok(())
    }
}

impl DataDecoder for IdentificationData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::V1 => {
                let mut idents = Vec::new();

                // first byte is the number of identifications
                let ident_count = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

                // second byte is whether or not extended coding is used
                let extended_encoding = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)? == 1;

                let preid_count = if extended_encoding {
                    // count of preid idents
                    bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?
                } else {
                    0
                };

                for i in 0..(ident_count + preid_count) {
                    // id of the ident
                    let id = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

                    let preid = i < preid_count;

                    // decode the possible baseval if using extended coding
                    let baseval = if extended_encoding {
                        Some(decode_varint(bytes)? as i32)
                    } else {
                        None
                    };

                    // if preid skip decoding the value
                    if preid {
                        idents.push(Stat {
                            kind: id,
                            base: baseval,
                            roll: RollType::PreIdentified,
                        });
                    } else {
                        // decode the roll
                        let introll = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

                        idents.push(Stat {
                            kind: id,
                            base: baseval,
                            roll: RollType::Value(introll),
                        });
                    }
                }

                Ok(Self {
                    identifications: idents,
                    extended_encoding,
                })
            }
        }
    }
}

impl From<IdentificationData> for AnyBlock {
    fn from(data: IdentificationData) -> Self {
        AnyBlock::IdentificationData(data)
    }
}
