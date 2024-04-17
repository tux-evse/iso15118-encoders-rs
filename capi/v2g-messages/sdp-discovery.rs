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
use std::mem;

pub const SDP_V2G_HEADER_LEN: usize = cglue::SDP_V2G_HEADER_LEN as usize;
pub const V2GTP20_SAP_PAYLOAD_ID: u16 = cglue::V2GTP20_SAP_PAYLOAD_ID as u16;

#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum SdpSecurityModel {
    TLS = cglue::SDP_V2G_SECURITY_TLS,
    NONE = cglue::SDP_V2G_SECURITY_NONE,
}

#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(u8)]
pub enum SdpTransportProtocol {
    TCP = cglue::SDP_V2G_TRANSPORT_TCP,
    UDP = cglue::SDP_V2G_TRANSPORT_UDP,
}

pub enum SdpMsgType {
    Request,
    Response,
}

// payload buffer data type
pub type SdpResponseBuffer =
    [u8; (cglue::SDP_V2G_RESPONSE_LEN + cglue::SDP_V2G_HEADER_LEN) as usize];
pub type SdpRequestBuffer = [u8; (cglue::SDP_V2G_REQUEST_LEN + cglue::SDP_V2G_HEADER_LEN) as usize];

#[derive(Clone)]
pub struct SdpRequest {
    payload: cglue::sdp_request,
}
impl SdpRequest {
    pub fn new(transport: SdpTransportProtocol, security: SdpSecurityModel) -> Self {
        Self {
            payload: cglue::sdp_request {
                header: cglue::sdp_msg_header {
                    version_std: cglue::SDP_V2G_VERSION,
                    version_not: cglue::SDP_V2G_VERSION_NOT,
                    msg_len: cglue::SDP_V2G_REQUEST_LEN,
                    msg_type: cglue::SDP_V2G_REQUEST_TYPE,
                },
                security: security as u8,
                transport: transport as u8,
            },
        }
    }

    pub fn decode(buffer: &SdpRequestBuffer) -> Result<Self, AfbError> {
        let mut request = mem::MaybeUninit::<cglue::sdp_request>::uninit();
        let status = unsafe {
            cglue::sdp_v2g_decode_req(
                buffer.as_ptr() as *mut u8,
                buffer.len(),
                request.as_mut_ptr(),
            )
        };
        if status != 0 {
            return afb_error!("sdp-response-encode", "fail to decode response");
        }
        let response = SdpRequest {
            payload: unsafe { request.assume_init() },
        };
        Ok(response)
    }

    pub fn encode(&self) -> Result<SdpRequestBuffer, AfbError> {
        let mut buffer = mem::MaybeUninit::<SdpRequestBuffer>::uninit();
        let status = unsafe {
            cglue::sdp_v2g_encode_req(
                &self.payload,
                buffer.as_mut_ptr() as *mut u8,
                mem::size_of::<SdpResponseBuffer>(),
            )
        };
        if status != 0 {
            return afb_error!("sdp-request-encode", "fail to encode request");
        }
        let buffer = unsafe { buffer.assume_init() };
        Ok(buffer)
    }

    pub fn check_header(&self) -> Result<&Self, AfbError> {
        let header = self.payload.header;
        if header.version_std != cglue::SDP_V2G_VERSION
            || header.version_not != cglue::SDP_V2G_VERSION_NOT
        {
            return afb_error!(
                "sdp-request-header",
                "invalid SDP/version expected:[{:#02x},{:#02x}] received:[{:#02x},{:#02x}]",
                header.version_std,
                header.version_not,
                cglue::SDP_V2G_VERSION,
                cglue::SDP_V2G_VERSION_NOT
            );
        }

        if header.msg_type != cglue::SDP_V2G_REQUEST_TYPE {
            return afb_error!(
                "sdp-request-header",
                "invalid SDP/type expected:{:#04x} received:{:#04x}",
                cglue::SDP_V2G_REQUEST_TYPE,
                header.msg_type
            );
        }

        //let rqt_len = unsafe { cglue::ntohl(sdp_len) };
        if header.msg_len != cglue::SDP_V2G_REQUEST_LEN {
            return afb_error!(
                "sdp-request-header",
                "invalid v2g/sdp lenght expected:{} received:{}",
                cglue::SDP_V2G_REQUEST_LEN,
                header.msg_type
            );
        }
        Ok(self)
    }

