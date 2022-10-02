use std::borrow::Cow;
use std::error::Error as StdError;

use heed::byteorder::{BigEndian, ByteOrder};
use heed::{BytesDecode, BytesEncode};
use serde::{Deserialize, Serialize};

use crate::kv::LogValueRef;
use crate::LogKey;

pub struct KeyFormat;

impl<'a> BytesEncode<'a> for KeyFormat {
    type EItem = LogKey;

    fn bytes_encode(item: &'a Self::EItem) -> Result<Cow<'a, [u8]>, Box<dyn StdError>> {
        let mut buf = [0; 10];
        BigEndian::write_u64(&mut buf, item.rev());
        BigEndian::write_u16(&mut buf[8 ..], item.ent());
        Ok(Cow::Owned(Vec::from(buf)))
    }
}

impl<'a> BytesDecode<'a> for KeyFormat {
    type DItem = LogKey;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, Box<dyn StdError>> {
        if bytes.len() != 10 {
            panic!("Unknown key length")
        }

        Ok(LogKey::new(
            BigEndian::read_u64(&bytes),
            BigEndian::read_u16(&bytes[8 ..]),
        ))
    }
}

pub struct ValueFormat;

impl<'a> BytesDecode<'a> for ValueFormat {
    type DItem = LogValueRef<'a>;

    fn bytes_decode(bytes: &'a [u8]) -> Result<Self::DItem, Box<dyn StdError>> {
        let del = bytes[0] != 0;

        let klen = BigEndian::read_u32(&bytes[1 .. 5]) as usize;
        let key = &bytes[5 .. 5 + klen];

        let vlen = BigEndian::read_u32(&bytes[5 + klen .. 9 + klen]) as usize;
        let val = &bytes[9 + klen .. 9 + klen + vlen];

        Ok(LogValueRef { del, key, val })
    }
}

impl<'a> BytesEncode<'a> for ValueFormat {
    type EItem = LogValueRef<'a>;

    fn bytes_encode(item: &'a Self::EItem) -> Result<Cow<'a, [u8]>, Box<dyn StdError>> {
        let mut buf: Vec<u8> = Vec::with_capacity(9 + item.key.len() + item.val.len());

        buf.push(if item.del { 1 } else { 0 });

        buf.extend_from_slice(&(item.key.len() as u32).to_be_bytes());
        buf.extend_from_slice(item.key);

        buf.extend_from_slice(&(item.val.len() as u32).to_be_bytes());
        buf.extend_from_slice(item.val);

        Ok(Cow::Owned(buf))
    }
}
