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
    payload: cglue::din_ChargingStatusReqType,
}
impl ChargingStatusRequest {
    pub fn new() -> Self {
        let payload = unsafe { mem::zeroed::<cglue::din_ChargingStatusReqType>() };
        Self { payload }
    }

    pub fn decode(payload: cglue::din_ChargingStatusReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.ChargingStatusReq = self.payload;
            exi_body.set_ChargingStatusReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct ChargingStatusResponse {
    payload: cglue::din_ChargingStatusResType,
}

impl ChargingStatusResponse {
    pub fn new(
        rcode: ResponseCode,
        evse_id: &[u8],
        tuple_id: i16,
        receipt_require: bool,
        status: &AcEvseStatusType,
    ) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_ChargingStatusResType>() };

        payload.ResponseCode = rcode as u32;
        payload.EVSEID.bytesLen = bytes_to_array(
            evse_id,
            &mut payload.EVSEID.bytes,
            cglue::din_evseIDType_BYTES_SIZE,
        )?;
        payload.SAScheduleTupleID = tuple_id;
        payload.AC_EVSEStatus = status.encode();

        if receipt_require {
            payload.ReceiptRequired = 1
        }

        Ok(Self { payload })
    }

    pub fn get_ac_evse_status(&self) -> AcEvseStatusType {
        AcEvseStatusType::decode(self.payload.AC_EVSEStatus)
    }

    pub fn get_evse_id(&self) -> &[u8] {
        array_to_bytes(&self.payload.EVSEID.bytes, self.payload.EVSEID.bytesLen)
    }

    pub fn get_tuple_id(&self) -> i16 {
        self.payload.SAScheduleTupleID
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn set_max_current(&mut self, max_current: &PhysicalValue) -> &mut Self {
        self.payload.EVSEMaxCurrent = max_current.encode();
        self.payload.set_EVSEMaxCurrent_isUsed(1);
        self
    }

    pub fn get_max_current(&self) -> Option<PhysicalValue> {
        if self.payload.EVSEMaxCurrent_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVSEMaxCurrent))
        }
    }
    pub fn get_receipt_require(&self) -> bool {
        if self.payload.ReceiptRequired == 0 {
            false
        } else {
            true
        }
    }

    pub fn set_meter_info(&mut self, meter: &MeterInfo) -> &mut Self {
        self.payload.MeterInfo = meter.encode();
        self.payload.set_MeterInfo_isUsed(1);
        self
    }

    pub fn get_meter_info(&self) -> Option<MeterInfo> {
        if self.payload.MeterInfo_isUsed() == 0 {
            None
        } else {
            Some(MeterInfo::decode(self.payload.MeterInfo))
        }
    }

    pub fn decode(payload: cglue::din_ChargingStatusResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.ChargingStatusRes = self.payload;
            exi_body.set_ChargingStatusRes_isUsed(1);
            exi_body
        };
        body
    }
}
