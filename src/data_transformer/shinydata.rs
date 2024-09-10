use crate::{encoding::encode_varint, types::transform::TransformVersion};

use super::{DataEncoder, DataTransformerTypes, EncodeError, TransformId};

pub struct ShinyData {
    /// The id of the Shiny stat
    ///
    /// The ids can be found on <https://github.com/Wynntils/Static-Storage/blob/main/Data-Storage/shiny_stats.json>
    pub id: u8,
    /// The value of the given shiny stat
    pub val: i64,
}

impl TransformId for ShinyData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::ShinyDataTransformer as u8;
}

impl DataEncoder for ShinyData {
    fn encode_data(
        &self,
        ver: crate::types::transform::TransformVersion,
        out: &mut Vec<u8>,
    ) -> Result<(), EncodeError> {
        match ver {
            TransformVersion::Version1 => {
                out.push(self.id);
                out.append(&mut encode_varint(self.val));
            }
        }

        Ok(())
    }
}
