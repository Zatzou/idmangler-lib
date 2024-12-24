use block::{AnyBlock, StartData};
use encoding::string::decode_string;

pub mod block;
pub mod encoding;
pub mod item;
pub mod types;

pub fn decode(input: &str) -> Result<Vec<AnyBlock>, encoding::DecoderError> {
    decode_bytes(&decode_string(input).map_err(|e| encoding::DecoderError {
        error: e.into(),
        during: None,
    })?)
}

pub fn decode_bytes(bytes: &[u8]) -> Result<Vec<AnyBlock>, encoding::DecoderError> {
    let mut out = Vec::new();

    let bytes = &mut bytes.iter().copied();

    // decode the start byte and version
    let ver = block::StartData::decode_start_bytes(bytes).map_err(|e| encoding::DecoderError {
        error: e,
        during: Some(block::DataBlockId::StartData),
    })?;

    out.push(StartData(ver).into());

    // decode the rest of the bytes
    while let Some(id) = bytes.next() {
        let id = block::DataBlockId::try_from(id).map_err(|e| encoding::DecoderError {
            error: e.into(),
            during: None,
        })?;

        out.push(id.decode(ver, bytes)?);
    }

    Ok(out)
}
