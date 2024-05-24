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
    payload: cglue::din_PaymentDetailsReqType,
}

impl PaymentDetailsRequest {
    pub fn new(contract_id: &str, contract_chain: &CertificateChainType) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_PaymentDetailsReqType>() };
        payload.ContractID.charactersLen = str_to_array(
            contract_id,
            &mut payload.ContractID.characters,
            cglue::din_ContractID_CHARACTER_SIZE,
        )?;
        payload.ContractSignatureCertChain = contract_chain.encode();
        Ok(Self { payload })
    }

    pub fn get_contract_chain(&self) -> CertificateChainType {
        CertificateChainType::decode(self.payload.ContractSignatureCertChain)
    }

    pub fn get_contract_id(&self) -> Result<&str, AfbError> {
        array_to_str(
            &self.payload.ContractID.characters,
            self.payload.ContractID.charactersLen,
        )
    }

    pub fn decode(payload: cglue::din_PaymentDetailsReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.PaymentDetailsReq = self.payload;
            exi_body.set_PaymentDetailsReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct PaymentDetailsResponse {
    payload: cglue::din_PaymentDetailsResType,
}

impl PaymentDetailsResponse {
    pub fn new(rcode: ResponseCode, challenge: &str) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_PaymentDetailsResType>() };
        payload.ResponseCode = rcode as u32;
        payload.GenChallenge.charactersLen = str_to_array(
            challenge,
            &mut payload.GenChallenge.characters,
            cglue::din_GenChallenge_CHARACTER_SIZE,
        )?;

        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(time) => {
                let epoch = time.as_secs();
                payload.DateTimeNow = epoch as i64;
            }
            Err(_) => {
                return afb_error!("din-Session-rsp", "Invalid system time (should be fixed)")
            }
        };
        Ok(Self { payload })
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn get_challenge(&self) -> Result<&str, AfbError> {
        array_to_str(
            &self.payload.GenChallenge.characters,
            self.payload.GenChallenge.charactersLen,
        )
    }

    // to test with a fix exi binary timestamp should be overloaded with to a fix value
    pub fn set_timestamp(&mut self, epoch: i64) -> &mut Self {
        self.payload.DateTimeNow = epoch;
        self
    }

    pub fn get_time_stamp(&self) -> i64 {
        self.payload.DateTimeNow
    }

    pub fn decode(payload: cglue::din_PaymentDetailsResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.PaymentDetailsRes = self.payload;
            exi_body.set_PaymentDetailsRes_isUsed(1);
            exi_body
        };
        body
    }

}
