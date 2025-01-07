use crate::{
    encoding::{BlockId, DataDecoder, DataEncoder, DecodeError, EncodeError},
    types::{Element, EncodingVersion, Powder},
};

use super::{AnyBlock, DataBlockId};

/// The block for powder data
#[derive(PartialEq, Eq, Clone, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PowderData {
    /// The number of powder slots on this item
    pub powder_slots: u8,
    /// The powders on this item along with the tier of the powders (currently unused as wynntils does not encode this data)
    pub powders: Vec<Powder>,
}

impl BlockId for PowderData {
    fn block_id(&self) -> DataBlockId {
        DataBlockId::PowderData
    }
}

impl DataEncoder for PowderData {
    fn encode_data(&self, ver: EncodingVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            EncodingVersion::Version1 => {
                let powders_len =
                    u8::try_from(self.powders.len()).map_err(|_| EncodeError::TooManyPowders)?;

                let bits_needed = self.powders.len() * 5;
                let total_bits = (bits_needed + 7) / 8;

                let mut powder_data = vec![0u8; total_bits];

                for (i, pow) in self.powders.iter().enumerate() {
                    let (elem, tier) = (pow.element() as u8, pow.tier());

                    // calculate the 5 bit powder value
                    let powder_num = (elem * 6 + tier) & 0b00011111;

                    // bit position where this specific powder starts
                    let powder_idx = i * 5;

                    // set the values
                    for j in 0..5 {
                        // calculate the bit position of this bit
                        let idx = powder_idx + j;
                        let bit = (powder_num >> (4 - j)) & 0b1;
                        powder_data[idx / 8] |= bit << (7 - (idx % 8));
                    }
                }

                out.push(self.powder_slots);
                out.push(powders_len);

                out.append(&mut powder_data);
            }
        }

        Ok(())
    }
}

impl DataDecoder for PowderData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: EncodingVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            EncodingVersion::Version1 => {
                let slots = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;
                let powder_count = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)? as usize;

                let bits_needed = powder_count * 5;
                let total_bytes = (bits_needed + 7) / 8;

                let bytes: Vec<u8> = bytes.take(total_bytes).collect();
                let mut powders = Vec::new();

                for powder_idx in 0..powder_count {
                    let mut powder = 0u8;

                    for i in 0..5 {
                        let idx = (powder_idx * 5) + i;
                        let bit = bytes[idx / 8] >> (7 - (idx % 8)) & 0b1;
                        powder |= bit << (4 - i);
                    }

                    if powder == 0 {
                        // ignore empty powders
                        continue;
                    }

                    let (elem, tier) = if powder % 6 == 0 {
                        ((powder / 6) - 1, 6)
                    } else {
                        ((powder / 6), powder % 6)
                    };
                    powders.push(Powder::try_from((Element::try_from(elem)?, tier))?);
                }

                Ok(Self {
                    powder_slots: slots,
                    powders,
                })
            }
        }
    }
}

impl From<PowderData> for AnyBlock {
    fn from(data: PowderData) -> Self {
        AnyBlock::PowderData(data)
    }
}
