use crate::types::{Powders, TransformVersion};

use super::{DataEncoder, DataTransformerTypes, EncodeError, TransformId};

/// The transformer for powder data
#[derive(Debug, Clone)]
pub struct PowderData {
    /// The number of powder slots on this item
    pub powder_slots: u8,
    /// The powders on this item along with the tier of the powders (currently unused as wynntils does not encode this data)
    pub powders: Vec<(Powders, u8)>,
}

impl TransformId for PowderData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::PowderData as u8;
}

impl DataEncoder for PowderData {
    fn encode_data(&self, ver: TransformVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            TransformVersion::Version1 => {
                let bits_needed = self.powders.len() * 5;
                let total_bits = (bits_needed + 7) / 8;

                let mut powder_data = vec![0u8; total_bits];

                for (i, pow) in self.powders.iter().enumerate() {
                    let elem = pow.0 as u8;
                    // TODO: figure out if wynntils fixes this and make the tier be encoded correctly
                    let tier = 0; //pow.1;

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
                out.push(self.powders.len() as u8);

                out.append(&mut powder_data);
            }
        }

        Ok(())
    }
}
