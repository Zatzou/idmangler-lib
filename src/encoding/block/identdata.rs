use crate::{
    encoding::{
        traits::{DataDecoder, DataEncoder, TransformId},
        varint::{decode_varint, encode_varint},
        AnyData, DecodeError, EncodeError,
    },
    types::{EncodingVersion, RollType, Stat},
};

use super::DataBlockId;

/// The transformer for identification data
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct IdentificationData {
    /// The identifications
    pub identifications: Vec<Stat>,
    /// Whether or not extended encoding is used or to be used for encoding.
    ///
    /// If extended encoding is used then all values will have their base values and rolls encoded.
    /// Without extended encoding only the rolls are encoded and pre-identified values are ignored.
    pub extended_encoding: bool,
}

impl TransformId for IdentificationData {
    const TRANSFORMER_ID: u8 = DataBlockId::IdentificationData as u8;
}

impl DataEncoder for IdentificationData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => {
                if self.identifications.len() > 255 {
                    return Err(EncodeError::TooManyIdentifications);
                }

                let encoded_id_count: u8 = self
                    .identifications
                    .iter()
                    .filter(|id| !id.pre_identified())
                    .count() as u8;

                out.push(encoded_id_count);
                out.push(self.extended_encoding as u8);

                self.encode_individual_idents(out)?;

                Ok(())
            }
        }
    }

    fn should_encode_data(&self, ver: EncodingVersion) -> bool {
        match ver {
            EncodingVersion::Version1 => {
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
                    stat.base.ok_or(EncodeError::NoBasevalueGiven(stat.kind))? as i64,
                ));
            }
        }

        for ident in self.identifications.iter() {
            // only handle non preids since preids are encoded using the earlier system
            if let RollType::Value(roll_val) = ident.roll {
                // add id of the ident
                bytes.push(ident.kind);

                if self.extended_encoding {
                    // push the baseval
                    bytes.append(&mut encode_varint(
                        ident
                            .base
                            .ok_or(EncodeError::NoBasevalueGiven(ident.kind))?
                            as i64,
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
            EncodingVersion::Version1 => {
                let mut idents = Vec::new();

                // first byte is the number of identifications
                let ident_count = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

                // second byte is whether or not extended coding is used
                let extended_encoding = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)? == 1;

                let mut preid_count = 0;
                if extended_encoding {
                    // count of preid idents
                    preid_count = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;
                }

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

impl From<IdentificationData> for AnyData {
    fn from(value: IdentificationData) -> Self {
        Self::IdentificationData(value)
    }
}
