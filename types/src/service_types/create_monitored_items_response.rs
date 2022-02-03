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
    response_header::ResponseHeader,
    diagnostic_info::DiagnosticInfo,
    service_types::MonitoredItemCreateResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CreateMonitoredItemsResponse {
    pub response_header: ResponseHeader,
    pub results: Option<Vec<MonitoredItemCreateResult>>,
    pub diagnostic_infos: Option<Vec<DiagnosticInfo>>,
}

impl MessageInfo for CreateMonitoredItemsResponse {
    fn object_id(&self) -> ObjectId {
        ObjectId::CreateMonitoredItemsResponse_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<CreateMonitoredItemsResponse> for CreateMonitoredItemsResponse {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.response_header.byte_len();
        size += byte_len_array(&self.results);
        size += byte_len_array(&self.diagnostic_infos);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.response_header.encode(stream)?;
        size += write_array(stream, &self.results)?;
        size += write_array(stream, &self.diagnostic_infos)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let response_header = ResponseHeader::decode(stream, decoding_options)?;
        let results: Option<Vec<MonitoredItemCreateResult>> = read_array(stream, decoding_options)?;
        let diagnostic_infos: Option<Vec<DiagnosticInfo>> = read_array(stream, decoding_options)?;
        Ok(CreateMonitoredItemsResponse {
            response_header,
            results,
            diagnostic_infos,
        })
    }
}
