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

pub struct DcEvChargeParam {
    payload: cglue::iso2_DC_EVChargeParameterType,
}

impl DcEvChargeParam {
    #[track_caller]
    pub fn new(
        status: &DcEvStatusType,
        max_voltage: &PhysicalValue,
        max_current: &PhysicalValue,
    ) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_DC_EVChargeParameterType>() };

        if max_current.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "dc-ev-charge-param",
                "max_current expect: PhysicalUnit::Ampere get:{}",
                max_current.get_unit()
            );
        }

        if max_voltage.get_unit() != PhysicalUnit::Volt {
            return afb_error!(
                "dc-ev-charge-param",
                "max_voltage expect: PhysicalUnit::Volt get:{}",
                max_voltage.get_unit()
            );
        }

        payload.DC_EVStatus = status.encode();
        payload.EVMaximumCurrentLimit = max_current.encode();
        payload.EVMaximumVoltageLimit = max_voltage.encode();
        Ok(Self { payload })
    }

    pub fn get_status(&self) -> DcEvStatusType {
        DcEvStatusType::decode(self.payload.DC_EVStatus)
    }

    pub fn get_max_voltage(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVMaximumVoltageLimit)
    }

    pub fn get_max_current(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVMaximumCurrentLimit)
    }

    #[track_caller]
    pub fn set_max_power(&mut self, power_limit: &PhysicalValue) -> Result<&mut Self, AfbError> {
        if power_limit.get_unit() != PhysicalUnit::Watt {
            return afb_error!(
                "dc-ev-charge-param",
                "max_power expect: PhysicalUnit::Watt get:{}",
                power_limit.get_unit()
            );
        }
        self.payload.EVMaximumPowerLimit = power_limit.encode();
        self.payload.set_EVMaximumPowerLimit_isUsed(1);
        Ok(self)
    }

    pub fn get_max_power(&self) -> Option<PhysicalValue> {
        if self.payload.EVMaximumPowerLimit_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVMaximumPowerLimit))
        }
    }

    #[track_caller]
    pub fn set_energy_capacity(
        &mut self,
        power_limit: &PhysicalValue,
    ) -> Result<&mut Self, AfbError> {
        if power_limit.get_unit() != PhysicalUnit::Wh {
            return afb_error!(
                "dc-ev-charge-param",
                "energy_capacity expect: PhysicalUnit::Wh get:{}",
                power_limit.get_unit()
            );
        }
        self.payload.EVEnergyCapacity = power_limit.encode();
        self.payload.set_EVEnergyCapacity_isUsed(1);
        Ok(self)
    }

    pub fn get_energy_capacity(&self) -> Option<PhysicalValue> {
        if self.payload.EVEnergyCapacity_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVEnergyCapacity))
        }
    }

    pub fn set_energy_request(
        &mut self,
        power_limit: &PhysicalValue,
    ) -> Result<&mut Self, AfbError> {
        if power_limit.get_unit() != PhysicalUnit::Wh {
            return afb_error!(
                "dc-ev-charge-param",
                "power_limit expect: PhysicalUnit::Wh get:{}",
                power_limit.get_unit()
            );
        }
        self.payload.EVEnergyRequest = power_limit.encode();
        self.payload.set_EVEnergyRequest_isUsed(1);
        Ok(self)
    }

    pub fn get_energy_request(&self) -> Option<PhysicalValue> {
        if self.payload.EVEnergyRequest_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVEnergyRequest))
        }
    }

    pub fn set_departure_time(&mut self, departure_time: u32) -> &mut Self {
        self.payload.DepartureTime = departure_time;
        self.payload.set_DepartureTime_isUsed(1);
        self
    }

    pub fn get_departure_time(&self) -> Option<u32> {
        if self.payload.DepartureTime_isUsed() == 0 {
            None
        } else {
            Some(self.payload.DepartureTime)
        }
    }

    pub fn set_bulk_soc(&mut self, bulk_soc: i8) -> &mut Self {
        self.payload.BulkSOC = bulk_soc;
        self.payload.set_BulkSOC_isUsed(1);
        self
    }

    pub fn get_bulk_soc(&self) -> Option<i8> {
        if self.payload.BulkSOC_isUsed() == 0 {
            None
        } else {
            Some(self.payload.BulkSOC)
        }
    }

    pub fn set_full_soc(&mut self, full_soc: i8) -> &mut Self {
        self.payload.FullSOC = full_soc;
        self.payload.set_FullSOC_isUsed(1);
        self
    }

    pub fn get_full_soc(&self) -> Option<i8> {
        if self.payload.FullSOC_isUsed() == 0 {
            None
        } else {
            Some(self.payload.FullSOC)
        }
    }

    pub fn decode(payload: cglue::iso2_DC_EVChargeParameterType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_DC_EVChargeParameterType {
        self.payload
    }
}

