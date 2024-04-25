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
pub struct PreChargeRequest {
    payload: cglue::iso2_PreChargeReqType,
}
impl PreChargeRequest {
    pub fn new(
        ev_status: &DcEvStatusType,
        target_voltage: &PhysicalValue,
        target_current: &PhysicalValue,
    ) -> Result<Self, AfbError> {
        if target_voltage.get_unit() != PhysicalUnit::Volt {
            return afb_error!(
                "pre-charge-req",
                "expect: PhysicalUnit::Volt get:{}",
                target_voltage.get_unit()
            );
        }

        if target_current.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "pre-charge-req",
                "expect: PhysicalUnit::Ampere get:{}",
                target_current.get_unit()
            );
        }

        let mut payload = unsafe { mem::zeroed::<cglue::iso2_PreChargeReqType>() };
        payload.DC_EVStatus = ev_status.encode();
        payload.EVTargetVoltage = target_voltage.encode();
        payload.EVTargetCurrent = target_current.encode();
        Ok(Self { payload })
    }

    pub fn get_status(&self) -> DcEvStatusType {
        DcEvStatusType::decode(self.payload.DC_EVStatus)
    }

    pub fn get_target_voltage(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVTargetVoltage)
    }

    pub fn get_target_current(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVTargetCurrent)
    }

    pub fn decode(payload: cglue::iso2_PreChargeReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.PreChargeReq = self.payload;
            exi_body.set_PreChargeReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct PreChargeResponse {
    payload: cglue::iso2_PreChargeResType,
}

impl PreChargeResponse {
    pub fn new(
        rcode: ResponseCode,
        evse_status: &DcEvseStatusType,
        evse_voltage: &PhysicalValue,
    ) -> Result<Self, AfbError> {
        if evse_voltage.get_unit() != PhysicalUnit::Volt {
            return afb_error!(
                "pre-charge-res",
                "expect: PhysicalUnit::Volt get:{}",
                evse_voltage.get_unit()
            );
        }
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_PreChargeResType>() };
        payload.ResponseCode = rcode as u32;
        payload.DC_EVSEStatus = evse_status.encode();
        payload.EVSEPresentVoltage = evse_voltage.encode();

        Ok(Self { payload })
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn get_status(&self) -> DcEvseStatusType {
        DcEvseStatusType::decode(self.payload.DC_EVSEStatus)
    }

    pub fn get_voltage(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEPresentVoltage)
    }

    pub fn decode(payload: cglue::iso2_PreChargeResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.PreChargeRes = self.payload;
            exi_body.set_PreChargeRes_isUsed(1);
            exi_body
        };
        body
    }
}
