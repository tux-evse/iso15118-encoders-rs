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

#[derive(Clone)]
pub struct SessionStopRequest {
    payload: cglue::iso2_SessionStopReqType,
}
impl SessionStopRequest {
    pub fn new(action: ChargingSessionType) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_SessionStopReqType>() };
        payload.ChargingSession= action as u32;
        SessionStopRequest { payload }
    }

    pub fn get_action (&self) -> ChargingSessionType {
        ChargingSessionType::from_u32(self.payload.ChargingSession)
    }

    pub fn decode(payload: cglue::iso2_SessionStopReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.SessionStopReq = self.payload;
            exi_body.set_SessionStopReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct SessionStopResponse {
    payload: cglue::iso2_SessionStopResType,
}

impl SessionStopResponse {
    pub fn new(code: ResponseCode) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_SessionStopResType>() };

        payload.ResponseCode = code as u32;
        Self { payload }
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn decode(payload: cglue::iso2_SessionStopResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.SessionStopRes = self.payload;
            exi_body.set_SessionStopRes_isUsed(1);
            exi_body
        };
        body
    }

}
