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

pub struct CableCheckRequest {
    payload: cglue::iso2_CableCheckReqType,
}

impl CableCheckRequest {
    pub fn new(status: &DcEvStatusType) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_CableCheckReqType>() };
        payload.DC_EVStatus = status.encode();
        Self { payload }
    }

    pub fn decode(payload: cglue::iso2_CableCheckReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.CableCheckReq = self.payload;
            exi_body.set_CableCheckReq_isUsed(1);
            exi_body
        };
        body
    }

    pub fn get_status(&self) -> DcEvStatusType {
        DcEvStatusType::decode(self.payload.DC_EVStatus)
    }
}

pub struct CableCheckResponse {
    payload: cglue::iso2_CableCheckResType,
}

impl CableCheckResponse {
    pub fn new(code: ResponseCode, status: &DcEvseStatusType, processing: EvseProcessing) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_CableCheckResType>() };
        payload.ResponseCode = code as u32;
        payload.EVSEProcessing = processing as u32;
        payload.DC_EVSEStatus = status.encode();
        Self { payload }
    }

    pub fn get_code(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn get_status(&self) -> DcEvseStatusType {
        DcEvseStatusType::decode(self.payload.DC_EVSEStatus)
    }

    pub fn get_processing(&self) -> EvseProcessing {
        EvseProcessing::from_u32(self.payload.EVSEProcessing)
    }

    pub fn decode(payload: cglue::iso2_CableCheckResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.CableCheckRes = self.payload;
            exi_body.set_CableCheckRes_isUsed(1);
            exi_body
        };
        body
    }
}
