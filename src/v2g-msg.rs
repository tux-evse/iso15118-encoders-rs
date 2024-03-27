/*
 * Copyright (C) 2015-2022 IoT.bzh Pionix, Chargebyte and Everest contributors
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Rust largely inspired from Everest C++ git@github.com:/EVerest/libiso15118.git
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 */

use crate::prelude::*;

#[derive(Debug, Clone)]
pub enum SupportedAppProtocolTagId {
    Iso20,
    Iso2,
    Din,
    Unknown,
}

pub struct SupportedAppProtocolConf {
    pub tag_id: SupportedAppProtocolTagId,
    pub name: &'static str,
    pub major: u32,
    pub minor: u32,
}

const PROTOCOLS_SUPPORTED_LIST: [&SupportedAppProtocolConf; 3] = [
    &SupportedAppProtocolConf {
        name: "urn:iso:15118:20:2022:MsgDef",
        tag_id: SupportedAppProtocolTagId::Iso20,
        major: 2,
        minor: 0,
    },
    &SupportedAppProtocolConf {
        tag_id: SupportedAppProtocolTagId::Iso2,
        name: "urn:iso:15118:2:2013:MsgDef",
        major: 2,
        minor: 0,
    },
    &SupportedAppProtocolConf {
        tag_id: SupportedAppProtocolTagId::Din,
        name: "urn:din:70121:2012:MsgDef",
        major: 2,
        minor: 0,
    },
];

pub struct SupportedAppProtocolMsg {
    tag_id: SupportedAppProtocolTagId,
    supported: &'static [&'static SupportedAppProtocolConf],
    requested: Vec<SupportedAppProtocolReq>,
    selected: Option<SupportedAppProtocolRes>,
}

impl SupportedAppProtocolMsg {
    pub fn decode_msg(stream: &RawStream) -> Result<Self, AfbError> {
        let app_hand = SupportedAppProtocolExi::decode_from_stream(stream)?;
        let requested_protos = app_hand.get_protocols()?;
        Ok(SupportedAppProtocolMsg {
            tag_id: SupportedAppProtocolTagId::Unknown,
            supported: &PROTOCOLS_SUPPORTED_LIST,
            requested: requested_protos,
            selected: None,
        })
    }

    pub fn get_requested(&self) -> &Vec<SupportedAppProtocolReq> {
        &self.requested
    }

    pub fn match_protocol(&mut self) -> &Self {
        for request in &self.requested {
            for provided in self.supported {
                if request.name_space.as_str() == provided.name
                    && request.version_number_major == provided.major
                {
                    afb_log_msg!(Debug, None, "Protocol:{} selected", request.name_space);
                    self.tag_id = provided.tag_id.clone();
                    if request.version_number_minor == provided.minor {
                        self.selected = Some(SupportedAppProtocolRes {
                            schema_id: request.schema_id,
                            response_code: SupportedAppResponseCode::Success,
                        });
                        break;
                    } else {
                        self.selected = Some(SupportedAppProtocolRes {
                            schema_id: request.schema_id,
                            response_code: SupportedAppResponseCode::SuccessWithMinorDeviation,
                        });
                        break;
                    }
                }
            }
            if let Some(_) = self.selected {
                break;
            }
        }
        self
    }

    pub fn put_response(&self, stream: &mut RawStream) -> Result<SupportedAppProtocolTagId, AfbError> {
        stream.reset(); // start from a clean stream

        // respond even when no matching protocol found
        SupportedAppProtocolExi::encode_to_stream(stream, &self.selected)?;
        match self.selected {
            Some(_) => {}
            None => {
                return afb_error!("supported-protocol-app", "fail to match requested protocol")
            }
        };
        Ok(self.tag_id.clone())
    }
}
