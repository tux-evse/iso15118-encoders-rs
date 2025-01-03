/*
 * Copyright (C) 2015-2022 IoT.bzh Company
 * Author: Hugo Mercier <hugo.mercier@iot.bzh>
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
use core::str;
use std::mem;

use crate::string_field_getter_and_setter;

#[derive(Clone)]
pub struct SessionSetupRequest {
    payload: cglue::iso20_SessionSetupReqType,
}
impl SessionSetupRequest {
    pub fn empty() -> Self {
        let payload = unsafe { mem::zeroed::<cglue::iso20_SessionSetupReqType>() };
        Self { payload }
    }

    pub fn new(evcc_id: &str) -> Result<Self, AfbError> {
        if evcc_id.len() == 0 {
            return afb_error!("session-setup-new", "EVCCID: should not be null");
        }
        let mut this = SessionSetupRequest::empty();
        this.set_evcc_id(evcc_id)?;
        Ok(this)
    }

    string_field_getter_and_setter!(
        payload.EVCCID.characters,
        payload.EVCCID.charactersLen,
        evcc_id,
        cglue::iso20_EVCCID_CHARACTER_SIZE
    );

}

impl EncodeToDocument for SessionSetupRequest {
    fn encode(&self) -> Box<cglue::iso20_exiDocument> {
        unsafe {
            let mut exi_body = Box::<cglue::iso20_exiDocument>::new_uninit().assume_init();
            exi_body.__bindgen_anon_1.SessionSetupReq = self.payload;
            exi_body.set_SessionSetupReq_isUsed(1);
            exi_body
        }
    }
}

// TODO: write a Derive macro
impl TryFrom<ExiMessageDoc> for SessionSetupRequest {
    type Error = AfbError;

    fn try_from(doc: ExiMessageDoc) -> Result<Self, Self::Error> {
        if doc.get_payload().SessionSetupReq_isUsed() == 0 {
            return afb_error!("ExiMessageDoc-from-SessionSetupReq", "Wrong type");
        }
        unsafe {
            Ok(SessionSetupRequest {
                payload: doc.get_payload().__bindgen_anon_1.SessionSetupReq,
            })
        }
    }
}

pub struct SessionSetupResponse {
    payload: cglue::iso20_SessionSetupResType,
}

impl SessionSetupResponse {
    pub fn new(evse_id: &str, code: ResponseCode) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::iso20_SessionSetupResType>() };

        payload.ResponseCode = code as u32;

        let mut this = Self { payload };
        this.set_evse_id(evse_id)?;

        Ok(this)
    }

    string_field_getter_and_setter!(
        payload.EVSEID.characters,
        payload.EVSEID.charactersLen,
        evse_id,
        cglue::iso20_EVSEID_CHARACTER_SIZE
    );

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }
}

impl TryFrom<ExiMessageDoc> for SessionSetupResponse {
    type Error = AfbError;

    fn try_from(doc: ExiMessageDoc) -> Result<Self, Self::Error> {
        if doc.get_payload().SessionSetupRes_isUsed() == 0 {
            return afb_error!("ExiMessageDoc-from-SessionSetupRes", "Wrong type");
        }
        unsafe {
            Ok(SessionSetupResponse {
                payload: doc.get_payload().__bindgen_anon_1.SessionSetupRes,
            })
        }
    }
}

impl EncodeToDocument for SessionSetupResponse {
    fn encode(&self) -> Box<cglue::iso20_exiDocument> {
        unsafe {
            let mut exi_body = Box::<cglue::iso20_exiDocument>::new_uninit().assume_init();
            exi_body.__bindgen_anon_1.SessionSetupRes = self.payload;
            exi_body.set_SessionSetupRes_isUsed(1);
            exi_body
        }
    }
}