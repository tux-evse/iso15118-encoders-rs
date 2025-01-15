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

use v2g_macros::CGlue;

#[derive(CGlue)]
#[cglue_union_member(SessionSetupReq)]
pub struct SessionSetupRequest {
    payload: cglue::iso20_SessionSetupReqType,
}
impl SessionSetupRequest {
    pub fn from_cglue(payload: cglue::iso20_SessionSetupReqType) -> Self {
        Self { payload }
    }

    pub fn new(evcc_id: &str) -> Result<Self, AfbError> {
        if evcc_id.len() == 0 {
            return afb_error!("session-setup-new", "EVCCID: should not be null");
        }
        let mut this = SessionSetupRequest::default();
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

#[derive(CGlue)]
#[cglue_union_member(SessionSetupRes)]
pub struct SessionSetupResponse {
    payload: cglue::iso20_SessionSetupResType,
}

impl SessionSetupResponse {
    pub fn new(evse_id: &str, code: ResponseCode) -> Result<Self, AfbError> {
        let mut this = SessionSetupResponse::default();

        this.payload.ResponseCode = code as u32;
        this.set_evse_id(evse_id)?;

        Ok(this)
    }

    pub fn from_cglue(payload: cglue::iso20_SessionSetupResType) -> Self {
        Self { payload }
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
