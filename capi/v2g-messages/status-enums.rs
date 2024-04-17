/*
 * Copyright (C) 2015-2022 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */
use super::*;
use serde::{Deserialize, Serialize};
use std::mem;

#[derive(Debug)]
#[allow(non_camel_case_types)]
#[repr(u16)]
pub enum PayloadMsgId {
    INVALID = 0,
    SAP = cglue::V2GTP20_SAP_PAYLOAD_ID as u16,
    MAIN = cglue::V2GTP20_MAINSTREAM_PAYLOAD_ID as u16,
    AC_MAIN = cglue::V2GTP20_AC_MAINSTREAM_PAYLOAD_ID as u16,
    DC_MAIN = cglue::V2GTP20_DC_MAINSTREAM_PAYLOAD_ID as u16,
    ACDP_MAIN = cglue::V2GTP20_ACDP_MAINSTREAM_PAYLOAD_ID as u16,
    WPT_MAIN = cglue::V2GTP20_WPT_MAINSTREAM_PAYLOAD_ID as u16,
    SCHED_NEGO = cglue::V2GTP20_SCHEDULE_RENEGOTIATION_PAYLOAD_ID as u16,
    METERING = cglue::V2GTP20_METERING_CONFIRMATION_PAYLOAD_ID as u16,
    ACDP_STATUS = cglue::V2GTP20_ACDP_SYSTEM_STATUS_PAYLOAD_ID as u16,
    PARKING_STATUS = cglue::V2GTP20_PARKING_STATUS_PAYLOAD_ID as u16,
    SDP_REQUEST = cglue::V2GTP20_SDP_REQUEST_PAYLOAD_ID as u16,
    SDP_RESPONSE = cglue::V2GTP20_SDP_RESPONSE_PAYLOAD_ID as u16,
    SDP_REQUEST_WIRELESS = cglue::V2GTP20_SDP_REQUEST_WIRELESS_PAYLOAD_ID as u16,
    SDP_RESPONSE_WIRELESS = cglue::V2GTP20_SDP_RESPONSE_WIRELESS_PAYLOAD_ID as u16,
}

impl PayloadMsgId {
    pub fn from_u16(code: u16) -> Self {
        unsafe { mem::transmute(code) }
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum ProtocolTagId {
    Iso20,
    Iso2,
    Din,
    Unknown=255,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[repr(u32)]
pub enum ResponseCode {
    Success = cglue::appHand_responseCodeType_appHand_responseCodeType_OK_SuccessfulNegotiation,
    SuccessWithMinorDeviation = cglue::appHand_responseCodeType_appHand_responseCodeType_OK_SuccessfulNegotiationWithMinorDeviation,
    Failed = cglue::appHand_responseCodeType_appHand_responseCodeType_Failed_NoNegotiation,
}

impl ResponseCode {
    pub fn from_u32(code: u32) -> Self {
        unsafe { mem::transmute(code) }
    }
    pub fn from_json(json: &str) -> Result<Self, AfbError> {
        match serde_json::from_str(json) {
            Ok(value) => Ok(value),
            Err(error) => return afb_error!("get-from-json", "fail deserialize:{}", error),
        }
    }

    pub fn to_json(self) -> Result<String, AfbError> {
        match serde_json::to_string(&self) {
            Ok(value) => Ok(value),
            Err(error) => return afb_error!("to-from-json", "fail serializing:{}", error),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SupportedAppProtocolConf {
    pub tag_id: ProtocolTagId,
    pub name: &'static str,
    pub major: u32,
    pub minor: u32,
}

impl SupportedAppProtocolConf {
    pub fn from_schema<'a>(
        schema_id: u8,
        protocols: &'a [&SupportedAppProtocolConf],
    ) -> Result<&'a SupportedAppProtocolConf, AfbError> {
        for idx in 0..protocols.len() {
            let protocol = &protocols[idx];
            let tag_id = protocol.tag_id.clone() as u8;
            if schema_id == tag_id {
                return Ok(protocols[idx]);
            }
        }
        afb_error!("support-protocol-conf", "Invalid:{} schema_id", schema_id)
    }

    pub fn to_json(&self) -> Result<String, AfbError> {
        match serde_json::to_string(self) {
            Ok(value) => Ok(value),
            Err(error) => return afb_error!("to-from-json", "fail serializing:{}", error),
        }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_schema(&self) -> ProtocolTagId {
        self.tag_id.clone()
    }
}

// default protocol configuration for test and simulation
pub const V2G_PROTOCOLS_SUPPORTED_LIST: [&SupportedAppProtocolConf; 3] = [
    &SupportedAppProtocolConf {
        name: "urn:iso:15118:20:2022:MsgDef",
        tag_id: ProtocolTagId::Iso20,
        major: 2,
        minor: 0,
    },
    &SupportedAppProtocolConf {
        tag_id: ProtocolTagId::Iso2,
        name: "urn:iso:15118:2:2013:MsgDef",
        major: 2,
        minor: 0,
    },
    &SupportedAppProtocolConf {
        tag_id: ProtocolTagId::Din,
        name: "urn:din:70121:2012:MsgDef",
        major: 2,
        minor: 0,
    },
];