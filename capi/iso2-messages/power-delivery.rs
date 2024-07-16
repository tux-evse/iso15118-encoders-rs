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

#[derive(Clone, Copy)]
pub struct ChargingProfileEntry {
    payload: cglue::iso2_ProfileEntryType,
}

impl ChargingProfileEntry {
    pub fn new(
        start: u32,
        power_max: &PhysicalValue,
    ) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_ProfileEntryType>() };
        payload.ChargingProfileEntryStart = start;

        if power_max.get_unit() != PhysicalUnit::Watt {
            return afb_error!(
                "chargin-profile-entry",
                "charging profile require PhysicalUnit::Watt get {}",
                power_max.get_unit()
            );
        }
        payload.ChargingProfileEntryMaxPower = power_max.encode();
        Ok(Self { payload })
    }

    pub fn get_start(&self) -> u32 {
        self.payload.ChargingProfileEntryStart
    }
    pub fn get_power_max(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.ChargingProfileEntryMaxPower)
    }

    pub fn set_phases_used(&mut self, phases: i8) -> &mut Self {
        self.payload.ChargingProfileEntryMaxNumberOfPhasesInUse = phases;
        self.payload
            .set_ChargingProfileEntryMaxNumberOfPhasesInUse_isUsed(1);
        self
    }

    pub fn get_phases_used(&self) -> Option<i8> {
        if self
            .payload
            .ChargingProfileEntryMaxNumberOfPhasesInUse_isUsed()
            == 0
        {
            None
        } else {
            Some(self.payload.ChargingProfileEntryMaxNumberOfPhasesInUse)
        }
    }

    pub fn decode(payload: cglue::iso2_ProfileEntryType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_ProfileEntryType {
        self.payload
    }
}

#[derive(Clone, Copy)]
pub struct DcEvPowerDeliveryParam {
    payload: cglue::iso2_DC_EVPowerDeliveryParameterType,
}

impl DcEvPowerDeliveryParam {
    pub fn new(status: &DcEvStatusType, charge_complete: bool) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_DC_EVPowerDeliveryParameterType>() };
        payload.DC_EVStatus = status.encode();
        if charge_complete {
            payload.ChargingComplete = 1;
        }
        Self { payload }
    }

    pub fn get_status(&self) -> DcEvStatusType {
        DcEvStatusType::decode(self.payload.DC_EVStatus)
    }

    pub fn get_charge_complete(&self) -> bool {
        if self.payload.ChargingComplete == 0 {
            false
        } else {
            true
        }
    }

    pub fn set_bulk_complete(&mut self, bulk_completed: bool) -> &mut Self {
        if bulk_completed {
            self.payload.BulkChargingComplete = 1;
        }
        self.payload.set_BulkChargingComplete_isUsed(1);
        self
    }

    pub fn get_bulk_complete(&self) -> Option<bool> {
        if self.payload.BulkChargingComplete_isUsed() == 0 {
            None
        } else {
            if self.payload.BulkChargingComplete == 0 {
                Some(false)
            } else {
                Some(true)
            }
        }
    }

    pub fn decode(payload: cglue::iso2_DC_EVPowerDeliveryParameterType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_DC_EVPowerDeliveryParameterType {
        self.payload
    }
}

