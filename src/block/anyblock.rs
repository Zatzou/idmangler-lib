use std::ops::{Deref, DerefMut};

use crate::encoding::{
    string::{decode_string, encode_string},
    DataBlock, DecoderError, EncodeError, EncoderError,
};

use super::{DataBlockId, StartData};

pub type AnyBlock = Box<dyn DataBlock>;

#[derive(Debug, Default)]
pub struct AnyBlockVec {
    inner: Vec<AnyBlock>,
}

impl AnyBlockVec {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn read<T: 'static>(&self) -> Option<&T> {
        self.inner
            .iter()
            .find_map(|b| b.as_any().downcast_ref::<T>())
    }

    pub fn take<T: 'static>(&mut self) -> Option<T> {
        let i = self
            .inner
            .iter()
            .enumerate()
            .find(|(_, b)| (*b).as_any().downcast_ref::<T>().is_some())
            .map(|(i, _)| i)?;

        Some(*self.inner.remove(i).into_any().downcast::<T>().unwrap())
    }

    pub fn encode(&self) -> Result<String, EncoderError> {
        Ok(encode_string(&self.encode_bytes()?))
    }

    pub fn encode_bytes(&self) -> Result<Vec<u8>, EncoderError> {
        let mut out = Vec::new();

        // Read the encoding version from the start block
        let ver = self
            .read::<StartData>()
            .ok_or(EncoderError {
                error: EncodeError::NoStartBlock,
                during: DataBlockId::StartData,
            })?
            .0;

        for block in &self.inner {
            block.encode(ver, &mut out)?;
        }

        Ok(out)
    }

    pub fn decode(input: &str) -> Result<Self, DecoderError> {
        Self::decode_bytes(&decode_string(input).map_err(|e| DecoderError {
            error: e.into(),
            during: None,
        })?)
    }

    pub fn decode_bytes(input: &[u8]) -> Result<Self, DecoderError> {
        let mut out = Self::new();

        let mut iter = input.iter().copied();
        let bytes = &mut iter;

        // decode the start byte and version
        let ver = StartData::decode_start_bytes(bytes).map_err(|e| DecoderError {
            error: e,
            during: Some(DataBlockId::StartData),
        })?;

        // push the start data to the output
        out.push(Box::new(StartData(ver)));

        while let Some(id) = bytes.next() {
            let block = DataBlockId::try_from(id)
                .map_err(|e| DecoderError {
                    error: e.into(),
                    during: None,
                })?
                .decode(ver, bytes)?;

            out.push(block);
        }

        Ok(out)
    }
}

impl Deref for AnyBlockVec {
    type Target = Vec<AnyBlock>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for AnyBlockVec {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