pub struct AcEvChargeParam {
    payload: cglue::iso2_AC_EVChargeParameterType,
}

impl AcEvChargeParam {
    #[track_caller]
    pub fn new(
        ea_mount: &PhysicalValue,
        max_voltage: &PhysicalValue,
        max_current: &PhysicalValue,
        min_current: &PhysicalValue,
    ) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_AC_EVChargeParameterType>() };

        if max_current.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "ac-ev-charge-param",
                "max_current expect: PhysicalUnit::Ampere get:{}",
                max_current.get_unit()
            );
        }

        if min_current.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "ac-ev-charge-param",
                "min_current expect: PhysicalUnit::Ampere get:{}",
                max_current.get_unit()
            );
        }

        if max_voltage.get_unit() != PhysicalUnit::Volt {
            return afb_error!(
                "ac-ev-charge-param",
                "max_voltage expect: PhysicalUnit::Volt get:{}",
                max_current.get_unit()
            );
        }

        payload.EAmount = ea_mount.encode();
        payload.EVMaxVoltage = max_voltage.encode();
        payload.EVMaxCurrent = max_current.encode();
        payload.EVMinCurrent = min_current.encode();
        Ok(Self { payload })
    }

    pub fn get_ea_mount(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EAmount)
    }

    pub fn get_max_voltage(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVMaxVoltage)
    }

    pub fn get_max_current(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVMaxCurrent)
    }

    pub fn get_min_current(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVMinCurrent)
    }

    pub fn set_departure_time(&mut self, departure_time: u32) -> &mut Self {
        self.payload.DepartureTime = departure_time;
        self.payload.set_DepartureTime_isUsed(1);
        self
    }

    pub fn get_departure_time(&self) -> Option<u32> {
        if self.payload.DepartureTime_isUsed() == 0 {
            None
        } else {
            Some(self.payload.DepartureTime)
        }
    }

    pub fn decode(payload: cglue::iso2_AC_EVChargeParameterType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_AC_EVChargeParameterType {
        self.payload
    }
}

pub struct EvChargeParam {
    payload: cglue::iso2_EVChargeParameterType,
}

impl EvChargeParam {
    #[track_caller]
    pub fn new(ac_param: &AcEvChargeParam, dc_param: &DcEvChargeParam) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_EVChargeParameterType>() };
        payload.AC_EVChargeParameter = ac_param.encode();
        payload.DC_EVChargeParameter = dc_param.encode();
        Self { payload }
    }

    pub fn get_ac_param(&self) -> AcEvChargeParam {
        AcEvChargeParam::decode(self.payload.AC_EVChargeParameter)
    }

    pub fn get_dc_param(&self) -> DcEvChargeParam {
        DcEvChargeParam::decode(self.payload.DC_EVChargeParameter)
    }

    pub fn set_departure_time(&mut self, departure_time: u32) -> &mut Self {
        self.payload.DepartureTime = departure_time;
        self.payload.set_DepartureTime_isUsed(1);
        self
    }

    pub fn get_departure_time(&self) -> Option<u32> {
        if self.payload.DepartureTime_isUsed() == 0 {
            None
        } else {
            Some(self.payload.DepartureTime)
        }
    }

    pub fn decode(payload: cglue::iso2_EVChargeParameterType) -> Self {
        Self { payload: payload }
    }

    pub fn encode(&self) -> cglue::iso2_EVChargeParameterType {
        self.payload
    }
}

pub struct ParamDiscoveryRequest {
    payload: cglue::iso2_ChargeParameterDiscoveryReqType,
}

