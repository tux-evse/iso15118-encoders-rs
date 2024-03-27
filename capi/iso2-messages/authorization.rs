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

pub struct AuthorizationRequest {
    payload: cglue::iso2_AuthorizationReqType,
}

impl AuthorizationRequest {
    pub fn new() -> Self {
        let payload = unsafe { mem::zeroed::<cglue::iso2_AuthorizationReqType>() };
        Self { payload }
    }

    pub fn set_id(&mut self, value: &str) -> Result<&mut Self, AfbError> {
        let len = str_to_array(
            value,
            &mut self.payload.Id.characters,
            cglue::iso2_Id_CHARACTER_SIZE,
        )?;

        if len > 0 {
            self.payload.Id.charactersLen = len;
            self.payload.set_Id_isUsed(1);
        }
        Ok(self)
    }


    pub fn get_id(&self) -> Option<&str> {
        let value = if self.payload.Id_isUsed() == 0 {
            None
        } else {
            array_to_str(
                &self.payload.Id.characters,
                self.payload.Id.charactersLen,
            ).ok()
        };
        value
    }

    pub fn set_challenge(&mut self, value: &[u8]) -> Result<&mut Self, AfbError> {
        let len = bytes_to_array(
            value,
            &mut self.payload.GenChallenge.bytes,
            cglue::iso2_genChallengeType_BYTES_SIZE,
        )?;

        if len > 0 {
            self.payload.GenChallenge.bytesLen = len;
            self.payload.set_GenChallenge_isUsed(1);
        }
        Ok(self)
    }

    pub fn get_challenge(&self) -> Option<&[u8]> {
        let value = if self.payload.GenChallenge_isUsed() == 0 {
            None
        } else {
            Some(array_to_bytes(
                &self.payload.GenChallenge.bytes,
                self.payload.GenChallenge.bytesLen,
            ))
        };
        value
    }

    pub fn decode(payload: cglue::iso2_AuthorizationReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.AuthorizationReq = self.payload;
            exi_body.set_AuthorizationReq_isUsed(1);
            exi_body
        };
        body
    }

}

pub struct AuthorizationResponse {
    payload: cglue::iso2_AuthorizationResType,
}

impl AuthorizationResponse {
    pub fn new(code: ResponseCode, processing: EvseProcessing) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_AuthorizationResType>() };
        payload.ResponseCode = code as u32;
        payload.EVSEProcessing= processing as u32;
        Self { payload }
    }

    pub fn decode(payload: cglue::iso2_AuthorizationResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.AuthorizationRes = self.payload;
            exi_body.set_AuthorizationRes_isUsed(1);
            exi_body
        };
        body
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn get_processing(&self) -> EvseProcessing {
        EvseProcessing::from_u32(self.payload.EVSEProcessing)
    }
}
