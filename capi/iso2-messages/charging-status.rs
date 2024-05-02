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
pub struct ChargingStatusRequest {
    payload: cglue::iso2_ChargingStatusReqType,
}
impl ChargingStatusRequest {
    pub fn new() -> Self {
        let payload = unsafe { mem::zeroed::<cglue::iso2_ChargingStatusReqType>() };
        Self { payload }
    }

    pub fn decode(payload: cglue::iso2_ChargingStatusReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.ChargingStatusReq = self.payload;
            exi_body.set_ChargingStatusReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct ChargingStatusResponse {
    payload: cglue::iso2_ChargingStatusResType,
}

impl ChargingStatusResponse {
    pub fn new(rcode: ResponseCode, evse_id: &str, tuple_id: u8, status: &AcEvseStatusType) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_ChargingStatusResType>() };

        payload.ResponseCode = rcode as u32;
        payload.EVSEID.charactersLen = str_to_array(
            evse_id,
            &mut payload.EVSEID.characters,
            cglue::iso2_EVSEID_CHARACTER_SIZE,
        )?;
        payload.SAScheduleTupleID = tuple_id;
        payload.AC_EVSEStatus = status.encode();

        Ok(Self { payload })
    }

    pub fn get_ac_evse_status(&self) -> AcEvseStatusType  {
        AcEvseStatusType::decode (self.payload.AC_EVSEStatus)
    }

    pub fn get_id(&self) -> Result<&str, AfbError> {
        array_to_str(
            &self.payload.EVSEID.characters,
            self.payload.EVSEID.charactersLen,
        )
    }

    pub fn get_tuple_id(&self) -> u8 {
            self.payload.SAScheduleTupleID
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn set_max_current(&mut self, max_current: PhysicalValue) -> &mut Self {
        self.payload.EVSEMaxCurrent = max_current.encode();
        self.payload.set_EVSEMaxCurrent_isUsed(1);
        self
    }

    pub fn get_max_current(&mut self) -> Option<PhysicalValue> {
        if self.payload.EVSEMaxCurrent_isUsed() == 0 {
            None
        } else {
           Some(PhysicalValue::decode(self.payload.EVSEMaxCurrent))
        }
    }

    pub fn set_receipt_require(&mut self, require: bool) -> &mut Self {
        self.payload.ReceiptRequired = if require { 1 } else { 0 };
        self.payload.set_ReceiptRequired_isUsed(1);
        self
    }

    pub fn get_receipt_require(&self) -> Option<bool> {
        if self.payload.ReceiptRequired_isUsed() == 0 {
            None
        } else {
            Some(if self.payload.ReceiptRequired == 0 {
                false
            } else {
                true
            })
        }
    }

    pub fn set_meter_info(&mut self, meter: MeterInfoType) -> &mut Self {
        self.payload.MeterInfo = meter.encode();
        self.payload.set_MeterInfo_isUsed(1);
        self
    }

    pub fn get_meter_info(&self) -> Option<MeterInfoType> {
        if self.payload.MeterInfo_isUsed() == 0 {
            None
        } else {
            Some(MeterInfoType::decode(self.payload.MeterInfo))
        }
    }

    pub fn decode(payload: cglue::iso2_ChargingStatusResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.ChargingStatusRes = self.payload;
            exi_body.set_ChargingStatusRes_isUsed(1);
            exi_body
        };
        body
    }
}