    pub fn get_transport(&self) -> SdpTransportProtocol {
        let transport = unsafe { mem::transmute(self.payload.transport) };
        transport
    }

    pub fn get_security(&self) -> SdpSecurityModel {
        let transport = unsafe { mem::transmute(self.payload.security) };
        transport
    }
}

pub struct SdpResponse {
    payload: cglue::sdp_response,
}

impl SdpResponse {
    pub fn new(
        addr6: cglue::sdp_in6_addr,
        port: u16,
        transport: SdpTransportProtocol,
        security: SdpSecurityModel,
    ) -> Self {
        Self {
            payload: cglue::sdp_response {
                header: cglue::sdp_msg_header {
                    version_std: cglue::SDP_V2G_VERSION,
                    version_not: cglue::SDP_V2G_VERSION_NOT,
                    msg_len: cglue::SDP_V2G_RESPONSE_LEN,
                    msg_type: cglue::SDP_V2G_RESPONSE_TYPE,
                },
                addr: addr6,
                port,
                security: security as u8,
                transport: transport as u8,
            },
        }
    }

    pub fn encode(&self) -> Result<SdpResponseBuffer, AfbError> {
        let mut buffer = mem::MaybeUninit::<SdpResponseBuffer>::uninit();
        let status = unsafe {
            cglue::sdp_v2g_encode_res(
                &self.payload,
                buffer.as_mut_ptr() as *mut u8,
                mem::size_of::<SdpResponseBuffer>(),
            )
        };
        if status != 0 {
            return afb_error!("sdp-response-encode", "fail to encode response");
        }
        let buffer = unsafe { buffer.assume_init() };
        Ok(buffer)
    }

    pub fn decode(buffer: &SdpResponseBuffer) -> Result<Self, AfbError> {
        let mut response = mem::MaybeUninit::<cglue::sdp_response>::uninit();
        let status = unsafe {
            cglue::sdp_v2g_decode_res(
                buffer.as_ptr() as *mut u8,
                buffer.len(),
                response.as_mut_ptr(),
            )
        };
        if status != 0 {
            return afb_error!("sdp-response-encode", "fail to decode response");
        }
        let response = SdpResponse {
            payload: unsafe { response.assume_init() },
        };
        Ok(response)
    }

    pub fn check_header(&self) -> Result<&Self, AfbError> {
        let header = self.payload.header;
        if header.version_std != cglue::SDP_V2G_VERSION
            || header.version_not != cglue::SDP_V2G_VERSION_NOT
        {
            return afb_error!(
                "sdp-response-header",
                "invalid v2g/sdp version expected:[{:#02x},{:#02x}] received:[{:#02x},{:#02x}]",
                header.version_std,
                header.version_not,
                cglue::SDP_V2G_VERSION,
                cglue::SDP_V2G_VERSION_NOT
            );
        }

        if header.msg_type != cglue::SDP_V2G_RESPONSE_TYPE {
            return afb_error!(
                "sdp-response-header",
                "invalid v2g/sdp type expected:{:#04x} received:{:#04x}",
                cglue::SDP_V2G_RESPONSE_TYPE,
                header.msg_type
            );
        }

        //let rqt_len = unsafe { cglue::ntohl(sdp_len) };
        if header.msg_len != cglue::SDP_V2G_RESPONSE_LEN {
            return afb_error!(
                "sdp-response-header",
                "invalid v2g/sdp lenght expected:{} received:{}",
                cglue::SDP_V2G_RESPONSE_LEN,
                header.msg_type
            );
        }
        Ok(self)
    }

    pub fn get_transport(&self) -> SdpTransportProtocol {
        let transport = unsafe { mem::transmute(self.payload.transport) };
        transport
    }

    pub fn get_security(&self) -> SdpSecurityModel {
        let transport = unsafe { mem::transmute(self.payload.security) };
        transport
    }

    pub fn get_port(&self) -> u16 {
        self.payload.port
    }

    pub fn get_addr6(&self) -> cglue::sdp_in6_addr {
        self.payload.addr
    }
}