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
};

#[derive(Debug, Clone, PartialEq)]
pub struct DataSetReaderMessageDataType {
}

impl MessageInfo for DataSetReaderMessageDataType {
    fn object_id(&self) -> ObjectId {
        ObjectId::DataSetReaderMessageDataType_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<DataSetReaderMessageDataType> for DataSetReaderMessageDataType {
    fn byte_len(&self) -> usize {
        0
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        Ok(0)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        Ok(DataSetReaderMessageDataType {
        })
    }
}
