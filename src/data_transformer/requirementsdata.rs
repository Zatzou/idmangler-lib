use crate::{
    encoding::{decode_varint, encode_varint},
    types::{ClassType, SkillType, TransformVersion},
};

use super::{
    AnyData, DataDecoder, DataEncoder, DataTransformerTypes, DecodeError, EncodeError, TransformId,
};

/// Requirements of a crafted item
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
pub struct RequirementsData {
    /// Level requirement of the item
    pub level: u8,
    /// Class requirement of the item
    ///
    /// This is [`None`] if there is no class requirement
    pub class: Option<ClassType>,
    /// Skillpoint requirements of the item
    pub skills: Vec<(SkillType, i32)>,
}

impl TransformId for RequirementsData {
    const TRANSFORMER_ID: u8 = DataTransformerTypes::RequirementsData as u8;
}

impl DataEncoder for RequirementsData {
    fn encode_data(&self, ver: TransformVersion, out: &mut Vec<u8>) -> Result<(), EncodeError> {
        match ver {
            TransformVersion::Version1 => {
                // level requirement
                out.push(self.level);

                // class requirement
                let class = self.class.map_or(0, Into::into);
                out.push(class);

                if self.skills.len() > 255 {
                    return Err(EncodeError::TooManySkills);
                }

                // encode number of skill requirements
                out.push(self.skills.len() as u8);

                for (skill, value) in self.skills.iter() {
                    // skill id
                    out.push((*skill).into());

                    // skill requirement value
                    out.append(&mut encode_varint(*value as i64));
                }

                Ok(())
            }
        }
    }
}

impl DataDecoder for RequirementsData {
    fn decode_data(
        bytes: &mut impl Iterator<Item = u8>,
        ver: TransformVersion,
    ) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        match ver {
            TransformVersion::Version1 => {
                let level = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;

                let class = match bytes.next() {
                    Some(0) => None,
                    Some(c) => Some(ClassType::try_from(c)?),
                    None => return Err(DecodeError::UnexpectedEndOfBytes),
                };

                let skill_count = bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?;
                let mut skills = Vec::with_capacity(skill_count as usize);

                for _ in 0..skill_count {
                    let skill = SkillType::try_from(
                        bytes.next().ok_or(DecodeError::UnexpectedEndOfBytes)?,
                    )?;
                    let value = decode_varint(bytes)?;

                    skills.push((skill, value as i32));
                }

                Ok(Self {
                    level,
                    class,
                    skills,
                })
            }
        }
    }
}

impl From<RequirementsData> for AnyData {
    fn from(data: RequirementsData) -> Self {
        AnyData::RequirementsData(data)
    }
}
