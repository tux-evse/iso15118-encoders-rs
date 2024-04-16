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
    payload: cglue::iso2_CurrentDemandReqType,
}
impl CurrentDemandRequest {
    pub fn new(
        dc_status: DcEvStatusType,
        current_target: &PhysicalValue,
        voltage_target: &PhysicalValue,
        charging_complete: bool,
    ) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_CurrentDemandReqType>() };
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

    pub fn set_voltage_limit(&mut self, voltage_limit: &PhysicalValue) -> &mut Self {
        self.payload.EVMaximumVoltageLimit = voltage_limit.encode();
        self.payload.set_EVMaximumVoltageLimit_isUsed(1);
        self
    }

    pub fn get_voltage_limit(&self) -> Option<PhysicalValue> {
        if self.payload.EVMaximumVoltageLimit_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(
                self.payload.EVMaximumVoltageLimit,
            ))
        }
    }

    pub fn set_current_limit(&mut self, current_limit: &PhysicalValue) -> &mut Self {
        self.payload.EVMaximumCurrentLimit = current_limit.encode();
        self.payload.set_EVMaximumCurrentLimit_isUsed(1);
        self
    }

    pub fn get_current_limit(&self) -> Option<PhysicalValue> {
        if self.payload.EVMaximumCurrentLimit_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(
                self.payload.EVMaximumCurrentLimit,
            ))
        }
    }

    pub fn set_power_limit(&mut self, power_limit: &PhysicalValue) -> &mut Self {
        self.payload.EVMaximumPowerLimit = power_limit.encode();
        self.payload.set_EVMaximumPowerLimit_isUsed(1);
        self
    }

    pub fn get_power_limit(&self) -> Option<PhysicalValue> {
        if self.payload.EVMaximumPowerLimit_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVMaximumPowerLimit))
        }
    }

    pub fn set_time_to_full_sock(&mut self, remaining_time: &PhysicalValue) -> &mut Self {
        self.payload.RemainingTimeToFullSoC = remaining_time.encode();
        self.payload.set_RemainingTimeToFullSoC_isUsed(1);
        self
    }

    pub fn get_time_to_full_sock(&self) -> Option<PhysicalValue> {
        if self.payload.RemainingTimeToFullSoC_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(
                self.payload.RemainingTimeToFullSoC,
            ))
        }
    }

    pub fn set_time_to_bulk_sock(&mut self, remaining_time: &PhysicalValue) -> &mut Self {
        self.payload.RemainingTimeToBulkSoC = remaining_time.encode();
        self.payload.set_RemainingTimeToBulkSoC_isUsed(1);
        self
    }

    pub fn get_time_to_bulk_sock(&self) -> Option<PhysicalValue> {
        if self.payload.RemainingTimeToBulkSoC_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(
                self.payload.RemainingTimeToBulkSoC,
            ))
        }
    }

    pub fn set_bulk_complete(&mut self, complete: bool) -> &mut Self {
        self.payload.BulkChargingComplete = if complete { 1 } else { 0 };
        self.payload.set_BulkChargingComplete_isUsed(1);
        self
    }

    pub fn get_bulk_complete(&mut self) -> Option<bool> {
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

    pub fn decode(payload: cglue::iso2_CurrentDemandReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.CurrentDemandReq = self.payload;
            exi_body.set_CurrentDemandReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct CurrentDemandResponse {
    payload: cglue::iso2_CurrentDemandResType,
}

impl CurrentDemandResponse {
    pub fn new(
        evse_id: &str,
        code: ResponseCode,
        dc_status: DcEvseStatusType,
        current: &PhysicalValue,
        voltage: PhysicalValue,
        current_limit: bool,
        voltage_limit: bool,
        power_limit: bool,
        schd_tuple_id: u8,
    ) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_CurrentDemandResType>() };

        payload.ResponseCode = code as u32;
        payload.EVSEID.charactersLen = str_to_array(
            evse_id,
            &mut payload.EVSEID.characters,
            cglue::iso2_EVSEID_CHARACTER_SIZE,
        )?;
        payload.DC_EVSEStatus= dc_status.encode();
        payload.EVSEPresentVoltage= voltage.encode();
        payload.EVSEPresentCurrent= current.encode();
        payload.EVSECurrentLimitAchieved= if current_limit {1} else {0};
        payload.EVSEVoltageLimitAchieved= if voltage_limit {1} else {0};
        payload.EVSEPowerLimitAchieved= if power_limit {1} else {0};
        payload.SAScheduleTupleID= schd_tuple_id;

        Ok(Self { payload })
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn get_id(&self) -> Result<&str, AfbError> {
        array_to_str(
            &self.payload.EVSEID.characters,
            self.payload.EVSEID.charactersLen,
        )
    }

    pub fn get_dc_status(&self) -> DcEvseStatusType {
        DcEvseStatusType::decode(self.payload.DC_EVSEStatus)
    }

    pub fn get_voltage(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEPresentVoltage)
    }

    pub fn get_current(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEPresentCurrent)
    }

    pub fn get_current_limit_reach(&self) -> bool {
        if self.payload.EVSECurrentLimitAchieved == 0 {false} else {true}
    }

    pub fn get_voltage_limit_reach(&self) -> bool {
        if self.payload.EVSEVoltageLimitAchieved == 0 {false} else {true}
    }

    pub fn get_power_limit_reach(&self) -> bool {
        if self.payload.EVSEPowerLimitAchieved == 0 {false} else {true}
    }

    pub fn get_tuple_id(&self) -> u8 {
        self.payload.SAScheduleTupleID
    }

    pub fn set_voltage_limit(&mut self, voltage: &PhysicalValue) -> &mut Self {
        self.payload.EVSEMaximumVoltageLimit= voltage.encode();
        self.payload.set_EVSEMaximumVoltageLimit_isUsed(1);
        self
    }

    pub fn get_voltage_limit(&self) -> Option<PhysicalValue> {
        if self.payload.EVSEMaximumVoltageLimit_isUsed() == 0 {
            None
        } else {
           Some(PhysicalValue::decode( self.payload.EVSEMaximumVoltageLimit))
        }
    }

    pub fn set_current_limit(&mut self, current: &PhysicalValue) -> &mut Self {
        self.payload.EVSEMaximumCurrentLimit= current.encode();
        self.payload.set_EVSEMaximumCurrentLimit_isUsed(1);
        self
    }

    pub fn get_current_limit(&self) -> Option<PhysicalValue> {
        if self.payload.EVSEMaximumCurrentLimit_isUsed() == 0 {
            None
        } else {
           Some(PhysicalValue::decode( self.payload.EVSEMaximumCurrentLimit))
        }
    }

    pub fn set_power_limit(&mut self, power: &PhysicalValue) -> &mut Self {
        self.payload.EVSEMaximumPowerLimit= power.encode();
        self.payload.set_EVSEMaximumPowerLimit_isUsed(1);
        self
    }

    pub fn get_power_limit(&self) -> Option<PhysicalValue> {
        if self.payload.EVSEMaximumPowerLimit_isUsed() == 0 {
            None
        } else {
           Some(PhysicalValue::decode( self.payload.EVSEMaximumPowerLimit))
        }
    }

    pub fn set_receipt_require(&mut self, require: bool) -> &mut Self {
        self.payload.ReceiptRequired= if require {1} else {0};
        self.payload.set_ReceiptRequired_isUsed(1);
        self
    }

    pub fn get_receipt_require(&self) -> Option<bool> {
        if self.payload.ReceiptRequired_isUsed() == 0 {
            None
        } else {
           Some(if self.payload.ReceiptRequired == 0 {false} else {true})
        }
    }

    pub fn set_meter_info (&mut self, meter: MeterInfoType) -> &mut Self {
        self.payload.MeterInfo= meter.encode();
        self.payload.set_MeterInfo_isUsed(1);
        self
    }

    pub fn get_meter_info(&mut self) -> Option<MeterInfoType> {
        if self.payload.MeterInfo_isUsed() == 0 {
            None
        } else {
            Some(MeterInfoType::decode(self.payload.MeterInfo))
        }
    }

    pub fn decode(payload: cglue::iso2_CurrentDemandResType) -> Self {
        Self { payload }
    }
    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.CurrentDemandRes = self.payload;
            exi_body.set_CurrentDemandRes_isUsed(1);
            exi_body
        };
        body
    }
}