impl ParamDiscoveryRequest {
    pub fn new(transfer: EngyTransfertMode) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_ChargeParameterDiscoveryReqType>() };
        payload.RequestedEnergyTransferMode = transfer as u32;
        Self { payload }
    }

    pub fn get_transfert_energy_mode(&self) -> EngyTransfertMode {
        EngyTransfertMode::from_u32(self.payload.RequestedEnergyTransferMode)
    }

    pub fn set_max_schedule_tuple(&mut self, max_entries: u16) -> &mut Self {
        self.payload.MaxEntriesSAScheduleTuple = max_entries;
        self.payload.set_MaxEntriesSAScheduleTuple_isUsed(1);
        self
    }

    pub fn get_max_schedule_tuple(&self) -> Option<u16> {
        if self.payload.MaxEntriesSAScheduleTuple_isUsed() == 0 {
            None
        } else {
            Some(self.payload.MaxEntriesSAScheduleTuple)
        }
    }

    #[track_caller]
    pub fn set_ac_charge_param(
        &mut self,
        ac_params: &AcEvChargeParam,
    ) -> Result<&mut Self, AfbError> {
        if self.payload.DC_EVChargeParameter_isUsed() != 0
            || self.payload.EVChargeParameter_isUsed() != 0
        {
            return afb_error!(
                "param-discovery-request",
                "fail set_ac_charge_param because dc already set"
            );
        }

        match EngyTransfertMode::from_u32(self.payload.RequestedEnergyTransferMode) {
            EngyTransfertMode::AcSinglePhase => {}
            EngyTransfertMode::AcTreePhase => {}
            _ => {
                return afb_error!(
                    "param-discovery-request",
                    "set_ac_charge_param incompatible with current RequestedEnergyTransferMode"
                )
            }
        }
        self.payload.AC_EVChargeParameter = ac_params.encode();
        self.payload.set_AC_EVChargeParameter_isUsed(1);
        Ok(self)
    }

    pub fn get_ac_charge_param(&self) -> Option<AcEvChargeParam> {
        if self.payload.AC_EVChargeParameter_isUsed() == 0 {
            None
        } else {
            Some(AcEvChargeParam::decode(self.payload.AC_EVChargeParameter))
        }
    }

    #[track_caller]
    pub fn set_dc_charge_param(
        &mut self,
        dc_params: &DcEvChargeParam,
    ) -> Result<&mut Self, AfbError> {
        if self.payload.AC_EVChargeParameter_isUsed() != 0
            || self.payload.EVChargeParameter_isUsed() != 0
        {
            return afb_error!(
                "param-discovery-request",
                "fail set_dc_charge_param because ac already set"
            );
        }
        match EngyTransfertMode::from_u32(self.payload.RequestedEnergyTransferMode) {
            EngyTransfertMode::DcBasic => {}
            EngyTransfertMode::DcExtended => {}
            EngyTransfertMode::DcCombo => {}
            EngyTransfertMode::DcUnique => {}
            _ => {
                return afb_error!(
                    "param-discovery-request",
                    "set_dc_charge_param incompatible with current RequestedEnergyTransferMode"
                )
            }
        }

        self.payload.DC_EVChargeParameter = dc_params.encode();
        self.payload.set_DC_EVChargeParameter_isUsed(1);
        Ok(self)
    }

    pub fn get_dc_charge_param(&self) -> Option<DcEvChargeParam> {
        if self.payload.DC_EVChargeParameter_isUsed() == 0 {
            None
        } else {
            Some(DcEvChargeParam::decode(self.payload.DC_EVChargeParameter))
        }
    }

    pub fn get_ev_charge_param(&self) -> Option<EvChargeParam> {
        if self.payload.EVChargeParameter_isUsed() == 0 {
            None
        } else {
            Some(EvChargeParam::decode(self.payload.EVChargeParameter))
        }
    }

    #[track_caller]
    pub fn set_ev_charge_param(
        &mut self,
        charge_params: &EvChargeParam,
    ) -> Result<&mut Self, AfbError> {
        if self.payload.DC_EVChargeParameter_isUsed() != 0
            || self.payload.AC_EVChargeParameter_isUsed() != 0
        {
            return afb_error!(
                "param-discovery-request",
                "fail set_charge_param bacause ac|dc already set"
            );
        }
        self.payload.EVChargeParameter = charge_params.encode();
        self.payload.set_EVChargeParameter_isUsed(1);
        Ok(self)
    }

    pub fn decode(payload: cglue::iso2_ChargeParameterDiscoveryReqType) -> Self {
        Self { payload: payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.ChargeParameterDiscoveryReq = self.payload;
            exi_body.set_ChargeParameterDiscoveryReq_isUsed(1);
            exi_body
        };
        body
    }
}

pub struct SalesTariff {
    payload: cglue::iso2_SalesTariffType,
}