#[derive(Clone)]
pub struct PowerDeliveryRequest {
    payload: cglue::iso2_PowerDeliveryReqType,
}
impl PowerDeliveryRequest {
    pub fn new(progress: ChargeProgress, schedule_id: u8) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_PowerDeliveryReqType>() };
        payload.ChargeProgress = progress as u32;
        payload.SAScheduleTupleID = schedule_id;
        Self { payload }
    }

    pub fn get_progress(&self) -> ChargeProgress {
        ChargeProgress::from_u32(self.payload.ChargeProgress)
    }

    pub fn get_schedule_id(&self) -> u8 {
        self.payload.SAScheduleTupleID
    }

    pub fn add_charging_profile(
        &mut self,
        entry: &ChargingProfileEntry,
    ) -> Result<&mut Self, AfbError> {
        let idx = self.payload.ChargingProfile.ProfileEntry.arrayLen;
        if idx == cglue::iso2_ProfileEntryType_24_ARRAY_SIZE as u16 {
            return afb_error!(
                "iso2-power-profile",
                "fail adding charging profile (array full)"
            );
        }
        self.payload.ChargingProfile.ProfileEntry.array[idx as usize] = entry.encode();
        self.payload.ChargingProfile.ProfileEntry.arrayLen = idx + 1;
        self.payload.set_ChargingProfile_isUsed(1);
        Ok(self)
    }

    pub fn get_charging_profiles(&self) -> Vec<ChargingProfileEntry> {
        let mut response = Vec::new();
        for idx in 0..self.payload.ChargingProfile.ProfileEntry.arrayLen as usize {
            response.push(ChargingProfileEntry::decode(
                self.payload.ChargingProfile.ProfileEntry.array[idx as usize],
            ));
        }
        response
    }

    pub fn set_dc_delivery_params(
        &mut self,
        params: &DcEvPowerDeliveryParam,
    ) -> Result<&mut Self, AfbError> {
        self.payload.DC_EVPowerDeliveryParameter = params.encode();
        self.payload.set_DC_EVPowerDeliveryParameter_isUsed(1);

        Ok(self)
    }

    pub fn get_dc_delivery_params(&self) -> Option<DcEvPowerDeliveryParam> {
        if self.payload.DC_EVPowerDeliveryParameter_isUsed() == 0 {
            None
        } else {
            Some(DcEvPowerDeliveryParam::decode(
                self.payload.DC_EVPowerDeliveryParameter,
            ))
        }
    }

    pub fn set_ev_delivery_params(&mut self, unused: i32) -> &mut Self {
        // unused attached data iso2_EVPowerDeliveryParameterType
        self.payload.set_EVPowerDeliveryParameter_isUsed(1);
        self.payload.EVPowerDeliveryParameter._unused = unused;
        self
    }

    pub fn get_ev_delivery_params(&self) -> Option<i32> {
        if self.payload.EVPowerDeliveryParameter_isUsed() == 0 {
            None
        } else {
            Some(self.payload.EVPowerDeliveryParameter._unused)
        }
    }

    pub fn decode(payload: cglue::iso2_PowerDeliveryReqType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.PowerDeliveryReq = self.payload;
            exi_body.set_PowerDeliveryReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct PowerDeliveryResponse {
    payload: cglue::iso2_PowerDeliveryResType,
}

impl PowerDeliveryResponse {
    pub fn new(rcode: ResponseCode) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_PowerDeliveryResType>() };

        payload.ResponseCode = rcode as u32;
        Self { payload }
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn set_ac_evse_status(&mut self, status: &AcEvseStatusType) -> Result<&mut Self, AfbError> {
        if self.payload.DC_EVSEStatus_isUsed() != 0 {
            return afb_error!("power-delivery-res", "cannot set both AC & DC status",);
        }
        self.payload.AC_EVSEStatus = status.encode();
        self.payload.set_AC_EVSEStatus_isUsed(1);
        Ok(self)
    }

    pub fn get_ac_evse_status(&self) -> Option<AcEvseStatusType> {
        if self.payload.AC_EVSEStatus_isUsed() == 0 {
            None
        } else {
            Some(AcEvseStatusType::decode(self.payload.AC_EVSEStatus))
        }
    }

    pub fn set_dc_evse_status(&mut self, status: &DcEvseStatusType) -> Result<&mut Self, AfbError> {
        if self.payload.AC_EVSEStatus_isUsed() != 0 {
            return afb_error!("power-delivery-res", "cannot set both AC & DC status",);
        }
        self.payload.DC_EVSEStatus = status.encode();
        self.payload.set_DC_EVSEStatus_isUsed(1);
        Ok(self)
    }

    pub fn get_dc_evse_status(&self) -> Option<DcEvseStatusType> {
        if self.payload.DC_EVSEStatus_isUsed() == 0 {
            None
        } else {
            Some(DcEvseStatusType::decode(self.payload.DC_EVSEStatus))
        }
    }

    pub fn decode(payload: cglue::iso2_PowerDeliveryResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.PowerDeliveryRes = self.payload;
            exi_body.set_PowerDeliveryRes_isUsed(1);
            exi_body
        };
        body
    }
}
