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
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PaymentDetailsRequest {
    payload: cglue::iso2_PaymentDetailsReqType,
}

impl PaymentDetailsRequest {
    pub fn new(emaid: &str, contract_chain: &CertificateChainType) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_PaymentDetailsReqType>() };
        payload.eMAID.charactersLen = str_to_array(
            emaid,
            &mut payload.eMAID.characters,
            cglue::iso2_eMAID_CHARACTER_SIZE,
        )?;
        payload.ContractSignatureCertChain = contract_chain.encode();
        Ok(Self { payload })
    }

    pub fn get_contract_chain(&self) -> CertificateChainType {
        CertificateChainType::decode(self.payload.ContractSignatureCertChain)
    }

    pub fn get_emaid(&self) -> Result<&str, AfbError> {
        array_to_str(
            &self.payload.eMAID.characters,
            self.payload.eMAID.charactersLen,
        )
    }

    pub fn decode(payload: cglue::iso2_PaymentDetailsReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.PaymentDetailsReq = self.payload;
            exi_body.set_PaymentDetailsReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct PaymentDetailsResponse {
    payload: cglue::iso2_PaymentDetailsResType,
}

impl PaymentDetailsResponse {
    pub fn new(code: ResponseCode, challenge: &[u8]) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_PaymentDetailsResType>() };
        payload.ResponseCode = code as u32;
        payload.GenChallenge.bytesLen = bytes_to_array(
            challenge,
            &mut payload.GenChallenge.bytes,
            cglue::iso2_genChallengeType_BYTES_SIZE,
        )?;

        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => {
                let epoch = time.as_secs();
                payload.EVSETimeStamp = epoch as i64;
            }
            Err(_) => {
                return afb_error!("iso2-Session-rsp", "Invalid system time (should be fixed)")
            }
        };
        Ok(Self { payload })
    }

    pub fn set_timestamp(&mut self, epoch: i64) -> &mut Self {
        self.payload.EVSETimeStamp = epoch;
        self
    }

    pub fn get_challenge(&self) -> &[u8] {
        array_to_bytes(
            &self.payload.GenChallenge.bytes,
            self.payload.GenChallenge.bytesLen,
        )
    }

    pub fn get_time_stamp(&self) -> i64 {
        self.payload.EVSETimeStamp
    }

    pub fn decode(payload: cglue::iso2_PaymentDetailsResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.PaymentDetailsRes = self.payload;
            exi_body.set_PaymentDetailsRes_isUsed(1);
            exi_body
        };
        body
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

}
