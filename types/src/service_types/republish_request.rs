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
    request_header::RequestHeader,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RepublishRequest {
    pub request_header: RequestHeader,
    pub subscription_id: u32,
    pub retransmit_sequence_number: u32,
}

impl MessageInfo for RepublishRequest {
    fn object_id(&self) -> ObjectId {
        ObjectId::RepublishRequest_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<RepublishRequest> for RepublishRequest {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.request_header.byte_len();
        size += self.subscription_id.byte_len();
        size += self.retransmit_sequence_number.byte_len();
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.request_header.encode(stream)?;
        size += self.subscription_id.encode(stream)?;
        size += self.retransmit_sequence_number.encode(stream)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let request_header = RequestHeader::decode(stream, decoding_options)?;
        let subscription_id = u32::decode(stream, decoding_options)?;
        let retransmit_sequence_number = u32::decode(stream, decoding_options)?;
        Ok(RepublishRequest {
            request_header,
            subscription_id,
            retransmit_sequence_number,
        })
    }
}
