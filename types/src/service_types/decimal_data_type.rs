// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock
//
// This file was autogenerated from Opc.Ua.Types.bsd by tools/schema/gen_types.js
//
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]
use std::io::{Read, Write};
#[allow(unused_imports)]
use crate::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    byte_string::ByteString,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DecimalDataType {
    pub scale: i16,
    pub value: ByteString,
}

impl MessageInfo for DecimalDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::DecimalDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<DecimalDataType> for DecimalDataType {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.scale.byte_len();
        size += self.value.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.scale.encode(stream)?;
        size += self.value.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let scale = i16::decode(stream, decoding_options)?;
        let value = ByteString::decode(stream, decoding_options)?;
        Ok(DecimalDataType {
            scale,
            value,
        })
    }
}
