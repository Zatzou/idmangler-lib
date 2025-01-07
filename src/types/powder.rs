use thiserror::Error;

use super::Element;

/// Struct representing a powder
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(try_from = "(Element, u8)"))]
pub struct Powder {
    element: Element,
    tier: u8,
}

impl Powder {
    /// Create a new powder with the given element and tier. You may also use `Powder::try_from((element, tier))` to create a powder.
    ///
    /// # Errors
    /// Creating a powder will fail if the tier is not between 1 and 6
    pub fn new(element: Element, tier: u8) -> Result<Self, InvalidPowderTier> {
        Self::try_from((element, tier))
    }

    /// Get the element type of this powder
    pub const fn element(&self) -> Element {
        self.element
    }

    /// Get the tier of this powder
    pub const fn tier(&self) -> u8 {
        self.tier
    }

    /// Set the element of this powder
    pub fn set_element(&mut self, element: Element) {
        self.element = element;
    }

    /// Set the tier of this powder
    ///
    /// # Errors
    /// Setting the tier will fail if the tier is not between 1 and 6
    pub fn set_tier(&mut self, tier: u8) -> Result<(), InvalidPowderTier> {
        if Self::valid_tier(tier) {
            self.tier = tier;
            Ok(())
        } else {
            Err(InvalidPowderTier(tier))
        }
    }

    /// Check if the given tier is valid for a powder
    pub fn valid_tier(tier: u8) -> bool {
        (1..=6).contains(&tier)
    }
}

#[derive(Error, Debug)]
#[error("Invalid powder tier: {0}")]
pub struct InvalidPowderTier(pub u8);

impl TryFrom<(Element, u8)> for Powder {
    type Error = InvalidPowderTier;

    fn try_from((element, tier): (Element, u8)) -> Result<Self, Self::Error> {
        if Self::valid_tier(tier) {
            Ok(Self { element, tier })
        } else {
            Err(InvalidPowderTier(tier))
        }
    }
}

impl PartialEq<(Element, u8)> for Powder {
    fn eq(&self, other: &(Element, u8)) -> bool {
        self.element == other.0 && self.tier == other.1
    }
}
