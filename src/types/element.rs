use thiserror::Error;

/// Enum representing the elements
#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Element {
    Earth = 0,
    Thunder = 1,
    Water = 2,
    Fire = 3,
    Air = 4,
}

impl From<Element> for u8 {
    fn from(value: Element) -> Self {
        value as u8
    }
}

#[derive(Error, Debug)]
#[error("Invalid element id:`{0}`")]
pub struct BadElement(pub u8);

impl TryFrom<u8> for Element {
    type Error = BadElement;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Earth),
            1 => Ok(Self::Thunder),
            2 => Ok(Self::Water),
            3 => Ok(Self::Fire),
            4 => Ok(Self::Air),

            _ => Err(BadElement(value)),
        }
    }
}
