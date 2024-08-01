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
pub struct CurrentDemandRequest {
    payload: cglue::din_CurrentDemandReqType,
}
impl CurrentDemandRequest {
    pub fn new(
        dc_status: &DcEvStatusType,
        current_target: &PhysicalValue,
        voltage_target: &PhysicalValue,
        charging_complete: bool,
    ) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::din_CurrentDemandReqType>() };
        payload.DC_EVStatus = dc_status.encode();
        payload.EVTargetCurrent = current_target.encode();
        payload.EVTargetVoltage = voltage_target.encode();
        payload.ChargingComplete = if charging_complete { 1 } else { 0 };
        Self { payload }
    }

    pub fn get_status(&self) -> DcEvStatusType {
        DcEvStatusType::decode(self.payload.DC_EVStatus)
    }

    pub fn get_current_target(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVTargetCurrent)
    }

    pub fn get_voltage_target(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVTargetVoltage)
    }

    pub fn get_charging_complete(&self) -> bool {
        if self.payload.ChargingComplete == 0 {
            false
        } else {
            true
        }
    }

    pub fn set_voltage_limit(
        &mut self,
        voltage_limit: &PhysicalValue,
    ) -> Result<&mut Self, AfbError> {
        if let Some(unit) = voltage_limit.get_unit() {
            if unit != PhysicalUnit::Volt {
                return afb_error!(
                    "current-demand-req",
                    "expect: PhysicalUnit::Volt get:{}",
                    unit
                );
            }
        }
        self.payload.EVMaximumVoltageLimit = voltage_limit.encode();
        self.payload.set_EVMaximumVoltageLimit_isUsed(1);
        Ok(self)
    }

    pub fn get_voltage_limit(&self) -> Option<PhysicalValue> {
        if self.payload.EVMaximumVoltageLimit_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVMaximumVoltageLimit))
        }
    }

    pub fn set_current_limit(
        &mut self,
        current_limit: &PhysicalValue,
    ) -> Result<&mut Self, AfbError> {
        if let Some(unit) = current_limit.get_unit() {
            if unit != PhysicalUnit::Ampere {
                return afb_error!(
                    "current-demand-req",
                    "expect: PhysicalUnit::Ampere get:{}",
                    unit
                );
            }
        }

        self.payload.EVMaximumCurrentLimit = current_limit.encode();
        self.payload.set_EVMaximumCurrentLimit_isUsed(1);
        Ok(self)
    }

    pub fn get_current_limit(&self) -> Option<PhysicalValue> {
        if self.payload.EVMaximumCurrentLimit_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVMaximumCurrentLimit))
        }
    }

    pub fn set_power_limit(&mut self, power_limit: &PhysicalValue) -> Result<&mut Self, AfbError> {
        if let Some(unit) = power_limit.get_unit() {
            if unit != PhysicalUnit::Watt {
                return afb_error!(
                    "current-demand-req",
                    "expect: PhysicalUnit::Watt get:{}",
                    unit
                );
            }
        }
        self.payload.EVMaximumPowerLimit = power_limit.encode();
        self.payload.set_EVMaximumPowerLimit_isUsed(1);
        Ok(self)
    }

    pub fn get_power_limit(&self) -> Option<PhysicalValue> {
        if self.payload.EVMaximumPowerLimit_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVMaximumPowerLimit))
        }
    }

    pub fn set_time_to_full_sock(
        &mut self,
        remaining_time: &PhysicalValue,
    ) -> Result<&mut Self, AfbError> {
        if let Some(unit) = remaining_time.get_unit() {
            if unit != PhysicalUnit::Hour
                && unit != PhysicalUnit::Minute
                && unit != PhysicalUnit::Second
            {
                return afb_error!(
                    "current-demand-req",
                    "expect: PhysicalUnit::(Hour|Minute|Second) got:{}",
                    unit
                );
            }
        }
        self.payload.RemainingTimeToFullSoC = remaining_time.encode();
        self.payload.set_RemainingTimeToFullSoC_isUsed(1);
        Ok(self)
    }

    pub fn get_time_to_full_sock(&self) -> Option<PhysicalValue> {
        if self.payload.RemainingTimeToFullSoC_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.RemainingTimeToFullSoC))
        }
    }

    pub fn set_time_to_bulk_sock(
        &mut self,
        remaining_time: &PhysicalValue,
    ) -> Result<&mut Self, AfbError> {
        if let Some(unit) = remaining_time.get_unit() {
            if unit != PhysicalUnit::Hour
                && unit != PhysicalUnit::Minute
                && unit != PhysicalUnit::Second
            {
                return afb_error!(
                    "current-demand-req",
                    "expect: PhysicalUnit::Percent get:{}",
                    unit
                );
            }
        }

        self.payload.RemainingTimeToBulkSoC = remaining_time.encode();
        self.payload.set_RemainingTimeToBulkSoC_isUsed(1);
        Ok(self)
    }

    pub fn get_time_to_bulk_sock(&self) -> Option<PhysicalValue> {
        if self.payload.RemainingTimeToBulkSoC_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.RemainingTimeToBulkSoC))
        }
    }

    pub fn set_bulk_complete(&mut self, complete: bool) -> &mut Self {
        self.payload.BulkChargingComplete = if complete { 1 } else { 0 };
        self.payload.set_BulkChargingComplete_isUsed(1);
        self
    }

    pub fn get_bulk_complete(&self) -> Option<bool> {
        if self.payload.BulkChargingComplete_isUsed() == 0 {
            None
        } else {
            let complete = if self.payload.BulkChargingComplete == 0 {
                false
            } else {
                true
            };
            Some(complete)
        }
    }

    pub fn decode(payload: cglue::din_CurrentDemandReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.CurrentDemandReq = self.payload;
            exi_body.set_CurrentDemandReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct CurrentDemandResponse {
    payload: cglue::din_CurrentDemandResType,
}

impl CurrentDemandResponse {
    pub fn new(
        rcode: ResponseCode,
        dc_status: &DcEvseStatusType,
        voltage_present: &PhysicalValue,
        current_present: &PhysicalValue,
        voltage_limit_reach: bool,
        current_limit_reach: bool,
        power_limit_reach: bool,
    ) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::din_CurrentDemandResType>() };

        payload.ResponseCode = rcode as u32;

        if let Some(unit) = current_present.get_unit() {
            if unit != PhysicalUnit::Ampere {
                return afb_error!(
                    "current-demand-res",
                    "expect: PhysicalUnit::Ampere get:{}",
                    unit
                );
            }
        }

        if let Some(unit) = voltage_present.get_unit() {
            if unit != PhysicalUnit::Volt {
                return afb_error!(
                    "current-demand-res",
                    "expect: PhysicalUnit::Volt get:{}",
                    unit
                );
            }
        }

        payload.DC_EVSEStatus = dc_status.encode();
        payload.EVSEPresentVoltage = voltage_present.encode();
        payload.EVSEPresentCurrent = current_present.encode();
        payload.EVSECurrentLimitAchieved = if current_limit_reach { 1 } else { 0 };
        payload.EVSEVoltageLimitAchieved = if voltage_limit_reach { 1 } else { 0 };
        payload.EVSEPowerLimitAchieved = if power_limit_reach { 1 } else { 0 };

        Ok(Self { payload })
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn get_status(&self) -> DcEvseStatusType {
        DcEvseStatusType::decode(self.payload.DC_EVSEStatus)
    }

    pub fn get_voltage_present(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEPresentVoltage)
    }

    pub fn get_current_present(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEPresentCurrent)
    }

    pub fn get_current_limit_reach(&self) -> bool {
        if self.payload.EVSECurrentLimitAchieved == 0 {
            false
        } else {
            true
        }
    }

    pub fn get_voltage_limit_reach(&self) -> bool {
        if self.payload.EVSEVoltageLimitAchieved == 0 {
            false
        } else {
            true
        }
    }

    pub fn get_power_limit_reach(&self) -> bool {
        if self.payload.EVSEPowerLimitAchieved == 0 {
            false
        } else {
            true
        }
    }

    pub fn set_voltage_limit(&mut self, voltage: &PhysicalValue) -> Result<&mut Self, AfbError> {
        if let Some(unit) = voltage.get_unit() {
            if unit != PhysicalUnit::Volt {
                return afb_error!(
                    "current-demand-res",
                    "expect: PhysicalUnit::Volt get:{}",
                    unit
                );
            }
        }

        self.payload.EVSEMaximumVoltageLimit = voltage.encode();
        self.payload.set_EVSEMaximumVoltageLimit_isUsed(1);
        Ok(self)
    }

    pub fn get_voltage_limit(&self) -> Option<PhysicalValue> {
        if self.payload.EVSEMaximumVoltageLimit_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVSEMaximumVoltageLimit))
        }
    }

    pub fn set_current_limit(&mut self, current: &PhysicalValue) -> Result<&mut Self, AfbError> {
        if let Some(unit) = current.get_unit() {
            if unit != PhysicalUnit::Ampere {
                return afb_error!(
                    "current-demand-res",
                    "expect: PhysicalUnit::Ampere get:{}",
                    unit
                );
            }
        }
        self.payload.EVSEMaximumCurrentLimit = current.encode();
        self.payload.set_EVSEMaximumCurrentLimit_isUsed(1);
        Ok(self)
    }

    pub fn get_current_limit(&self) -> Option<PhysicalValue> {
        if self.payload.EVSEMaximumCurrentLimit_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVSEMaximumCurrentLimit))
        }
    }

    pub fn set_power_limit(&mut self, power: &PhysicalValue) -> Result<&mut Self, AfbError> {
        if let Some(unit) = power.get_unit() {
            if unit != PhysicalUnit::Watt {
                return afb_error!(
                    "current-demand-res",
                    "expect: PhysicalUnit::Volt get:{}",
                    unit
                );
            }
        }
        self.payload.EVSEMaximumPowerLimit = power.encode();
        self.payload.set_EVSEMaximumPowerLimit_isUsed(1);
        Ok(self)
    }

    pub fn get_power_limit(&self) -> Option<PhysicalValue> {
        if self.payload.EVSEMaximumPowerLimit_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVSEMaximumPowerLimit))
        }
    }

    pub fn decode(payload: cglue::din_CurrentDemandResType) -> Self {
        Self { payload }
    }
    pub fn encode(&self) -> DinBodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<DinBodyType>();
            exi_body.__bindgen_anon_1.CurrentDemandRes = self.payload;
            exi_body.set_CurrentDemandRes_isUsed(1);
            exi_body
        };
        body
    }
}