impl SalesTariff {
    pub fn new(tariff_id: u8) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_SalesTariffType>() };
        payload.SalesTariffID = tariff_id;
        Self { payload }
    }

    pub fn get_tariff_id(&self) -> u8 {
        self.payload.SalesTariffID
    }

    #[track_caller]
    pub fn set_id(&mut self, text: &str) -> Result<&mut Self, AfbError> {
        self.payload.Id.charactersLen = str_to_array(
            text,
            &mut self.payload.Id.characters,
            cglue::iso2_Id_CHARACTER_SIZE,
        )?;
        if self.payload.Id.charactersLen > 0 {
            self.payload.set_Id_isUsed(1);
        }
        Ok(self)
    }

    pub fn get_id(&self) -> Option<&str> {
        if self.payload.Id_isUsed() == 0 {
            None
        } else {
            // note if string is not UTF8 compatible return none
            array_to_str(&self.payload.Id.characters, self.payload.Id.charactersLen).ok()
        }
    }

    #[track_caller]
    pub fn set_description(&mut self, text: &str) -> Result<&mut Self, AfbError> {
        self.payload.SalesTariffDescription.charactersLen = str_to_array(
            text,
            &mut self.payload.SalesTariffDescription.characters,
            cglue::iso2_SalesTariffDescription_CHARACTER_SIZE,
        )?;
        if self.payload.SalesTariffDescription.charactersLen > 0 {
            self.payload.set_SalesTariffDescription_isUsed(1);
        }
        Ok(self)
    }

    pub fn get_description(&self) -> Option<&str> {
        if self.payload.SalesTariffDescription_isUsed() == 0 {
            None
        } else {
            // if string not UTF8 return None
            array_to_str(
                &self.payload.SalesTariffDescription.characters,
                self.payload.SalesTariffDescription.charactersLen,
            )
            .ok()
        }
    }

    pub fn set_tariff_level(&mut self, level: u8) -> &mut Self {
        self.payload.NumEPriceLevels = level;
        self.payload.set_NumEPriceLevels_isUsed(1);
        self
    }

    pub fn get_tariff_level(&self) -> Option<u8> {
        if self.payload.SalesTariffDescription_isUsed() == 0 {
            None
        } else {
            Some(self.payload.NumEPriceLevels)
        }
    }

    #[track_caller]
    pub fn add_entry(&mut self, entry: &SaleTariffEntry) -> Result<&mut Self, AfbError> {
        let idx = self.payload.SalesTariffEntry.arrayLen;
        if idx == cglue::iso2_SalesTariffEntryType_12_ARRAY_SIZE as u16 {
            return afb_error!("iso2-tarrif-entry", "fail to add tariff entry (array full)");
        }
        self.payload.SalesTariffEntry.array[idx as usize] = entry.encode();
        self.payload.SalesTariffEntry.arrayLen = idx + 1;
        Ok(self)
    }

    pub fn get_entries(&self) -> Vec<SaleTariffEntry> {
        let mut entries = Vec::new();
        for idx in 0..self.payload.SalesTariffEntry.arrayLen as usize {
            entries.push(SaleTariffEntry::decode(
                self.payload.SalesTariffEntry.array[idx],
            ));
        }
        entries
    }

    pub fn decode(payload: cglue::iso2_SalesTariffType) -> Self {
        Self { payload: payload }
    }

    pub fn encode(&self) -> cglue::iso2_SalesTariffType {
        self.payload
    }
}

pub struct RelativeTimeInterval {
    payload: cglue::iso2_RelativeTimeIntervalType,
}

impl RelativeTimeInterval {
    pub fn new(start: u32) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_RelativeTimeIntervalType>() };
        payload.start = start;
        Self { payload }
    }

    pub fn get_start(&self) -> u32 {
        self.payload.start
    }

    pub fn set_duration(&mut self, duration: u32) -> &mut Self {
        self.payload.duration = duration;
        self.payload.set_duration_isUsed(1);
        self
    }

    pub fn get_duration(&self) -> Option<u32> {
        if self.payload.duration_isUsed() == 0 {
            None
        } else {
            Some(self.payload.duration)
        }
    }

    pub fn decode(payload: cglue::iso2_RelativeTimeIntervalType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_RelativeTimeIntervalType {
        self.payload
    }
}

pub struct PMaxScheduleEntry {
    payload: cglue::iso2_PMaxScheduleEntryType,
}

