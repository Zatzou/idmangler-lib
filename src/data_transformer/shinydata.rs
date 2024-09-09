use crate::{encoding::encode_varint, types::transform::TransformVersion};

use super::{DataEncoder, DataTransformerTypes, EncodeError, TransformId};

pub struct ShinyData {
    pub id: u8,
    pub val: i64,
}

impl TransformId for ShinyData {
    fn get_id() -> u8 {
        DataTransformerTypes::ShinyDataTransformer as u8
    }
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
