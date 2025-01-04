use crate::macros::numbered_enum;

numbered_enum! {
    /// This enum represents the version of the encoding being used
    ///
    /// At the current time the only version of the encoding is the first version
    #[repr(u8)]
    pub enum EncodingVersion {
        /// Represents the version 1 of the wynntils encoding scheme
        Version1 = 0,
    }

    #[error("Unknown encoding version: {0}")]
    etype UnknownEncodingVersion;
}