impl PMaxScheduleEntry {
    pub fn new(pmax: &PhysicalValue) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_PMaxScheduleEntryType>() };
        payload.PMax = pmax.encode();
        Self { payload }
    }

    pub fn get_pmax(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.PMax)
    }

    pub fn set_relative_time_interval(
        &mut self,
        relative_time: &RelativeTimeInterval,
    ) -> &mut Self {
        self.payload.RelativeTimeInterval = relative_time.encode();
        self.payload.set_RelativeTimeInterval_isUsed(1);
        self
    }

    pub fn get_relative_time_interval(&self) -> Option<RelativeTimeInterval> {
        if self.payload.RelativeTimeInterval_isUsed() == 0 {
            None
        } else {
            Some(RelativeTimeInterval::decode(
                self.payload.RelativeTimeInterval,
            ))
        }
    }

    pub fn set_time_interval(&mut self, _unused: i32) -> &mut Self {
        self.payload.TimeInterval._unused = _unused;
        self.payload.set_TimeInterval_isUsed(1);
        self
    }

    pub fn get_time_interval(&mut self, _unused: i32) -> i32 {
        if self.payload.TimeInterval_isUsed() != 0 {
            self.payload.TimeInterval._unused
        } else {
            0
        }
    }

    pub fn decode(payload: cglue::iso2_PMaxScheduleEntryType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_PMaxScheduleEntryType {
        self.payload
    }
}

pub struct CostType {
    payload: cglue::iso2_CostType,
}

impl CostType {
    pub fn new(kind: CostKind, amount: u32, multiplier: i8) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_CostType>() };
        payload.costKind = kind as u32;
        payload.amount = amount;
        if multiplier != 0 {
            payload.amountMultiplier = multiplier;
            payload.set_amountMultiplier_isUsed(1);
        }
        Self { payload }
    }

    pub fn get_kind(&self) -> CostKind {
        CostKind::from_u32(self.payload.costKind)
    }

    pub fn get_amount(&self) -> u32 {
        self.payload.amount
    }

    pub fn get_multiplier(&self) -> Option<i8> {
        if self.payload.amountMultiplier_isUsed() == 0 {
            None
        } else {
            Some(self.payload.amountMultiplier)
        }
    }

    pub fn decode(payload: cglue::iso2_CostType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_CostType {
        self.payload
    }
}

pub struct ConsumptionCost {
    payload: cglue::iso2_ConsumptionCostType,
}

impl ConsumptionCost {
    pub fn new(start_value: PhysicalValue) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_ConsumptionCostType>() };
        payload.startValue = start_value.encode();
        Self { payload }
    }

    #[track_caller]
    pub fn add_cost(&mut self, cost: CostType) -> Result<&mut Self, AfbError> {
        let idx = self.payload.Cost.arrayLen;
        if idx == cglue::iso2_CostType_3_ARRAY_SIZE as u16 {
            return afb_error!("consumption-add-cost", "Fail to add cost (array full");
        }
        self.payload.Cost.array[idx as usize] = cost.encode();
        self.payload.Cost.arrayLen = idx + 1;
        Ok(self)
    }

    pub fn get_costs(&self) -> Vec<CostType> {
        let mut response = Vec::new();
        for idx in 0..self.payload.Cost.arrayLen as usize {
            response.push(CostType::decode(self.payload.Cost.array[idx]));
        }
        response
    }

    pub fn decode(payload: cglue::iso2_ConsumptionCostType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_ConsumptionCostType {
        self.payload
    }
}

pub struct SaleTariffEntry {
    payload: cglue::iso2_SalesTariffEntryType,
}

impl SaleTariffEntry {
    pub fn new() -> Self {
        let payload = unsafe { mem::zeroed::<cglue::iso2_SalesTariffEntryType>() };
        Self { payload }
    }

    pub fn set_start(&mut self, start: u32) -> &mut Self {
        self.payload.set_RelativeTimeInterval_isUsed(1);
        self.payload.RelativeTimeInterval.start = start;
        self
    }

    pub fn get_start(&self) -> Option<u32> {
        if self.payload.RelativeTimeInterval_isUsed() == 0 {
            None
        } else {
            Some(self.payload.RelativeTimeInterval.start)
        }
    }

    pub fn set_price_level(&mut self, tariff_level: u8) -> &mut Self {
        self.payload.set_EPriceLevel_isUsed(1);
        self.payload.EPriceLevel = tariff_level;
        self
    }

    pub fn get_price_level(&self) -> Option<u8> {
        if self.payload.EPriceLevel_isUsed() == 0 {
            None
        } else {
            Some(self.payload.EPriceLevel)
        }
    }

    pub fn set_duration(&mut self, duration: u32) -> &mut Self {
        self.payload.set_RelativeTimeInterval_isUsed(1);
        self.payload.RelativeTimeInterval.duration = duration;
        self
    }

