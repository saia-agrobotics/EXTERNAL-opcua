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
    service_types::enums::FilterOperator,
    extension_object::ExtensionObject,
};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ContentFilterElement {
    pub filter_operator: FilterOperator,
    pub filter_operands: Option<Vec<ExtensionObject>>,
}

impl MessageInfo for ContentFilterElement {
    fn object_id(&self) -> ObjectId {
        ObjectId::ContentFilterElement_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<ContentFilterElement> for ContentFilterElement {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.filter_operator.byte_len();
        size += byte_len_array(&self.filter_operands);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.filter_operator.encode(stream)?;
        size += write_array(stream, &self.filter_operands)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let filter_operator = FilterOperator::decode(stream, decoding_options)?;
        let filter_operands: Option<Vec<ExtensionObject>> = read_array(stream, decoding_options)?;
        Ok(ContentFilterElement {
            filter_operator,
            filter_operands,
        })
    }
}
