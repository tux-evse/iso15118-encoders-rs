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
 use std::time::{SystemTime,UNIX_EPOCH};

#[derive(Clone)]
pub struct SessionSetupRequest {
    payload: cglue::din_SessionSetupReqType,
}
impl SessionSetupRequest {
    pub fn new(evcc_id: &[u8]) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_SessionSetupReqType>() };
        payload.EVCCID.bytesLen = bytes_to_array(
            evcc_id,
            &mut payload.EVCCID.bytes,
            cglue::din_evccIDType_BYTES_SIZE,
        )?;
        if payload.EVCCID.bytesLen  == 0 {
            return afb_error!("session-setup-new", "session-id: should not be null");
        }
        Ok(SessionSetupRequest { payload })
    }

    pub fn get_id(&self) -> &[u8] {
        &self.payload.EVCCID.bytes[0..self.payload.EVCCID.bytesLen as usize]
    }

    pub fn empty() -> Self {
        let payload = unsafe { mem::zeroed::<cglue::din_SessionSetupReqType>() };
        Self { payload }
    }

    pub fn equal(&self, value: &[u8]) -> bool {
        byte_equal_array(
            value,
            &self.payload.EVCCID.bytes,
            self.payload.EVCCID.bytesLen,
        )
    }

    pub fn decode(payload: cglue::din_SessionSetupReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.SessionSetupReq = self.payload;
            exi_body.set_SessionSetupReq_isUsed(1);
            exi_body
        };
        body
    }

}

pub struct SessionSetupResponse {
    payload: cglue::din_SessionSetupResType,
}

impl SessionSetupResponse {
    pub fn new(id: &[u8], code: ResponseCode) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_SessionSetupResType>() };

        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => {
                let epoch = time.as_secs();
                payload.DateTimeNow = epoch as i64;
                payload.set_DateTimeNow_isUsed(1);
            }
            Err(_) => {
                return afb_error!("din-Session-rsp", "Invalid system time (should be fixed)")
            }
        };

        payload.ResponseCode = code as u32;
        payload.EVSEID.bytesLen = bytes_to_array(
            id,
            &mut payload.EVSEID.bytes,
            cglue::din_evseIDType_BYTES_SIZE,
        )?;

        Ok(Self { payload })
    }

    pub fn get_id(&self) ->&[u8] {
        array_to_bytes(
            &self.payload.EVSEID.bytes,
            self.payload.EVSEID.bytesLen,
        )
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn set_timestamp(&mut self, timestamp: i64) -> &mut Self {
        self.payload.DateTimeNow= timestamp;
        self.payload.set_DateTimeNow_isUsed(1);
        self
    }

    pub fn get_time_stamp(&self) -> i64 {
        self.payload.DateTimeNow
    }

    pub fn decode(payload: cglue::din_SessionSetupResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.SessionSetupRes = self.payload;
            exi_body.set_SessionSetupRes_isUsed(1);
            exi_body
        };
        body
    }

}