    pub fn get_duration(&self) -> Option<u32> {
        if self.payload.RelativeTimeInterval_isUsed() == 0 {
            None
        } else {
            Some(self.payload.RelativeTimeInterval.duration)
        }
    }

    #[track_caller]
    pub fn add_comsumption_cost(&mut self, cost: ConsumptionCost) -> Result<&mut Self, AfbError> {
        let idx = self.payload.ConsumptionCost.arrayLen;
        if idx == cglue::iso2_ConsumptionCostType_3_ARRAY_SIZE as u16 {
            return afb_error!(
                "sale-consumption-entry",
                "fail to add consumption cost (array full)"
            );
        }
        self.payload.ConsumptionCost.array[idx as usize] = cost.encode();
        self.payload.ConsumptionCost.arrayLen = idx + 1;

        Ok(self)
    }

    pub fn decode(payload: cglue::iso2_SalesTariffEntryType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_SalesTariffEntryType {
        self.payload
    }
}
pub struct SasScheduleTuple {
    payload: cglue::iso2_SAScheduleTupleType,
}

impl SasScheduleTuple {
    pub fn new(description: u8) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_SAScheduleTupleType>() };
        payload.SAScheduleTupleID = description;
        Self { payload }
    }

    pub fn get_description(&self) -> u8 {
        self.payload.SAScheduleTupleID
    }

    #[track_caller]
    pub fn add_pmax(&mut self, pmax: &PMaxScheduleEntry) -> Result<&mut Self, AfbError> {
        let idx = self.payload.PMaxSchedule.PMaxScheduleEntry.arrayLen;
        if idx == cglue::iso2_PMaxScheduleEntryType_12_ARRAY_SIZE as u16 {
            return afb_error!("iso2-schedule-tuple", "fail to add tuple (array full)");
        }
        self.payload.PMaxSchedule.PMaxScheduleEntry.array[idx as usize] = pmax.encode();
        self.payload.PMaxSchedule.PMaxScheduleEntry.arrayLen = idx + 1;
        Ok(self)
    }

    pub fn get_pmaxs(&self) -> Vec<PMaxScheduleEntry> {
        let mut response = Vec::new();
        for idx in 0..self.payload.PMaxSchedule.PMaxScheduleEntry.arrayLen as usize {
            response.push(PMaxScheduleEntry::decode(
                self.payload.PMaxSchedule.PMaxScheduleEntry.array[idx as usize],
            ));
        }
        response
    }

    pub fn set_tariff(&mut self, tariff: &SalesTariff) -> &mut Self {
        self.payload.SalesTariff = tariff.encode();
        self.payload.set_SalesTariff_isUsed(1);
        self
    }

    pub fn get_tariff(&self) -> Option<SalesTariff> {
        if self.payload.SalesTariff_isUsed() == 0 {
            None
        } else {
            Some(SalesTariff::decode(self.payload.SalesTariff))
        }
    }

    pub fn decode(payload: cglue::iso2_SAScheduleTupleType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_SAScheduleTupleType {
        self.payload
    }
}

pub struct AcEvseChargeParam {
    payload: cglue::iso2_AC_EVSEChargeParameterType,
}

impl AcEvseChargeParam {
    pub fn new(
        status: &AcEvseStatusType,
        max_current: &PhysicalValue,
        nominate_voltage: &PhysicalValue,
    ) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_AC_EVSEChargeParameterType>() };

        payload.AC_EVSEStatus = status.encode();
        payload.EVSENominalVoltage = nominate_voltage.encode();
        payload.EVSEMaxCurrent = max_current.encode();

        Self { payload }
    }

    pub fn get_status(&self) -> AcEvseStatusType {
        AcEvseStatusType::decode(self.payload.AC_EVSEStatus)
    }

    pub fn get_nominate_voltage(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSENominalVoltage)
    }

    pub fn get_max_current(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEMaxCurrent)
    }

    pub fn decode(payload: cglue::iso2_AC_EVSEChargeParameterType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_AC_EVSEChargeParameterType {
        self.payload
    }
}

pub struct DcEvseChargeParam {
    payload: cglue::iso2_DC_EVSEChargeParameterType,
}

impl DcEvseChargeParam {
    #[track_caller]
    pub fn new(
        status: &DcEvseStatusType,
        max_voltage: &PhysicalValue,
        min_voltage: &PhysicalValue,
        max_current: &PhysicalValue,
        min_current: &PhysicalValue,
        max_power: &PhysicalValue,
        current_ripple: &PhysicalValue,
    ) -> Result<Self, AfbError> {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_DC_EVSEChargeParameterType>() };

