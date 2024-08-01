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

pub struct ContractAuthenticationRequest {
    payload: cglue::din_ContractAuthenticationReqType,
}

impl ContractAuthenticationRequest {
    pub fn new() -> Self {
        let payload = unsafe { mem::zeroed::<cglue::din_ContractAuthenticationReqType>() };

        Self { payload }
    }

    pub fn set_id(&mut self, id: &str) -> Result<&Self, AfbError> {
        self.payload.Id.charactersLen = str_to_array(
            id,
            &mut self.payload.Id.characters,
            cglue::din_ContractID_CHARACTER_SIZE,
        )?;
        self.payload.set_Id_isUsed(1);
        Ok(self)
    }

    pub fn get_id(&self) -> Result<Option<&str>, AfbError> {
        if self.payload.Id_isUsed() == 0 {
            Ok(None)
        } else {
            let id = array_to_str(&self.payload.Id.characters, self.payload.Id.charactersLen)?;
            Ok(Some(id))
        }
    }

    pub fn set_challenge(&mut self, challenge: &str) -> Result<&Self, AfbError> {
        self.payload.GenChallenge.charactersLen = str_to_array(
            challenge,
            &mut self.payload.GenChallenge.characters,
            cglue::din_GenChallenge_CHARACTER_SIZE,
        )?;
        self.payload.set_GenChallenge_isUsed(1);
        Ok(self)
    }

    pub fn get_challenge(&self) -> Result<Option<&str>, AfbError> {
        if self.payload.GenChallenge_isUsed() == 0 {
            Ok(None)
        } else {
            let challenge = array_to_str(
                &self.payload.GenChallenge.characters,
                self.payload.GenChallenge.charactersLen,
            )?;
            Ok(Some(challenge))
        }
    }

    pub fn decode(payload: cglue::din_ContractAuthenticationReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.ContractAuthenticationReq = self.payload;
            exi_body.set_ContractAuthenticationReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct ContractAuthenticationResponse {
    payload: cglue::din_ContractAuthenticationResType,
}

impl ContractAuthenticationResponse {
    pub fn new(rcode: ResponseCode, processing: EvseProcessing) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_ContractAuthenticationResType>() };
        payload.ResponseCode = rcode as u32;
        payload.EVSEProcessing = processing as u32;

        Self { payload }
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn get_processing(&self) -> EvseProcessing {
        EvseProcessing::from_u32(self.payload.EVSEProcessing)
    }

    pub fn decode(payload: cglue::din_ContractAuthenticationResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.ContractAuthenticationRes = self.payload;
            exi_body.set_ContractAuthenticationRes_isUsed(1);
            exi_body
        };
        body
    }
}
