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
    payload: cglue::din_SessionStopType,
}
impl SessionStopRequest {
    pub fn new(unused: i32) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_SessionStopType>() };
        payload._unused= unused;
        SessionStopRequest { payload }
    }

    pub fn decode(payload: cglue::din_SessionStopType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.SessionStopReq = self.payload;
            exi_body.set_SessionStopReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct SessionStopResponse {
    payload: cglue::din_SessionStopResType,
}

impl SessionStopResponse {
    pub fn new(code: ResponseCode) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_SessionStopResType>() };

        payload.ResponseCode = code as u32;
        Self { payload }
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn decode(payload: cglue::din_SessionStopResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.SessionStopRes = self.payload;
            exi_body.set_SessionStopRes_isUsed(1);
            exi_body
        };
        body
    }

}