        if max_voltage.get_unit() != PhysicalUnit::Volt {
            return afb_error!(
                "dc-charge-param",
                "max_voltage expect: PhysicalUnit::Volt get:{}",
                max_voltage.get_unit()
            );
        }
        if min_voltage.get_unit() != PhysicalUnit::Volt {
            return afb_error!(
                "dc-charge-param",
                "min_voltage expect: PhysicalUnit::Volt get:{}",
                min_voltage.get_unit()
            );
        }
        if max_current.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "pre-charge-req",
                "max_current expect: PhysicalUnit::Ampere get:{}",
                max_current.get_unit()
            );
        }
        if min_current.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "pre-charge-req",
                "min_current expect: PhysicalUnit::Ampere get:{}",
                min_current.get_unit()
            );
        }
        if max_power.get_unit() != PhysicalUnit::Watt {
            return afb_error!(
                "pre-charge-req",
                "max_power expect: PhysicalUnit::Watt get:{}",
                max_power.get_unit()
            );
        }
        if current_ripple.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "pre-charge-req",
                "current_ripple expect: PhysicalUnit::Ampere get:{}",
                current_ripple.get_unit()
            );
        }

        payload.DC_EVSEStatus = status.encode();
        payload.EVSEMaximumCurrentLimit = max_current.encode();
        payload.EVSEMaximumVoltageLimit = max_voltage.encode();
        payload.EVSEMaximumPowerLimit = max_power.encode();
        payload.EVSEMinimumCurrentLimit = min_current.encode();
        payload.EVSEMinimumVoltageLimit = min_voltage.encode();
        payload.EVSEPeakCurrentRipple = current_ripple.encode();

        Ok(Self { payload })
    }

    pub fn get_status(&self) -> DcEvseStatusType {
        DcEvseStatusType::decode(self.payload.DC_EVSEStatus)
    }

    pub fn get_max_voltage(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEMaximumVoltageLimit)
    }

    pub fn get_min_voltage(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEMinimumVoltageLimit)
    }

    pub fn get_max_current(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEMaximumCurrentLimit)
    }

    pub fn get_min_current(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEMinimumCurrentLimit)
    }
    pub fn get_max_power(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEMaximumPowerLimit)
    }

    pub fn set_regul_tolerance(&mut self, tolerance: &PhysicalValue) -> Result<&mut Self, AfbError> {
        if tolerance.get_unit() != PhysicalUnit::Ampere {
            return afb_error!(
                "dc-ev-charge-param",
                "regul_tolerance expect: PhysicalUnit::Ampere get:{}",
                tolerance.get_unit()
            );
        }
        self.payload.EVSECurrentRegulationTolerance = tolerance.encode();
        self.payload.set_EVSECurrentRegulationTolerance_isUsed(1);
        Ok(self)
    }

    pub fn get_regul_tolerance(&self) -> Option<PhysicalValue> {
        if self.payload.EVSECurrentRegulationTolerance_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(
                self.payload.EVSECurrentRegulationTolerance,
            ))
        }
    }

    pub fn set_energy_to_deliver(&mut self, energy: &PhysicalValue) -> Result<&mut Self, AfbError> {
        if energy.get_unit() != PhysicalUnit::Wh {
            return afb_error!(
                "dc-ev-charge-param",
                "energy_to_deliver expect: PhysicalUnit::Wh get:{}",
                energy.get_unit()
            );
        }
        self.payload.EVSEEnergyToBeDelivered = energy.encode();
        self.payload.set_EVSEEnergyToBeDelivered_isUsed(1);
        Ok(self)
    }

    pub fn get_energy_to_deliver(&self) -> Option<PhysicalValue> {
        if self.payload.EVSEEnergyToBeDelivered_isUsed() == 0 {
            None
        } else {
            Some(PhysicalValue::decode(self.payload.EVSEEnergyToBeDelivered))
        }
    }

    pub fn get_peak_current_ripple(&self) -> PhysicalValue {
        PhysicalValue::decode(self.payload.EVSEPeakCurrentRipple)
    }

    pub fn decode(payload: cglue::iso2_DC_EVSEChargeParameterType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> cglue::iso2_DC_EVSEChargeParameterType {
        self.payload
    }
}

pub struct ParamDiscoveryResponse {
    // note: EVSEChargeParameter unused
    payload: cglue::iso2_ChargeParameterDiscoveryResType,
}

impl ParamDiscoveryResponse {
    pub fn new(code: ResponseCode, processing: EvseProcessing) -> Self {
        let mut payload = unsafe { mem::zeroed::<cglue::iso2_ChargeParameterDiscoveryResType>() };
        payload.ResponseCode = code as u32;
        payload.EVSEProcessing = processing as u32;
        Self { payload }
    }

    pub fn get_rcode(&self) -> ResponseCode {
        ResponseCode::from_u32(self.payload.ResponseCode)
    }

    pub fn get_processing(&self) -> EvseProcessing {
        EvseProcessing::from_u32(self.payload.EVSEProcessing)
    }

    pub fn set_schedules(&mut self, unused: i32) -> &mut Self {
        self.payload.set_SASchedules_isUsed(1);
        self.payload.SASchedules._unused = unused;
        self
    }

    pub fn get_schedules(&self) -> Option<i32> {
        if self.payload.SASchedules_isUsed() == 0 {
            None
        } else {
            Some(self.payload.SASchedules._unused)
        }
    }

    pub fn set_evse_charge_param(&mut self, unused: i32) -> &mut Self {
        self.payload.set_EVSEChargeParameter_isUsed(1);
        self.payload.SASchedules._unused = unused;
        self
    }

    pub fn get_evse_charge_param(&self) -> Option<i32> {
        if self.payload.EVSEChargeParameter_isUsed() == 0 {
            None
        } else {
            Some(self.payload.SASchedules._unused)
        }
    }

    #[track_caller]
    pub fn add_schedule_tuple(&mut self, tuple: &SasScheduleTuple) -> Result<&mut Self, AfbError> {
        let idx = self.payload.SAScheduleList.SAScheduleTuple.arrayLen;
        if idx == cglue::iso2_SAScheduleTupleType_3_ARRAY_SIZE as u16 {
            return afb_error!(
                "iso2-param-disco-res",
                "fail to add schedule_tuple array full"
            );
        }
        self.payload.SAScheduleList.SAScheduleTuple.array[idx as usize] = tuple.encode();
        self.payload.SAScheduleList.SAScheduleTuple.arrayLen = idx + 1;
        self.payload.set_SAScheduleList_isUsed(1);
        Ok(self)
    }

    pub fn get_schedule_tuples(&self) -> Vec<SasScheduleTuple> {
        let mut tuples = Vec::new();
        for idx in 0..self.payload.SAScheduleList.SAScheduleTuple.arrayLen as usize {
            tuples.push(SasScheduleTuple::decode(
                self.payload.SAScheduleList.SAScheduleTuple.array[idx],
            ));
        }
        tuples
    }

    pub fn set_evse_dc_charge_param(&mut self, dc_charge_param: &DcEvseChargeParam) -> &mut Self {
        self.payload.DC_EVSEChargeParameter = dc_charge_param.encode();
        self.payload.set_DC_EVSEChargeParameter_isUsed(1);
        self
    }

    pub fn get_evse_dc_charge_param(&self) -> Option<DcEvseChargeParam> {
        if self.payload.DC_EVSEChargeParameter_isUsed() == 0 {
            None
        } else {
            Some(DcEvseChargeParam::decode(
                self.payload.DC_EVSEChargeParameter,
            ))
        }
    }

    pub fn set_evse_ac_charge_param(&mut self, ac_charge_param: &AcEvseChargeParam) -> &mut Self {
        self.payload.AC_EVSEChargeParameter = ac_charge_param.encode();
        self.payload.set_AC_EVSEChargeParameter_isUsed(1);
        self
    }

    pub fn get_evse_ac_charge_param(&self) -> Option<AcEvseChargeParam> {
        if self.payload.AC_EVSEChargeParameter_isUsed() == 0 {
            None
        } else {
            Some(AcEvseChargeParam::decode(
                self.payload.AC_EVSEChargeParameter,
            ))
        }
    }

    pub fn decode(payload: cglue::iso2_ChargeParameterDiscoveryResType) -> Self {
        Self { payload }
    }

    pub fn encode(&self) -> Iso2BodyType {
        let body = unsafe {
            let mut exi_body = mem::zeroed::<Iso2BodyType>();
            exi_body.__bindgen_anon_1.ChargeParameterDiscoveryRes = self.payload;
            exi_body.set_ChargeParameterDiscoveryRes_isUsed(1);
            exi_body
        };
        body
    }
}
